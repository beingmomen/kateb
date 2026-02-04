use crate::db::models::{DictationEntry, SummaryStats, UsageStat};
use crate::db::Database;
use tauri::State;

fn map_dictation_row(row: &rusqlite::Row) -> rusqlite::Result<DictationEntry> {
    Ok(DictationEntry {
        id: row.get(0)?,
        text: row.get(1)?,
        duration: row.get(2)?,
        language: row.get(3)?,
        created_at: row.get(4)?,
    })
}

#[tauri::command]
pub fn get_history(
    db: State<'_, Database>,
    search: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<DictationEntry>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    if let Some(ref query) = search {
        let pattern = format!("%{}%", query);
        let mut stmt = conn
            .prepare(
                "SELECT id, text, duration, language, created_at
                 FROM dictation_history
                 WHERE text LIKE ?1
                 ORDER BY created_at DESC
                 LIMIT ?2 OFFSET ?3",
            )
            .map_err(|e| e.to_string())?;

        let result: Vec<DictationEntry> = stmt
            .query_map(rusqlite::params![pattern, limit, offset], map_dictation_row)
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        Ok(result)
    } else {
        let mut stmt = conn
            .prepare(
                "SELECT id, text, duration, language, created_at
                 FROM dictation_history
                 ORDER BY created_at DESC
                 LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| e.to_string())?;

        let result: Vec<DictationEntry> = stmt
            .query_map(rusqlite::params![limit, offset], map_dictation_row)
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        Ok(result)
    }
}

#[tauri::command]
pub fn delete_history_item(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM dictation_history WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn clear_history(db: State<'_, Database>) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM dictation_history", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_usage_stats(
    db: State<'_, Database>,
    days: Option<i64>,
) -> Result<Vec<UsageStat>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let days = days.unwrap_or(30);

    let mut stmt = conn
        .prepare(
            "SELECT id, date, total_dictations, total_words, total_duration
             FROM usage_stats
             WHERE date >= date('now', ?1)
             ORDER BY date ASC",
        )
        .map_err(|e| e.to_string())?;

    let modifier = format!("-{} days", days);
    let stats: Vec<UsageStat> = stmt
        .query_map([&modifier], |row| {
            Ok(UsageStat {
                id: row.get(0)?,
                date: row.get(1)?,
                total_dictations: row.get(2)?,
                total_words: row.get(3)?,
                total_duration: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(stats)
}

#[tauri::command]
pub fn get_summary_stats(db: State<'_, Database>) -> Result<SummaryStats, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;

    let stats = conn
        .query_row(
            "SELECT
                COALESCE(SUM(total_dictations), 0),
                COALESCE(SUM(total_words), 0),
                COALESCE(SUM(total_duration), 0),
                COUNT(DISTINCT date)
             FROM usage_stats",
            [],
            |row| {
                Ok(SummaryStats {
                    total_dictations: row.get(0)?,
                    total_words: row.get(1)?,
                    total_duration: row.get(2)?,
                    days_active: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(stats)
}
