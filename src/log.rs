#![allow(dead_code)]

use std::fs::Metadata;

use bytes::Bytes;

use crate::options::SequenceNumber;

pub enum RecordType {
    Full = 1, 
    First = 2,
    Middle = 3,
    Last = 4,
}

/// **Legacy Record Format**
/// ```text
/// +---------+-----------+-----------+--- ... ---+
/// |CRC (4B) | Size (2B) | Type (1B) | Payload   |
/// +---------+-----------+-----------+--- ... ---+
/// ```
/// CRC = 32bit hash computed over the payload using CRC
/// Size = Length of the payload data
/// Type = Type of record
///        (kZeroType, kFullType, kFirstType, kLastType, kMiddleType)
///        The type is used to group a bunch of records together to represent
///        blocks that are larger than kBlockSize
/// Payload = Byte stream as long as specified by the payload size

// Comment(kamille):
// I think it may be better to be a trait? for example:
// pub trait LogEntry {
//     // Somethings like `sequence_num`, `table_id` can be place in MetaData.
//     type MetaData: Send + 'static;
    
//     /// Get meta data part of the log entry.
//     fn metadata(&self) -> Metadata;
    
//     /// Encode the log entry(include data and metadata) to bytes.
//     fn to_bytes(&self) -> Vec<u8>;

//     /// Decode from bytes
//     fn from_bytes(raw: Vec<u8>);
// }

pub struct LogEntry {
    sequence_num: SequenceNumber,
    payload: Vec<u8>,
}

impl LogEntry {
    pub fn new() -> LogEntry {
        unimplemented!()
    }

    pub fn encode(&self) -> Bytes {
        unimplemented!()
    }

    pub fn decode(&self, _data: &[u8]) -> LogEntry {
        unimplemented!()
    }
}
