use crate::db::Database;
use crate::security::keychain;
use serde_json::{Map, Value};
use tauri::State;

#[tauri::command]
pub fn export_settings(db: State<'_, Database>) -> Result<String, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT key, value FROM settings ORDER BY key")
        .map_err(|e| e.to_string())?;

    let mut map = Map::new();

    let rows: Vec<(String, String)> = stmt
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    for (key, value) in rows {
        if keychain::is_api_key_setting(&key) {
            continue;
        }
        map.insert(key, Value::String(value));
    }

    serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_settings(db: State<'_, Database>, json: String) -> Result<u32, String> {
    let parsed: Map<String, Value> =
        serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut count = 0u32;

    for (key, value) in &parsed {
        if keychain::is_api_key_setting(key) {
            continue;
        }

        let val_str = match value {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            _ => continue,
        };

        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now')",
            [&key, &val_str],
        )
        .map_err(|e| e.to_string())?;

        count += 1;
    }

    Ok(count)
}
