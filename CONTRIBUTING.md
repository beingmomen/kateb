# Contributing to Kateb | المساهمة في كاتِب

Thank you for your interest in contributing to Kateb! | شكراً لاهتمامك بالمساهمة في كاتِب!

## How to Contribute | كيفية المساهمة

### Reporting Bugs | الإبلاغ عن أخطاء

1. Check [existing issues](https://github.com/beingmomen/kateb/issues) first
2. Create a new issue using the bug report template
3. Include steps to reproduce, expected behavior, and screenshots if applicable

### Suggesting Features | اقتراح ميزات جديدة

1. Check [existing issues](https://github.com/beingmomen/kateb/issues) for similar requests
2. Create a new issue using the feature request template
3. Describe the feature, its use case, and potential implementation

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run checks:
   ```bash
   pnpm lint
   pnpm typecheck
   cd src-tauri && cargo check
   ```
5. Commit with a descriptive message
6. Push and create a Pull Request

## Development Setup | إعداد بيئة التطوير

### Prerequisites | المتطلبات

- Node.js 18+
- pnpm 10+
- Rust 1.77+

### System Libraries (Linux)

```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev libasound2-dev
```

### Setup

```bash
git clone https://github.com/beingmomen/kateb.git
cd kateb
pnpm install
pnpm tauri dev
```

## Project Structure | هيكل المشروع

```
kateb/
├── app/                    # Nuxt 4 frontend
│   ├── pages/              # App pages (index, settings, history, stats)
│   ├── composables/        # Vue composables (useDictation, useSettings, etc.)
│   ├── components/         # Vue components
│   └── utils/              # Utilities (tauri.js wrapper)
├── src-tauri/              # Tauri 2 / Rust backend
│   └── src/
│       ├── ai/             # AI refinement (providers, factory pattern)
│       ├── audio/          # Audio recording, VAD, preprocessing
│       ├── commands/       # Tauri commands (dictation, settings, models)
│       ├── whisper/        # Whisper transcriber
│       └── db/             # SQLite database
└── public/                 # Static assets
```

## Code Guidelines | قواعد الكود

- Use JavaScript in Vue components (not TypeScript)
- TypeScript only in `*.config.ts` files
- No comments in code files
- Use composables instead of stores
- Handle errors gracefully with loading states
- Follow RTL layout for Arabic UI elements

## License | الرخصة

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
