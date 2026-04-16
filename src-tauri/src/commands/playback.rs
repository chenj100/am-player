use tauri::command;

#[command]
pub async fn build_playback_queue(library_item_id: String, scope: String) -> serde_json::Value {
    serde_json::json!({
        "queueId": format!("queue-{library_item_id}"),
        "items": [],
        "currentIndex": if scope == "single" { 0 } else { 0 }
    })
}

#[command]
pub async fn normalize_metadata(library_item_id: String) -> serde_json::Value {
    serde_json::json!({
        "item": {
            "libraryItemId": library_item_id
        },
        "changedFields": []
    })
}
