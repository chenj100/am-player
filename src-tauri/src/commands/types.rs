use serde::Serialize;

use crate::domain::{job::DownloadJob, library_item::LibraryItem, source::TrackCandidate};

#[derive(Debug, Serialize)]
pub struct SearchTracksResponse {
    pub results: Vec<TrackCandidate>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateSaveJobResponse {
    pub job_id: String,
    pub duplicate_warning: bool,
    pub existing_library_item_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JobStatusResponse {
    pub job: DownloadJob,
}

#[derive(Debug, Serialize)]
pub struct LibraryListResponse {
    pub items: Vec<LibraryItem>,
}
