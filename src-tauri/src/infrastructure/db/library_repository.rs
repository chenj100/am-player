use crate::domain::library_item::LibraryItem;

pub trait LibraryRepository {
    fn list(&self) -> Vec<LibraryItem>;
}

#[derive(Default)]
pub struct SqliteLibraryRepository;

impl LibraryRepository for SqliteLibraryRepository {
    fn list(&self) -> Vec<LibraryItem> {
        Vec::new()
    }
}
