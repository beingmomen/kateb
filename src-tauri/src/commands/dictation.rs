use crate::ai::AIFactory;
use crate::audio::recorder::AudioRecorder;
use crate::constants::{audio::*, hallucination::*};
use crate::db::Database;
use crate::keyboard::simulator::KeyboardSimulator;
use crate::whisper::transcriber::WhisperTranscriber;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, State};

pub struct DictationState {
    pub recorder: Mutex<AudioRecorder>,
    pub transcriber: Mutex<WhisperTranscriber>,
    pub is_recording: Mutex<bool>,
    pub is_processing: Mutex<bool>,
    pub streaming_active: Arc<AtomicBool>,
    pub streaming_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

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
    for pattern in CONTAINS_PATTERNS {
        if trimmed.contains(pattern) {
            return true;
        }
    }
    for pattern in EXACT_PATTERNS {
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
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    if words.len() >= 4 {
        let first_word = words[0];
        let repeat_count = words.iter().filter(|&&w| w == first_word).count();
        if repeat_count as f32 / words.len() as f32 > 0.5 {
            return true;
        }
    }
    false
}

fn emit_status(app: &tauri::AppHandle, is_recording: bool, is_processing: bool) {
    let _ = app.emit(
        "dictation-status",
        serde_json::json!({
            "is_recording": is_recording,
            "is_processing": is_processing
        }),
    );
}

fn streaming_transcription_loop(streaming_active: Arc<AtomicBool>, app: tauri::AppHandle) {
    let mut last_processed_pos: usize = 0;
    let mut chunk_index: u32 = 0;

    eprintln!("[streaming] Loop started");

    while streaming_active.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));

        if !streaming_active.load(Ordering::SeqCst) {
            break;
        }

        let state: tauri::State<'_, DictationState> = app.state();

        let (current_len, audio_level) = {
            let recorder = state.recorder.lock().unwrap();
            (recorder.get_buffer_len(), recorder.get_audio_level())
        };

        let _ = app.emit("audio-level", serde_json::json!({ "level": audio_level }));

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

        eprintln!(
            "[streaming] Processing chunk {} ({} samples, {:.1}s, RMS: {:.4})",
            chunk_index,
            chunk_audio.len(),
            chunk_audio.len() as f64 / SAMPLE_RATE as f64,
            rms
        );

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
            let _ = app.emit(
                "dictation-partial",
                serde_json::json!({
                    "text": text.trim(),
                    "chunk_index": chunk_index,
                    "is_final": false
                }),
            );
        } else if !text.trim().is_empty() {
            eprintln!("[streaming] Filtered chunk hallucination: '{}'", text.trim());
        }
    }
    eprintln!("[streaming] Loop ended");
}

async fn stop_streaming_thread(state: &State<'_, DictationState>) -> Result<(), String> {
    state.streaming_active.store(false, Ordering::SeqCst);
    eprintln!("[dictation] Waiting for streaming thread to finish...");

    let handle = {
        let mut thread_handle = state.streaming_thread.lock().map_err(|e| e.to_string())?;
        thread_handle.take()
    };
    if let Some(h) = handle {
        tokio::task::spawn_blocking(move || {
            let _ = h.join();
        })
        .await
        .map_err(|e| e.to_string())?;
    }
    eprintln!("[dictation] Streaming thread joined successfully");
    Ok(())
}

fn capture_audio(state: &State<'_, DictationState>) -> Result<(Vec<f32>, u64), String> {
    let recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    let audio_data = match recorder.stop() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[dictation] ERROR stopping recorder: {}", e);
            return Err(e.to_string());
        }
    };
    let duration = recorder.get_duration_seconds();
    eprintln!(
        "[dictation] Audio captured: {} samples ({:.1}s), duration: {}s",
        audio_data.len(),
        audio_data.len() as f64 / SAMPLE_RATE as f64,
        duration
    );
    Ok((audio_data, duration))
}

fn transcribe_audio(
    state: &State<'_, DictationState>,
    audio_data: &[f32],
) -> Result<String, String> {
    eprintln!("[dictation] Starting final Whisper transcription on full audio...");
    let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
    match transcriber.transcribe(audio_data) {
        Ok(t) => {
            eprintln!(
                "[dictation] Transcription complete: '{}' ({} chars)",
                t,
                t.len()
            );
            Ok(t)
        }
        Err(e) => {
            eprintln!("[dictation] ERROR in transcription: {}", e);
            Err(e.to_string())
        }
    }
}

