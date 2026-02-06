use crate::constants::model::*;
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

    pub fn get_model_path(app: &tauri::AppHandle) -> Result<PathBuf, AppError> {
        let model_dir = Self::get_model_dir(app)?;
        Ok(model_dir.join(FILENAME))
    }

    pub async fn check_model_exists(app: &tauri::AppHandle) -> Result<bool, AppError> {
        let model_path = Self::get_model_path(app)?;

        if !model_path.exists() {
            return Ok(false);
        }

        let metadata = fs::metadata(&model_path)
            .await
            .map_err(|e| AppError::DownloadError(e.to_string()))?;

        Ok(metadata.len() >= EXPECTED_SIZE_BYTES)
    }

    pub async fn download_model(&self, app: &tauri::AppHandle) -> Result<PathBuf, AppError> {
        let model_dir = Self::get_model_dir(app)?;
        let model_path = Self::get_model_path(app)?;

        fs::create_dir_all(&model_dir)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to create model directory: {}", e)))?;

        let _ = app.emit(
            "model-download-status",
            serde_json::json!({ "status": "started" }),
        );

        eprintln!("[model] Starting download from: {}", HUGGINGFACE_URL);

        let response = self
            .client
            .get(HUGGINGFACE_URL)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let _ = app.emit(
                "model-download-status",
                serde_json::json!({ "status": "error", "message": format!("HTTP {}", status) }),
            );
            return Err(AppError::DownloadError(format!(
                "Failed to download model: HTTP {}",
                status
            )));
        }

        let total_size = response.content_length().unwrap_or(EXPECTED_SIZE_BYTES);
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
                    "progress": progress,
                    "downloaded": downloaded,
                    "total": total_size
                }),
            );

            if downloaded % (10 * 1024 * 1024) < chunk.len() as u64 {
                eprintln!(
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

        fs::rename(&temp_path, &model_path)
            .await
            .map_err(|e| AppError::DownloadError(format!("Failed to rename: {}", e)))?;

        let _ = app.emit(
            "model-download-status",
            serde_json::json!({ "status": "completed" }),
        );

        eprintln!("[model] Download completed: {:?}", model_path);
        Ok(model_path)
    }

    pub async fn delete_model(app: &tauri::AppHandle) -> Result<(), AppError> {
        let model_path = Self::get_model_path(app)?;
        if model_path.exists() {
            fs::remove_file(&model_path)
                .await
                .map_err(|e| AppError::DownloadError(format!("Failed to delete: {}", e)))?;
            eprintln!("[model] Model deleted: {:?}", model_path);
        }
        Ok(())
    }
}

impl Default for ModelDownloader {
    fn default() -> Self {
        Self::new()
    }
}
