//! Implement the trait `Page` for `File` and the trait `PageManager` for `FileSystem`.

use std::fs::File;

use crate::storage::page_manager::{Page, PageManager};

struct FileSystem;

impl Page for File {
    fn read(&self, _buf: &mut [u8]) {
        unimplemented!() 
    }
     
    fn write(&mut self, _data: &[u8]) {
        unimplemented!() 
    }
}

impl PageManager for FileSystem {
    type Page = Box<dyn Page>;

    fn create(&self) -> Box<dyn Page> {
        unimplemented!()
    }
    
    fn delete<P: AsRef<std::path::Path>>(&self, _path: P) {
        unimplemented!() 
    }
}
