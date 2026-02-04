# CLAUDE.md

This file provides MANDATORY guidance for AI assistants working on this project.

## Project Overview

- **Project Name**: إملاء صوتي عربي
- **Technical Name**: arabic-voice-dictation
- **Type**: fullstack
- **Nature**: tool
- **Runtime Targets**: web, desktop, system
- **Intelligence Level**: ai-core
- **Size**: medium

## 🛑 MCP FIRST — NON-NEGOTIABLE

> **This is the #1 rule of this project. Break this rule and ALL your code is invalid.**

For EVERY file you create or modify:
1. IDENTIFY which frameworks/libraries the file uses
2. CALL the MCP tool for each one BEFORE writing code
3. USE the exact syntax from MCP responses
4. NEVER write framework code from memory

This applies to: components, pages, layouts, composables, middleware, plugins, config files.
No exceptions. No shortcuts. No "I already know this component."

## 🚀 PROJECT SETUP (MANDATORY FIRST STEP)

This project uses **Nuxt UI**. You MUST use the MCP template setup to create the project:

1. Run: `/nuxt-ui-remote:setup_project_with_template`
2. Select the **Dashboard** template
3. After setup completes, copy the downloaded files (CLAUDE.md, fix.md, .env.example) to the project root
4. Then proceed with implementing the specification in project-spec.md

> **WARNING**: DO NOT manually create nuxt.config.ts, package.json, or install Nuxt/Nuxt UI manually.
> The MCP template handles all of this automatically with the correct versions and configuration.

> **FALLBACK if MCP template unavailable**: `pnpm dlx nuxi@latest init .` then `pnpm add @nuxt/ui`, configure `nuxt.config.ts` with `modules: ['@nuxt/ui']` and `ssr: false` (desktop app).

## ⚠️ VERSION REQUIREMENTS

Always use the **Latest** version of each technology unless a specific version is noted below.

| Technology | Version |
|------------|---------|
| Frontend | Nuxt (Latest) |
| UI Library | Nuxt UI (Latest) |
| Backend | Tauri (Latest) |
| Database | SQLite (Latest) |

## 🔴 BLOCKING REQUIREMENT: MCP SERVERS

### ⛔ STOP — READ THIS BEFORE WRITING ANY CODE

You MUST call the relevant MCP tool BEFORE writing ANY code that uses a framework component.
Code written WITHOUT first querying MCP is INVALID and must be rewritten.

### Pre-Code Checklist (MANDATORY for every file)

Before creating or modifying ANY file, verify:

- [ ] Did I call the MCP tool for EVERY framework component I'm about to use?
- [ ] Am I using the EXACT syntax returned by MCP (not from memory/training data)?
- [ ] Did I verify the component props and slots match the MCP response?

### MCP Tools Reference

> **FALLBACK**: If `mcp__nuxt-ui-remote` or `mcp__nuxt-remote` servers are unavailable, use Context7 MCP as fallback:
> - Nuxt UI docs: `query-docs` with libraryId `/websites/ui3_nuxt`
> - Nuxt docs: `query-docs` with libraryId `/websites/nuxt`
> - Tauri v2 docs: `query-docs` with libraryId `/websites/v2_tauri_app`
> - Tauri plugins: `query-docs` with libraryId `/tauri-apps/plugins-workspace`

| When you write... | You MUST first call... |
|---|---|
| Any `<U...>` component (UButton, USelect, UModal, UTable, etc.) | `mcp__nuxt-ui-remote__get-component` |
| Any Nuxt file (page, component, composable, middleware) | `mcp__nuxt-remote__get-documentation-page` |
| Any Nuxt module or plugin | `mcp__nuxt-remote__list-modules` or `mcp__nuxt-remote__get-module` |

### Per-Server Instructions

#### Nuxt MCP Server
⛔ BLOCKING: Before creating ANY Nuxt file (page, component, composable, middleware, plugin), you MUST call:
- `mcp__nuxt-remote__get-documentation-page` to verify file conventions
- `mcp__nuxt-remote__list-modules` when adding any module
Do NOT assume folder structure or file naming from memory.

#### Nuxt UI MCP Server
⛔ BLOCKING: Before writing ANY <U...> component tag, you MUST call:
- `mcp__nuxt-ui-remote__get-component` to get the component documentation
- `mcp__nuxt-ui-remote__get-component-metadata` to verify props, slots, and events
Writing a Nuxt UI component WITHOUT calling these tools first makes your code INVALID.
Do NOT rely on training data — component APIs change between versions.

