use std::path::PathBuf;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    ctx: Option<WhisperContext>,
    language: String,
}

impl WhisperTranscriber {
    pub fn new() -> Self {
        Self {
            ctx: None,
            language: "ar".to_string(),
        }
    }

    pub fn load_model(&mut self, model_path: &PathBuf, use_gpu: bool) -> Result<(), anyhow::Error> {
        let mut params = WhisperContextParameters::default();
        params.use_gpu(use_gpu);
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid model path"))?,
            params,
        )
        .map_err(|e| anyhow::anyhow!("فشل تحميل نموذج Whisper: {}", e))?;
        self.ctx = Some(ctx);
        Ok(())
    }

    pub fn get_language(&self) -> String {
        self.language.clone()
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
    }

    fn apply_anti_hallucination(params: &mut FullParams, language: &str) {
        params.set_suppress_blank(true);
        params.set_suppress_nst(true);
        params.set_no_speech_thold(0.6);
        params.set_entropy_thold(2.4);
        params.set_logprob_thold(-1.0);
        params.set_temperature(0.0);
        params.set_temperature_inc(0.0);
        let initial_prompt = match language {
            "en" => "Voice dictation in English. The text contains complete sentences with proper punctuation. No songs, music, or subtitles.",
            _ => "إملاء صوتي باللغة العربية الفصحى والعامية. النص يحتوي على جمل كاملة مع علامات ترقيم صحيحة، ولا يحتوي على أناشيد أو موسيقى أو ترجمات.",
        };
        params.set_initial_prompt(initial_prompt);
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

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 3 });
        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_no_timestamps(true);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_special(false);
        Self::apply_anti_hallucination(&mut params, &self.language);

        eprintln!("[whisper] Running transcription with best_of=3 (language: {})...", self.language);
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

    pub fn transcribe_chunk(&self, audio_chunk: &[f32]) -> Result<String, anyhow::Error> {
        let ctx = self
            .ctx
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("النموذج غير محمّل"))?;

        let mut state = ctx.create_state()
            .map_err(|e| anyhow::anyhow!("فشل إنشاء حالة Whisper: {}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_no_timestamps(true);
        params.set_single_segment(true);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_no_context(true);
        Self::apply_anti_hallucination(&mut params, &self.language);

        state
            .full(params, audio_chunk)
            .map_err(|e| anyhow::anyhow!("فشل التحويل: {}", e))?;

        let num_segments = state.full_n_segments();
        let mut text = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(seg_text) = segment.to_str() {
                    text.push_str(seg_text);
                }
            }
        }

        Ok(text.trim().to_string())
    }
}

unsafe impl Send for WhisperTranscriber {}
unsafe impl Sync for WhisperTranscriber {}
