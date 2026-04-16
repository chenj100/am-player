mod application;
mod commands;
mod domain;
mod infrastructure;
mod mobile;
mod playback;

use application::{
    job_service::JobService, library_service::LibraryService, source_service::SourceService,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .manage(SourceService::default())
        .manage(JobService::default())
        .manage(LibraryService::default())
        .invoke_handler(tauri::generate_handler![
            commands::search::search_tracks,
            commands::jobs::create_save_job,
            commands::jobs::get_job_status,
            commands::library::list_library,
            commands::playback::build_playback_queue,
            commands::playback::normalize_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running AM Player");
}
