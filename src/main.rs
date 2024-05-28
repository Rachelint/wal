mod deleter;
mod log;
mod manager;
mod options;
mod storage;
mod reader;
mod writer;
mod meta_data;

// Comment(kamille):
// The general design I think may be like:
//  
// - Logic level
//   - WalManager, total read/write/delete entry
//     - LogWrite, write logic
//     - LogReader, read logic
//     - LogCleaner, clean logic
//     - MetaDataStore, record meta to help determine which pages can be cleaned
//     - LogEntry, a abstract appended log, should include data and metadata
//
// - Storage level
//   - PageManager, manage the page
//   - Page, the abstraction of writing unit, can be Kafka topic, file...
fn main() {
    println!("Hello, World!");
}
