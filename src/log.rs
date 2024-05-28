#![allow(dead_code)]

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
pub struct LogEntry {
    sequence_num: SequenceNumber,
    data: Vec<u8>,
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
