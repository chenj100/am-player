pub mod bilibili;
pub mod youtube;

use async_trait::async_trait;

use crate::domain::source::TrackCandidate;

#[async_trait]
pub trait SourceAdapter {
    async fn search(&self, query: &str) -> Vec<TrackCandidate>;
}
