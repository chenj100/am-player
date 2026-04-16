use crate::application::errors::AppError;
use crate::domain::source::{SourcePlatform, TrackCandidate};
use crate::infrastructure::sources::{bilibili::BilibiliSourceAdapter, youtube::YouTubeSourceAdapter, SourceAdapter};

pub struct SourceService {
    bilibili: BilibiliSourceAdapter,
    youtube: YouTubeSourceAdapter,
}

impl Default for SourceService {
    fn default() -> Self {
        Self {
            bilibili: BilibiliSourceAdapter::default(),
            youtube: YouTubeSourceAdapter::default(),
        }
    }
}

impl SourceService {
    pub async fn search(
        &self,
        query: &str,
        source_filter: &str,
    ) -> Result<(Vec<TrackCandidate>, Vec<String>), AppError> {
        if query.trim().is_empty() {
            return Err(AppError::InvalidQuery);
        }

        let mut results = Vec::new();
        let mut warnings = Vec::new();

        if matches!(source_filter, "all" | "bilibili") {
            results.extend(self.bilibili.search(query).await);
        }

        if matches!(source_filter, "all" | "youtube") {
            results.extend(self.youtube.search(query).await);
            warnings.push(
                "YouTube save capability is policy-gated and may remain disabled in production."
                    .to_string(),
            );
        }

        results.sort_by_key(|candidate| candidate.search_rank);
        Ok((results, warnings))
    }

    pub fn source_enabled(&self, platform: &SourcePlatform) -> bool {
        match platform {
            SourcePlatform::Bilibili => true,
            SourcePlatform::Youtube => false,
        }
    }
}
