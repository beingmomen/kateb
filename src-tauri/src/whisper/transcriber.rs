use std::path::PathBuf;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    ctx: Option<WhisperContext>,
    language: String,
    auto_punctuation: bool,
}

impl WhisperTranscriber {
    pub fn new() -> Self {
        Self {
            ctx: None,
            language: "ar".to_string(),
            auto_punctuation: true,
        }
    }

    pub fn load_model(&mut self, model_path: &PathBuf) -> Result<(), anyhow::Error> {
        let mut params = WhisperContextParameters::default();
        params.use_gpu(true);
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid model path"))?,
            params,
        )
        .map_err(|e| anyhow::anyhow!("فشل تحميل نموذج Whisper: {}", e))?;
        self.ctx = Some(ctx);
        Ok(())
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
    }

    pub fn get_language(&self) -> String {
        self.language.clone()
    }

    pub fn set_auto_punctuation(&mut self, enabled: bool) {
        self.auto_punctuation = enabled;
    }

    pub fn transcribe(&self, audio_data: &[f32]) -> Result<String, anyhow::Error> {
        eprintln!("[whisper] transcribe called with {} samples ({:.1}s of audio)", audio_data.len(), audio_data.len() as f64 / 16000.0);

        let ctx = self
            .ctx
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("النموذج غير محمّل"))?;

        eprintln!("[whisper] Model loaded, creating state...");
        let mut state = ctx.create_state()
            .map_err(|e| anyhow::anyhow!("فشل إنشاء حالة Whisper: {}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_no_timestamps(true);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_special(false);

        eprintln!("[whisper] Running transcription (language: {})...", self.language);
        let start = std::time::Instant::now();
        state
            .full(params, audio_data)
            .map_err(|e| anyhow::anyhow!("فشل التحويل: {}", e))?;
        eprintln!("[whisper] Transcription took {:.1}s", start.elapsed().as_secs_f64());

        let num_segments = state.full_n_segments();

        eprintln!("[whisper] Got {} segments", num_segments);
        let mut text = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(seg_text) = segment.to_str() {
                    eprintln!("[whisper] Segment {}: '{}'", i, seg_text);
                    text.push_str(seg_text);
                }
            }
        }

        eprintln!("[whisper] Final text: '{}'", text.trim());
        Ok(text.trim().to_string())
    }
}

unsafe impl Send for WhisperTranscriber {}
unsafe impl Sync for WhisperTranscriber {}
