use crate::constants::model;
use crate::error::AppError;
use futures_util::StreamExt;
use reqwest::Client;
use std::path::PathBuf;
use tauri::{Emitter, Manager};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

pub struct ModelDownloader {
    client: Client,
}

impl ModelDownloader {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_model_dir(app: &tauri::AppHandle) -> Result<PathBuf, AppError> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| AppError::DownloadError(e.to_string()))?;
        Ok(data_dir.join("models"))
    }

    pub fn get_model_path_by_id(app: &tauri::AppHandle, model_id: &str) -> Result<PathBuf, AppError> {
        let info = model::find_model(model_id)
            .ok_or_else(|| AppError::DownloadError(format!("Unknown model: {}", model_id)))?;
        let model_dir = Self::get_model_dir(app)?;
        Ok(model_dir.join(info.filename))
    }

    pub async fn check_model_exists(app: &tauri::AppHandle) -> Result<bool, AppError> {
        let model_dir = Self::get_model_dir(app)?;
        if !model_dir.exists() {
            return Ok(false);
        }
        for info in model::AVAILABLE_MODELS {
            let path = model_dir.join(info.filename);
            if path.exists() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn download_model_by_id(
        &self,
        app: &tauri::AppHandle,
        model_id: &str,
    ) -> Result<PathBuf, AppError> {
        let info = model::find_model(model_id)
            .ok_or_else(|| AppError::DownloadError(format!("Unknown model: {}", model_id)))?;

        let model_dir = Self::get_model_dir(app)?;
        let model_path = model_dir.join(info.filename);

        fs::create_dir_all(&model_dir)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to create directory: {}", e)))?;

        let url = info.download_url();
        let _ = app.emit(
            "model-download-status",
            serde_json::json!({ "status": "started", "model_id": model_id }),
        );

        tracing::info!("[model] Starting download of '{}' from: {}", info.name, url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let _ = app.emit(
                "model-download-status",
                serde_json::json!({ "status": "error", "model_id": model_id, "message": format!("HTTP {}", status) }),
            );
            return Err(AppError::DownloadError(format!(
                "Failed to download model: HTTP {}",
                status
            )));
        }

        let total_size = response.content_length().unwrap_or(info.size_bytes);
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        let temp_path = model_path.with_extension("bin.tmp");
        let mut file = File::create(&temp_path)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to create file: {}", e)))?;

        while let Some(chunk) = stream.next().await {
            let chunk =
                chunk.map_err(|e| AppError::NetworkError(format!("Download interrupted: {}", e)))?;

            file.write_all(&chunk)
                .await
                .map_err(|e| AppError::DownloadError(format!("Failed to write: {}", e)))?;

            downloaded += chunk.len() as u64;
            let progress = (downloaded as f64 / total_size as f64 * 100.0).min(100.0);

            let _ = app.emit(
                "model-download-progress",
                serde_json::json!({
                    "model_id": model_id,
                    "progress": progress,
                    "downloaded": downloaded,
                    "total": total_size
                }),
            );

            if downloaded % (10 * 1024 * 1024) < chunk.len() as u64 {
                tracing::debug!(
                    "[model] Downloaded {:.1}MB / {:.1}MB ({:.1}%)",
                    downloaded as f64 / 1024.0 / 1024.0,
                    total_size as f64 / 1024.0 / 1024.0,
                    progress
                );
            }
        }

        file.flush()
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to flush: {}", e)))?;
        drop(file);

        Self::verify_download(&temp_path, info.size_bytes).await?;

        fs::rename(&temp_path, &model_path)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to rename: {}", e)))?;

        let _ = app.emit(
            "model-download-status",
            serde_json::json!({ "status": "completed", "model_id": model_id }),
        );

        tracing::info!("[model] Download completed: {:?}", model_path);
        Ok(model_path)
    }

    async fn verify_download(path: &std::path::Path, expected_size: u64) -> Result<(), AppError> {
        let metadata = fs::metadata(path)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to read file metadata: {}", e)))?;

        let actual_size = metadata.len();
        let size_diff = (actual_size as i64 - expected_size as i64).unsigned_abs();
        let tolerance = expected_size / 20;

        if size_diff > tolerance {
            let _ = fs::remove_file(path).await;
            return Err(AppError::DownloadError(format!(
                "Download verification failed: expected ~{:.1}MB but got {:.1}MB",
                expected_size as f64 / 1024.0 / 1024.0,
                actual_size as f64 / 1024.0 / 1024.0,
            )));
        }

        tracing::info!(
            "[model] Download verified: {:.1}MB (expected ~{:.1}MB)",
            actual_size as f64 / 1024.0 / 1024.0,
            expected_size as f64 / 1024.0 / 1024.0,
        );
        Ok(())
    }

    pub async fn delete_model_by_id(app: &tauri::AppHandle, model_id: &str) -> Result<(), AppError> {
        let model_path = Self::get_model_path_by_id(app, model_id)?;
        if model_path.exists() {
            fs::remove_file(&model_path)
                .await
                .map_err(|e| AppError::DownloadError(format!("Failed to delete: {}", e)))?;
            tracing::info!("[model] Model '{}' deleted: {:?}", model_id, model_path);
        }
        Ok(())
    }
}

impl Default for ModelDownloader {
    fn default() -> Self {
        Self::new()
    }
}
