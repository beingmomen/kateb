# Changelog

All notable changes to Kateb will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

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
