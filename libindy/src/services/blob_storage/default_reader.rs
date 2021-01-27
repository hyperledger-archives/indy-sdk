use std::{io::SeekFrom, path::PathBuf};

use std::fs::File as SyncFile;
use async_std::{fs::File, prelude::*};
use async_trait::async_trait;
use indy_api_types::errors::prelude::*;
use indy_utils::crypto::hash::Hash;
use rust_base58::ToBase58;
use serde_json;

use super::{ReadableBlob, Reader, ReaderType};
use std::io::{Read, Seek};

pub struct DefaultReader {
    file: SyncFile,
    hash: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct DefaultReaderConfig {
    base_dir: String,
}

#[async_trait]
impl ReaderType for DefaultReaderType {
    async fn open(&self, config: &str) -> IndyResult<Box<dyn Reader>> {
        let config: DefaultReaderConfig = serde_json::from_str(config).to_indy(
            IndyErrorKind::InvalidStructure,
            "Can't deserialize DefaultReaderConfig",
        )?;

        Ok(Box::new(config))
    }
}

#[async_trait]
impl Reader for DefaultReaderConfig {
    async fn open(&self, hash: &[u8], _location: &str) -> IndyResult<Box<dyn ReadableBlob>> {
        let mut path = PathBuf::from(&self.base_dir);
        path.push(hash.to_base58());

        let file = SyncFile::open(path)?;

        Ok(Box::new(DefaultReader {
            file,
            hash: hash.to_owned(),
        }))
    }
}

#[async_trait]
impl ReadableBlob for DefaultReader {
    async fn verify(&mut self) -> IndyResult<bool> {
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
