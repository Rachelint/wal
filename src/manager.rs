#![allow(dead_code)]

use crate::{cleaner::LogCleaner, log::{LogEntry, SequenceNumber}, meta_data::MetaDataStore, reader::LogReader, storage::page_manager::{Page, PageManager}, writer::LogWriter};

// TODO: Make the `WalManager` more general.
pub struct WalManager<P: Page, M: PageManager, T: MetaDataStore> {
    sequence_num: SequenceNumber,
    writer: LogWriter<P, M>,
    reader: LogReader,
    cleaner: LogCleaner<M, T>,
}

impl<P: Page, M: PageManager, T: MetaDataStore> WalManager<P, M, T> {
    pub fn new() -> WalManager<P, M, T> {
        unimplemented!()
    }

    /// Provide iterator on necessary entries.
    pub fn read(&self) {
        unimplemented!()
    }

    /// Append the logs to the underlying storage.
    // TODO: Implement `write_batch()`.
    pub fn write<L: LogEntry>(&mut self, _log: L) {
        unimplemented!()
    }

    /// Mark the entries whose sequence number is in [0, `_sequence_num`] to 
    /// be deleted in the future.
    pub fn mark_entries_up_to(&self, _sequence_num: SequenceNumber) {
        unimplemented!()
    }
}
