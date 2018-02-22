extern crate indy_crypto;

use super::{Reader, ReaderType};

pub struct DefaultReader {}

impl ReaderType for DefaultReaderType {
    fn open(&self, config: &str) -> Box<Reader> {
        unimplemented!()
    }
}

impl Reader for DefaultReader {
    fn verify(&self) -> () {
        unimplemented!()
    }

    fn close(&self) -> () {
        unimplemented!()
    }

    fn read(&self, size: usize, offset: usize) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct DefaultReaderType {}

impl DefaultReaderType {
    pub fn new() -> Self {
        DefaultReaderType {}
    }
}
