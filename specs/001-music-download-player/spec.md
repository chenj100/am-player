# Feature Specification: Mobile Music Download Player

**Feature Branch**: `[pending-git-feature-hook]`  
**Created**: 2026-04-16  
**Status**: Draft  
**Input**: User description: "Build an android app that help me search, download and organize bilibili and youtube music audio streams to mobile device local storage or google drive storage. It should be able to extract consistent song names, singer and album information. It should have basic audio player functionalities to play the songs downloaded to the mobile device."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Find and Save Music (Priority: P1)

As a listener, I want to search Bilibili and YouTube music content, review audio-oriented
results, and download selected tracks to my device or Google Drive so that I can build a
personal library without leaving the app.

**Why this priority**: Discovering and saving tracks is the core value of the product. Without
this flow, the app does not solve the primary user problem.

**Independent Test**: A user can search for a song, view matching results from supported
sources, choose a storage destination, start a download, and confirm that the saved item appears
in the library with an understandable status.

**Acceptance Scenarios**:

1. **Given** the user enters a song or artist query, **When** matching source results are found,
   **Then** the app shows a result list with enough metadata to choose the intended track.
2. **Given** the user selects a result and chooses device storage, **When** the download
   completes, **Then** the track appears in the local library with extracted metadata and a
   playable file.
3. **Given** the user selects a result and chooses Google Drive storage, **When** the export
   completes, **Then** the app confirms the saved destination and records the item in the
   library.

---

### User Story 2 - Organize a Clean Music Library (Priority: P2)

As a listener, I want downloaded tracks to have consistent song title, singer, and album
information so that I can browse, sort, and manage my library without cleaning metadata by hand.

**Why this priority**: Downloading alone is not enough if the library becomes messy and hard to
use. Consistent organization is central to long-term usability.

**Independent Test**: A user downloads multiple tracks with mixed source naming styles and can
view a normalized library where duplicate-looking titles are reduced and metadata fields are
structured consistently.

**Acceptance Scenarios**:

1. **Given** downloaded items have inconsistent source titles, **When** the app processes
   metadata, **Then** it produces normalized song title, singer, and album fields using a
   consistent rule set.
2. **Given** metadata cannot be confidently determined for one or more fields, **When** the item
   is added to the library, **Then** the app preserves the original source title and marks the
   uncertain fields clearly instead of inventing misleading data.
3. **Given** the user views the library, **When** they sort or filter by title, singer, or
   album, **Then** the downloaded tracks follow the normalized metadata values.

---

### User Story 3 - Play Downloaded Songs (Priority: P3)

As a listener, I want basic playback controls for songs stored on my device so that I can listen
to my downloaded music directly inside the app.

**Why this priority**: Playback is the expected follow-up action after building a library, but it
is less critical than acquisition and organization.

**Independent Test**: A user opens a downloaded local track, starts playback, pauses, resumes,
seeks, and moves between tracks in the local library without leaving the app.

**Acceptance Scenarios**:

1. **Given** the user selects a locally stored track, **When** they press play, **Then** audio
   playback starts and the current track information is shown.
2. **Given** a track is playing, **When** the user pauses, resumes, or seeks, **Then** playback
   responds correctly and the position indicator stays accurate.
3. **Given** multiple local tracks exist, **When** the user moves to the next or previous track,
   **Then** playback advances within the current playback list.

### Edge Cases

- What happens when a source result is unavailable, removed, region-restricted, or no longer
  downloadable by the time the user starts the download?
- How does the system handle duplicate downloads of the same song to the same destination?
- What happens when the device has insufficient storage space or loses connectivity during a
  download or cloud export?
- How does the system behave when metadata extraction returns multiple plausible singers or albums?
- What happens when a user tries to play an item that has been exported to Google Drive but is no
  longer present on the device?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST allow users to search supported music sources using song, singer,
  album, or free-text queries.
- **FR-002**: The system MUST present search results in a way that helps users distinguish likely
  music tracks from unrelated videos or duplicate uploads.
- **FR-003**: The system MUST allow users to start a download or save action from a selected
  search result.
- **FR-004**: The system MUST support saving downloaded audio to device local storage.
- **FR-005**: The system MUST support saving downloaded audio or exported files to Google Drive.
- **FR-006**: The system MUST display progress, completion, failure, and retry status for each
  download or export job.
