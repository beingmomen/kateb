use async_trait::async_trait;
use crate::error::AppError;

pub const SYSTEM_PROMPT: &str = "\
You are a text processing engine. You execute text transformation functions and return ONLY the raw output. \
You never explain, comment, or add anything beyond the function output. \
You never use tools. You never write code. You only return the processed result.";

pub fn build_user_message(raw_text: &str) -> String {
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

#[derive(Debug, Clone, PartialEq)]
pub enum AIProvider {
    Claude,
    OpenAI,
    Gemini,
    Grok,
    Local,
}

impl AIProvider {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "claude" => AIProvider::Claude,
            "openai" => AIProvider::OpenAI,
            "gemini" => AIProvider::Gemini,
            "grok" => AIProvider::Grok,
            "local" | _ => AIProvider::Local,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AIProvider::Claude => "claude",
            AIProvider::OpenAI => "openai",
            AIProvider::Gemini => "gemini",
            AIProvider::Grok => "grok",
            AIProvider::Local => "local",
        }
    }
}

#[async_trait]
pub trait AIRefiner: Send + Sync {
    async fn refine_streaming(
        &self,
        text: &str,
        app: &tauri::AppHandle,
    ) -> Result<String, AppError>;

    fn provider_name(&self) -> &'static str;

    async fn test_connection(&self) -> Result<bool, AppError>;
}

pub fn clean_refined_text(text: &str) -> String {
    text.trim()
        .trim_matches('"')
        .trim_matches('`')
        .trim()
        .to_string()
}
