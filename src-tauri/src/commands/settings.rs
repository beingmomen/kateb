use crate::db::models::Setting;
use crate::db::Database;
use crate::security::keychain;
use tauri::State;

#[tauri::command]
pub fn get_all_settings(db: State<'_, Database>) -> Result<Vec<Setting>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, key, value, updated_at FROM settings ORDER BY key")
        .map_err(|e| e.to_string())?;

    let mut settings: Vec<Setting> = stmt
        .query_map([], |row| {
            Ok(Setting {
                id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    if keychain::is_available() {
        for setting in &mut settings {
            if keychain::is_api_key_setting(&setting.key) {
                if let Some(val) = keychain::retrieve_api_key(&setting.key) {
                    setting.value = val;
                }
            }
        }
    }

    Ok(settings)
}

#[tauri::command]
pub fn get_setting(db: State<'_, Database>, key: String) -> Result<Option<String>, String> {
    if keychain::is_api_key_setting(&key) && keychain::is_available() {
        if let Some(val) = keychain::retrieve_api_key(&key) {
            return Ok(Some(val));
        }
    }

    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let result = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [&key],
            |row| row.get(0),
        )
        .ok();

    Ok(result)
}

#[tauri::command]
pub fn update_setting(db: State<'_, Database>, key: String, value: String) -> Result<(), String> {
    if keychain::is_api_key_setting(&key) && keychain::is_available() {
        keychain::store_api_key(&key, &value)?;
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, '', datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = '', updated_at = datetime('now')",
            [&key],
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now')",
        [&key, &value],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
