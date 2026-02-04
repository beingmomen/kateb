mod commands;
mod db;
mod whisper;
mod audio;
mod keyboard;

use commands::dictation::DictationState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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

            let resource_path = app
                .path()
                .resource_dir()
                .expect("failed to get resource dir");
            let model_path = resource_path.join("resources").join("ggml-large-v3.bin");

            if model_path.exists() {
                if let Err(e) = transcriber.load_model(&model_path) {
                    eprintln!("Warning: Failed to load Whisper model: {}", e);
                }
            } else {
                eprintln!("Warning: Whisper model not found at {:?}", model_path);
            }

            app.manage(DictationState {
                recorder: Mutex::new(audio::recorder::AudioRecorder::new()),
                transcriber: Mutex::new(transcriber),
                is_recording: Mutex::new(false),
                is_processing: Mutex::new(false),
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
