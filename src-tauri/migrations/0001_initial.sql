CREATE TABLE IF NOT EXISTS library_items (
  library_item_id TEXT PRIMARY KEY,
  source_platform TEXT NOT NULL,
  source_item_id TEXT NOT NULL,
  source_url TEXT NOT NULL,
  original_source_title TEXT NOT NULL,
  normalized_song_title TEXT,
  normalized_singer TEXT,
  normalized_album TEXT,
  metadata_confidence TEXT NOT NULL,
  metadata_status TEXT NOT NULL,
  local_file_path TEXT,
  local_playback_state TEXT NOT NULL,
  drive_backup_state TEXT NOT NULL,
  duplicate_group_key TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS download_jobs (
  job_id TEXT PRIMARY KEY,
  job_type TEXT NOT NULL,
  source_platform TEXT NOT NULL,
  source_item_id TEXT NOT NULL,
  target_library_item_id TEXT,
  requested_destination TEXT NOT NULL,
  status TEXT NOT NULL,
  progress_bytes INTEGER,
  total_bytes INTEGER,
  failure_code TEXT,
  failure_message TEXT,
  retry_count INTEGER NOT NULL DEFAULT 0,
  started_at TEXT,
  finished_at TEXT
);

CREATE TABLE IF NOT EXISTS storage_destinations (
  destination_id TEXT PRIMARY KEY,
  destination_type TEXT NOT NULL,
  display_name TEXT NOT NULL,
  writable INTEGER NOT NULL,
  requires_auth INTEGER NOT NULL,
  capacity_hint_bytes INTEGER,
  availability_state TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS playback_queues (
  queue_id TEXT PRIMARY KEY,
  item_ids TEXT NOT NULL,
  current_index INTEGER NOT NULL,
  playback_state TEXT NOT NULL,
  current_position_ms INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS user_policy (
  policy_id TEXT PRIMARY KEY,
  personal_archiving_only INTEGER NOT NULL,
  youtube_downloads_enabled INTEGER NOT NULL,
  bilibili_downloads_enabled INTEGER NOT NULL,
  last_reviewed_at TEXT DEFAULT CURRENT_TIMESTAMP
);
