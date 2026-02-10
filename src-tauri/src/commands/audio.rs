use crate::audio::recorder::AudioRecorder;
use crate::commands::dictation::DictationState;
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn get_audio_devices() -> Result<serde_json::Value, String> {
    let devices = AudioRecorder::list_devices();
    let result: Vec<serde_json::Value> = devices
        .into_iter()
        .map(|(name, is_default)| {
            serde_json::json!({
                "name": name,
                "is_default": is_default
            })
        })
        .collect();
    Ok(serde_json::json!(result))
}

#[tauri::command]
pub fn set_audio_device(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    device_name: Option<String>,
) -> Result<(), String> {
    let recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    recorder.set_device(device_name.clone());

    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let value = device_name.unwrap_or_default();
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('audio_device', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
        rusqlite::params![&value],
    )
    .map_err(|e| e.to_string())?;

    tracing::info!("[audio] Device set to: '{}'", if value.is_empty() { "default" } else { &value });
    Ok(())
}
