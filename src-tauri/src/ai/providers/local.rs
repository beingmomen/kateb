use crate::ai::provider::{build_user_message, clean_refined_text, AIRefiner, SYSTEM_PROMPT};
use crate::constants::ai::LOCAL_API_URL;
use crate::error::AppError;
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tauri::Emitter;

const LOCAL_MODEL: &str = "claude-sonnet-4-5-20250929";
const LOCAL_API_PATH: &str = "/v1/chat/completions";

pub struct LocalRefiner {
    client: Client,
    api_key: Option<String>,
    base_url: String,
}

impl LocalRefiner {
    pub fn new(api_key: Option<String>, base_url: Option<String>) -> Self {
        let url = match base_url {
            Some(domain) => format!("{}{}", domain.trim().trim_end_matches('/'), LOCAL_API_PATH),
            None => LOCAL_API_URL.to_string(),
        };
        Self {
            client: Client::new(),
            api_key,
            base_url: url,
        }
    }
}

impl Default for LocalRefiner {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[async_trait]
impl AIRefiner for LocalRefiner {
    fn provider_name(&self) -> &'static str {
        "Local"
    }

    async fn test_connection(&self) -> Result<bool, AppError> {
        let mut req = self
            .client
            .post(&self.base_url)
            .header("content-type", "application/json");
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }
        let response = req
            .json(&json!({
                "model": LOCAL_MODEL,
                "max_tokens": 10,
                "messages": [{"role": "user", "content": "Hi"}]
            }))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Ok(false);
        }
        let body = response.text().await.unwrap_or_default();
        if body.contains("Invalid API key") || body.contains("Unauthorized") {
            return Ok(false);
        }
        Ok(true)
    }

    async fn refine_streaming(
        &self,
        text: &str,
        language: &str,
        app: &tauri::AppHandle,
    ) -> Result<String, AppError> {
        if text.trim().is_empty() {
            return Ok(text.to_string());
        }

        tracing::debug!("[local] Sending text for refinement: '{}'", text);
        let _ = app.emit("ai-refine-status", json!({ "status": "started" }));

        let user_message = build_user_message(text, language);

        let mut req = self
            .client
            .post(&self.base_url)
            .header("content-type", "application/json")
            .header("X-Claude-Max-Turns", "1")
            .header("X-Claude-Allowed-Tools", "");
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }
        let response = req
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

        if refined.contains("Invalid API key")
            || refined.contains("Unauthorized")
            || refined.contains("Please run /login")
        {
            let _ = app.emit("ai-refine-status", json!({ "status": "error" }));
            tracing::error!("[local] Auth error in response: '{}'", refined);
            return Err(AppError::AIError(format!("خطأ في المصادقة: {}", refined)));
        }

        let _ = app.emit("ai-refine-status", json!({ "status": "done" }));
        tracing::debug!("[local] Refinement complete: '{}'", refined);
        Ok(refined)
    }
}
