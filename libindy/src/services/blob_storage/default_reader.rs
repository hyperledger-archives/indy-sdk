use rust_base58::ToBase58;

use super::{ReadableBlob, Reader, ReaderType};
use indy_api_types::errors::prelude::*;

use serde_json;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use indy_utils::crypto::hash::Hash;

pub struct DefaultReader {
    file: File,
    hash: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct DefaultReaderConfig {
    base_dir: String,
}

impl ReaderType for DefaultReaderType {
    fn open(&self, config: &str) -> IndyResult<Box<dyn Reader>> {
        let config: DefaultReaderConfig = serde_json::from_str(config)
            .to_indy(IndyErrorKind::InvalidStructure, "Can't deserialize DefaultReaderConfig")?;

        Ok(Box::new(config))
    }
}

impl Reader for DefaultReaderConfig {
    fn open(&self, hash: &[u8], _location: &str) -> IndyResult<Box<dyn ReadableBlob>> {
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

    fn verify(&mut self) -> IndyResult<bool> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut hasher = Hash::new_context()?;
        let mut buf = [0u8; 1024];

        loop {
            let sz = self.file.read(&mut buf)?;

            if sz == 0 {
                return Ok(hasher.finish()?.to_vec().eq(&self.hash));
            }

            hasher.update(&buf[0..sz])?;
        }
    }

    fn close(&self) -> IndyResult<()> {
        /* nothing to do */
        Ok(())
    }

    fn read(&mut self, size: usize, offset: usize) -> IndyResult<Vec<u8>> {
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
