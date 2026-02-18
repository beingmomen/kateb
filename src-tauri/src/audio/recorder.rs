use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const TARGET_SAMPLE_RATE: u32 = 16000;
const MAX_BUFFER_SAMPLES: usize = TARGET_SAMPLE_RATE as usize * 600;

pub struct AudioRecorder {
    buffer: Arc<Mutex<Vec<f32>>>,
    stream: Mutex<Option<cpal::Stream>>,
    start_time: Mutex<Option<Instant>>,
    last_duration: Mutex<u64>,
    selected_device: Mutex<Option<String>>,
    actual_sample_rate: Mutex<u32>,
    actual_channels: Mutex<u16>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            stream: Mutex::new(None),
            start_time: Mutex::new(None),
            last_duration: Mutex::new(0),
            selected_device: Mutex::new(None),
            actual_sample_rate: Mutex::new(TARGET_SAMPLE_RATE),
            actual_channels: Mutex::new(1),
        }
    }

    pub fn list_devices() -> Vec<(String, bool)> {
        let host = cpal::default_host();
        let default_name = host
            .default_input_device()
            .and_then(|d| d.name().ok())
            .unwrap_or_default();

        host.input_devices()
            .map(|devices| {
                devices
                    .filter_map(|d| {
                        let name = d.name().ok()?;
                        let is_default = name == default_name;
                        Some((name, is_default))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn set_device(&self, device_name: Option<String>) {
        let mut selected = self.selected_device.lock().unwrap();
        *selected = device_name;
    }

    fn negotiate_config(&self, device: &cpal::Device) -> Result<(cpal::StreamConfig, u32, u16), anyhow::Error> {
        let preferred = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(TARGET_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        let supported = device.supported_input_configs();
        if let Ok(configs) = supported {
            let configs: Vec<_> = configs.collect();

            for cfg in &configs {
                if cfg.sample_format() == SampleFormat::F32
                    && cfg.channels() == 1
                    && cfg.min_sample_rate().0 <= TARGET_SAMPLE_RATE
                    && cfg.max_sample_rate().0 >= TARGET_SAMPLE_RATE
                {
                    tracing::info!("[recorder] Device supports 16kHz mono f32 directly");
                    return Ok((preferred, TARGET_SAMPLE_RATE, 1));
                }
            }

            for cfg in &configs {
                if cfg.sample_format() == SampleFormat::F32
                    && cfg.min_sample_rate().0 <= TARGET_SAMPLE_RATE
                    && cfg.max_sample_rate().0 >= TARGET_SAMPLE_RATE
                {
                    let channels = cfg.channels();
                    tracing::info!("[recorder] Using {}ch at 16kHz (will convert to mono)", channels);
                    return Ok((
                        cpal::StreamConfig {
                            channels,
                            sample_rate: cpal::SampleRate(TARGET_SAMPLE_RATE),
                            buffer_size: cpal::BufferSize::Default,
                        },
                        TARGET_SAMPLE_RATE,
                        channels,
                    ));
                }
            }
        }

        let default_config = device.default_input_config()?;
        let rate = default_config.sample_rate().0;
        let channels = default_config.channels();
        tracing::info!("[recorder] Falling back to device default: {}Hz, {}ch, {:?}", rate, channels, default_config.sample_format());

        Ok((
            cpal::StreamConfig {
                channels,
                sample_rate: cpal::SampleRate(rate),
                buffer_size: cpal::BufferSize::Default,
            },
            rate,
            channels,
        ))
    }

    pub fn start(&self) -> Result<(), anyhow::Error> {
        let host = cpal::default_host();
        let selected = self.selected_device.lock().unwrap();
        let device = if let Some(ref name) = *selected {
            host.input_devices()?
                .find(|d| d.name().ok().as_deref() == Some(name))
                .ok_or_else(|| anyhow::anyhow!("جهاز الصوت '{}' غير موجود", name))?
        } else {
            host.default_input_device()
                .ok_or_else(|| anyhow::anyhow!("لا يوجد ميكروفون متصل"))?
        };

        let (config, actual_rate, actual_channels) = self.negotiate_config(&device)?;

        {
            let mut r = self.actual_sample_rate.lock().unwrap();
            *r = actual_rate;
        }
        {
            let mut c = self.actual_channels.lock().unwrap();
            *c = actual_channels;
        }

        {
            let mut buf = self.buffer.lock().unwrap();
            buf.clear();
        }

        let buffer_clone = Arc::clone(&self.buffer);
        let log_counter = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let log_counter_clone = Arc::clone(&log_counter);

        tracing::info!("[recorder] Starting audio stream: {}Hz, {} channel(s)", actual_rate, actual_channels);

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buffer_clone.lock().unwrap();
                if buf.len() + data.len() <= MAX_BUFFER_SAMPLES * (actual_rate / TARGET_SAMPLE_RATE).max(1) as usize * actual_channels as usize {
                    buf.extend_from_slice(data);
                }
                let count = log_counter_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count % 100 == 0 {
                    let effective_samples = buf.len() / actual_channels as usize;
                    tracing::debug!("[recorder] Buffer: {} samples ({:.1}s)", effective_samples, effective_samples as f64 / actual_rate as f64);
                }
            },
            |err| {
                tracing::error!("Audio stream error: {}", err);
            },
            None,
        )?;

        stream.play()?;

        let mut stream_lock = self.stream.lock().unwrap();
        *stream_lock = Some(stream);

        let mut start = self.start_time.lock().unwrap();
        *start = Some(Instant::now());

        Ok(())
    }

    pub fn stop(&self) -> Result<Vec<f32>, anyhow::Error> {
        let duration = {
            let start = self.start_time.lock().unwrap();
            match *start {
                Some(t) => t.elapsed().as_secs(),
                None => 0,
            }
        };

        {
            let mut d = self.last_duration.lock().unwrap();
            *d = duration;
        }

        let mut stream_lock = self.stream.lock().unwrap();
        *stream_lock = None;

        let mut start = self.start_time.lock().unwrap();
        *start = None;

        let raw_buf = {
            let buf = self.buffer.lock().unwrap();
            buf.clone()
        };

        let actual_rate = *self.actual_sample_rate.lock().unwrap();
        let actual_channels = *self.actual_channels.lock().unwrap();

        let mono = if actual_channels > 1 {
            stereo_to_mono(&raw_buf, actual_channels)
        } else {
            raw_buf
        };

        if actual_rate != TARGET_SAMPLE_RATE {
            Ok(resample(&mono, actual_rate, TARGET_SAMPLE_RATE))
        } else {
            Ok(mono)
        }
    }

    pub fn get_duration_seconds(&self) -> u64 {
        let start = self.start_time.lock().unwrap();
        match *start {
            Some(t) => t.elapsed().as_secs(),
            None => {
                let d = self.last_duration.lock().unwrap();
                *d
            }
        }
    }

    pub fn get_buffer_snapshot(&self) -> Vec<f32> {
        let buf = self.buffer.lock().unwrap();
        buf.clone()
    }

    pub fn get_buffer_len(&self) -> usize {
        let buf = self.buffer.lock().unwrap();
        buf.len()
    }

    pub fn get_buffer_range(&self, start: usize, end: usize) -> Vec<f32> {
        let buf = self.buffer.lock().unwrap();
        let end = end.min(buf.len());
        let start = start.min(end);
        buf[start..end].to_vec()
    }

    pub fn get_actual_sample_rate(&self) -> u32 {
        *self.actual_sample_rate.lock().unwrap()
    }

    pub fn get_actual_channels(&self) -> u16 {
        *self.actual_channels.lock().unwrap()
    }

    pub fn get_audio_level(&self) -> f32 {
        let buf = self.buffer.lock().unwrap();
        if buf.is_empty() {
            return 0.0;
        }
        let actual_channels = *self.actual_channels.lock().unwrap();
        let sample_count = (1600 * actual_channels as usize).min(buf.len());
        let recent: Vec<&f32> = buf.iter().rev().take(sample_count).collect();
        let rms: f32 = (recent.iter().map(|&&s| s * s).sum::<f32>() / recent.len() as f32).sqrt();
        rms.min(1.0)
    }
}

fn stereo_to_mono(data: &[f32], channels: u16) -> Vec<f32> {
    let ch = channels as usize;
    data.chunks(ch)
        .map(|frame| frame.iter().sum::<f32>() / ch as f32)
        .collect()
}

fn resample(data: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate || data.is_empty() {
        return data.to_vec();
    }
    let ratio = from_rate as f64 / to_rate as f64;
    let output_len = (data.len() as f64 / ratio) as usize;
    let mut output = Vec::with_capacity(output_len);
    for i in 0..output_len {
        let src_pos = i as f64 * ratio;
        let idx = src_pos as usize;
        let frac = src_pos - idx as f64;
        let sample = if idx + 1 < data.len() {
            data[idx] * (1.0 - frac as f32) + data[idx + 1] * frac as f32
        } else if idx < data.len() {
            data[idx]
        } else {
            0.0
        };
        output.push(sample);
    }
    output
}

unsafe impl Send for AudioRecorder {}
unsafe impl Sync for AudioRecorder {}
