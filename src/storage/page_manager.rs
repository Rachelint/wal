use std::path::Path;

/// The manager for multiple pages.
pub trait PageManager {
    type Page;

    /// Create a new page.
    fn create(&self) -> Box<dyn Page>;

    /// Delete the target page.
    // FIX: Replace the `P`. 
    fn delete<P: AsRef<Path>>(&self, path: P);
}

/// The storage unit.
pub trait Page {
    /// Read the page data to the `buf`.
    fn read(&self, buf: &mut [u8]);

    /// Write the data to the page.
    fn write(&mut self, data: &[u8]);
}

pub type PageId = u32;
