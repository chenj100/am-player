# Data Model: Mobile Music Download Player

## TrackCandidate

**Purpose**: Represents a searchable item from an external provider before it is
saved into the local library.

**Fields**:
- `candidate_id`: internal identifier
- `source_platform`: `youtube` | `bilibili` | future provider
- `source_item_id`: provider-specific identifier
- `source_url`
- `source_title`
- `uploader_name`
- `duration_ms`
- `thumbnail_url`
- `search_query`
- `search_rank`
- `availability_state`: `available` | `unavailable` | `restricted` | `unknown`
- `metadata_hint_title`
- `metadata_hint_singer`
- `metadata_hint_album`
- `fetched_at`

**Validation**:
- `source_platform`, `source_item_id`, and `source_url` are required
- `duration_ms` must be non-negative if present

## LibraryItem

**Purpose**: Canonical record for a saved song or backup-exported item.

**Fields**:
- `library_item_id`
- `source_platform`
- `source_item_id`
- `source_url`
- `original_source_title`
- `normalized_song_title`
- `normalized_singer`
- `normalized_album`
- `metadata_confidence`: `high` | `medium` | `low` | `unknown`
- `metadata_status`: `normalized` | `partial` | `unresolved`
- `local_file_path` nullable
- `local_playback_state`: `playable` | `missing` | `pending` | `not_local`
- `drive_backup_state`: `not_exported` | `queued` | `exporting` | `exported` | `failed`
- `duplicate_group_key` nullable
- `created_at`
- `updated_at`

**Validation**:
- At least one of `local_file_path` or `drive_backup_state != not_exported`
  must be true after a successful save/export flow
- `normalized_song_title` may be blank only if `metadata_status != normalized`

## DownloadJob

**Purpose**: Tracks a save or export action through its lifecycle.

**Fields**:
- `job_id`
- `job_type`: `local_save` | `drive_export`
- `source_platform`
- `source_item_id`
- `target_library_item_id` nullable until item creation
- `requested_destination`
- `status`: `queued` | `running` | `paused` | `retry_waiting` | `completed` | `failed` | `cancelled`
- `progress_bytes` nullable
- `total_bytes` nullable
- `failure_code` nullable
- `failure_message` nullable
- `retry_count`
- `started_at` nullable
- `finished_at` nullable

**State transitions**:
- `queued -> running`
- `running -> completed`
- `running -> failed`
- `running -> cancelled`
- `failed -> retry_waiting -> running`

## StorageDestination

**Purpose**: Describes where the app can store durable media.

**Fields**:
- `destination_id`
- `destination_type`: `shared_local` | `picked_folder` | `google_drive`
- `display_name`
- `writable`: boolean
- `requires_auth`: boolean
- `capacity_hint_bytes` nullable
- `availability_state`: `ready` | `unavailable` | `permission_required` | `auth_required`

## PlaybackQueue

**Purpose**: Current local playback context.

**Fields**:
- `queue_id`
- `item_ids[]`
- `current_index`
- `playback_state`: `idle` | `buffering` | `playing` | `paused` | `ended` | `error`
- `current_position_ms`
- `updated_at`

## UserPolicy

**Purpose**: Captures product policy choices that gate provider behavior.

**Fields**:
- `policy_id`
- `personal_archiving_only`: boolean
- `youtube_downloads_enabled`: boolean
- `bilibili_downloads_enabled`: boolean
- `last_reviewed_at`

**Validation**:
- Provider-specific save actions must be disabled when the corresponding policy
  flag is false
