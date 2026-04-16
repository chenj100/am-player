use serde::Serialize;
use tauri::Emitter;

#[derive(Debug, Clone, Serialize)]
pub struct JobProgressEvent {
    pub job_id: String,
    pub status: String,
}

pub fn emit_job_progress<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    payload: JobProgressEvent,
) -> tauri::Result<()> {
    app.emit("job-progress", payload)
}