async fn refine_with_ai(
    text: &str,
    db: &State<'_, Database>,
    app: &tauri::AppHandle,
) -> String {
    let ai_enabled = {
        let conn = match db.0.lock() {
            Ok(c) => c,
            Err(_) => return text.to_string(),
        };
        conn.query_row(
            "SELECT value FROM settings WHERE key = 'ai_refinement'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "false".to_string())
    };

    if ai_enabled != "true" || text.trim().is_empty() {
        return text.to_string();
    }

    let refiner = match AIFactory::create_from_settings(db) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[ai] Failed to create refiner: {}", e);
            return text.to_string();
        }
    };

    eprintln!("[ai] AI refinement enabled, using {}...", refiner.provider_name());
    match refiner.refine_streaming(text, app).await {
        Ok(refined) if !refined.trim().is_empty() => {
            eprintln!("[ai] Refinement successful");
            refined
        }
        Ok(_) => {
            eprintln!("[ai] Refinement returned empty, using original");
            text.to_string()
        }
        Err(e) => {
            eprintln!("[ai] Refinement failed, using original: {}", e);
            text.to_string()
        }
    }
}

fn save_to_history(
    state: &State<'_, DictationState>,
    db: &State<'_, Database>,
    text: &str,
    duration: u64,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let language = {
        let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
        transcriber.get_language()
    };

    conn.execute(
        "INSERT INTO dictation_history (text, duration, language) VALUES (?1, ?2, ?3)",
        rusqlite::params![text, duration as i64, &language],
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

    Ok(())
}

fn auto_type_text(db: &State<'_, Database>, text: &str) -> Result<(), String> {
    let auto_type = {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        conn.query_row(
            "SELECT value FROM settings WHERE key = 'auto_type'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "true".to_string())
    };

    if auto_type == "true" {
        let simulator = KeyboardSimulator::new();
        if let Err(e) = simulator.type_text(text) {
            eprintln!("Auto-type failed: {}", e);
        }
    }
    Ok(())
}

fn emit_final_result(
    app: &tauri::AppHandle,
    text: &str,
    duration: u64,
    language: &str,
) {
    let _ = app.emit(
        "dictation-partial",
        serde_json::json!({
            "text": text,
            "chunk_index": -1,
            "is_final": true
        }),
    );

    let _ = app.emit(
        "dictation-result",
        serde_json::json!({
            "text": text,
            "duration": duration,
            "language": language
        }),
    );
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
    emit_status(&app, true, false);

    let streaming_active = Arc::clone(&state.streaming_active);
    let app_handle = app.clone();

    let handle = std::thread::spawn(move || {
        streaming_transcription_loop(streaming_active, app_handle);
    });

    {
        let mut thread_handle = state.streaming_thread.lock().map_err(|e| e.to_string())?;
        *thread_handle = Some(handle);
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_dictation(
    state: State<'_, DictationState>,
    db: State<'_, Database>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    {
        let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
        if !*is_recording {
            return Err("لا يوجد تسجيل نشط".to_string());
        }
        *is_recording = false;
    }

    stop_streaming_thread(&state).await?;

    let (audio_data, duration) = capture_audio(&state)?;

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = true;
    }
    emit_status(&app, false, true);

    if audio_data.is_empty() {
        eprintln!("[dictation] WARNING: Audio buffer is empty, nothing to transcribe");
        let mut is_processing = state
            .is_processing
            .lock()
            .map_err(|_| "lock error".to_string())?;
        *is_processing = false;
        emit_status(&app, false, false);
        return Ok(String::new());
    }

    let text = match transcribe_audio(&state, &audio_data) {
        Ok(t) => t,
        Err(e) => {
            let mut is_processing = state
                .is_processing
                .lock()
                .map_err(|_| "lock error".to_string())?;
            *is_processing = false;
            emit_status(&app, false, false);
            return Err(e);
        }
    };

    let text = refine_with_ai(&text, &db, &app).await;

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = false;
    }
    emit_status(&app, false, false);

    if !text.trim().is_empty() && !is_chunk_hallucination(&text) {
        save_to_history(&state, &db, &text, duration)?;
        auto_type_text(&db, &text)?;

        let language = {
            let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
            transcriber.get_language()
        };
        emit_final_result(&app, &text, duration, &language);
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
