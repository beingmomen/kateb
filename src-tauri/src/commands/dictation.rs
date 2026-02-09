use crate::ai::AIFactory;
use crate::audio::preprocessor::AudioPreprocessor;
use crate::audio::recorder::AudioRecorder;
use crate::audio::vad::AdaptiveVAD;
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
    pub accumulated_text: Arc<Mutex<Vec<String>>>,
    pub vad: Arc<Mutex<AdaptiveVAD>>,
}

fn is_chunk_hallucination(text: &str, audio_duration_secs: f32) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return true;
    }
    let char_count = trimmed.chars().count();
    if char_count < 2 {
        return true;
    }
    if audio_duration_secs > 2.0 && char_count < 3 {
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

    let state: tauri::State<'_, DictationState> = app.state();

    {
        let mut vad = state.vad.lock().unwrap();
        vad.reset();
    }

    let auto_stop_enabled = {
        let db: tauri::State<'_, Database> = app.state();
        let conn = db.0.lock().unwrap();
        let enabled = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'auto_stop_silence'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "true".to_string());
        let seconds: f32 = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'auto_stop_seconds'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(AUTO_STOP_SILENCE_SECS);
        (enabled == "true", seconds)
    };

    while streaming_active.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));

        if !streaming_active.load(Ordering::SeqCst) {
            break;
        }

        let (current_len, audio_level) = {
            let recorder = state.recorder.lock().unwrap();
            (recorder.get_buffer_len(), recorder.get_audio_level())
        };

        let _ = app.emit("audio-level", serde_json::json!({ "level": audio_level }));

        let new_samples = current_len.saturating_sub(last_processed_pos);
        if new_samples < CHUNK_SAMPLES {
            if new_samples > 0 {
                let recent_audio = {
                    let recorder = state.recorder.lock().unwrap();
                    let full_buffer = recorder.get_buffer_snapshot();
                    let start = current_len.saturating_sub(new_samples);
                    full_buffer[start..current_len.min(full_buffer.len())].to_vec()
                };
                let mut vad = state.vad.lock().unwrap();
                vad.feed(&recent_audio);

                if auto_stop_enabled.0 {
                    let silence_dur = vad.silence_duration_secs();
                    let has_text = {
                        let acc = state.accumulated_text.lock().unwrap();
                        !acc.is_empty()
                    };
                    if has_text {
                        let remaining = auto_stop_enabled.1 - silence_dur;
                        let _ = app.emit("silence-countdown", serde_json::json!({
                            "remaining": remaining,
                            "total": auto_stop_enabled.1
                        }));
                        if silence_dur >= auto_stop_enabled.1 {
                            eprintln!("[streaming] Auto-stop: {:.1}s silence detected", silence_dur);
                            let _ = app.emit("dictation-auto-stop", serde_json::json!({}));
                            break;
                        }
                    }
                }
            }
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

        {
            let mut vad = state.vad.lock().unwrap();
            let is_speech = vad.feed(&chunk_audio);
            if !is_speech {
                eprintln!("[streaming] VAD: no speech detected, skipping chunk");
                last_processed_pos = current_len;

                if auto_stop_enabled.0 {
                    let silence_dur = vad.silence_duration_secs();
                    let has_text = {
                        let acc = state.accumulated_text.lock().unwrap();
                        !acc.is_empty()
                    };
                    if has_text {
                        let remaining = auto_stop_enabled.1 - silence_dur;
                        let _ = app.emit("silence-countdown", serde_json::json!({
                            "remaining": remaining,
                            "total": auto_stop_enabled.1
                        }));
                        if silence_dur >= auto_stop_enabled.1 {
                            eprintln!("[streaming] Auto-stop: {:.1}s silence detected", silence_dur);
                            let _ = app.emit("dictation-auto-stop", serde_json::json!({}));
                            break;
                        }
                    }
                }

                continue;
            }

            if auto_stop_enabled.0 {
                let _ = app.emit("silence-countdown", serde_json::json!({
                    "remaining": auto_stop_enabled.1,
                    "total": auto_stop_enabled.1
                }));
            }
        }

        let processed_audio = AudioPreprocessor::process(&chunk_audio);

        eprintln!(
            "[streaming] Processing chunk {} ({} samples, {:.1}s)",
            chunk_index,
            processed_audio.len(),
            processed_audio.len() as f64 / SAMPLE_RATE as f64,
        );

        let text = {
            let transcriber = state.transcriber.lock().unwrap();
            match transcriber.transcribe_chunk(&processed_audio) {
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

        let chunk_duration = chunk_audio.len() as f32 / SAMPLE_RATE as f32;
        if !text.trim().is_empty() && !is_chunk_hallucination(&text, chunk_duration) {
            eprintln!("[streaming] Chunk {} result: '{}'", chunk_index, text.trim());

            {
                let mut acc = state.accumulated_text.lock().unwrap();
                acc.push(text.trim().to_string());
            }

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

struct RefinementResult {
    text: String,
    ai_provider: String,
    processing_time_ms: u64,
}

async fn refine_with_ai(
    text: &str,
    db: &State<'_, Database>,
    app: &tauri::AppHandle,
) -> RefinementResult {
    let ai_enabled = {
        let conn = match db.0.lock() {
            Ok(c) => c,
            Err(_) => return RefinementResult { text: text.to_string(), ai_provider: String::new(), processing_time_ms: 0 },
        };
        conn.query_row(
            "SELECT value FROM settings WHERE key = 'ai_refinement'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "false".to_string())
    };

    if ai_enabled != "true" || text.trim().is_empty() {
        return RefinementResult { text: text.to_string(), ai_provider: String::new(), processing_time_ms: 0 };
    }

    let refiner = match AIFactory::create_from_settings(db) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[ai] Failed to create refiner: {}", e);
            return RefinementResult { text: text.to_string(), ai_provider: String::new(), processing_time_ms: 0 };
        }
    };

    let provider_name = refiner.provider_name().to_string();
    eprintln!("[ai] AI refinement enabled, using {}...", provider_name);

    let ai_start = std::time::Instant::now();
    let result = match refiner.refine_streaming(text, app).await {
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
    };
    let processing_time_ms = ai_start.elapsed().as_millis() as u64;
    eprintln!("[ai] Processing took {}ms", processing_time_ms);

    RefinementResult { text: result, ai_provider: provider_name, processing_time_ms }
}

fn save_to_history(
    state: &State<'_, DictationState>,
    db: &State<'_, Database>,
    text: &str,
    raw_text: &str,
    duration: u64,
    ai_provider: &str,
    processing_time_ms: u64,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let language = {
        let transcriber = state.transcriber.lock().map_err(|e| e.to_string())?;
        transcriber.get_language()
    };

    conn.execute(
        "INSERT INTO dictation_history (text, raw_text, duration, language, ai_provider, processing_time_ms) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![text, raw_text, duration as i64, &language, ai_provider, processing_time_ms as i64],
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

    {
        let mut acc = state.accumulated_text.lock().map_err(|e| e.to_string())?;
        acc.clear();
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

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = true;
    }
    emit_status(&app, false, true);

    stop_streaming_thread(&state).await?;

    let (audio_data, duration) = capture_audio(&state)?;

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

    let accumulated = {
        let acc = state.accumulated_text.lock().map_err(|e| e.to_string())?;
        acc.join(" ")
    };

    let text = if !accumulated.trim().is_empty() {
        eprintln!("[dictation] Using accumulated streaming text ({} chars), skipping re-transcription", accumulated.len());
        accumulated
    } else {
        eprintln!("[dictation] No accumulated text, falling back to full re-transcription");
        match transcribe_audio(&state, &audio_data) {
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
        }
    };

    let raw_text = text.clone();
    let refinement = refine_with_ai(&text, &db, &app).await;
    let text = refinement.text;
    let ai_provider = refinement.ai_provider;
    let processing_time_ms = refinement.processing_time_ms;

    {
        let mut is_processing = state.is_processing.lock().map_err(|e| e.to_string())?;
        *is_processing = false;
    }
    emit_status(&app, false, false);

    let total_duration = audio_data.len() as f32 / SAMPLE_RATE as f32;
    if !text.trim().is_empty() && !is_chunk_hallucination(&text, total_duration) {
        let save_raw = if ai_provider.is_empty() { "" } else { &raw_text };
        save_to_history(&state, &db, &text, save_raw, duration, &ai_provider, processing_time_ms)?;
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
