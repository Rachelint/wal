#![allow(dead_code)]

/// An abstract appended log.
pub trait LogEntry {
    type MetaData;

    /// Get the metadata part of the log entry.
    fn metadata(&self) -> Self::MetaData;

    /// Encode the log entry (include the data and metadata) to bytes.
    fn to_bytes(&self) -> Vec<u8>;

    /// Decode from bytes.
    fn from_bytes(raw: Vec<u8>) -> Self;
}

pub enum RecordType {
    Full = 1, 
    First = 2,
    Middle = 3,
    Last = 4,
}

// TODO: Put this type in a better place.
pub type SequenceNumber = u64;

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
// TODO: Should we create a new folder for different implementation for `LogEntry`?
pub struct LegacyLogEntry {
    sequence_num: SequenceNumber,
    payload: Vec<u8>,
}

impl LogEntry for LegacyLogEntry {
    type MetaData = SequenceNumber;

    fn metadata(&self) -> Self::MetaData {
        unimplemented!() 
    }

    fn from_bytes(_raw: Vec<u8>) -> Self {
        unimplemented!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        unimplemented!() 
    }
}
