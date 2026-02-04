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
    pub duration: i64,
    pub language: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictationStatus {
    pub is_recording: bool,
    pub is_processing: bool,
    pub duration_seconds: u64,
    pub error: Option<String>,
}

impl Default for DictationStatus {
    fn default() -> Self {
        Self {
            is_recording: false,
            is_processing: false,
            duration_seconds: 0,
            error: None,
        }
    }
}
