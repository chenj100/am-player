use crate::domain::library_item::LibraryItem;

#[derive(Default)]
pub struct LibraryService;

impl LibraryService {
    pub fn detect_duplicate(&self, _source_item_id: &str, _destination_id: &str) -> bool {
        false
    }

    pub fn list_items(&self) -> Vec<LibraryItem> {
        Vec::new()
    }
}
