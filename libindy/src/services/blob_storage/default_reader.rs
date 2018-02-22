extern crate indy_crypto;

use base64;

use errors::common::CommonError;

use super::{Reader, ReaderType};
use self::indy_crypto::utils::json::JsonDecodable;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct DefaultReader {
    file: File
}

#[derive(Serialize, Deserialize)]
struct DefaultReaderConfig {
    base_dir: String,
}

impl<'a> JsonDecodable<'a> for DefaultReaderConfig {}

impl ReaderType for DefaultReaderType {
    fn open(&self, config: &str, hash: &[u8], location: &str) -> Result<Box<Reader>, CommonError> {
        let cfg = DefaultReaderConfig::from_json(config)?;
        let mut path = PathBuf::from(cfg.base_dir);
        path.push(base64::encode(hash));
        let file = File::open(path)?;
        Ok(Box::new(DefaultReader {
            file
        }))
    }
}

impl Reader for DefaultReader {
    fn verify(&self) -> () {
        unimplemented!()
    }

    fn close(&self) -> () {
        /* nothing to do */
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
