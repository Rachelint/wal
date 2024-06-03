#![allow(dead_code)]

use crate::storage::page_manager::{Page, PageManager};
use crate::log::LogEntry;

use std::sync::{Arc, Mutex};

/// The `LogWriter` is designed to append the logs to the page.
pub struct LogWriter<P: Page, M: PageManager> {
    dest: Arc<Mutex<P>>,
    page_creator: M,
}

impl<P: Page, M: PageManager> LogWriter<P, M> {
    pub fn new() -> LogWriter<P, M> {
        unimplemented!()
    }

    /// Append the log to the dest page.
    pub fn append_log<L: LogEntry>(&mut self, _log: L) {
        unimplemented!()
    } 

    /// Decide whether the `dest` change to the new page or not.
    fn full_after_write(&self) -> bool {
        unimplemented!()
    }

    /// Create a new page and change the `dest` to the new page.
    pub fn change_page(&mut self, _new_page: Option<P>) {
        unimplemented!()
    }
}
