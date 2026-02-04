pub mod migrations;
pub mod models;

use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct Database(pub Mutex<Connection>);

pub fn init_database(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("dictation.db");
    let conn = Connection::open(db_path)?;

    migrations::run_migrations(&conn)?;
    migrations::seed_default_settings(&conn)?;

    app.manage(Database(Mutex::new(conn)));
    Ok(())
}
