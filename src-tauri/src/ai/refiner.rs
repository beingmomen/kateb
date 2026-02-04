use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tauri::Emitter;

const API_URL: &str = "http://localhost:8000/v1/chat/completions";
const MODEL: &str = "claude-sonnet-4-5-20250929";

const SYSTEM_PROMPT: &str = "\
You are a text processing engine. You execute text transformation functions and return ONLY the raw output. \
You never explain, comment, or add anything beyond the function output. \
You never use tools. You never write code. You only return the processed result.";

fn build_user_message(raw_text: &str) -> String {
    format!(
        r#"Execute this text processing function and return ONLY its output, nothing else:

```
function correctArabicText(input: string): string {{
    // 1. Fix Arabic spelling errors (e.g., انا → أنا, اشتري → أشتري)
    // 2. Fix Arabic grammar errors
    // 3. Add proper Arabic punctuation: commas (،), periods (.), question marks (؟)
    // 4. Do NOT add, remove, or translate any words
    // 5. Return the corrected Arabic text string only
}}
```

correctArabicText("{}")

Output:"#,
        raw_text
    )
}

pub struct TextRefiner {
    client: Client,
}

impl TextRefiner {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn refine_streaming(
        &self,
        raw_text: &str,
        app: &tauri::AppHandle,
    ) -> Result<String, anyhow::Error> {
        if raw_text.trim().is_empty() {
            return Ok(raw_text.to_string());
        }

        eprintln!("[ai] Sending text for refinement: '{}'", raw_text);

        let _ = app.emit("ai-refine-status", json!({ "status": "started" }));

        let user_message = build_user_message(raw_text);

        let response = self
            .client
            .post(API_URL)
            .header("content-type", "application/json")
            .header("X-Claude-Max-Turns", "1")
            .header("X-Claude-Allowed-Tools", "")
            .json(&json!({
                "model": MODEL,
                "max_tokens": 1024,
                "temperature": 0.0,
                "stream": true,
                "enable_tools": false,
                "messages": [
                    {
                        "role": "system",
                        "content": SYSTEM_PROMPT
                    },
                    {
                        "role": "user",
                        "content": &user_message
                    }
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let _ = app.emit("ai-refine-status", json!({ "status": "error" }));
            return Err(anyhow::anyhow!("API error {}: {}", status, body));
        }

        let mut full_text = String::new();
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
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

                        if let Some(text) = content {
                            if !text.is_empty() {
                                full_text.push_str(text);
                                let _ = app.emit("ai-refine-chunk", json!({
                                    "chunk": text,
                                    "accumulated": &full_text
                                }));
                            }
                        }
                    }
                }
            }
        }

        let refined = full_text
            .trim()
            .trim_matches('"')
            .trim_matches('`')
            .trim()
            .to_string();

        let _ = app.emit("ai-refine-status", json!({ "status": "done" }));
        eprintln!("[ai] Refinement complete: '{}'", refined);
        Ok(refined)
    }
}
