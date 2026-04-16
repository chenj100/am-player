use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourcePlatform {
    Youtube,
    Bilibili,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackCandidate {
    pub candidate_id: String,
    pub source_platform: SourcePlatform,
    pub source_item_id: String,
    pub source_url: String,
    pub source_title: String,
    pub uploader_name: String,
    pub duration_ms: Option<i64>,
    pub thumbnail_url: Option<String>,
    pub search_query: String,
    pub search_rank: i32,
    pub availability_state: String,
    pub metadata_hint_title: Option<String>,
    pub metadata_hint_singer: Option<String>,
    pub metadata_hint_album: Option<String>,
}
