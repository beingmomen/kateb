pub struct AudioPreprocessor;

impl AudioPreprocessor {
    pub fn process(samples: &[f32]) -> Vec<f32> {
        let mut data = Self::high_pass_filter(samples);
        Self::rms_normalize(&mut data);
        Self::pre_emphasis(&mut data);
        data
    }

    fn high_pass_filter(samples: &[f32]) -> Vec<f32> {
        let alpha = 0.995;
        let mut output = vec![0.0f32; samples.len()];
        if samples.is_empty() {
            return output;
        }
        output[0] = samples[0];
        for i in 1..samples.len() {
            output[i] = alpha * (output[i - 1] + samples[i] - samples[i - 1]);
        }
        output
    }

    fn rms_normalize(samples: &mut [f32]) {
        if samples.is_empty() {
            return;
        }
        let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
        let rms = (sum_sq / samples.len() as f32).sqrt();
        if rms < 1e-8 {
            return;
        }
        let target_rms = 0.05;
        let gain = target_rms / rms;
        for s in samples.iter_mut() {
            *s = (*s * gain).clamp(-1.0, 1.0);
        }
    }

    fn pre_emphasis(samples: &mut [f32]) {
        if samples.len() < 2 {
            return;
        }
        let coeff = 0.97;
        for i in (1..samples.len()).rev() {
            samples[i] -= coeff * samples[i - 1];
        }
    }
}
