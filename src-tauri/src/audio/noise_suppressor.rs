use nnnoiseless::DenoiseState;

pub struct NoiseSuppressor {
    enabled: bool,
}

impl NoiseSuppressor {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn suppress(&self, samples_16k: &[f32]) -> Vec<f32> {
        if !self.enabled || samples_16k.is_empty() {
            return samples_16k.to_vec();
        }

        let samples_48k = upsample_3x(samples_16k);

        let mut denoiser = DenoiseState::new();
        let frame_size = DenoiseState::FRAME_SIZE;

        let scaled: Vec<f32> = samples_48k.iter().map(|&s| s * 32767.0).collect();

        let mut output_48k = Vec::with_capacity(scaled.len());
        let num_full_frames = scaled.len() / frame_size;

        for i in 0..num_full_frames {
            let start = i * frame_size;
            let input_frame = &scaled[start..start + frame_size];
            let mut output_frame = vec![0.0f32; frame_size];
            denoiser.process_frame(&mut output_frame, input_frame);
            output_48k.extend_from_slice(&output_frame);
        }

        let remainder = scaled.len() % frame_size;
        if remainder > 0 {
            let mut padded = vec![0.0f32; frame_size];
            let start = num_full_frames * frame_size;
            padded[..remainder].copy_from_slice(&scaled[start..]);
            let mut output_frame = vec![0.0f32; frame_size];
            denoiser.process_frame(&mut output_frame, &padded);
            output_48k.extend_from_slice(&output_frame[..remainder]);
        }

        let descaled: Vec<f32> = output_48k
            .iter()
            .map(|&s| {
                let v = s / 32767.0;
                if v.is_finite() { v.clamp(-1.0, 1.0) } else { 0.0 }
            })
            .collect();

        let result = downsample_3x(&descaled);

        let has_bad_values = result.iter().any(|v| !v.is_finite());
        if has_bad_values {
            tracing::warn!("[noise] Output contains non-finite values, returning original audio");
            return samples_16k.to_vec();
        }

        result
    }
}

fn upsample_3x(data: &[f32]) -> Vec<f32> {
    let mut output = Vec::with_capacity(data.len() * 3);
    for i in 0..data.len() {
        let current = data[i];
        let next = if i + 1 < data.len() {
            data[i + 1]
        } else {
            current
        };
        output.push(current);
        output.push(current + (next - current) / 3.0);
        output.push(current + 2.0 * (next - current) / 3.0);
    }
    output
}

fn downsample_3x(data: &[f32]) -> Vec<f32> {
    data.chunks(3).map(|chunk| chunk[0]).collect()
}
