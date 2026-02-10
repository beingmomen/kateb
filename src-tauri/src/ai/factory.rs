use crate::ai::provider::{AIProvider, AIRefiner};
use crate::ai::providers::{ClaudeRefiner, GeminiRefiner, GrokRefiner, LocalRefiner, OpenAIRefiner};
use crate::db::Database;
use crate::error::AppError;
use crate::security::keychain;
use std::sync::Arc;

pub struct AIFactory;

impl AIFactory {
    pub fn create(provider: AIProvider, api_key: Option<String>, base_url: Option<String>) -> Result<Arc<dyn AIRefiner>, AppError> {
        match provider {
            AIProvider::Claude => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("Claude API key is required".to_string())
                })?;
                Ok(Arc::new(ClaudeRefiner::new(key, base_url)))
            }
            AIProvider::OpenAI => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("OpenAI API key is required".to_string())
                })?;
                Ok(Arc::new(OpenAIRefiner::new(key, base_url)))
            }
            AIProvider::Gemini => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("Gemini API key is required".to_string())
                })?;
                Ok(Arc::new(GeminiRefiner::new(key, base_url)))
            }
            AIProvider::Grok => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("Grok API key is required".to_string())
                })?;
                Ok(Arc::new(GrokRefiner::new(key, base_url)))
            }
            AIProvider::Local => Ok(Arc::new(LocalRefiner::new(api_key, base_url))),
        }
    }

    fn get_api_key(provider: &AIProvider, provider_str: &str, conn: &rusqlite::Connection) -> Option<String> {
        let key_name = format!("{}_api_key", provider_str);

        if keychain::is_available() {
            if let Some(val) = keychain::retrieve_api_key(&key_name) {
                return Some(val);
            }
        }

        let db_key = match provider {
            AIProvider::Claude => "claude_api_key",
            AIProvider::OpenAI => "openai_api_key",
            AIProvider::Gemini => "gemini_api_key",
            AIProvider::Grok => "grok_api_key",
            AIProvider::Local => "local_api_key",
        };

        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [db_key],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .filter(|k| !k.is_empty())
    }

    pub fn create_from_settings(db: &Database) -> Result<Arc<dyn AIRefiner>, AppError> {
        let conn = db.0.lock().map_err(|e| AppError::LockError(e.to_string()))?;

        let provider_str = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'ai_provider'",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "local".to_string());

        let provider = AIProvider::from_str(&provider_str);
        let api_key = Self::get_api_key(&provider, &provider_str, &conn);

        let url_key = format!("{}_api_url", provider_str);
        let base_url = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [&url_key],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .filter(|u| !u.is_empty());

        tracing::info!("[ai-factory] provider = '{}', base_url = {:?}, api_key present = {}", provider_str, base_url, api_key.is_some());

        Self::create(provider, api_key, base_url)
    }
}
