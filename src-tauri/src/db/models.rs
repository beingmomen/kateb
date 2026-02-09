use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictationEntry {
    pub id: i64,
    pub text: String,
    pub raw_text: String,
    pub duration: i64,
    pub language: String,
    pub ai_provider: String,
    pub processing_time_ms: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageStat {
    pub id: i64,
    pub date: String,
    pub total_dictations: i64,
    pub total_words: i64,
    pub total_duration: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryStats {
    pub total_dictations: i64,
    pub total_words: i64,
    pub total_duration: i64,
    pub days_active: i64,
}