### ❌ EXAMPLES OF INVALID BEHAVIOR
- Writing `<USelect :items="..." />` without calling get-component for USelect first
- Creating a Nuxt page without checking the pages documentation
- Using component props from memory instead of MCP response
- Assuming component API hasn't changed between versions

### ✅ CORRECT WORKFLOW
1. Identify which components/APIs you need
2. Call the relevant MCP tool for EACH one
3. Read the MCP response carefully
4. Write code using ONLY the syntax from MCP responses
5. If unsure about any prop or slot, call MCP again

## 🏗️ Architecture Decisions

- **Whisper model**: Bundled with app (not downloaded at runtime)
- **Target OS**: Cross-platform (Windows, macOS, Linux)
- **Build approach**: Full-stack simultaneous (frontend + Tauri backend together)
- **Rust crates**: `whisper-rs` (STT), `cpal` (audio capture), `enigo` (keyboard simulation), `rusqlite` (SQLite)
- **Nuxt UI version**: v3 (uses `UDashboardGroup`, `UDashboardSidebar`, `UDashboardPanel`, `UNavigationMenu`)
- **Nuxt config**: SSR disabled (`ssr: false`) since this is a Tauri desktop app
- **State management**: Composables wrapping Tauri `invoke()` calls, no Pinia/Vuex

## 📋 CODE GUIDELINES

### MUST Follow
- **NO COMMENTS**: Do not add any comments to code files
- **VERSION CHECK**: Always verify you're using the correct versions specified above
- **MCP FIRST**: Follow the MCP FIRST checklist above — query MCP servers before creating ANY file, do NOT rely on training data for framework APIs
- **README**: After project setup is complete, create README.md with setup and run instructions
- **ENV VARIABLES**: Never duplicate environment variables between global and local .env files
- **FIX LOG**: Document all issues and their solutions in fix.md

### TypeScript Mode
- Use JavaScript in Vue components
- TypeScript only in *.config.ts files
- Use JSDoc for type hints when needed

### AI Integration Guidelines
- Intelligence Level: ai-core
- Always handle AI API failures gracefully with fallback behavior
- Implement rate limiting for external AI API calls
- Cache AI responses where appropriate to reduce costs
- Log AI model inputs/outputs for debugging (exclude in production)

### Desktop/System Guidelines
- Handle file system permissions gracefully
- Implement proper error handling for system-level operations
- Test on multiple operating systems if applicable
- Use secure IPC communication between processes

### Development Warnings
- قبل إنشاء أي صفحة أو component، قم بمراجعة MCP servers الخاصة بـ Nuxt & Nuxt UI
- استخدم Zod schemas للتحقق من جميع المدخلات في Backend
- اتبع نمط RTL في جميع عناصر الواجهة العربية
- لا تستخدم TypeScript في ملفات Vue ما عدا إذا تم اختيار "TypeScript كامل"
- استخدم composables بدلاً من mixins أو store مباشر
- تجنب استخدام any في TypeScript - استخدم أنواع محددة
- تأكد من معالجة الأخطاء (Error handling) في كل API call
- أضف loading states لكل عملية async
- استخدم دائماً أحدث إصدارات Nuxt & Nuxt UI & Zod

## 📝 fix.md Usage

When solving ANY issue during development, log it immediately in fix.md:

```markdown
### [YYYY-MM-DD] - Issue Title
**Problem**: Clear description of the issue
**Root Cause**: Why it happened
**Solution**: How it was fixed
**Files Modified**: List of affected files
**Prevention**: How to avoid this in future
```

This helps track issues and improve future project planning.

## 🚀 Commands

```bash
pnpm install          # Install dependencies
pnpm dev              # Start development server
pnpm build            # Build for production
pnpm preview          # Preview production build
pnpm lint             # Run linter
pnpm typecheck        # Run type checking
pnpm tauri dev        # Start Tauri desktop dev mode
pnpm tauri build      # Build Tauri desktop app
pnpm tauri init       # Initialize Tauri in existing project
```

## 📦 Tech Stack

| Category | Technology |
|----------|------------|
| Frontend | Nuxt (Latest) |
| Backend | Tauri (Latest) |
| Database | SQLite (Latest) |
| Authentication | None |
| UI Library | Nuxt UI (Latest) |
| Package Manager | pnpm |
