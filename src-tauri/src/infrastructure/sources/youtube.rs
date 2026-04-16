use async_trait::async_trait;

use crate::domain::source::{SourcePlatform, TrackCandidate};
use crate::infrastructure::sources::SourceAdapter;

#[derive(Default)]
pub struct YouTubeSourceAdapter;

#[async_trait]
impl SourceAdapter for YouTubeSourceAdapter {
    async fn search(&self, query: &str) -> Vec<TrackCandidate> {
        vec![TrackCandidate {
            candidate_id: format!("yt-{query}"),
            source_platform: SourcePlatform::Youtube,
            source_item_id: format!("yt-{query}"),
            source_url: format!("https://www.youtube.com/results?search_query={query}"),
            source_title: format!("YouTube result for {query}"),
            uploader_name: "YouTube artist".to_string(),
            duration_ms: Some(180_000),
            thumbnail_url: None,
            search_query: query.to_string(),
            search_rank: 2,
            availability_state: "restricted".to_string(),
            metadata_hint_title: Some(query.to_string()),
            metadata_hint_singer: None,
            metadata_hint_album: None,
        }]
    }
}
