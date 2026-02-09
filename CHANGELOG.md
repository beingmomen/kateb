# Changelog

All notable changes to Kateb will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

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
