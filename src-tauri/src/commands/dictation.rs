use crate::audio::recorder::AudioRecorder;
use crate::db::Database;
use crate::keyboard::simulator::KeyboardSimulator;
use crate::whisper::transcriber::WhisperTranscriber;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, State};

const CHUNK_DURATION_SECS: f32 = 3.0;
const CHUNK_SAMPLES: usize = (16000.0 * CHUNK_DURATION_SECS) as usize;
const OVERLAP_SAMPLES: usize = (16000.0 * 0.5) as usize;
const POLL_INTERVAL_MS: u64 = 500;
const STREAMING_SILENCE_RMS: f32 = 0.001;

const HALLUCINATION_CONTAINS: &[&str] = &[
    "ترجمة",
    "نانسي",
    "قنقر",
    "شكرا لمشاهدتكم",
    "شكراً للمشاهدة",
    "شكرا للمشاهدة",
    "لا تنسى الاشتراك",
    "مشاهدة ممتعة",
    "تابعونا",
];

const HALLUCINATION_EXACT: &[&str] = &[
    "أعوذ بالله من الشيطان الرجيم",
    "بسم الله الرحمن الرحيم",
    "السلام عليكم",
    "اشترك",
    "مرحبا بكم",
    "صوت",
];

fn compute_rms(audio: &[f32]) -> f32 {
    if audio.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = audio.iter().map(|s| s * s).sum();
    (sum_sq / audio.len() as f32).sqrt()
}

fn is_chunk_hallucination(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return true;
    }
    let char_count = trimmed.chars().count();
    if char_count < 2 {
        return true;
    }
    for pattern in HALLUCINATION_CONTAINS {
        if trimmed.contains(pattern) {
            return true;
        }
    }
    for pattern in HALLUCINATION_EXACT {
        if trimmed == *pattern {
            return true;
        }
    }
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() > 2 {
        let first = chars[0];
        if chars.iter().all(|&c| c == first || c == ' ') {
            return true;
        }
    }
    false
}

pub struct DictationState {
    pub recorder: Mutex<AudioRecorder>,
    pub transcriber: Mutex<WhisperTranscriber>,
    pub is_recording: Mutex<bool>,
    pub is_processing: Mutex<bool>,
    pub streaming_active: Arc<AtomicBool>,
}

fn streaming_transcription_loop(
    streaming_active: Arc<AtomicBool>,
    app: tauri::AppHandle,
) {
    let mut last_processed_pos: usize = 0;
    let mut chunk_index: u32 = 0;

    eprintln!("[streaming] Loop started");

    while streaming_active.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));

        if !streaming_active.load(Ordering::SeqCst) {
            break;
        }

        let state: tauri::State<'_, DictationState> = app.state();

        let current_len = {
            let recorder = state.recorder.lock().unwrap();
            recorder.get_buffer_len()
        };

        let new_samples = current_len.saturating_sub(last_processed_pos);
        if new_samples < CHUNK_SAMPLES {
            continue;
        }

        let chunk_start = if last_processed_pos > OVERLAP_SAMPLES {
            last_processed_pos - OVERLAP_SAMPLES
        } else {
            0
        };

        let chunk_audio = {
            let recorder = state.recorder.lock().unwrap();
            let full_buffer = recorder.get_buffer_snapshot();
            full_buffer[chunk_start..current_len.min(full_buffer.len())].to_vec()
        };

        if chunk_audio.is_empty() {
            continue;
        }

        let rms = compute_rms(&chunk_audio);
        eprintln!("[streaming] Chunk RMS: {:.6}", rms);
        if rms < STREAMING_SILENCE_RMS {
            eprintln!("[streaming] Skipping silent chunk (RMS: {:.6})", rms);
            last_processed_pos = current_len;
            continue;
        }

        eprintln!("[streaming] Processing chunk {} ({} samples, {:.1}s, RMS: {:.4})", chunk_index, chunk_audio.len(), chunk_audio.len() as f64 / 16000.0, rms);

        let text = {
            let transcriber = state.transcriber.lock().unwrap();
            match transcriber.transcribe_chunk(&chunk_audio) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("[streaming] Chunk transcription error: {}", e);
                    last_processed_pos = current_len;
                    continue;
                }
            }
        };

        last_processed_pos = current_len;
        chunk_index += 1;

        if !text.trim().is_empty() && !is_chunk_hallucination(&text) {
            eprintln!("[streaming] Chunk {} result: '{}'", chunk_index, text.trim());
            let _ = app.emit("dictation-partial", serde_json::json!({
                "text": text.trim(),
                "chunk_index": chunk_index,
                "is_final": false
            }));
        } else if !text.trim().is_empty() {
            eprintln!("[streaming] Filtered chunk hallucination: '{}'", text.trim());
        }
    }
    eprintln!("[streaming] Loop ended");
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
    recorder.start().map_err(|e| e.to_string())?;
    *is_recording = true;
    drop(recorder);
    drop(is_recording);

    state.streaming_active.store(true, Ordering::SeqCst);

    let _ = app.emit("dictation-status", serde_json::json!({
        "is_recording": true,
        "is_processing": false
    }));

    let streaming_active = Arc::clone(&state.streaming_active);
    let app_handle = app.clone();

    std::thread::spawn(move || {
        streaming_transcription_loop(streaming_active, app_handle);
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_dictation(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    state.streaming_active.store(false, Ordering::SeqCst);
    eprintln!("[dictation] Waiting for streaming thread to finish...");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

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

    eprintln!("[dictation] Starting final Whisper transcription on full audio...");
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

    let _ = app.emit("dictation-partial", serde_json::json!({
        "text": &text,
        "chunk_index": -1,
        "is_final": true
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
