# Research: Mobile Music Download Player

## Decision: Use Tauri v2 mobile with a SvelteKit frontend, Rust core, and a thin Android-native bridge

**Rationale**:
- Tauri’s current docs position Rust as the application backend and support
  mobile targets with Android setup and distribution tooling.
- SvelteKit is a good fit for Tauri because it gives a structured frontend
  application model, route organization, and component/state patterns without
  changing the Rust-side architecture.
- Tauri commands and managed state provide a clean bridge between the frontend
  and Rust services, which fits a library manager/player better than stuffing
  logic into the web layer.
- A thin Kotlin bridge is justified for Android-only concerns such as
  WorkManager, notifications, and storage interaction patterns that should
  survive app lifecycle changes.

**Alternatives considered**:
- Pure native Android with Kotlin only: strongest Android integration, but
  worse code sharing if desktop or other targets matter later.
- Plain Vite SPA without SvelteKit conventions: workable, but weaker in route
  organization and frontend structure as the app grows.
- Flutter or React Native plus Rust core: viable, but outside the user’s stated
  Tauri direction.
- Pure Tauri without native Android bridge: too weak for persistent background
  work and Android storage ergonomics.

**Sources**:
- Tauri start page: https://v2.tauri.app/start/
- Tauri prerequisites: https://v2.tauri.app/start/prerequisites/
- Tauri command system: https://v2.tauri.app/develop/calling-rust/

## Decision: Do not embed Python libraries as the production mobile runtime

**Rationale**:
- Tauri’s shell plugin supports only opening URLs on Android and iOS, which
  means process execution is not a first-class mobile path.
- Although Tauri supports embedded sidecar binaries generally, the mobile shell
  limitations mean a Python sidecar strategy is a poor fit on Android.
- Shipping Python plus native extension dependencies on Android would raise APK
  size, startup cost, packaging complexity, sandboxing complexity, and
  long-term maintenance burden.
- `yt-dlp` and `bilibili-api` are still useful as research references and
  desktop prototypes for understanding extractor behavior and metadata shape.

**Alternatives considered**:
- Bundle Python via sidecar: poor Android fit and operationally fragile.
- Call remote Python services: would introduce a backend dependency that fights
  the product’s local-first goal and worsens privacy.
- Reimplement only the required source adapter logic in Rust/Kotlin: best fit
  for mobile runtime quality.

**Sources**:
- Tauri shell plugin platform support: https://v2.tauri.app/plugin/shell/
- Tauri sidecars: https://v2.tauri.app/develop/sidecar/

## Decision: Treat `yt-dlp` as a research/prototyping reference, not a shipping dependency

**Rationale**:
- The `yt-dlp` project clearly supports YouTube search, YouTube Music search,
  and Bilibili extractors, so it is a strong reference for understanding the
  shape of source discovery and extraction coverage.
- Its extractor behavior and supported site matrix make it a useful benchmark
  for feasibility and metadata normalization rules during research.
- It is still a Python CLI-oriented downloader, so it is not a natural mobile
  runtime dependency for a Tauri Android app.

**Alternatives considered**:
- Ignore `yt-dlp` entirely: loses valuable extractor knowledge.
- Use it directly in production: runtime mismatch on Android and higher
  operational complexity.

**Sources**:
- `yt-dlp` README and extractor notes: https://github.com/yt-dlp/yt-dlp

## Decision: `bilibili-api` is helpful for Bilibili metadata/search research but not a production mobile dependency

**Rationale**:
- The `bilibili-api` project covers video, audio, user, and search-related API
  access patterns and is useful for understanding Bilibili metadata structures.
- Its documentation explicitly frames it as a Python async API/crawling library
  and notes interface instability and anti-abuse concerns.
- That makes it suitable for protocol discovery and prototyping, but not a good
  fit for an Android Tauri production runtime.

**Alternatives considered**:
- Depend on `bilibili-api` in production: same embedded-Python problem as above.
- Ignore it and rely only on `yt-dlp`: loses Bilibili-specific metadata and API
  understanding.
- Recreate only the required Bilibili adapter functionality in Rust/Kotlin:
  best production fit if the product proceeds.

**Sources**:
- `bilibili-api` repository and docs: https://github.com/Nemo2011/bilibili-api

## Decision: Use a local-first Android architecture with SQLite as source of truth

**Rationale**:
- Android’s recommended architecture is layered, with UI, data, and optionally
  domain layers. That maps cleanly onto a Tauri frontend plus Rust service core.
