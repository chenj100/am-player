use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPolicy {
    pub policy_id: String,
    pub personal_archiving_only: bool,
    pub youtube_downloads_enabled: bool,
    pub bilibili_downloads_enabled: bool,
}
