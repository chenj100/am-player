use tauri::State;

use crate::application::library_service::LibraryService;
use crate::commands::types::LibraryListResponse;

#[tauri::command]
pub async fn list_library(library_service: State<'_, LibraryService>) -> LibraryListResponse {
    LibraryListResponse {
        items: library_service.list_items(),
    }
}
