mod ai;
mod audio;
mod commands;
mod constants;
mod db;
mod error;
mod keyboard;
mod logging;
mod models;
mod security;
mod whisper;

use commands::dictation::DictationState;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;

pub struct ShortcutState(pub Arc<Mutex<String>>);

fn parse_shortcut(shortcut: &str) -> ShortcutType {
    let parts: Vec<&str> = shortcut.split('+').map(|s| s.trim()).collect();

    if parts.len() == 2 && parts[0] == parts[1] && parts[0].len() == 1 {
        return ShortcutType::DoubleTap(parts[0].to_uppercase());
    }

    let mut ctrl = false;
    let mut shift = false;
    let mut alt = false;
    let mut meta = false;
    let mut key = String::new();

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => ctrl = true,
            "shift" => shift = true,
            "alt" => alt = true,
            "meta" | "super" | "cmd" => meta = true,
            _ => key = part.to_string(),
        }
    }

    if key.is_empty() {
        return ShortcutType::DoubleTap("Z".to_string());
    }

    ShortcutType::ModifierCombo {
        ctrl,
        shift,
        alt,
        meta,
        key,
    }
}

enum ShortcutType {
    DoubleTap(String),
    ModifierCombo {
        ctrl: bool,
        shift: bool,
        alt: bool,
        meta: bool,
        key: String,
    },
}

fn rdev_key_matches(rdev_key: &rdev::Key, target: &str) -> bool {
    let target_upper = target.to_uppercase();
    match rdev_key {
        rdev::Key::KeyA => target_upper == "A",
        rdev::Key::KeyB => target_upper == "B",
        rdev::Key::KeyC => target_upper == "C",
        rdev::Key::KeyD => target_upper == "D",
        rdev::Key::KeyE => target_upper == "E",
        rdev::Key::KeyF => target_upper == "F",
        rdev::Key::KeyG => target_upper == "G",
        rdev::Key::KeyH => target_upper == "H",
        rdev::Key::KeyI => target_upper == "I",
        rdev::Key::KeyJ => target_upper == "J",
        rdev::Key::KeyK => target_upper == "K",
        rdev::Key::KeyL => target_upper == "L",
        rdev::Key::KeyM => target_upper == "M",
        rdev::Key::KeyN => target_upper == "N",
        rdev::Key::KeyO => target_upper == "O",
        rdev::Key::KeyP => target_upper == "P",
        rdev::Key::KeyQ => target_upper == "Q",
        rdev::Key::KeyR => target_upper == "R",
        rdev::Key::KeyS => target_upper == "S",
        rdev::Key::KeyT => target_upper == "T",
        rdev::Key::KeyU => target_upper == "U",
        rdev::Key::KeyV => target_upper == "V",
        rdev::Key::KeyW => target_upper == "W",
        rdev::Key::KeyX => target_upper == "X",
        rdev::Key::KeyY => target_upper == "Y",
        rdev::Key::KeyZ => target_upper == "Z",
        rdev::Key::Num0 => target_upper == "0",
        rdev::Key::Num1 => target_upper == "1",
        rdev::Key::Num2 => target_upper == "2",
        rdev::Key::Num3 => target_upper == "3",
        rdev::Key::Num4 => target_upper == "4",
        rdev::Key::Num5 => target_upper == "5",
        rdev::Key::Num6 => target_upper == "6",
        rdev::Key::Num7 => target_upper == "7",
        rdev::Key::Num8 => target_upper == "8",
        rdev::Key::Num9 => target_upper == "9",
        rdev::Key::F1 => target_upper == "F1",
        rdev::Key::F2 => target_upper == "F2",
        rdev::Key::F3 => target_upper == "F3",
        rdev::Key::F4 => target_upper == "F4",
        rdev::Key::F5 => target_upper == "F5",
        rdev::Key::F6 => target_upper == "F6",
        rdev::Key::F7 => target_upper == "F7",
        rdev::Key::F8 => target_upper == "F8",
        rdev::Key::F9 => target_upper == "F9",
        rdev::Key::F10 => target_upper == "F10",
        rdev::Key::F11 => target_upper == "F11",
        rdev::Key::F12 => target_upper == "F12",
        rdev::Key::Space => target_upper == "SPACE",
        rdev::Key::Tab => target_upper == "TAB",
        rdev::Key::Escape => target_upper == "ESCAPE" || target_upper == "ESC",
        _ => false,
    }
}

fn is_modifier_key(key: &rdev::Key) -> bool {
    matches!(
        key,
        rdev::Key::ControlLeft
            | rdev::Key::ControlRight
            | rdev::Key::ShiftLeft
            | rdev::Key::ShiftRight
            | rdev::Key::Alt
            | rdev::Key::AltGr
            | rdev::Key::MetaLeft
            | rdev::Key::MetaRight
    )
}

