# Fix Log

This file documents issues encountered during development and their solutions.

---

### [2026-02-03] - Tauri Capabilities Permission Not Found
**Problem**: `cargo check` failed with `Permission path:default not found in any manifest`
**Root Cause**: Tauri v2 uses `core:default` as the umbrella permission, not individual `path:default`, `event:default`, etc.
**Solution**: Changed `capabilities/default.json` permissions from individual entries to `["core:default"]`
**Files Modified**: `src-tauri/capabilities/default.json`
**Prevention**: Always check Tauri v2 docs for capability naming conventions

---

### [2026-02-03] - Tauri Resources Glob No Files Found
**Problem**: `cargo check` failed with `resource path resources/* has no files` error
**Root Cause**: The Whisper model file is not bundled during development; `resources/*` glob requires at least one matching file
**Solution**: Changed `"resources": ["resources/*"]` to `"resources": []` in tauri.conf.json. Users download the model separately per README instructions.
**Files Modified**: `src-tauri/tauri.conf.json`
**Prevention**: Don't glob for optional/large resources that may not exist at build time

---

### [2026-02-03] - Rust Borrow Checker Error in History Query
**Problem**: `cargo check` failed with `E0597: stmt does not live long enough` in `get_history`
**Root Cause**: The `Statement` was being dropped while query results still held references to it. Returning iterator-based results directly from `query_map` caused lifetime issues.
**Solution**: Extracted a `map_dictation_row` helper function and collected query results into a local `Vec<DictationEntry>` variable within each branch before returning.
**Files Modified**: `src-tauri/src/commands/history.rs`
**Prevention**: Always collect rusqlite query results into owned types before returning from functions

---

### [2026-02-03] - Race Condition in Dictation State Management
**Problem**: Code review found that if `recorder.start()` or `recorder.stop()` failed, the `is_recording` mutex could be left in an inconsistent state
**Root Cause**: Error paths did not properly clean up the `is_recording` flag before returning errors
**Solution**: Restructured `start_dictation` and `stop_dictation` with match blocks that explicitly set `is_recording = false` in error branches, and used scoped blocks for proper lock management
**Files Modified**: `src-tauri/src/commands/dictation.rs`
**Prevention**: Always handle error paths for mutex-guarded state; use scoped blocks to release locks

---

### [2026-02-03] - Unbounded Audio Buffer Growth
**Problem**: Code review found the audio recording buffer had no size limit, risking memory exhaustion on long recordings
**Root Cause**: The audio callback appended samples indefinitely without checking buffer size
**Solution**: Added `MAX_BUFFER_SAMPLES` constant (16000 samples/sec * 600 sec = 10 min max) and a size check in the audio callback that stops appending when the limit is reached
**Files Modified**: `src-tauri/src/audio/recorder.rs`
**Prevention**: Always set upper bounds on buffers that grow from external input

---

### [2026-02-03] - Duration Calculation Returns Zero After Stop
**Problem**: `get_duration_seconds()` returned 0 after `stop()` because `start_time` was cleared during stop
**Root Cause**: The `stop()` method set `start_time` to `None` before `get_duration_seconds()` was called, so elapsed time could not be calculated
**Solution**: Added a `last_duration: Mutex<u64>` field that stores the elapsed time in `stop()` before clearing `start_time`, and `get_duration_seconds()` returns this cached value when not actively recording
**Files Modified**: `src-tauri/src/audio/recorder.rs`
**Prevention**: Cache computed values before clearing the data they depend on

---

### [2026-02-03] - Tailwind CSS Non-Canonical Class Warning
**Problem**: IDE warning about `!p-6` being non-canonical Tailwind CSS syntax
**Root Cause**: Tailwind v4 changed the important modifier syntax from prefix `!` to suffix `!`
**Solution**: Changed `!p-6` to `p-6!` in affected component
**Files Modified**: `app/pages/index.vue`
**Prevention**: Use the canonical suffix `!` form for Tailwind important modifiers

---

### [2026-02-04] - Stats Page Blank/White Screen
**Problem**: Stats page (`/stats`) rendered completely blank with no errors in console
**Root Cause**: `await Promise.all([fetchDailyStats(), fetchSummary()])` in `<script setup>` caused a Suspense issue when Tauri mock returned null (same issue as settings page)
**Solution**: Replaced top-level await with `onMounted(() => { fetchDailyStats(); fetchSummary() })`
**Files Modified**: `app/pages/stats.vue`
**Prevention**: Never use top-level await for Tauri invoke calls in pages; use onMounted instead

---

### [2026-02-04] - CUDA Build Failed with whisper-rs 0.12
**Problem**: `cargo build` with `whisper-rs = { version = "0.12", features = ["cuda"] }` failed: `ggml-cuda/common.cuh: No such file or directory`
**Root Cause**: whisper-rs 0.12 bundles an older whisper.cpp that doesn't have the restructured CUDA source files
**Solution**: Upgraded to `whisper-rs = { version = "0.15", features = ["cuda"] }` which has updated whisper.cpp with proper CUDA support
**Files Modified**: `src-tauri/Cargo.toml`, `src-tauri/src/whisper/transcriber.rs`
**API Changes in 0.15**: `full_n_segments()` returns `i32` (not `Result`), `full_get_segment_text(i)` replaced by `get_segment(i)` returning `Option<WhisperSegment>`, segment text accessed via `segment.to_str()`
**Prevention**: Check crate changelog before upgrading major features

---

