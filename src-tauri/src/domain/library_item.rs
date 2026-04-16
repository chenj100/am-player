use serde::{Deserialize, Serialize};

use super::source::SourcePlatform;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub library_item_id: String,
    pub source_platform: SourcePlatform,
    pub source_item_id: String,
    pub source_url: String,
    pub original_source_title: String,
    pub normalized_song_title: Option<String>,
    pub normalized_singer: Option<String>,
    pub normalized_album: Option<String>,
    pub metadata_confidence: String,
    pub metadata_status: String,
    pub local_file_path: Option<String>,
    pub local_playback_state: String,
    pub drive_backup_state: String,
    pub duplicate_group_key: Option<String>,
}
