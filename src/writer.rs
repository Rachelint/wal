#![allow(dead_code)]

use crate::storage::page_manager::{Page, PageManager};
use crate::log::LogEntry;

use std::sync::{Arc, Mutex};

/// The `LogWriter` is designed to append the logs to the page.
pub struct LogWriter<P: Page, M: PageManager> {
    dest: Arc<Mutex<P>>,
    
    creator: M,
    // TODO: Calculate the `checksum` with `crc`.
    
    // I think it is not the thing should be cared in a general wal crate?
    // As I see, maybe better for specific app to consider that?
    checksum: u32, 
}

impl<P: Page, M: PageManager> LogWriter<P, M> {
    pub fn new() -> LogWriter<P, M> {
        unimplemented!()
    }

    /// Append the log to the dest page.
    pub fn append_log(&mut self, _log: LogEntry) {
        unimplemented!()
    } 

    // Comment(kamille):
    // I think flush should be a specific behavior of file system wal?
    // Not so good to expose it in logic layer?    
    pub fn flush(&mut self) {
        unimplemented!()
    }

    /// Decide whether change to the new page or not.
    fn full_after_write(&self) -> bool {
        unimplemented!()
    }

    /// Create a new page and change the `dest` to the new page.
    fn change_page(&mut self) {
        unimplemented!()
    }
}