fn is_ctrl_key(key: &rdev::Key) -> bool {
    matches!(key, rdev::Key::ControlLeft | rdev::Key::ControlRight)
}

fn is_shift_key(key: &rdev::Key) -> bool {
    matches!(key, rdev::Key::ShiftLeft | rdev::Key::ShiftRight)
}

fn is_alt_key(key: &rdev::Key) -> bool {
    matches!(key, rdev::Key::Alt | rdev::Key::AltGr)
}

fn is_meta_key(key: &rdev::Key) -> bool {
    matches!(key, rdev::Key::MetaLeft | rdev::Key::MetaRight)
}

#[tauri::command]
fn update_shortcut(
    shortcut: String,
    shortcut_state: tauri::State<'_, ShortcutState>,
) -> Result<(), String> {
    let mut current = shortcut_state.0.lock().map_err(|e| e.to_string())?;
    *current = shortcut;
    tracing::info!("[shortcut] Updated to: {}", *current);
    Ok(())
}

#[tauri::command]
fn show_overlay(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.show();
        tracing::debug!("[overlay] Shown");
    }
    Ok(())
}

#[tauri::command]
fn hide_overlay(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.hide();
        tracing::debug!("[overlay] Hidden");
    }
    Ok(())
}

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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            logging::init(&app_handle);
            db::init_database(&app_handle)?;

            security::keychain::init();
            {
                let db_state: tauri::State<'_, db::Database> = app.state();
                let conn = db_state.0.lock().unwrap();
                security::keychain::migrate_from_db(&conn);
            }

            let mut transcriber = whisper::transcriber::WhisperTranscriber::new();

            let (active_model_id, use_gpu, shortcut_setting) = {
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
                let shortcut = conn.query_row(
                    "SELECT value FROM settings WHERE key = 'shortcut'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .unwrap_or_else(|_| "Z+Z".to_string());
                let shortcut = shortcut.trim_matches('"').to_string();
                (model_id, gpu, shortcut)
            };

            tracing::info!("[shortcut] Loaded shortcut: {}", shortcut_setting);

            if !active_model_id.is_empty() {
                if let Ok(path) = models::ModelDownloader::get_model_path_by_id(&app_handle, &active_model_id) {
                    if path.exists() {
                        if let Err(e) = transcriber.load_model(&path, use_gpu) {
                            tracing::warn!("[model] Failed to load '{}': {}", active_model_id, e);
                        } else {
                            tracing::info!("[model] Loaded active model '{}' (GPU: {}): {:?}", active_model_id, use_gpu, path);
                        }
                    } else {
                        tracing::warn!("[model] Active model '{}' not found at {:?}", active_model_id, path);
                    }
                }
            } else {
                tracing::info!("[model] No active model set, user needs to download one");
            }

            let shortcut_arc = Arc::new(Mutex::new(shortcut_setting));
            app.manage(ShortcutState(Arc::clone(&shortcut_arc)));

            let recorder = audio::recorder::AudioRecorder::new();
            {
                let db_state: tauri::State<'_, db::Database> = app.state();
                let conn = db_state.0.lock().unwrap();
                let saved_device: Option<String> = conn
                    .query_row(
                        "SELECT value FROM settings WHERE key = 'audio_device'",
                        [],
                        |row| row.get::<_, String>(0),
                    )
                    .ok()
                    .filter(|v| !v.is_empty());
                if let Some(ref device) = saved_device {
                    tracing::info!("[audio] Restoring saved device: {}", device);
                    recorder.set_device(Some(device.clone()));
                }
            }

            app.manage(DictationState {
                recorder: Mutex::new(recorder),
                transcriber: Mutex::new(transcriber),
                is_recording: Mutex::new(false),
                is_processing: Mutex::new(false),
                streaming_active: Arc::new(AtomicBool::new(false)),
                streaming_thread: Mutex::new(None),
                accumulated_text: Arc::new(Mutex::new(Vec::new())),
                vad: Arc::new(Mutex::new(audio::vad::AdaptiveVAD::new())),
            });

            let handle = app.handle().clone();
            let shortcut_for_thread = Arc::clone(&shortcut_arc);

            std::thread::spawn(move || {
                let last_press_time = Arc::new(Mutex::new(Instant::now()));
                let first_press = Arc::new(Mutex::new(false));
                let ctrl_held = Arc::new(Mutex::new(false));
                let shift_held = Arc::new(Mutex::new(false));
                let alt_held = Arc::new(Mutex::new(false));
                let meta_held = Arc::new(Mutex::new(false));

                let last_press_cb = Arc::clone(&last_press_time);
                let first_press_cb = Arc::clone(&first_press);
                let ctrl_cb = Arc::clone(&ctrl_held);
                let shift_cb = Arc::clone(&shift_held);
                let alt_cb = Arc::clone(&alt_held);
                let meta_cb = Arc::clone(&meta_held);
                let shortcut_cb = Arc::clone(&shortcut_for_thread);
                let handle_cb = handle.clone();

                rdev::listen(move |event| {
                    match event.event_type {
                        rdev::EventType::KeyPress(key) => {
                            if is_ctrl_key(&key) { *ctrl_cb.lock().unwrap() = true; return; }
                            if is_shift_key(&key) { *shift_cb.lock().unwrap() = true; return; }
                            if is_alt_key(&key) { *alt_cb.lock().unwrap() = true; return; }
                            if is_meta_key(&key) { *meta_cb.lock().unwrap() = true; return; }

                            if is_modifier_key(&key) { return; }

                            let shortcut_str = shortcut_cb.lock().unwrap().clone();
                            let shortcut_type = parse_shortcut(&shortcut_str);

                            match shortcut_type {
                                ShortcutType::DoubleTap(target) => {
                                    if rdev_key_matches(&key, &target) {
                                        let now = Instant::now();
                                        let mut last = last_press_cb.lock().unwrap();
                                        let mut first = first_press_cb.lock().unwrap();

                                        if *first && now.duration_since(*last).as_millis() < 400 {
                                            *first = false;
                                            tracing::debug!("[shortcut] Double-{} detected! Toggling dictation...", target);
                                            let _ = handle_cb.emit("toggle-dictation", ());
                                        } else {
                                            *first = true;
                                            *last = now;
                                        }
                                    } else {
                                        *first_press_cb.lock().unwrap() = false;
                                    }
                                }
                                ShortcutType::ModifierCombo { ctrl, shift, alt, meta, key: target } => {
                                    let ctrl_ok = !ctrl || *ctrl_cb.lock().unwrap();
                                    let shift_ok = !shift || *shift_cb.lock().unwrap();
                                    let alt_ok = !alt || *alt_cb.lock().unwrap();
                                    let meta_ok = !meta || *meta_cb.lock().unwrap();

                                    if ctrl_ok && shift_ok && alt_ok && meta_ok && rdev_key_matches(&key, &target) {
                                        tracing::debug!("[shortcut] {} detected! Toggling dictation...", shortcut_str);
                                        let _ = handle_cb.emit("toggle-dictation", ());
                                    }
                                }
                            }
                        }
                        rdev::EventType::KeyRelease(key) => {
                            if is_ctrl_key(&key) { *ctrl_cb.lock().unwrap() = false; }
                            if is_shift_key(&key) { *shift_cb.lock().unwrap() = false; }
                            if is_alt_key(&key) { *alt_cb.lock().unwrap() = false; }
                            if is_meta_key(&key) { *meta_cb.lock().unwrap() = false; }
                        }
                        _ => {}
                    }
                })
                .expect("Failed to start global key listener");
            });

            let show_hide = MenuItemBuilder::with_id("show_hide", "إظهار/إخفاء النافذة").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "إنهاء التطبيق").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show_hide, &quit]).build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                                if let Some(state) = app.try_state::<DictationState>() {
                                    let active = state.is_recording.lock().map(|r| *r).unwrap_or(false)
                                        || state.is_processing.lock().map(|p| *p).unwrap_or(false);
                                    if active {
                                        commands::dictation::show_overlay_window(app);
                                    }
                                }
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                                commands::dictation::hide_overlay_window(app);
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                        commands::dictation::hide_overlay_window(app);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }

            if window.label() == "main" {
                let main_hidden = !window.is_visible().unwrap_or(true);
                let app = window.app_handle();
                if let Some(state) = app.try_state::<DictationState>() {
                    let is_recording = state.is_recording.lock().map(|r| *r).unwrap_or(false);
                    let is_processing = state.is_processing.lock().map(|p| *p).unwrap_or(false);
                    if main_hidden && (is_recording || is_processing) {
                        commands::dictation::show_overlay_window(app);
                    } else if !main_hidden {
                        commands::dictation::hide_overlay_window(app);
                    }
                }
            }
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
            commands::audio::get_audio_devices,
            commands::audio::set_audio_device,
            commands::ai::test_ai_connection,
            commands::ai::test_specific_provider,
            commands::ai::get_ai_providers,
            commands::ai::get_current_ai_provider,
            commands::ai::detect_gpu,
            commands::backup::export_settings,
            commands::backup::import_settings,
            update_shortcut,
            show_overlay,
            hide_overlay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