- **FR-007**: The system MUST create a library record for each saved item with source reference,
  storage destination, save status, and extracted metadata.
- **FR-008**: The system MUST extract and store normalized song title, singer, and album
  information for each saved item when those values can be determined with reasonable confidence.
- **FR-009**: The system MUST preserve the original source title and indicate uncertainty when
  normalized metadata cannot be confidently resolved.
- **FR-010**: Users MUST be able to browse, search, sort, and filter the saved library by title,
  singer, album, source, destination, and download status.
- **FR-011**: The system MUST prevent accidental duplicate saves by warning the user when a likely
  duplicate already exists in the chosen destination.
- **FR-012**: The system MUST provide basic playback controls for locally stored tracks, including
  play, pause, resume, seek, next, and previous.
- **FR-013**: The system MUST remember the last known playback position for a track during the
  current app usage session.
- **FR-014**: The system MUST clearly distinguish which items are available for immediate local
  playback and which items have only been exported to Google Drive as backup copies.
- **FR-015**: The system MUST restrict download and export operations to content the user elects
  to save for personal archiving from publicly accessible music streams under the product policy.
- **FR-016**: The system MUST treat Google Drive as a backup and export destination only, rather
  than as part of the main browsable library model.

### Quality & Experience Requirements *(mandatory)*

- **QX-001**: The feature MUST define automated and manual validation for searching, download
  states, metadata normalization, duplicate handling, and local playback behavior.
- **QX-002**: The feature MUST provide clear loading, empty, success, failure, and retry states
  for search, save, export, and playback flows.
- **QX-003**: The feature MUST use consistent metadata labels and browsing patterns across search
  results, download history, library views, and player screens.
- **QX-004**: The feature MUST make playback availability obvious before the user taps an item.
- **QX-005**: The feature MUST ensure the primary flows remain usable on a typical phone-sized
  screen with one-handed navigation patterns where practical.

### Key Entities *(include if feature involves data)*

- **Track Candidate**: A search result from a supported source, including source identifier,
  source title, display thumbnail, duration, channel or uploader name, and enough metadata to
  decide whether it is the intended song.
- **Library Item**: A saved music record that tracks normalized song title, singer, album,
  original source title, source platform, source link, storage destination, save status, and
  playback availability.
- **Download Job**: A save operation with a source item, chosen destination, progress state,
  timestamps, failure reason, and retry state.
- **Storage Destination**: A user-selected save target such as local device storage or Google
  Drive, including availability and capacity-related attributes exposed to the user.
- **Playback Queue**: An ordered list of locally playable library items with current track,
  current position, and queue navigation state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 90% of users can search for a song and start saving a chosen result in under 2
  minutes on their first attempt.
- **SC-002**: 95% of completed save operations appear in the library with song title and singer
  populated in structured fields.
- **SC-003**: 90% of sampled library items that share the same source song are grouped under
  consistent title and singer naming conventions after normalization.
- **SC-004**: 95% of locally stored tracks selected from the library begin playback within 3
  seconds under normal network-independent conditions.
- **SC-005**: 90% of users in validation testing can correctly tell whether a saved item is
  available for local playback or stored only in Google Drive.

## Assumptions

- The initial release targets a single personal-library owner rather than a multi-user shared
  service.
- Users are willing to sign in to Google Drive when they choose that destination.
- Local playback applies only to files that are present on the device; Google Drive exports are
  treated as backup copies and are not directly playable in the initial release.
- Metadata normalization uses a deterministic rule set and may leave uncertain fields blank or
  marked as unknown rather than guessing.
- The first release focuses on audio-oriented saving, organization, and playback rather than
  social sharing, lyrics, playlists, or recommendation features.

## Validation & Quality Gates *(mandatory)*

- **Code Quality Checks**: Validate search, save, metadata, library, and playback flows with
  repeatable checks; require reviewable state handling for errors, retries, and duplicates.
- **Test Coverage Plan**: Cover source search results, download and export state transitions,
  metadata normalization outcomes, duplicate handling, local playback controls, and library
  sorting and filtering. Manual validation must confirm destination visibility and playback
  availability messaging.
- **UX Consistency Notes**: Search, library, and player surfaces must use the same song title,
  singer, album, source, destination, and availability labels. Action states for saving and retry
  must remain consistent across result cards, job history, and item detail views.
