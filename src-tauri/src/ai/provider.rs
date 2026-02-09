use async_trait::async_trait;
use crate::error::AppError;

pub const SYSTEM_PROMPT: &str = "\
You are a text processing engine. You execute text transformation functions and return ONLY the raw output. \
You never explain, comment, or add anything beyond the function output. \
You never use tools. You never write code. You only return the processed result.";

pub fn build_user_message(raw_text: &str, language: &str) -> String {
    match language {
        "en" => format!(
            r#"Execute this text processing function and return ONLY its output, nothing else:

```
function correctEnglishText(input: string): string {{
    // 1. Remove filler words: um, uh, like, you know, basically, so, well, I mean
    // 2. Fix spelling errors
    // 3. Fix grammar errors
    // 4. Add proper punctuation: commas, periods, question marks
    // 5. Preserve the speaker's style (formal/informal) - do NOT make casual speech formal
    // 6. Do NOT add new content, do NOT translate, do NOT summarize
    // 7. Return the corrected text string only
}}
```

correctEnglishText("{}")

Output:"#,
            raw_text
        ),
        _ => format!(
            r#"Execute this text processing function and return ONLY its output, nothing else:

```
function correctArabicText(input: string): string {{
    // 1. Remove Arabic filler words: أم، آه، يعني، مم، إيه، طيب، خلاص، هاه، إممم، آآه
    // 2. Fix Arabic spelling errors (e.g., انا → أنا, اشتري → أشتري)
    // 3. Fix Arabic grammar errors
    // 4. Add proper Arabic punctuation: commas (،), periods (.), question marks (؟)
    // 5. Keep English words as-is (do NOT translate them to Arabic)
    // 6. Preserve the speaker's style (formal/informal) - do NOT make casual speech formal
    // 7. Do NOT add new content, do NOT translate, do NOT summarize
    // 8. Return the corrected text string only
}}
```

correctArabicText("{}")

Output:"#,
            raw_text
        ),
    }
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
        language: &str,
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
