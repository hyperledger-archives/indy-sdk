extern crate indy_crypto;

use super::{TailsReader, TailsReaderType};

pub struct DefaultTailsReader {}

impl TailsReaderType for DefaultTailsReaderType {
    fn open(&self, config: &str) -> Box<TailsReader> {
        unimplemented!()
    }
}

impl TailsReader for DefaultTailsReader {
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

pub struct DefaultTailsReaderType {}

impl DefaultTailsReaderType {
    pub fn new() -> Self {
        DefaultTailsReaderType {}
    }
}
