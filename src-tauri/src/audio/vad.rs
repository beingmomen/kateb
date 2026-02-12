use crate::constants::audio::SAMPLE_RATE;

pub struct AdaptiveVAD {
    noise_floor: f32,
    speech_threshold: f32,
    silence_frames: u32,
    total_frames: u32,
    speech_frames: u32,
    calibrated: bool,
    calibration_samples: Vec<f32>,
}

impl AdaptiveVAD {
    pub fn new() -> Self {
        Self {
            noise_floor: 0.003,
            speech_threshold: 0.009,
            silence_frames: 0,
            total_frames: 0,
            speech_frames: 0,
            calibrated: false,
            calibration_samples: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.noise_floor = 0.003;
        self.speech_threshold = 0.009;
        self.silence_frames = 0;
        self.total_frames = 0;
        self.speech_frames = 0;
        self.calibrated = false;
        self.calibration_samples.clear();
    }

    fn compute_rms(audio: &[f32]) -> f32 {
        if audio.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = audio.iter().map(|s| s * s).sum();
        (sum_sq / audio.len() as f32).sqrt()
    }

    pub fn feed(&mut self, audio: &[f32]) -> bool {
        let rms = Self::compute_rms(audio);

        let calibration_duration = (SAMPLE_RATE as f32 * 0.5) as usize;
        if !self.calibrated {
            self.calibration_samples.extend_from_slice(audio);
            if self.calibration_samples.len() >= calibration_duration {
                self.noise_floor = Self::compute_rms(&self.calibration_samples);
                self.speech_threshold = (self.noise_floor * 3.0).max(0.003);
                self.calibrated = true;
                self.calibration_samples.clear();
                tracing::info!(
                    "[vad] Calibrated: noise_floor={:.6}, speech_threshold={:.6}",
                    self.noise_floor, self.speech_threshold
                );
            }
            return false;
        }

        let is_speech = rms > self.speech_threshold;

        self.total_frames += 1;
        if is_speech {
            self.speech_frames += 1;
            self.silence_frames = 0;
        } else {
            self.silence_frames += 1;
            self.noise_floor = self.noise_floor * 0.95 + rms * 0.05;
            self.speech_threshold = (self.noise_floor * 3.0).max(0.003);
        }

        is_speech
    }

    pub fn silence_duration_secs(&self) -> f32 {
        let samples_per_frame = SAMPLE_RATE as f32 * 0.25;
        self.silence_frames as f32 * samples_per_frame / SAMPLE_RATE as f32
    }

    pub fn speech_ratio(&self) -> f32 {
        if self.total_frames == 0 {
            return 0.0;
        }
        self.speech_frames as f32 / self.total_frames as f32
    }
}
