use tauri::State;

use crate::application::{errors::AppError, source_service::SourceService};
use crate::commands::types::SearchTracksResponse;

#[tauri::command]
pub async fn search_tracks(
    query: String,
    source_filter: String,
    source_service: State<'_, SourceService>,
) -> Result<SearchTracksResponse, AppError> {
    let (results, warnings) = source_service.search(&query, &source_filter).await?;
    Ok(SearchTracksResponse { results, warnings })
}
