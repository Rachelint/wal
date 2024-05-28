#![allow(dead_code)]

use crate::{deleter::WalDeletionMarker, log::LogEntry, options::SequenceNumber, page::{Page, PageManager}, reader::LogReader, writer::LogWriter};

pub struct WalManager<P: Page, M: PageManager> {
    sequence_num: SequenceNumber,
    writer: LogWriter<P, M>,
    reader: LogReader,
    deletion_marker: WalDeletionMarker<P, M>,
}

impl<P: Page, M: PageManager> WalManager<P, M> {
    pub fn new() -> WalManager<P, M> {
        unimplemented!()
    }

    /// Get the current sequence number.
    pub fn sequence_number(&self) -> SequenceNumber {
        unimplemented!()
    }

    /// Provide iterator on necessary entries.
    pub fn read(&self) {
        unimplemented!()
    }

    /// Append the logs to the underlying storage.
    // TODO: Implement `write_batch()`.
    pub fn write(&mut self, _log: LogEntry) {
        unimplemented!()
    }

    /// Mark the entries whose sequence number is in [0, `_sequence_num`] to 
    /// be deleted in the future.
    pub fn mark_entries_up_to(&self, _sequence_num: SequenceNumber) {
        unimplemented!()
    }
}
