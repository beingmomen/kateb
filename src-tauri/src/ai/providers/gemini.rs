use crate::ai::provider::{build_user_message, clean_refined_text, AIRefiner, SYSTEM_PROMPT};
use crate::error::AppError;
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tauri::Emitter;

const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const GEMINI_MODEL: &str = "gemini-2.0-flash";

pub struct GeminiRefiner {
    client: Client,
    api_key: String,
}

impl GeminiRefiner {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    fn build_url(&self, stream: bool) -> String {
        let action = if stream {
            "streamGenerateContent"
        } else {
            "generateContent"
        };
        format!(
            "{}/{}:{}?key={}",
            GEMINI_BASE_URL, GEMINI_MODEL, action, self.api_key
        )
    }
}

#[async_trait]
impl AIRefiner for GeminiRefiner {
    fn provider_name(&self) -> &'static str {
        "Gemini"
    }

    async fn test_connection(&self) -> Result<bool, AppError> {
        let url = self.build_url(false);
        let response = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .json(&json!({
                "contents": [{"parts": [{"text": "Hi"}]}],
                "generationConfig": {"maxOutputTokens": 10}
            }))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        Ok(response.status().is_success())
    }

    async fn refine_streaming(
        &self,
        text: &str,
        app: &tauri::AppHandle,
    ) -> Result<String, AppError> {
        if text.trim().is_empty() {
            return Ok(text.to_string());
        }

        eprintln!("[gemini] Sending text for refinement: '{}'", text);
        let _ = app.emit("ai-refine-status", json!({ "status": "started" }));

        let user_message = build_user_message(text);
        let url = self.build_url(true);

        let response = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .json(&json!({
                "systemInstruction": {"parts": [{"text": SYSTEM_PROMPT}]},
                "contents": [{"parts": [{"text": user_message}]}],
                "generationConfig": {
                    "temperature": 0.0,
                    "maxOutputTokens": 1024
                }
            }))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let _ = app.emit("ai-refine-status", json!({ "status": "error" }));
            return Err(AppError::AIError(format!("Gemini API error {}: {}", status, body)));
        }

        let mut full_text = String::new();
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| AppError::NetworkError(e.to_string()))?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.push_str(&chunk_str);

            while let Some(obj_start) = buffer.find('{') {
                let remaining = &buffer[obj_start..];
                let mut depth = 0;
                let mut obj_end = None;

                for (i, c) in remaining.char_indices() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                obj_end = Some(obj_start + i + 1);
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(end) = obj_end {
                    let json_str = buffer[obj_start..end].to_string();
                    buffer = buffer[end..].to_string();

                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        let content = data["candidates"]
                            .as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|c| c["content"]["parts"].as_array())
                            .and_then(|parts| parts.first())
                            .and_then(|p| p["text"].as_str());

                        if let Some(text_chunk) = content {
                            if !text_chunk.is_empty() {
                                full_text.push_str(text_chunk);
                                let _ = app.emit("ai-refine-chunk", json!({
                                    "chunk": text_chunk,
                                    "accumulated": &full_text
                                }));
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }

        let refined = clean_refined_text(&full_text);
        let _ = app.emit("ai-refine-status", json!({ "status": "done" }));
        eprintln!("[gemini] Refinement complete: '{}'", refined);
        Ok(refined)
    }
}
