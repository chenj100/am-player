# am-player Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-16

## Active Technologies

- Rust stable (>= 1.77.2 for current Tauri plugins), TypeScript, SvelteKit, HTML/CSS, small Kotlin bridge for Android-specific integrations + Tauri v2 mobile, SvelteKit, Tauri command/event system, Tauri SQL plugin with SQLite, Android Media3 ExoPlayer for local playback, Android WorkManager for persistent download/export jobs, Google Drive upload integration, Rust HTTP client and serialization stack (002-music-download-player)
- SQLite for library metadata and job state; shared or user-selected device storage for durable audio files; app-private storage for transient work files; Google Drive for backup/export only (002-music-download-player)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test && cargo clippy

## Code Style

Rust stable (>= 1.77.2 for current Tauri plugins), TypeScript, SvelteKit, HTML/CSS, small Kotlin bridge for Android-specific integrations: Follow standard conventions

## Recent Changes
- 002-music-download-player: Added Rust stable (>= 1.77.2 for current Tauri plugins), TypeScript, SvelteKit, HTML/CSS, small Kotlin bridge for Android-specific integrations + Tauri v2 mobile, SvelteKit, Tauri command/event system, Tauri SQL plugin with SQLite, Android Media3 ExoPlayer for local playback, Android WorkManager for persistent download/export jobs, Google Drive upload integration, Rust HTTP client and serialization stack

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
