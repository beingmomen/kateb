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
            raw_text TEXT NOT NULL DEFAULT '',
            duration INTEGER NOT NULL,
            language TEXT NOT NULL,
            ai_provider TEXT NOT NULL DEFAULT '',
            processing_time_ms INTEGER NOT NULL DEFAULT 0,
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
    let columns: Vec<String> = conn
        .prepare("PRAGMA table_info(dictation_history)")?
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .collect();

    if !columns.iter().any(|c| c == "raw_text") {
        conn.execute_batch(
            "
            ALTER TABLE dictation_history ADD COLUMN raw_text TEXT NOT NULL DEFAULT '';
            ALTER TABLE dictation_history ADD COLUMN ai_provider TEXT NOT NULL DEFAULT '';
            ALTER TABLE dictation_history ADD COLUMN processing_time_ms INTEGER NOT NULL DEFAULT 0;
            ",
        )?;
    }

    conn.execute(
        "UPDATE settings SET value = 'false' WHERE key = 'auto_stop_silence' AND value = 'true'",
        [],
    )?;
    conn.execute(
        "UPDATE settings SET value = '10' WHERE key = 'auto_stop_seconds' AND value = '5'",
        [],
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
        ("use_gpu", "false"),
        ("ai_refinement", "false"),
        ("ai_provider", "local"),
        ("claude_api_key", ""),
        ("openai_api_key", ""),
        ("gemini_api_key", ""),
        ("grok_api_key", ""),
        ("local_api_key", ""),
        ("claude_api_url", ""),
        ("openai_api_url", ""),
        ("gemini_api_url", ""),
        ("grok_api_url", ""),
        ("local_api_url", ""),
        ("auto_stop_silence", "false"),
        ("auto_stop_seconds", "10"),
    ];

    for (key, value) in defaults {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
    }

    Ok(())
}
