use crate::ai::provider::{build_user_message, clean_refined_text, AIRefiner, SYSTEM_PROMPT};
use crate::constants::ai::LOCAL_API_URL;
use crate::error::AppError;
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tauri::Emitter;

const LOCAL_MODEL: &str = "claude-sonnet-4-5-20250929";

pub struct LocalRefiner {
    client: Client,
}

impl LocalRefiner {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for LocalRefiner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIRefiner for LocalRefiner {
    fn provider_name(&self) -> &'static str {
        "Local"
    }

    async fn test_connection(&self) -> Result<bool, AppError> {
        let response = self
            .client
            .post(LOCAL_API_URL)
            .header("content-type", "application/json")
            .json(&json!({
                "model": LOCAL_MODEL,
                "max_tokens": 10,
                "messages": [{"role": "user", "content": "Hi"}]
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

        eprintln!("[local] Sending text for refinement: '{}'", text);
        let _ = app.emit("ai-refine-status", json!({ "status": "started" }));

        let user_message = build_user_message(text);

        let response = self
            .client
            .post(LOCAL_API_URL)
            .header("content-type", "application/json")
            .header("X-Claude-Max-Turns", "1")
            .header("X-Claude-Allowed-Tools", "")
            .json(&json!({
                "model": LOCAL_MODEL,
                "max_tokens": 1024,
                "temperature": 0.0,
                "stream": true,
                "enable_tools": false,
                "messages": [
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": user_message}
                ]
            }))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let _ = app.emit("ai-refine-status", json!({ "status": "error" }));
            return Err(AppError::AIError(format!("Local API error {}: {}", status, body)));
        }

        let mut full_text = String::new();
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| AppError::NetworkError(e.to_string()))?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.push_str(&chunk_str);

            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_string();
                buffer = buffer[line_end + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(json_str) = line.strip_prefix("data: ") {
                    if json_str == "[DONE]" {
                        continue;
                    }
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let content = data["choices"]
                            .as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|c| c["delta"]["content"].as_str())
                            .or_else(|| data["delta"]["text"].as_str());

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
                }
            }
        }

        let refined = clean_refined_text(&full_text);
        let _ = app.emit("ai-refine-status", json!({ "status": "done" }));
        eprintln!("[local] Refinement complete: '{}'", refined);
        Ok(refined)
    }
}
