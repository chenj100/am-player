# Contract: App Commands

## Overview

These are the logical command boundaries between the Tauri frontend and the Rust
application core. Commands return structured results and explicit failure codes.

## `search_tracks`

**Input**:
- `query: string`
- `source_filter: "all" | "youtube" | "bilibili"`

**Output**:
- `results: TrackCandidate[]`
- `warnings: string[]`

**Failure cases**:
- invalid query
- source unavailable
- rate limited

## `create_save_job`

**Input**:
- `candidate_id: string`
- `destination_id: string`

**Output**:
- `job_id: string`
- `duplicate_warning: boolean`
- `existing_library_item_id?: string`

**Failure cases**:
- destination unavailable
- policy blocked
- duplicate rejected

## `list_library`

**Input**:
- `query?: string`
- `title?: string`
- `singer?: string`
- `album?: string`
- `source?: string`
- `status?: string`

**Output**:
- `items: LibraryItem[]`

## `get_job_status`

**Input**:
- `job_id: string`

**Output**:
- `job: DownloadJob`

## `retry_job`

**Input**:
- `job_id: string`

**Output**:
- `job: DownloadJob`

## `build_playback_queue`

**Input**:
- `library_item_id: string`
- `scope: "single" | "filtered_library"`

**Output**:
- `queue_id: string`
- `items: LibraryItem[]`
- `current_index: number`

## `normalize_metadata`

**Input**:
- `library_item_id: string`

**Output**:
- `item: LibraryItem`
- `changed_fields: string[]`

## Event Stream

Frontend subscribes to:
- `job-progress`
- `job-status-changed`
- `library-item-updated`
- `playback-state-changed`
