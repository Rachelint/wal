#![allow(dead_code)]

use crate::{options::SequenceNumber, page::{Page, PageManager}};

// FIX: Redefine the membership of the `WalDeletionMarker`.
// TODO: Make use of `HashSet` to record.
pub struct WalDeletionMarker<P: Page, M: PageManager> {
    page: P, 
    creator: M,
}

impl<P: Page, M: PageManager> WalDeletionMarker<P, M> {
    pub fn new() -> WalDeletionMarker<P, M> {
        unimplemented!()
    }

    pub fn mark_delete_entries_up_to(&self, _sequence_num: SequenceNumber) {
        unimplemented!()
    }
}

pub struct WalCleaner<M: PageManager> {
    checker: DeleteChecker,
    deleter: M,
}

impl<M: PageManager> WalCleaner<M> {
    pub fn new() -> WalCleaner<M> {
        unimplemented!()
    }

    /// Remove the obsolete page.
    pub fn purge_obsolete_page(&self) {
        unimplemented!()
    } 

    /// Scan the pages' metadata and get the obsolete pages.
    fn get_obsolete_page(&self) -> Vec<Box<dyn Page>> {
        unimplemented!()
    }
}

pub struct DeleteChecker {
    // TODO
}

impl DeleteChecker {
    pub fn new() -> DeleteChecker {
        unimplemented!() 
    }

    pub fn check_obsolete_page(&self) -> Vec<Box<dyn Page>> {
        unimplemented!()
    }

    fn is_obsolete(&self, _page: &dyn Page) -> bool {
        unimplemented!()
    }
}
