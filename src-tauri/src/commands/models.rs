use crate::commands::dictation::DictationState;
use crate::constants::model::{self, AVAILABLE_MODELS};
use crate::db::Database;
use crate::models::ModelDownloader;
use tauri::State;

#[tauri::command]
pub fn get_available_models(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let model_dir = ModelDownloader::get_model_dir(&app).map_err(|e| e.to_string())?;

    let models: Vec<serde_json::Value> = AVAILABLE_MODELS
        .iter()
        .map(|info| {
            let path = model_dir.join(info.filename);
            let installed = path.exists();
            let installed_size = if installed {
                std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
            } else {
                0
            };

            serde_json::json!({
                "id": info.id,
                "name": info.name,
                "filename": info.filename,
                "size_bytes": info.size_bytes,
                "size_display": info.size_display,
                "accuracy": info.accuracy,
                "speed": info.speed,
                "ram_mb": info.ram_mb,
                "description_ar": info.description_ar,
                "pros_ar": info.pros_ar,
                "cons_ar": info.cons_ar,
                "recommended": info.recommended,
                "installed": installed,
                "installed_size": installed_size,
            })
        })
        .collect();

    Ok(serde_json::json!(models))
}

#[tauri::command]
pub async fn download_specific_model(
    app: tauri::AppHandle,
    model_id: String,
) -> Result<String, String> {
    let downloader = ModelDownloader::new();
    let path = downloader
        .download_model_by_id(&app, &model_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_active_model(db: State<'_, Database>) -> Result<serde_json::Value, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let active_id = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'active_model'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| String::new());

    if active_id.is_empty() {
        return Ok(serde_json::json!({ "id": null, "name": null }));
    }

    if let Some(info) = model::find_model(&active_id) {
        Ok(serde_json::json!({
            "id": info.id,
            "name": info.name,
            "size_display": info.size_display,
        }))
    } else {
        Ok(serde_json::json!({ "id": active_id, "name": active_id }))
    }
}

#[tauri::command]
pub async fn set_active_model(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    app: tauri::AppHandle,
    model_id: String,
) -> Result<(), String> {
    let path = ModelDownloader::get_model_path_by_id(&app, &model_id)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("النموذج غير مثبّت، يرجى تحميله أولاً".to_string());
    }

    let use_gpu = {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('active_model', ?1)
             ON CONFLICT(key) DO UPDATE SET value = ?1",
            rusqlite::params![&model_id],
        )
        .map_err(|e| e.to_string())?;

        conn.query_row(
            "SELECT value FROM settings WHERE key = 'use_gpu'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "false".to_string()) == "true"
    };

    let mut transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
    transcriber.load_model(&path, use_gpu).map_err(|e| e.to_string())?;

    tracing::info!("[model] Active model set to '{}': {:?}", model_id, path);
    Ok(())
}

#[tauri::command]
pub async fn check_model_exists(app: tauri::AppHandle) -> Result<bool, String> {
    ModelDownloader::check_model_exists(&app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_model(
    app: tauri::AppHandle,
    model_id: String,
) -> Result<(), String> {
    ModelDownloader::delete_model_by_id(&app, &model_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_any_model_installed(app: tauri::AppHandle) -> Result<bool, String> {
    ModelDownloader::check_model_exists(&app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reload_model(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let (active_model_id, use_gpu) = {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let model_id = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'active_model'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_default();
        let gpu = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'use_gpu'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "false".to_string())
            == "true";
        (model_id, gpu)
    };

    if active_model_id.is_empty() {
        return Err("لا يوجد نموذج نشط لإعادة تحميله".to_string());
    }

    let path = ModelDownloader::get_model_path_by_id(&app, &active_model_id)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("ملف النموذج غير موجود".to_string());
    }

    let mut transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
    transcriber
        .load_model(&path, use_gpu)
        .map_err(|e| e.to_string())?;

    tracing::info!(
        "[model] Reloaded model '{}' (GPU: {})",
        active_model_id, use_gpu
    );
    Ok(())
}

#[tauri::command]
pub fn has_active_model(db: State<'_, Database>) -> Result<bool, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let active_id = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'active_model'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| String::new());

    Ok(!active_id.is_empty())
}