### [2026-02-15] - Windows Release Crashes with Missing CUDA DLLs
**Problem**: `kateb.exe` fails to launch on Windows with errors: `cublas64_12.dll`, `nvcuda.dll`, and `cudart64_12.dll` not found
**Root Cause**: `Cargo.toml` had `default = ["cuda"]` which compiled whisper-rs with CUDA support, linking NVIDIA libraries that don't exist on machines without NVIDIA GPUs
**Solution**: Changed default features to `default = []` (CPU-only). Removed CUDA toolkit installation step from `release.yml`. CUDA remains opt-in via `--features cuda`
**Files Modified**: `src-tauri/Cargo.toml`, `.github/workflows/release.yml`
**Prevention**: Never enable GPU-specific features by default in cross-platform apps; keep them opt-in

---

### [2026-02-15] - Add dual Windows release (CPU + GPU)
**Problem**: Users with NVIDIA GPUs want faster Whisper inference via CUDA, but a single CUDA build breaks on non-NVIDIA machines
**Root Cause**: Single build can't serve both GPU and non-GPU users
**Solution**: Added separate `release-windows-gpu` job in release workflow. CPU build uses `tauri-action` (default, no CUDA). GPU build compiles with `--features cuda`, renames artifacts to `Kateb-GPU_*`, and uploads alongside CPU artifacts
**Files Modified**: `.github/workflows/release.yml`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`
**Prevention**: Always provide CPU-only as default; GPU as separate opt-in download

---

### [2026-02-15] - Add Linux GPU build to release
**Problem**: Linux users with NVIDIA GPUs also need a CUDA-accelerated build for faster Whisper inference
**Root Cause**: Only Windows had a GPU build variant; Linux was CPU-only
**Solution**: Added `release-linux-gpu` job that builds with `--features cuda`, renames `.deb`/`.AppImage` artifacts to `kateb-gpu_*`, and uploads alongside CPU artifacts
**Files Modified**: `.github/workflows/release.yml`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`
**Prevention**: When adding GPU builds, cover all target platforms

---

### [2026-02-16] - GPU build jobs fail: missing TAURI_SIGNING_PRIVATE_KEY
**Problem**: `release-windows-gpu` and `release-linux-gpu` jobs fail with `A public key has been found, but no private key`
**Root Cause**: GPU jobs use `pnpm tauri build` directly instead of `tauri-action`, but `tauri.conf.json` has `createUpdaterArtifacts: true` with a pubkey, so Tauri requires the signing key to create updater artifacts
**Solution**: Added `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` env vars to the "Build GPU version" step in both GPU jobs
**Files Modified**: `.github/workflows/release.yml`
**Prevention**: When using `pnpm tauri build` directly (outside `tauri-action`), always pass the same env vars that `tauri-action` uses

---

### [2026-02-16] - Linux GPU rename fails on Kateb.AppDir directory
**Problem**: `release-linux-gpu` rename step fails: `mv: cannot move 'Kateb.AppDir' to a subdirectory of itself`
**Root Cause**: The `appimage/` bundle directory contains `Kateb.AppDir/` (temp build dir) alongside `Kateb_*.AppImage`. The `for f in "$dir"/*` glob matched both files and directories
**Solution**: Replaced glob with `find -maxdepth 1 -type f` to only rename files, and added `-name` filters to only target `kateb_*`/`Kateb_*` patterns
**Files Modified**: `.github/workflows/release.yml`
**Prevention**: When renaming build artifacts, always filter by `-type f` and specific name patterns to avoid temp directories

---

### [2026-02-16] - Linux GPU upload fails: glob case mismatch
**Problem**: Upload step fails with `no matches found for src-tauri/target/release/bundle/deb/*gpu*`
**Root Cause**: Tauri names deb files `Kateb_*` (capital K). After rename → `Kateb-GPU_*`. Upload glob used `*gpu*` (lowercase) which doesn't match
**Solution**: Changed upload glob from `*gpu*` to `*GPU*` for the deb directory
**Files Modified**: `.github/workflows/release.yml`
**Prevention**: Match exact casing of renamed artifacts in upload globs

---

### [2026-02-16] - CPU Whisper Transcription Extremely Slow (500+ seconds)
**Problem**: Windows CPU version takes 500+ seconds to process 9 seconds of audio, never finishes
**Root Cause**: `FullParams` in `transcriber.rs` never called `set_n_threads()`. The whisper-rs default is `min(4, hardware_concurrency)` — only 4 threads max. For large models (1.6GB large-v3-turbo) on CPU with limited threads, this is expected to be extremely slow
**Solution**: Added `set_n_threads()` using `std::thread::available_parallelism()` to utilize all available CPU cores in both `transcribe()` and `transcribe_chunk()` methods
**Files Modified**: `src-tauri/src/whisper/transcriber.rs`
**Prevention**: Always configure thread count explicitly for CPU-intensive operations; don't rely on library defaults

---

### [2026-02-16] - Auto-Updater TypeError: Cannot read private member
**Problem**: Ubuntu v1.0.13 auto-updater fails with `TypeError: Cannot read private member from an object whose class did not declare it`
**Root Cause**: The `Resource` class in `@tauri-apps/api/core` uses `tslib`'s `WeakMap`-based private field storage. Dynamic imports (`await import('@tauri-apps/plugin-updater')`) caused Vite's code-splitting to create duplicate copies of `@tauri-apps/api/core` in separate chunks. The `Update` object was registered in one WeakMap but the `rid` getter tried to read from another
**Solution**: Changed all dynamic imports (`await import(...)`) to static imports (`import { ... } from ...`) in `useUpdater.js` to prevent code-splitting duplication of the `Resource` class
**Files Modified**: `app/composables/useUpdater.js`
**Prevention**: Use static imports for Tauri plugins that return `Resource`-based objects (updater, etc.) to avoid `WeakMap` duplication from code-splitting
