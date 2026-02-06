use crate::commands::dictation::DictationState;
use crate::models::ModelDownloader;
use tauri::{Manager, State};

#[tauri::command]
pub async fn check_model_exists(app: tauri::AppHandle) -> Result<bool, String> {
    ModelDownloader::check_model_exists(&app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_model(app: tauri::AppHandle) -> Result<String, String> {
    let downloader = ModelDownloader::new();
    let path = downloader
        .download_model(&app)
        .await
        .map_err(|e| e.to_string())?;

    let state: State<'_, DictationState> = app.state();
    let mut transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
    if let Err(e) = transcriber.load_model(&path) {
        eprintln!("[model] Warning: Failed to load model after download: {}", e);
    }

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_model_path(app: tauri::AppHandle) -> Result<String, String> {
    let path = ModelDownloader::get_model_path(&app).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn delete_model(app: tauri::AppHandle) -> Result<(), String> {
    ModelDownloader::delete_model(&app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_model_info(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let exists = ModelDownloader::check_model_exists(&app)
        .await
        .map_err(|e| e.to_string())?;

    let path = ModelDownloader::get_model_path(&app).map_err(|e| e.to_string())?;

    let size = if exists && path.exists() {
        std::fs::metadata(&path)
            .map(|m| m.len())
            .unwrap_or(0)
    } else {
        0
    };

    Ok(serde_json::json!({
        "exists": exists,
        "path": path.to_string_lossy(),
        "size": size,
        "filename": crate::constants::model::FILENAME,
        "expected_size": crate::constants::model::EXPECTED_SIZE_BYTES,
    }))
}

#[tauri::command]
pub async fn load_model(
    state: State<'_, DictationState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let path = ModelDownloader::get_model_path(&app).map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("النموذج غير موجود، يرجى تحميله أولاً".to_string());
    }

    let mut transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
    transcriber.load_model(&path).map_err(|e| e.to_string())?;

    eprintln!("[model] Model loaded successfully: {:?}", path);
    Ok(())
}
