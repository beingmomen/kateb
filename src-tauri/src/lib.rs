mod ai;
mod audio;
mod commands;
mod constants;
mod db;
mod error;
mod keyboard;
mod models;
mod whisper;

use commands::dictation::DictationState;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();
            db::init_database(&app_handle)?;

            let mut transcriber = whisper::transcriber::WhisperTranscriber::new();

            let (active_model_id, use_gpu) = {
                let db_state: tauri::State<'_, db::Database> = app.state();
                let conn = db_state.0.lock().unwrap();
                let model_id = conn.query_row(
                    "SELECT value FROM settings WHERE key = 'active_model'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .unwrap_or_default();
                let gpu = conn.query_row(
                    "SELECT value FROM settings WHERE key = 'use_gpu'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .unwrap_or_else(|_| "false".to_string()) == "true";
                (model_id, gpu)
            };

            if !active_model_id.is_empty() {
                if let Ok(path) = models::ModelDownloader::get_model_path_by_id(&app_handle, &active_model_id) {
                    if path.exists() {
                        if let Err(e) = transcriber.load_model(&path, use_gpu) {
                            eprintln!("[model] Warning: Failed to load '{}': {}", active_model_id, e);
                        } else {
                            eprintln!("[model] Loaded active model '{}' (GPU: {}): {:?}", active_model_id, use_gpu, path);
                        }
                    } else {
                        eprintln!("[model] Active model '{}' not found at {:?}", active_model_id, path);
                    }
                }
            } else {
                eprintln!("[model] No active model set, user needs to download one");
            }

            app.manage(DictationState {
                recorder: Mutex::new(audio::recorder::AudioRecorder::new()),
                transcriber: Mutex::new(transcriber),
                is_recording: Mutex::new(false),
                is_processing: Mutex::new(false),
                streaming_active: Arc::new(AtomicBool::new(false)),
                streaming_thread: Mutex::new(None),
            });

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let last_z = Arc::new(Mutex::new(Instant::now()));
                let first_press = Arc::new(Mutex::new(false));

                let last_z_cb = Arc::clone(&last_z);
                let first_press_cb = Arc::clone(&first_press);
                let handle_cb = handle.clone();

                rdev::listen(move |event| {
                    if let rdev::EventType::KeyPress(rdev::Key::KeyZ) = event.event_type {
                        let now = Instant::now();
                        let mut last = last_z_cb.lock().unwrap();
                        let mut first = first_press_cb.lock().unwrap();

                        if *first && now.duration_since(*last).as_millis() < 400 {
                            *first = false;
                            eprintln!("[shortcut] Double-Z detected! Toggling dictation...");
                            let _ = handle_cb.emit("toggle-dictation", ());
                        } else {
                            *first = true;
                            *last = now;
                        }
                    }
                })
                .expect("Failed to start global key listener");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::dictation::start_dictation,
            commands::dictation::stop_dictation,
            commands::dictation::get_dictation_status,
            commands::settings::get_all_settings,
            commands::settings::update_setting,
            commands::settings::get_setting,
            commands::history::get_history,
            commands::history::delete_history_item,
            commands::history::clear_history,
            commands::history::get_usage_stats,
            commands::history::get_summary_stats,
            commands::models::get_available_models,
            commands::models::download_specific_model,
            commands::models::get_active_model,
            commands::models::set_active_model,
            commands::models::check_model_exists,
            commands::models::delete_model,
            commands::models::check_any_model_installed,
            commands::models::has_active_model,
            commands::models::reload_model,
            commands::ai::test_ai_connection,
            commands::ai::test_specific_provider,
            commands::ai::get_ai_providers,
            commands::ai::get_current_ai_provider,
            commands::ai::detect_gpu,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
