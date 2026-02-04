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