- SQLite is the right persistent store for library items, normalization results,
  download job state, and playback state.
- Tauri’s SQL plugin supports Android and SQLite, which keeps the persistence
  layer close to the Rust core and avoids a backend service.

**Alternatives considered**:
- JSON files only: too brittle for indexed library queries and job state.
- Remote backend DB: unnecessary for a personal local-first library.

**Sources**:
- Android app architecture guide: https://developer.android.com/topic/architecture
- Tauri SQL plugin: https://v2.tauri.app/plugin/sql/

## Decision: Store durable audio in user-visible device storage, not only app-specific storage

**Rationale**:
- Android’s storage guidance says app-specific storage is removed on uninstall.
- Downloaded songs are user-valued media files, so treating them as durable user
  assets fits shared or user-chosen storage better than app-private storage.
- The app can still use app-private storage for temporary download fragments,
  caches, and database files.

**Alternatives considered**:
- Keep all audio in app-specific storage: simpler, but breaks user expectation
  that downloaded songs persist independently of the app.
- Store everything in shared storage without a DB: poor for job tracking and
  library state.

**Sources**:
- Android storage overview: https://developer.android.com/training/data-storage
- App-specific storage guidance: https://developer.android.com/training/data-storage/app-specific

## Decision: Use Android WorkManager for persistent save/export jobs

**Rationale**:
- Android recommends WorkManager for persistent work that must survive process
  death, app exits, and device restarts.
- Long-running workers with foreground notifications fit the save/export job
  model for large media transfers.
- This is a better Android fit than trying to keep all download execution
  inside the Tauri webview lifecycle.

**Alternatives considered**:
- In-process async tasks in the Tauri runtime only: too fragile if the app is
  backgrounded or killed.
- Foreground services only: usable, but WorkManager gives better scheduling and
  lifecycle integration for persistent jobs.

**Sources**:
- WorkManager overview: https://developer.android.com/topic/libraries/architecture/workmanager/
- Long-running workers: https://developer.android.com/topic/libraries/architecture/workmanager/advanced/long-running

## Decision: Use Media3 ExoPlayer for local playback

**Rationale**:
- Android recommends Media3 ExoPlayer as the default player implementation for
  local and streamed audio/video playback.
- It provides queueing, seeking, playback state, and device compatibility that
  are all relevant to the app’s local player requirements.
- On Android, playback is a platform concern strong enough that using Media3 via
  the Android bridge is a better fit than inventing a custom player in the web
  layer.

**Alternatives considered**:
- HTML audio element in the webview: weaker background/media-session behavior
  and less Android integration.
- Custom Rust audio engine: unnecessary complexity for v1.

**Sources**:
- Media3 ExoPlayer overview: https://developer.android.com/media/media3/exoplayer
- Basic playback app guide: https://developer.android.com/media/implement/playback-app

## Decision: Separate product viability from technical feasibility for YouTube

**Rationale**:
- YouTube’s official developer policies prohibit API clients from downloading,
  importing, backing up, caching, or storing copies of YouTube audiovisual
  content without prior written approval, making content available for offline
  playback, or separating/promoting audio components separately.
- That means a direct “YouTube downloader” architecture does not make sense if
  the goal is a durable, policy-compliant mobile app distributed normally.
- If the product proceeds, the architecture should isolate source adapters
  behind explicit policy boundaries so unsupported providers can be disabled or
  replaced without destabilizing the rest of the app.

**Alternatives considered**:
- Assume YouTube download is a normal supported capability: high policy risk.
- Drop provider abstraction and hard-code YouTube behavior: amplifies product
  risk and rework cost.

**Sources**:
- YouTube API Services Developer Policies: https://developers.google.com/youtube/terms/developer-policies

## Conclusion: Does the proposed direction make sense?

**Yes, partially.**

- **Tauri + Rust for the Android app**: yes, this makes sense.
- **Python libraries embedded in the Android app**: no, this does not make
  sense as the production architecture.
- **Using `yt-dlp` and `bilibili-api` for research and protocol prototyping**:
  yes, this makes sense.
- **A store-friendly YouTube audio downloader product**: no, this does not make
  sense without special rights or a different provider strategy.

The best architecture is therefore:

1. Tauri mobile UI
2. SvelteKit frontend for product UI
3. Rust core for domain logic and provider abstraction
4. Kotlin Android bridge for WorkManager, Media3, notifications, and storage
5. SQLite library state
6. User-visible local storage plus Drive backup/export
7. Provider adapters that can be enabled only where policy and maintenance risk
   are acceptable
