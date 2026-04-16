use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDestination {
    pub destination_id: String,
    pub destination_type: String,
    pub display_name: String,
    pub writable: bool,
    pub requires_auth: bool,
    pub capacity_hint_bytes: Option<i64>,
    pub availability_state: String,
}
