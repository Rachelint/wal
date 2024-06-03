#![allow(dead_code)]

/// The manager for multiple pages.
pub trait PageManager {
    type Page;
    type Path;

    /// Create a new page.
    fn create(&mut self) -> Self::Page;

    /// Delete the target page.
    fn delete(&mut self, path: Self::Path);
}

/// The storage unit.
pub trait Page {
    /// Read the page data to the `buf`.
    fn read(&self, buf: &mut [u8]);

    /// Write the data to the page.
    fn write(&mut self, data: &[u8]);
}

pub type PageId = u32;
