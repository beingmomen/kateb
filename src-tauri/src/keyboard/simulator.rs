use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

pub struct KeyboardSimulator;

impl KeyboardSimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn type_text(&self, text: &str) -> Result<(), anyhow::Error> {
        let mut clipboard = Clipboard::new()
            .map_err(|e| anyhow::anyhow!("فشل الوصول للحافظة: {}", e))?;

        let old_clipboard = clipboard.get_text().unwrap_or_default();

        clipboard
            .set_text(text)
            .map_err(|e| anyhow::anyhow!("فشل نسخ النص للحافظة: {}", e))?;

        thread::sleep(Duration::from_millis(50));

        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| anyhow::anyhow!("فشل تهيئة محاكي لوحة المفاتيح: {}", e))?;

        enigo.key(Key::Control, Direction::Press)
            .map_err(|e| anyhow::anyhow!("فشل محاكاة Ctrl: {}", e))?;
        enigo.key(Key::Unicode('v'), Direction::Click)
            .map_err(|e| anyhow::anyhow!("فشل محاكاة V: {}", e))?;
        enigo.key(Key::Control, Direction::Release)
            .map_err(|e| anyhow::anyhow!("فشل تحرير Ctrl: {}", e))?;

        thread::sleep(Duration::from_millis(100));

        let _ = clipboard.set_text(&old_clipboard);

        Ok(())
    }
}
