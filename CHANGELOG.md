# Changelog

All notable changes to Kateb will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.3.1] - 2026-02-20

### Fixed
- Updater "Cannot read private member" error caused by Vue Proxy wrapping Tauri updater objects with private class fields

## [1.3.0] - 2026-02-20

### Added
- Voice commands: 8 commands (new line, paragraph, period, comma, question mark, exclamation, space, delete) supporting Arabic and English
- Voice commands guide in settings page with examples and tips (shown when enabled)
- Noise suppression module for audio preprocessing (experimental)
- Custom vocabulary support for improved speech recognition
- Frontend safety mechanisms: 45s processing timeout, backend status polling, toggle guards

### Fixed
- UTF-8 boundary panic in voice command processing for Arabic text
- Frontend stuck in "processing" state when backend panics

### Changed
- Moved documentation files to `docs/` directory

## [1.0.9] - 2026-02-12

### Added
- Updates section in Settings page with current version display, manual update check, and download progress bar
- App version number displayed in sidebar footer
- Visual progress bar (`UProgress`) during update download in dashboard banner
- Download size info (e.g., "15.2 MB / 45.0 MB") shown during update download
- Error state with retry button when update check or download fails
- New i18n translations for update UI (Arabic + English)

## [1.0.8] - 2026-02-12

### Fixed
- Auto-stop firing during active speech — VAD calibration was capturing speech as noise baseline, causing all subsequent speech to be classified as silence
- Capped VAD `noise_floor` at 0.01 to prevent speech-during-calibration from inflating the speech detection threshold
- Increased minimum recording duration before auto-stop can activate from 2s to 5s

## [1.0.7] - 2026-02-12

### Fixed
- Linux: app crashing with `Illegal instruction` on CPUs without AVX-512 — limited CI build to x86-64-v3 (AVX2 max)

## [1.0.6] - 2026-02-12

### Fixed
- Auto-stop silence detection now works even when no speech is detected (removed `has_text` guard)
- Recording silence no longer causes 38-second processing delay — speech ratio check skips transcription when < 10% speech detected
- Sidebar navigation icons now visible — added explicit icon bundle include list for all 37 Lucide icons
- Connection test button loading spinner now visible for at least 500ms

### Added
- `speech_ratio()` method to VAD for measuring speech-to-silence ratio
- Iconify CDN (`api.iconify.design`) added to CSP as icon fallback
- 2-second minimum recording duration before auto-stop can activate

## [1.0.5] - 2026-02-12

### Fixed
- Windows: custom AI provider URLs failing with "connection error" — added native OS certificate store support alongside bundled Mozilla CAs (`rustls-tls-native-roots`)

### Changed
- Windows installer: NSIS install mode set to "both" (per-user and per-machine) for better compatibility

## [1.0.4] - 2026-02-12

### Fixed
- Sidebar toggle: removed broken theme override, now uses built-in DashboardNavbar mobile toggle

### Added
- Desktop overlay widget: floating always-on-top window showing real-time dictation status
- Overlay shows recording (with audio level bar), processing, AI refinement, and result preview
- Overlay auto-shows on dictation start and auto-hides 3s after result
- `show_overlay` / `hide_overlay` Tauri commands for overlay window management

## [1.0.3] - 2026-02-12

### Fixed
- Sidebar toggle icon now visible on all screen sizes (was hidden below lg breakpoint)
- Dashboard shortcut display is now dynamic — shows actual configured shortcut instead of hardcoded Z+Z
- AI refinement timeout: processing now auto-cancels after 30s and returns raw text
- Cross-platform TLS: switched to rustls-tls for reliable HTTPS on Windows builds
- All AI provider HTTP clients now have connect (10s) and request (60s) timeouts

### Added
- Global toast notifications for dictation status when triggered via keyboard shortcut from any page
- Visual feedback for recording, processing, and refinement stages outside the home page

## [1.0.2] - 2026-02-11

### Fixed
- Models page showing blank — added loading spinner and error handling
- Active model banner showing incorrectly when no model is set
- Hardcoded 'English' strings now use proper i18n translations
- Statistics time labels showing Arabic when app is in English mode
- Modal placement in models page moved inside proper slot
- Settings page SelectItem error — mic device select had empty string value
- Enhanced browser mock data for development/testing

## [1.0.1] - 2026-02-10

### Fixed
- Icons not showing in built app — bundled all icons locally instead of loading from CDN
- Default language now correctly set to Arabic — disabled browser language detection override
- Language setting now applies immediately when changed in settings
- Windows audio recording error "stream configuration not supported" — added automatic fallback to device-supported sample rate with resampling to 16kHz for Whisper compatibility

## [1.0.0] - 2025-02-09

### Added
- Offline voice dictation using Whisper AI (5 models: tiny, base, small, medium, large-v3-turbo)
- Arabic and English language support with seamless switching
- AI text refinement with 5 providers (Claude, OpenAI, Gemini, Grok, Local)
- Custom API URL support for all AI providers
- Streaming transcription with real-time text display
- Anti-hallucination system for Whisper output
- Voice Activity Detection (VAD) with adaptive noise calibration
- Audio preprocessing (normalization + noise gate)
- Silence countdown timer with auto-stop
- Double-tap `Z Z` keyboard shortcut for start/stop
- Dictation history with search and statistics
- GPU acceleration support (CUDA)
- Settings page for language, model, AI provider configuration
- Welcome wizard for first-time setup
- System tray integration
- Cross-platform support (Linux, Windows, macOS)
