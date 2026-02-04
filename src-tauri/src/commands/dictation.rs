use crate::audio::recorder::AudioRecorder;
use crate::db::Database;
use crate::keyboard::simulator::KeyboardSimulator;
use crate::whisper::transcriber::WhisperTranscriber;
use std::sync::Mutex;
use tauri::{Emitter, State};

pub struct DictationState {
    pub recorder: Mutex<AudioRecorder>,
    pub transcriber: Mutex<WhisperTranscriber>,
    pub is_recording: Mutex<bool>,
    pub is_processing: Mutex<bool>,
}

#[tauri::command]
pub async fn start_dictation(
    state: State<'_, DictationState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if *is_recording {
        return Err("التسجيل قيد التشغيل بالفعل".to_string());
    }

    let recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    match recorder.start() {
        Ok(()) => {
            *is_recording = true;
            let _ = app.emit("dictation-status", serde_json::json!({
                "is_recording": true,
                "is_processing": false
            }));
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn stop_dictation(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let (audio_data, duration) = {
        let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
        if !*is_recording {
            return Err("لا يوجد تسجيل نشط".to_string());
        }

        let recorder = state.recorder.lock().map_err(|e| e.to_string())?;
        let audio_data = match recorder.stop() {
            Ok(data) => data,
            Err(e) => {
                *is_recording = false;
                eprintln!("[dictation] ERROR stopping recorder: {}", e);
                return Err(e.to_string());
            }
        };
        let duration = recorder.get_duration_seconds();
        *is_recording = false;
        eprintln!("[dictation] Audio captured: {} samples ({:.1}s), duration: {}s", audio_data.len(), audio_data.len() as f64 / 16000.0, duration);
        (audio_data, duration)
    };

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = true;
    }

    let _ = app.emit("dictation-status", serde_json::json!({
        "is_recording": false,
        "is_processing": true
    }));

    if audio_data.is_empty() {
        eprintln!("[dictation] WARNING: Audio buffer is empty, nothing to transcribe");
        let mut is_processing = state.is_processing.lock().map_err(|_| "lock error".to_string())?;
        *is_processing = false;
        let _ = app.emit("dictation-status", serde_json::json!({
            "is_recording": false,
            "is_processing": false
        }));
        return Ok(String::new());
    }

    eprintln!("[dictation] Starting Whisper transcription...");
    let text = {
        let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
        match transcriber.transcribe(&audio_data) {
            Ok(t) => {
                eprintln!("[dictation] Transcription complete: '{}' ({} chars)", t, t.len());
                t
            }
            Err(e) => {
                eprintln!("[dictation] ERROR in transcription: {}", e);
                let mut is_processing = state.is_processing.lock().map_err(|_| "lock error".to_string())?;
                *is_processing = false;
                let _ = app.emit("dictation-status", serde_json::json!({
                    "is_recording": false,
                    "is_processing": false
                }));
                return Err(e.to_string());
            }
        }
    };

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = false;
    }

    let _ = app.emit("dictation-status", serde_json::json!({
        "is_recording": false,
        "is_processing": false
    }));

    if !text.trim().is_empty() {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let language = {
            let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
            transcriber.get_language()
        };

        conn.execute(
            "INSERT INTO dictation_history (text, duration, language) VALUES (?1, ?2, ?3)",
            rusqlite::params![&text, duration as i64, &language],
        )
        .map_err(|e| e.to_string())?;

        let word_count = text.split_whitespace().count() as i64;
        conn.execute(
            "INSERT INTO usage_stats (date, total_dictations, total_words, total_duration)
             VALUES (date('now'), 1, ?1, ?2)
             ON CONFLICT(date) DO UPDATE SET
                total_dictations = total_dictations + 1,
                total_words = total_words + ?1,
                total_duration = total_duration + ?2",
            rusqlite::params![word_count, duration as i64],
        )
        .map_err(|e| e.to_string())?;

        let auto_type = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'auto_type'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "true".to_string());

        if auto_type == "true" {
            let simulator = KeyboardSimulator::new();
            if let Err(e) = simulator.type_text(&text) {
                eprintln!("Auto-type failed: {}", e);
            }
        }

        let _ = app.emit("dictation-result", serde_json::json!({
            "text": &text,
            "duration": duration,
            "language": &language
        }));
    }

    Ok(text)
}

#[tauri::command]
pub fn get_dictation_status(state: State<'_, DictationState>) -> Result<serde_json::Value, String> {
    let is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    let is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "is_recording": *is_recording,
        "is_processing": *is_processing
    }))
}
