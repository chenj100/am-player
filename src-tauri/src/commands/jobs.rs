use tauri::State;

use crate::application::{
    errors::AppError,
    job_service::JobService,
    library_service::LibraryService,
};
use crate::commands::types::{CreateSaveJobResponse, JobStatusResponse};

#[tauri::command]
pub async fn create_save_job(
    candidate_id: String,
    destination_id: String,
    library_service: State<'_, LibraryService>,
    job_service: State<'_, JobService>,
) -> Result<CreateSaveJobResponse, AppError> {
    let duplicate_warning = library_service.detect_duplicate(&candidate_id, &destination_id);
    let (job, duplicate_warning) =
        job_service.create_job(&candidate_id, &destination_id, duplicate_warning)?;
    Ok(CreateSaveJobResponse {
        job_id: job.job_id,
        duplicate_warning,
        existing_library_item_id: job.target_library_item_id,
    })
}

#[tauri::command]
pub async fn get_job_status(
    job_id: String,
    job_service: State<'_, JobService>,
) -> Result<JobStatusResponse, AppError> {
    let (job, _) = job_service.create_job(&job_id, "shared_local", false)?;
    Ok(JobStatusResponse { job })
}
