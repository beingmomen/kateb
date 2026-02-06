use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("التسجيل قيد التشغيل بالفعل")]
    AlreadyRecording,

    #[error("لا يوجد تسجيل نشط")]
    NotRecording,

    #[error("النموذج غير محمّل")]
    ModelNotLoaded,

    #[error("فشل تحميل نموذج Whisper: {0}")]
    ModelLoadFailed(String),

    #[error("فشل التحويل: {0}")]
    TranscriptionFailed(String),

    #[error("خطأ في قاعدة البيانات: {0}")]
    DatabaseError(String),

    #[error("خطأ في AI: {0}")]
    AIError(String),

    #[error("خطأ في القفل: {0}")]
    LockError(String),

    #[error("خطأ في تحميل الموديل: {0}")]
    DownloadError(String),

    #[error("خطأ في الشبكة: {0}")]
    NetworkError(String),
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}
