use enigo::{Enigo, Keyboard, Settings};

pub struct KeyboardSimulator;

impl KeyboardSimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn type_text(&self, text: &str) -> Result<(), anyhow::Error> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| anyhow::anyhow!("فشل تهيئة محاكي لوحة المفاتيح: {}", e))?;

        enigo
            .text(text)
            .map_err(|e| anyhow::anyhow!("فشل كتابة النص: {}", e))?;

        Ok(())
    }
}
