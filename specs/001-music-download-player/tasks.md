# Tasks: Mobile Music Download Player

**Input**: Design documents from `/specs/001-music-download-player/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Include the test and validation tasks required by the feature
specification and constitution. If behavior changes, tests are NOT optional.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- SvelteKit frontend: `src/routes/`, `src/lib/`
- Rust/Tauri core: `src-tauri/src/`
- Android-native bridge: `android/app/src/main/`
- Tests: `tests/rust/`, `tests/integration/`, `tests/android/`, `src/lib/**/*.test.ts`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and baseline tooling

- [X] T001 Scaffold the SvelteKit + Tauri mobile project structure in `src/`, `src-tauri/`, `android/`, and `tests/`
- [X] T002 Initialize frontend and Rust dependencies in `package.json`, `svelte.config.js`, `vite.config.ts`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`
- [X] T003 [P] Configure formatting and linting in `eslint.config.js`, `prettier.config.*`, `src-tauri/rustfmt.toml`, and `src-tauri/.clippy.toml`
- [X] T004 [P] Define automated quality checks in `.github/workflows/ci.yml` or equivalent project automation files

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Create SQLite schema and migration setup for library items, download jobs, destinations, playback queues, and user policy in `src-tauri/src/infrastructure/db/` and `src-tauri/migrations/`
- [X] T006 [P] Implement core domain models for `TrackCandidate`, `LibraryItem`, `DownloadJob`, `StorageDestination`, `PlaybackQueue`, and `UserPolicy` in `src-tauri/src/domain/`
- [X] T007 [P] Implement repository traits and SQLite-backed persistence adapters in `src-tauri/src/application/` and `src-tauri/src/infrastructure/db/`
- [X] T008 [P] Create the Tauri command registry and shared command result/error types in `src-tauri/src/commands/mod.rs`, `src-tauri/src/commands/types.rs`, and `src-tauri/src/lib.rs`
- [X] T009 [P] Implement the source adapter interfaces and policy-gated provider capability flags in `src-tauri/src/infrastructure/sources/mod.rs` and `src-tauri/src/application/source_service.rs`
- [X] T010 [P] Add Android bridge skeletons for WorkManager jobs, Media3 playback hooks, notifications, and storage access in `android/app/src/main/java/.../bridge/` and `android/app/src/main/java/.../work/`
- [X] T011 Create shared frontend domain types and Tauri invoke wrappers in `src/lib/types/`, `src/lib/api/commands.ts`, and `src/lib/stores/`
- [X] T012 Configure app-wide UX state patterns for loading, empty, error, retry, and backup-only indicators in `src/lib/components/` and `src/lib/stores/ui.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Find and Save Music (Priority: P1) 🎯 MVP

**Goal**: Let users search supported sources, choose a destination, and create save/export jobs with visible progress and failure handling

**Independent Test**: A user can search for a song, inspect results, choose local storage or Google Drive backup, start a save job, and see the new item appear in the library with correct job status.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T013 [P] [US1] Add Rust command contract tests for `search_tracks`, `create_save_job`, and `get_job_status` in `tests/rust/test_search_and_save_commands.rs`
- [ ] T014 [P] [US1] Add integration tests for search, duplicate warning, and job progress flows in `tests/integration/test_search_save_flow.ts`
- [ ] T015 [P] [US1] Add Svelte UI/state tests for search results, destination selection, and status messaging in `src/lib/features/search/search-flow.test.ts`

### Implementation for User Story 1

- [ ] T016 [P] [US1] Implement search query form, source filters, and result list routes in `src/routes/+page.svelte` and `src/lib/features/search/`
- [ ] T017 [P] [US1] Implement destination picker, duplicate warning modal, and job progress UI in `src/lib/features/downloads/` and `src/lib/components/`
- [ ] T018 [P] [US1] Implement Rust search and save job commands in `src-tauri/src/commands/search.rs` and `src-tauri/src/commands/jobs.rs`
- [ ] T019 [P] [US1] Implement Bilibili and YouTube source adapter search/inspection stubs with policy gating in `src-tauri/src/infrastructure/sources/bilibili.rs` and `src-tauri/src/infrastructure/sources/youtube.rs`
- [ ] T020 [US1] Implement save job orchestration and duplicate detection in `src-tauri/src/application/job_service.rs` and `src-tauri/src/application/library_service.rs`
- [ ] T021 [US1] Implement Android WorkManager-backed local save and Drive export workers in `android/app/src/main/java/.../work/SaveWorker.kt` and `android/app/src/main/java/.../work/DriveExportWorker.kt`
- [ ] T022 [US1] Wire Tauri event emission and frontend stores for job progress, completion, retry, and failure states in `src-tauri/src/commands/events.rs` and `src/lib/stores/jobs.ts`
- [ ] T023 [US1] Add validation and error classification for unavailable sources, permissions, policy blocks, and storage failures in `src-tauri/src/application/errors.rs` and `src-tauri/src/infrastructure/sources/`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Organize a Clean Music Library (Priority: P2)

**Goal**: Normalize metadata, surface uncertainty clearly, and let users browse and filter a clean library

**Independent Test**: A user can download multiple items with messy titles, see normalized metadata in the library, identify uncertain fields, and browse/filter by title, singer, album, source, destination, and status.

### Tests for User Story 2

- [ ] T024 [P] [US2] Add Rust tests for normalization rules and metadata confidence handling in `tests/rust/test_metadata_normalization.rs`
- [ ] T025 [P] [US2] Add integration tests for library filtering, duplicate grouping, and uncertainty display in `tests/integration/test_library_organization.ts`
- [ ] T026 [P] [US2] Add Svelte UI/state tests for library filters, status chips, and metadata fallback states in `src/lib/features/library/library-view.test.ts`

### Implementation for User Story 2

- [ ] T027 [P] [US2] Implement metadata normalization service and confidence scoring in `src-tauri/src/application/metadata_service.rs`
- [ ] T028 [P] [US2] Implement library query commands and repository filters in `src-tauri/src/commands/library.rs` and `src-tauri/src/infrastructure/db/library_repository.rs`
- [ ] T029 [P] [US2] Build the SvelteKit library route, filter controls, and item cards in `src/routes/library/+page.svelte` and `src/lib/features/library/`
- [ ] T030 [US2] Integrate normalization updates into save completion and library refresh flows in `src-tauri/src/application/job_service.rs` and `src/lib/stores/library.ts`
- [ ] T031 [US2] Add duplicate grouping, uncertain metadata badges, and backup-only availability labels in `src/lib/features/library/` and `src/lib/components/`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Play Downloaded Songs (Priority: P3)

**Goal**: Provide a local playback queue with basic transport controls and accurate availability messaging

**Independent Test**: A user opens a locally stored track, starts playback, pauses, resumes, seeks, navigates next/previous, and sees backup-only items blocked from local playback with clear messaging.

### Tests for User Story 3

- [ ] T032 [P] [US3] Add Rust tests for playback queue creation and session position updates in `tests/rust/test_playback_queue.rs`
- [ ] T033 [P] [US3] Add Android integration tests for Media3 playback state transitions in `tests/android/test_media3_playback.kt`
- [ ] T034 [P] [US3] Add Svelte UI/state tests for player controls and local-vs-backup availability messaging in `src/lib/features/player/player-shell.test.ts`

### Implementation for User Story 3

- [ ] T035 [P] [US3] Implement playback queue commands and session state persistence in `src-tauri/src/commands/playback.rs` and `src-tauri/src/application/playback_service.rs`
- [ ] T036 [P] [US3] Implement the Android Media3 playback bridge and player event callbacks in `android/app/src/main/java/.../bridge/PlaybackBridge.kt`
- [ ] T037 [P] [US3] Build the SvelteKit player shell, transport controls, and queue view in `src/routes/player/+page.svelte` and `src/lib/features/player/`
- [ ] T038 [US3] Wire playback events, current track state, and seek/next/previous actions between the frontend, Tauri commands, and Android bridge in `src/lib/stores/player.ts` and `src-tauri/src/commands/playback.rs`
- [ ] T039 [US3] Enforce local-playback-only validation and backup-only blocking states in `src-tauri/src/application/playback_service.rs` and `src/lib/features/player/`

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T040 [P] Document setup, architecture decisions, and developer workflows in `README.md` and `specs/001-music-download-player/quickstart.md`
- [ ] T041 Refine error copy, retry UX, and status consistency across search, library, downloads, and player screens in `src/lib/components/` and `src/lib/features/`
- [ ] T042 [P] Add regression coverage for policy gating, missing local files, and worker retry behavior in `tests/rust/`, `tests/integration/`, and `tests/android/`
- [ ] T043 Run quickstart validation and record any deviations in `specs/001-music-download-player/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel if staffed
  - Or sequentially in priority order (US1 → US2 → US3)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependency on later stories
