# إملاء صوتي عربي | Arabic Voice Dictation

تطبيق سطح مكتب يحول الصوت إلى نص عربي وإنجليزي باستخدام Whisper Large V3 محليًا بدون إنترنت.

## المتطلبات

- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) 10+
- [Rust](https://www.rust-lang.org/tools/install) 1.77+
- نظام Linux/Windows/macOS مع مكتبات Tauri المطلوبة

### مكتبات النظام (Linux)

```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev libasound2-dev
```

### نموذج Whisper

قم بتنزيل نموذج Whisper Large V3 بصيغة GGML ووضعه في:

```
src-tauri/resources/ggml-large-v3.bin
```

يمكن تنزيله من: https://huggingface.co/ggerganov/whisper.cpp/tree/main

## التثبيت

```bash
pnpm install
```

## التطوير

```bash
# تشغيل واجهة Nuxt فقط
pnpm dev

# تشغيل تطبيق Tauri الكامل (واجهة + backend)
pnpm tauri dev
```

## البناء

```bash
# بناء تطبيق سطح المكتب
pnpm tauri build
```

## الاستخدام

1. شغّل التطبيق
2. اضغط `Ctrl+Shift+D` لبدء الإملاء
3. تحدث بالعربية أو الإنجليزية
4. اضغط `Ctrl+Shift+D` مرة أخرى لإيقاف التسجيل
5. سيتم تحويل الصوت إلى نص وكتابته في البرنامج النشط

## الصفحات

| الصفحة | المسار | الوصف |
|--------|--------|-------|
| الرئيسية | `/` | التحكم بالإملاء |
| السجل | `/history` | سجل الإملاءات السابقة |
| الإحصائيات | `/stats` | إحصائيات الاستخدام |
| الإعدادات | `/settings` | إعدادات التطبيق |

## التقنيات

| التقنية | الاستخدام |
|---------|----------|
| Nuxt 4 | الواجهة الأمامية |
| Nuxt UI 4 | مكتبة المكونات |
| Tauri 2 | إطار سطح المكتب |
| SQLite | قاعدة البيانات |
| whisper-rs | تحويل الصوت إلى نص |
| cpal | تسجيل الصوت |
| enigo | محاكاة لوحة المفاتيح |
