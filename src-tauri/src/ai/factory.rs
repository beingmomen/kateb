use crate::ai::provider::{AIProvider, AIRefiner};
use crate::ai::providers::{ClaudeRefiner, GeminiRefiner, GrokRefiner, LocalRefiner, OpenAIRefiner};
use crate::db::Database;
use crate::error::AppError;
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

        let api_key = match provider {
            AIProvider::Claude => conn
                .query_row(
                    "SELECT value FROM settings WHERE key = 'claude_api_key'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .ok()
                .filter(|k| !k.is_empty()),
            AIProvider::OpenAI => conn
                .query_row(
                    "SELECT value FROM settings WHERE key = 'openai_api_key'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .ok()
                .filter(|k| !k.is_empty()),
            AIProvider::Gemini => conn
                .query_row(
                    "SELECT value FROM settings WHERE key = 'gemini_api_key'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .ok()
                .filter(|k| !k.is_empty()),
            AIProvider::Grok => conn
                .query_row(
                    "SELECT value FROM settings WHERE key = 'grok_api_key'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .ok()
                .filter(|k| !k.is_empty()),
            AIProvider::Local => conn
                .query_row(
                    "SELECT value FROM settings WHERE key = 'local_api_key'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .ok()
                .filter(|k| !k.is_empty()),
        };

        let url_key = format!("{}_api_url", provider_str);
        let base_url = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [&url_key],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .filter(|u| !u.is_empty());

        eprintln!("[ai-factory] provider = '{}', url_key = '{}', base_url = {:?}, api_key present = {}", provider_str, url_key, base_url, api_key.is_some());

        Self::create(provider, api_key, base_url)
    }
}
