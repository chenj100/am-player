# Implementation Plan: Mobile Music Download Player

**Branch**: `002-music-download-player` | **Date**: 2026-04-16 | **Spec**: [/home/junrui/code/am-player/specs/001-music-download-player/spec.md](/home/junrui/code/am-player/specs/001-music-download-player/spec.md)
**Input**: Feature specification from `/specs/001-music-download-player/spec.md`

**Note**: This plan reflects an architecture review first. It recommends a
Tauri v2 mobile app with a Rust core and a SvelteKit frontend, but explicitly
treats Python libraries such as `yt-dlp` and `bilibili-api` as research inputs
or desktop prototypes, not as the mobile runtime foundation.

## Summary

Build a local-first Android app using Tauri v2 for the mobile shell, SvelteKit
for the frontend screens and stateful UI, Rust for domain logic and
stateful services, and a small Android-native bridge only where the mobile
platform requires it. The best-fit architecture is a layered app with:

- SvelteKit frontend running inside the Tauri mobile webview for search,
  library, queue, and player UI
- Rust command layer for library management, metadata normalization, queue
  logic, and source-adapter orchestration
- Android-native bridge for storage picking, notifications, persistent
  background work, and Media3-backed playback integration
- SQLite-backed library state and user-chosen storage locations for durable
  audio files

This makes sense for product maintainability. What does **not** make sense is
shipping a Python runtime inside the Android app just to execute `yt-dlp` or
`bilibili-api`. That adds packaging, startup, binary-size, and process-model
complexity without aligning with Tauri mobile’s strengths. It also does not
solve the bigger issue that YouTube’s official API policies prohibit
downloading or separating audiovisual content via YouTube API Services.

## Technical Context

**Language/Version**: Rust stable (>= 1.77.2 for current Tauri plugins), TypeScript, SvelteKit, HTML/CSS, small Kotlin bridge for Android-specific integrations  
**Primary Dependencies**: Tauri v2 mobile, SvelteKit, Tauri command/event system, Tauri SQL plugin with SQLite, Android Media3 ExoPlayer for local playback, Android WorkManager for persistent download/export jobs, Google Drive upload integration, Rust HTTP client and serialization stack  
**Storage**: SQLite for library metadata and job state; shared or user-selected device storage for durable audio files; app-private storage for transient work files; Google Drive for backup/export only  
**Testing**: Rust unit tests, Svelte component/state tests, Android integration tests for storage and background jobs, manual validation for source compatibility and playback flows  
**Target Platform**: Android phone first, API 29+ target with Tauri mobile and modern scoped storage behavior  
**Project Type**: Mobile app with embedded Rust core, SvelteKit frontend, and Android-native bridge  
**Performance Goals**: Search results render in under 2 seconds for typical queries, local playback starts in under 3 seconds, metadata normalization completes without blocking UI interactions  
**Constraints**: Must remain local-first, must clearly separate locally playable files from Drive backups, must survive intermittent network failures, must avoid architectures that depend on spawning Python processes on Android, must treat YouTube download behavior as policy-constrained  
**Scale/Scope**: Single-user personal media library, hundreds to low-thousands of tracks, one Android client, no multi-user backend in v1

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Code Quality Gate**: Pass. Architecture is split into reviewable boundaries:
  SvelteKit UI, Rust application services, Android bridge, and storage adapters.
  The plan avoids a mixed Rust plus embedded Python production runtime, which
  would sharply increase review and maintenance cost.
- **Testing Gate**: Pass. The design includes unit tests for normalization and
  library logic, Svelte state/component tests for the frontend, integration
  tests for command flows and persistence, Android integration tests for
  storage/background work, and manual validation for source-policy-sensitive
  flows and playback UX.
- **UX Consistency Gate**: Pass. The design keeps one library model as the
  source of truth, with explicit states for local-only, backup-exported,
  in-progress, failed, duplicate, and unplayable items. Search, library, and
  player screens reuse the same metadata vocabulary and status model.
- **Traceability Gate**: Pass. The work can be decomposed into SvelteKit UI,
  Rust core, Android bridge, storage, playback, and source-adapter slices with
  clear contracts.

## Project Structure

### Documentation (this feature)

```text
specs/001-music-download-player/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── app-commands.md
│   └── source-adapter-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── routes/
├── lib/
│   ├── components/
│   ├── features/
│   ├── stores/
│   ├── types/
│   └── utils/
└── app.html

src-tauri/
├── src/
│   ├── commands/
│   ├── domain/
│   ├── application/
│   ├── infrastructure/
│   │   ├── db/
│   │   ├── sources/
│   │   ├── storage/
│   │   └── drive/
│   ├── playback/
│   └── mobile/
├── capabilities/
└── Cargo.toml

android/
└── app/src/main/
    ├── java/.../bridge/
    ├── java/.../work/
    └── AndroidManifest.xml

tests/
├── rust/
├── integration/
└── android/
```

**Structure Decision**: Use a single Tauri mobile project with a SvelteKit
frontend, Rust-heavy core, and a small Android bridge. Reject a separate
backend service for v1 because the product is local-first and backup-oriented,
and reject a Python sidecar architecture because Tauri mobile does not support
shell execution on Android beyond opening URLs.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Android bridge alongside Rust core | Background jobs, storage pickers, and playback integration need platform-native hooks | Pure webview-only Tauri would be weaker at Android lifecycle, storage, and long-running work |
| Source adapter abstraction | Search and save behavior differ by provider and policy boundary | Hard-coding provider logic into UI or one giant service would make failures and policy changes harder to isolate |
