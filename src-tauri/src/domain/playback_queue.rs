use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackQueue {
    pub queue_id: String,
    pub item_ids: Vec<String>,
    pub current_index: usize,
    pub playback_state: String,
    pub current_position_ms: i64,
}
