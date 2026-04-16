use uuid::Uuid;

use crate::application::errors::AppError;
use crate::domain::job::DownloadJob;
use crate::domain::source::SourcePlatform;

#[derive(Default)]
pub struct JobService;

impl JobService {
    pub fn create_job(
        &self,
        candidate_id: &str,
        destination_id: &str,
        duplicate_warning: bool,
    ) -> Result<(DownloadJob, bool), AppError> {
        let platform = if candidate_id.starts_with("yt") {
            SourcePlatform::Youtube
        } else {
            SourcePlatform::Bilibili
        };

        Ok((
            DownloadJob {
                job_id: Uuid::new_v4().to_string(),
                job_type: if destination_id == "google_drive" {
                    "drive_export".to_string()
                } else {
                    "local_save".to_string()
                },
                source_platform: platform,
                source_item_id: candidate_id.to_string(),
                target_library_item_id: None,
                requested_destination: destination_id.to_string(),
                status: "queued".to_string(),
                progress_bytes: Some(0),
                total_bytes: None,
                failure_code: None,
                failure_message: duplicate_warning.then_some("Possible duplicate detected.".to_string()),
                retry_count: 0,
            },
            duplicate_warning,
        ))
    }
}
