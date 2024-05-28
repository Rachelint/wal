use std::path::Path;

/// The manager for multiple pages.
pub trait PageManager {
    type Page;

    /// Create a new page.
    fn create(&self) -> Box<dyn Page>;

    /// Delete the target page.
    // FIX: Replace the `P`. 
    fn delete<P: AsRef<Path>>(&self, path: P);

    /// Write the data to the target page.
    fn write(&mut self, target: Self::Page, data: &[u8]);

    /// Read the data in the target page to the `buf`.
    fn read(&self, target: Self::Page, buf: &mut [u8]);
}

/// The storage unit.
pub trait Page {
    /// Read the page data to the `buf`.
    fn read(&self, buf: &mut [u8]);

    /// Write the data to the page.
    fn write(&mut self, data: &[u8]);
}
