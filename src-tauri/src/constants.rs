pub mod audio {
    pub const SAMPLE_RATE: u32 = 16000;
    pub const CHUNK_DURATION_SECS: f32 = 3.0;
    pub const CHUNK_SAMPLES: usize = (16000.0 * CHUNK_DURATION_SECS) as usize;
    pub const OVERLAP_SAMPLES: usize = (16000.0 * 0.5) as usize;
    pub const POLL_INTERVAL_MS: u64 = 500;
    pub const STREAMING_SILENCE_RMS: f32 = 0.001;
    pub const MAX_BUFFER_SAMPLES: usize = 16000 * 600;
}

pub mod hallucination {
    pub const CONTAINS_PATTERNS: &[&str] = &[
        "ترجمة",
        "نانسي",
        "قنقر",
        "شكرا لمشاهدتكم",
        "شكراً للمشاهدة",
        "شكرا للمشاهدة",
        "لا تنسى الاشتراك",
        "مشاهدة ممتعة",
        "تابعونا",
    ];

    pub const EXACT_PATTERNS: &[&str] = &[
        "أعوذ بالله من الشيطان الرجيم",
        "بسم الله الرحمن الرحيم",
        "السلام عليكم",
        "اشترك",
        "مرحبا بكم",
        "صوت",
    ];
}

pub mod model {
    use serde::Serialize;

    pub const HUGGINGFACE_BASE: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";
    pub const DEFAULT_MODEL_ID: &str = "large-v3-turbo";

    #[derive(Debug, Clone, Serialize)]
    pub struct ModelInfo {
        pub id: &'static str,
        pub name: &'static str,
        pub filename: &'static str,
        pub size_bytes: u64,
        pub size_display: &'static str,
        pub accuracy: u8,
        pub speed: u8,
        pub ram_mb: u32,
        pub description_ar: &'static str,
        pub pros_ar: &'static [&'static str],
        pub cons_ar: &'static [&'static str],
        pub recommended: bool,
    }

    impl ModelInfo {
        pub fn download_url(&self) -> String {
            format!("{}/{}", HUGGINGFACE_BASE, self.filename)
        }
    }

    pub const AVAILABLE_MODELS: &[ModelInfo] = &[
        ModelInfo {
            id: "tiny",
            name: "Whisper Tiny",
            filename: "ggml-tiny.bin",
            size_bytes: 77_704_715,
            size_display: "75 MB",
            accuracy: 2,
            speed: 5,
            ram_mb: 273,
            description_ar: "نموذج صغير جداً وسريع، مناسب للاختبار والأجهزة الضعيفة",
            pros_ar: &["سريع جداً", "حجم صغير", "يعمل على أجهزة ضعيفة"],
            cons_ar: &["دقة منخفضة للعربية", "أخطاء كثيرة في النصوص الطويلة"],
            recommended: false,
        },
        ModelInfo {
            id: "base",
            name: "Whisper Base",
            filename: "ggml-base.bin",
            size_bytes: 147_964_211,
            size_display: "142 MB",
            accuracy: 3,
            speed: 4,
            ram_mb: 388,
            description_ar: "نموذج أساسي بتوازن بين السرعة والحجم",
            pros_ar: &["سريع", "حجم معقول", "مناسب للاستخدام اليومي البسيط"],
            cons_ar: &["دقة متوسطة للعربية", "قد يخطئ في الكلمات المعقدة"],
            recommended: false,
        },
        ModelInfo {
            id: "small",
            name: "Whisper Small",
            filename: "ggml-small.bin",
            size_bytes: 488_184_065,
            size_display: "466 MB",
            accuracy: 4,
            speed: 3,
            ram_mb: 852,
            description_ar: "نموذج متوسط بدقة جيدة، خيار ممتاز للاستخدام اليومي",
            pros_ar: &["دقة جيدة للعربية", "حجم معقول", "سرعة مقبولة"],
            cons_ar: &["أبطأ من tiny و base", "يحتاج ذاكرة أكثر"],
            recommended: false,
        },
        ModelInfo {
            id: "medium",
            name: "Whisper Medium",
            filename: "ggml-medium.bin",
            size_bytes: 1_533_774_781,
            size_display: "1.5 GB",
            accuracy: 4,
            speed: 2,
            ram_mb: 2100,
            description_ar: "نموذج كبير بدقة عالية للنصوص المعقدة",
            pros_ar: &["دقة عالية", "يتعامل جيداً مع اللهجات"],
            cons_ar: &["حجم كبير", "بطيء نسبياً", "يحتاج ذاكرة كبيرة"],
            recommended: false,
        },
        ModelInfo {
            id: "large-v3-turbo",
            name: "Whisper Large V3 Turbo",
            filename: "ggml-large-v3-turbo.bin",
            size_bytes: 1_624_555_520,
            size_display: "1.6 GB",
            accuracy: 5,
            speed: 3,
            ram_mb: 2100,
            description_ar: "أفضل نموذج من حيث الدقة مع سرعة محسّنة - الخيار الموصى به",
            pros_ar: &["أعلى دقة للعربية", "أسرع 6 مرات من large-v3", "يدعم كل اللهجات"],
            cons_ar: &["حجم كبير (1.6 GB)", "يحتاج ذاكرة 2+ GB"],
            recommended: true,
        },
    ];

    pub fn find_model(id: &str) -> Option<&'static ModelInfo> {
        AVAILABLE_MODELS.iter().find(|m| m.id == id)
    }
}

pub mod ai {
    pub const DEFAULT_PROVIDER: &str = "local";
    pub const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
    pub const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
    pub const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";
    pub const LOCAL_API_URL: &str = "http://localhost:8000/v1/chat/completions";
    pub const GROK_API_URL: &str = "https://api.x.ai/v1/chat/completions";
}
