# Quickstart: Mobile Music Download Player

## 1. Install Tooling

- Install Rust stable
- Install Android Studio and Android SDK/NDK required by Tauri mobile
- Install Node.js package tooling for the web frontend

Reference:
- https://v2.tauri.app/start/prerequisites/

## 2. Initialize the Tauri Mobile App

- Create a Tauri v2 mobile project with the SvelteKit frontend template or add
  SvelteKit immediately after scaffolding
- Enable Android target support
- Add the SQL plugin with SQLite support

## 3. Establish the Core Architecture

- Organize the frontend as SvelteKit routes, reusable components, and stores
- Create Tauri commands for search, save jobs, library listing, metadata
  normalization, and playback queue building
- Add a Rust application layer that owns repositories, source adapters, and job
  orchestration
- Keep provider-specific logic in `src-tauri/src/infrastructure/sources/`

## 4. Add Android-Specific Integration

- Introduce a Kotlin bridge for:
  - WorkManager-backed persistent jobs
  - Media3 playback integration
  - notifications
  - storage picking and user-visible file destinations

## 5. Persist Library State

- Create SQLite tables for library items, download jobs, destinations, and
  playback queue state
- Keep the DB as the source of truth for UI rendering

## 6. Prototype Provider Adapters Safely

- Use `yt-dlp` and `bilibili-api` only as research aids while defining adapter
  expectations and metadata normalization rules
- Do not make embedded Python a requirement for mobile runtime behavior
- Keep provider enablement behind configuration flags and policy checks

## 7. Validate the Main Flows

- Search and render candidates
- Create a local save job
- Export an existing local item to Google Drive backup
- Browse and filter the library
- Play a local item with queue controls
- Confirm the UI distinguishes local playback from backup-only state
