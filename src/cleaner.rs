#![allow(dead_code)]

use crate::{log::SequenceNumber, meta_data::MetaDataStore, storage::page_manager::{Page, PageManager}};

pub struct LogCleaner<M: PageManager, T: MetaDataStore> {
    page_creator: M,
    metadata_store: T,
}

impl<M: PageManager, T: MetaDataStore> LogCleaner<M, T> {
    pub fn new() -> LogCleaner<M, T> {
        unimplemented!()
    }

    pub fn mark_delete_entries_up_to<P: Page>(&mut self, _sequence_num: SequenceNumber) -> P {
        unimplemented!()
    }
}
