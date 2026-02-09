use crate::ai::provider::{build_user_message, clean_refined_text, AIRefiner, SYSTEM_PROMPT};
use crate::constants::ai::OPENAI_API_URL;
use crate::error::AppError;
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tauri::Emitter;

const OPENAI_MODEL: &str = "gpt-4o";
const OPENAI_API_PATH: &str = "/v1/chat/completions";

pub struct OpenAIRefiner {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIRefiner {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let url = match base_url {
            Some(domain) => format!("{}{}", domain.trim().trim_end_matches('/'), OPENAI_API_PATH),
            None => OPENAI_API_URL.to_string(),
        };
        Self {
            client: Client::new(),
            api_key,
            base_url: url,
        }
    }
}

#[async_trait]
impl AIRefiner for OpenAIRefiner {
    fn provider_name(&self) -> &'static str {
        "OpenAI"
    }

    async fn test_connection(&self) -> Result<bool, AppError> {
        let response = self
            .client
            .post(&self.base_url)
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&json!({
                "model": OPENAI_MODEL,
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

        eprintln!("[openai] Sending text for refinement: '{}'", text);
        let _ = app.emit("ai-refine-status", json!({ "status": "started" }));

        let user_message = build_user_message(text);

        let response = self
            .client
            .post(&self.base_url)
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&json!({
                "model": OPENAI_MODEL,
                "max_tokens": 1024,
                "temperature": 0.0,
                "stream": true,
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
            return Err(AppError::AIError(format!("OpenAI API error {}: {}", status, body)));
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
                            .and_then(|c| c["delta"]["content"].as_str());

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
        eprintln!("[openai] Refinement complete: '{}'", refined);
        Ok(refined)
    }
}
