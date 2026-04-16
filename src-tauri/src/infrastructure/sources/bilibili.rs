use async_trait::async_trait;

use crate::domain::source::{SourcePlatform, TrackCandidate};
use crate::infrastructure::sources::SourceAdapter;

#[derive(Default)]
pub struct BilibiliSourceAdapter;

#[async_trait]
impl SourceAdapter for BilibiliSourceAdapter {
    async fn search(&self, query: &str) -> Vec<TrackCandidate> {
        vec![TrackCandidate {
            candidate_id: format!("bili-{query}"),
            source_platform: SourcePlatform::Bilibili,
            source_item_id: format!("bili-{query}"),
            source_url: format!("https://www.bilibili.com/search?keyword={query}"),
            source_title: format!("Bilibili result for {query}"),
            uploader_name: "Bilibili artist".to_string(),
            duration_ms: Some(180_000),
            thumbnail_url: None,
            search_query: query.to_string(),
            search_rank: 1,
            availability_state: "available".to_string(),
            metadata_hint_title: Some(query.to_string()),
            metadata_hint_singer: None,
            metadata_hint_album: None,
        }]
    }
}
