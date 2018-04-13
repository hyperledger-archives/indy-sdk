extern crate digest;
extern crate indy_crypto;
extern crate sha2;
extern crate rust_base58;

use errors::common::CommonError;

use super::{ReadableBlob, Reader, ReaderType};
use self::indy_crypto::utils::json::JsonDecodable;
use self::digest::{FixedOutput, Input};
use self::sha2::Sha256;
use self::rust_base58::ToBase58;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct DefaultReader {
    file: File,
    hash: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct DefaultReaderConfig {
    base_dir: String,
}

impl<'a> JsonDecodable<'a> for DefaultReaderConfig {}

impl ReaderType for DefaultReaderType {
    fn open(&self, config: &str) -> Result<Box<Reader>, CommonError> {
        let cfg = DefaultReaderConfig::from_json(config)?;
        Ok(Box::new(cfg))
    }
}

impl Reader for DefaultReaderConfig {
    fn open(&self, hash: &[u8], _location: &str) -> Result<Box<ReadableBlob>, CommonError> {
        let mut path = PathBuf::from(&self.base_dir);
        path.push(hash.to_base58());
        let file = File::open(path)?;
        Ok(Box::new(DefaultReader {
            file,
            hash: hash.to_owned()
        }))
    }
}

impl ReadableBlob for DefaultReader {
    fn verify(&mut self) -> Result<bool, CommonError> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut hasher = Sha256::default();
        let mut buf = [0u8; 1024];

        loop {
            let sz = self.file.read(&mut buf)?;
            if sz == 0 {
                return Ok(hasher.fixed_result().as_slice().eq(self.hash.as_slice()));
            }
            hasher.process(&buf[0..sz])
        }
    }

    fn close(&self) -> Result<(), CommonError> {
        /* nothing to do */
        Ok(())
    }

    fn read(&mut self, size: usize, offset: usize) -> Result<Vec<u8>, CommonError> {
        let mut buf = vec![0u8; size];

        self.file.seek(SeekFrom::Start(offset as u64))?;
        let act_size = self.file.read(buf.as_mut_slice())?;

        buf.truncate(act_size);

        Ok(buf)
    }
}

pub struct DefaultReaderType {}

impl DefaultReaderType {
    pub fn new() -> Self {
        DefaultReaderType {}
    }
}
