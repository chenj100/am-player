use serde::{Deserialize, Serialize};

use super::source::SourcePlatform;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadJob {
    pub job_id: String,
    pub job_type: String,
    pub source_platform: SourcePlatform,
    pub source_item_id: String,
    pub target_library_item_id: Option<String>,
    pub requested_destination: String,
    pub status: String,
    pub progress_bytes: Option<i64>,
    pub total_bytes: Option<i64>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub retry_count: i32,
}