- **User Story 2 (P2)**: Can start after Foundational, but benefits from US1 job completion flow because metadata normalization is triggered from saved items
- **User Story 3 (P3)**: Can start after Foundational, but depends on local library items from US1 for realistic playback validation

### Within Each User Story

- Tests MUST be written and FAIL before implementation when they cover changed behavior
- Frontend state/tests before final UI wiring where possible
- Rust/domain logic before command exposure
- Android bridge work before end-to-end integration
- UX/state validation before story sign-off

### Parallel Opportunities

- T003 and T004 can run in parallel
- T006 through T011 can run in parallel after T005 where their file ownership does not overlap
- In US1, T013, T014, T015, T016, T017, T018, and T019 can run in parallel initially
- In US2, T024, T025, T026, T027, T028, and T029 can run in parallel initially
- In US3, T032, T033, T034, T035, T036, and T037 can run in parallel initially

---

## Parallel Example: User Story 1

```bash
# Launch User Story 1 validation tasks together:
Task: "Add Rust command contract tests in tests/rust/test_search_and_save_commands.rs"
Task: "Add integration tests in tests/integration/test_search_save_flow.ts"
Task: "Add Svelte UI/state tests in src/lib/features/search/search-flow.test.ts"

# Launch User Story 1 implementation tasks with disjoint ownership:
Task: "Implement search UI in src/routes/+page.svelte and src/lib/features/search/"
Task: "Implement destination and progress UI in src/lib/features/downloads/"
Task: "Implement Rust commands in src-tauri/src/commands/search.rs and src-tauri/src/commands/jobs.rs"
Task: "Implement source adapters in src-tauri/src/infrastructure/sources/bilibili.rs and src-tauri/src/infrastructure/sources/youtube.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Confirm search, save, duplicate warning, and progress flows
5. Demo the MVP before metadata refinement or playback

### Incremental Delivery

1. Setup + Foundational → foundation ready
2. Add User Story 1 → validate save flow and destination handling
3. Add User Story 2 → validate normalized library browsing
4. Add User Story 3 → validate local playback controls
5. Finish with cross-cutting polish and quickstart validation

### Parallel Team Strategy

With multiple developers:

1. Developer A: SvelteKit frontend routes and stores
2. Developer B: Rust domain, commands, and SQLite persistence
3. Developer C: Android bridge for WorkManager and Media3
4. Merge per story only after tests and UX state validation pass

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- Every user story includes automated validation tasks and explicit UX/state checks
- User Story 1 is the suggested MVP scope
- All tasks include exact file paths and follow the required checklist format
