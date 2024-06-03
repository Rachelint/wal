use crate::storage::page_manager::PageId;

/// Record the metadata to help determine which pages can be cleaned.
pub trait MetaDataStore {
    type MetaData;

    /// Update metadata of the page.
    fn update_page_meta(&self, page_id: PageId, update: Self::MetaData);

    /// Flush and persist page meta.
    /// Generally, it will be called before page switching.
    /// If page metadata failed to flush, page switching should fail and retry later.
    fn flush_page_meta(&self, page_id: PageId);

    /// Update the deleted point, used to determine which pages can be cleaned.
    fn update_deleted_point(&self, deleted_point: Self::MetaData);
}
