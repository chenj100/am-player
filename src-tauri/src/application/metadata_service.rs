#[derive(Default)]
pub struct MetadataService;

impl MetadataService {
    pub fn normalize_title(&self, title: &str) -> String {
        title.trim().replace('_', " ")
    }
}
