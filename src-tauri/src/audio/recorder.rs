use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const MAX_BUFFER_SAMPLES: usize = 16000 * 600;

pub struct AudioRecorder {
    buffer: Arc<Mutex<Vec<f32>>>,
    stream: Mutex<Option<cpal::Stream>>,
    start_time: Mutex<Option<Instant>>,
    last_duration: Mutex<u64>,
    sample_rate: u32,
    selected_device: Mutex<Option<String>>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            stream: Mutex::new(None),
            start_time: Mutex::new(None),
            last_duration: Mutex::new(0),
            sample_rate: 16000,
            selected_device: Mutex::new(None),
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

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        {
            let mut buf = self.buffer.lock().unwrap();
            buf.clear();
        }

        let buffer_clone = Arc::clone(&self.buffer);
        let log_counter = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let log_counter_clone = Arc::clone(&log_counter);

        tracing::info!("[recorder] Starting audio stream: {}Hz, {} channel(s)", self.sample_rate, config.channels);

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buffer_clone.lock().unwrap();
                if buf.len() + data.len() <= MAX_BUFFER_SAMPLES {
                    buf.extend_from_slice(data);
                }
                let count = log_counter_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count % 100 == 0 {
                    tracing::debug!("[recorder] Buffer: {} samples ({:.1}s)", buf.len(), buf.len() as f64 / 16000.0);
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

        let buf = self.buffer.lock().unwrap();
        Ok(buf.clone())
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

    pub fn get_audio_level(&self) -> f32 {
        let buf = self.buffer.lock().unwrap();
        if buf.is_empty() {
            return 0.0;
        }
        let recent: Vec<&f32> = buf.iter().rev().take(1600).collect();
        let rms: f32 = (recent.iter().map(|&&s| s * s).sum::<f32>() / recent.len() as f32).sqrt();
        rms.min(1.0)
    }
}

unsafe impl Send for AudioRecorder {}
unsafe impl Sync for AudioRecorder {}
