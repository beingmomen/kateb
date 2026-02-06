use crate::ai::provider::{AIProvider, AIRefiner};
use crate::ai::providers::{ClaudeRefiner, GeminiRefiner, LocalRefiner, OpenAIRefiner};
use crate::db::Database;
use crate::error::AppError;
use std::sync::Arc;

pub struct AIFactory;

impl AIFactory {
    pub fn create(provider: AIProvider, api_key: Option<String>) -> Result<Arc<dyn AIRefiner>, AppError> {
        match provider {
            AIProvider::Claude => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("Claude API key is required".to_string())
                })?;
                Ok(Arc::new(ClaudeRefiner::new(key)))
            }
            AIProvider::OpenAI => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("OpenAI API key is required".to_string())
                })?;
                Ok(Arc::new(OpenAIRefiner::new(key)))
            }
            AIProvider::Gemini => {
                let key = api_key.ok_or_else(|| {
                    AppError::AIError("Gemini API key is required".to_string())
                })?;
                Ok(Arc::new(GeminiRefiner::new(key)))
            }
            AIProvider::Local => Ok(Arc::new(LocalRefiner::new())),
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
            AIProvider::Local => None,
        };

        Self::create(provider, api_key)
    }
}
