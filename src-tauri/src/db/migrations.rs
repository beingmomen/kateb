use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS dictation_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            duration INTEGER NOT NULL,
            language TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS usage_stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT UNIQUE NOT NULL,
            total_dictations INTEGER NOT NULL DEFAULT 0,
            total_words INTEGER NOT NULL DEFAULT 0,
            total_duration INTEGER NOT NULL DEFAULT 0
        );
        ",
    )?;
    Ok(())
}

pub fn seed_default_settings(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let defaults = vec![
        ("shortcut", r#""Ctrl+Shift+D""#),
        ("language", r#""ar""#),
        ("auto_punctuation", "true"),
        ("sound_notifications", "true"),
        ("auto_start", "false"),
        ("whisper_model", r#""large-v3""#),
        ("max_recording_duration", "300"),
        ("auto_type", "true"),
    ];

    for (key, value) in defaults {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
    }

    Ok(())
}
