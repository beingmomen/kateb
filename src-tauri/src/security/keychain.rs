use std::sync::atomic::{AtomicBool, Ordering};

const SERVICE_NAME: &str = "com.beingmomen.kateb";

const API_KEY_SETTINGS: &[&str] = &[
    "claude_api_key",
    "openai_api_key",
    "gemini_api_key",
    "grok_api_key",
    "local_api_key",
];

static KEYCHAIN_AVAILABLE: AtomicBool = AtomicBool::new(false);

pub fn is_api_key_setting(key: &str) -> bool {
    API_KEY_SETTINGS.contains(&key)
}

pub fn init() -> bool {
    match keyring::Entry::new(SERVICE_NAME, "test") {
        Ok(_) => {
            KEYCHAIN_AVAILABLE.store(true, Ordering::SeqCst);
            tracing::info!("[security] Keychain initialized successfully");
            true
        }
        Err(e) => {
            KEYCHAIN_AVAILABLE.store(false, Ordering::SeqCst);
            tracing::warn!("[security] Keychain unavailable, falling back to database: {}", e);
            false
        }
    }
}

pub fn is_available() -> bool {
    KEYCHAIN_AVAILABLE.load(Ordering::SeqCst)
}

pub fn store_api_key(key_name: &str, value: &str) -> Result<(), String> {
    if !is_available() {
        return Err("Keychain not available".to_string());
    }

    let entry = keyring::Entry::new(SERVICE_NAME, key_name)
        .map_err(|e| format!("Keychain entry error: {}", e))?;

    if value.is_empty() {
        let _ = entry.delete_credential();
        Ok(())
    } else {
        entry
            .set_password(value)
            .map_err(|e| format!("Failed to store key in keychain: {}", e))
    }
}

pub fn retrieve_api_key(key_name: &str) -> Option<String> {
    if !is_available() {
        return None;
    }

    let entry = keyring::Entry::new(SERVICE_NAME, key_name).ok()?;
    entry.get_password().ok().filter(|k| !k.is_empty())
}

pub fn delete_api_key(key_name: &str) -> Result<(), String> {
    if !is_available() {
        return Ok(());
    }

    let entry = keyring::Entry::new(SERVICE_NAME, key_name)
        .map_err(|e| format!("Keychain entry error: {}", e))?;
    let _ = entry.delete_credential();
    Ok(())
}

pub fn migrate_from_db(conn: &rusqlite::Connection) {
    if !is_available() {
        return;
    }

    for key_name in API_KEY_SETTINGS {
        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [key_name],
                |row| row.get(0),
            )
            .ok()
            .filter(|v: &String| !v.is_empty());

        if let Some(val) = value {
            if store_api_key(key_name, &val).is_ok() {
                let _ = conn.execute(
                    "UPDATE settings SET value = '', updated_at = datetime('now') WHERE key = ?1",
                    [key_name],
                );
                tracing::info!("[security] Migrated {} to keychain", key_name);
            }
        }
    }
}
