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
    pub const HUGGINGFACE_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo.bin";
    pub const FILENAME: &str = "ggml-large-v3-turbo.bin";
    pub const EXPECTED_SIZE_BYTES: u64 = 1_624_555_520;
}

pub mod ai {
    pub const DEFAULT_PROVIDER: &str = "local";
    pub const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
    pub const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
    pub const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";
    pub const LOCAL_API_URL: &str = "http://localhost:8000/v1/chat/completions";
}
