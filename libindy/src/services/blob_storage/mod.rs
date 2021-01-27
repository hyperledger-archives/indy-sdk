use std::collections::HashMap;

use async_trait::async_trait;
use indy_api_types::errors::prelude::*;
use indy_utils::sequence;

use std::sync::Mutex as SyncMutex;
use futures::lock::Mutex;
use sha2::{
    digest::{FixedOutput, Input},
    Sha256,
};
use failure::_core::cell::RefCell;

mod default_reader;
mod default_writer;

#[async_trait]
trait WriterType: Send + Sync {
    async fn open(&self, config: &str) -> IndyResult<Box<dyn Writer>>;
}

#[async_trait]
trait Writer: Send + Sync {
    async fn create(&self, id: i32) -> IndyResult<Box<dyn WritableBlob>>;
}

#[async_trait]
trait WritableBlob: Send + Sync {
    async fn append(&mut self, bytes: &[u8]) -> IndyResult<usize>;
    async fn finalize(&mut self, hash: &[u8]) -> IndyResult<String>;
}

#[async_trait]
trait ReaderType: Send + Sync {
    async fn open(&self, config: &str) -> IndyResult<Box<dyn Reader>>;
}

#[async_trait]
trait Reader: Send + Sync {
    async fn open(&self, hash: &[u8], location: &str) -> IndyResult<Box<dyn ReadableBlob>>;
}

#[async_trait]
trait ReadableBlob: Send + Sync {
    fn read(&mut self, size: usize, offset: usize) -> IndyResult<Vec<u8>>;
    async fn verify(&mut self) -> IndyResult<bool>;
    fn close(&self) -> IndyResult<()>;
}

pub struct BlobStorageService {
    writer_types: Mutex<HashMap<String, Box<dyn WriterType>>>,
    writer_configs: Mutex<HashMap<i32, Box<dyn Writer>>>,
    writer_blobs: Mutex<HashMap<i32, (Box<dyn WritableBlob>, Sha256)>>,

    reader_types: Mutex<HashMap<String, Box<dyn ReaderType>>>,
    reader_configs: Mutex<HashMap<i32, Box<dyn Reader>>>,
    reader_blobs: SyncMutex<HashMap<i32, Box<dyn ReadableBlob>>>,
}

impl BlobStorageService {
    pub fn new() -> BlobStorageService {
        let mut writer_types: HashMap<String, Box<dyn WriterType>> = HashMap::new();
        writer_types.insert(
            "default".to_owned(),
            Box::new(default_writer::DefaultWriterType::new()),
        );

        let mut reader_types: HashMap<String, Box<dyn ReaderType>> = HashMap::new();
        reader_types.insert(
            "default".to_owned(),
            Box::new(default_reader::DefaultReaderType::new()),
        );

        BlobStorageService {
            writer_types: Mutex::new(writer_types),
            writer_configs: Mutex::new(HashMap::new()),
            writer_blobs: Mutex::new(HashMap::new()),

            reader_types: Mutex::new(reader_types),
            reader_configs: Mutex::new(HashMap::new()),
            reader_blobs: SyncMutex::new(HashMap::new()),
        }
    }
}

/* Writer */
impl BlobStorageService {
    pub async fn open_writer(&self, type_: &str, config: &str) -> IndyResult<i32> {
        let writer_config = self
            .writer_types
            .lock()
            .await
            .get(type_)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Unknown BlobStorage Writer type",
                )
            })?
            .open(config)
            .await?;

        let config_handle = sequence::get_next_id();

        self.writer_configs
            .lock()
            .await
            .insert(config_handle, writer_config);

        Ok(config_handle)
    }

    pub async fn create_blob(&self, config_handle: i32) -> IndyResult<i32> {
        let blob_handle = sequence::get_next_id();

        let writer = self
            .writer_configs
            .lock()
            .await
            .get(&config_handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage config handle",
                )
            })? // FIXME: Review error kind
            .create(blob_handle)
            .await?;

        self.writer_blobs
            .lock()
            .await
            .insert(blob_handle, (writer, Sha256::default()));

        Ok(blob_handle)
    }

    pub async fn append(&self, handle: i32, bytes: &[u8]) -> IndyResult<usize> {
        let mut writers = self.writer_blobs.lock().await;

        let &mut (ref mut writer, ref mut hasher) = writers.get_mut(&handle).ok_or_else(|| {
            err_msg(
                IndyErrorKind::InvalidStructure,
                "Invalid BlobStorage handle",
            )
        })?; // FIXME: Review error kind

        hasher.input(bytes);
        let res = writer.append(bytes).await?;
        Ok(res)
    }

    pub async fn finalize(&self, handle: i32) -> IndyResult<(String, Vec<u8>)> {
        let mut writers = self.writer_blobs.lock().await;

        let (mut writer, hasher) = writers.remove(&handle).ok_or_else(|| {
            err_msg(
                IndyErrorKind::InvalidStructure,
                "Invalid BlobStorage handle",
            )
        })?; // FIXME: Review error kind

        let hash = hasher.fixed_result().to_vec();

        writer
            .finalize(hash.as_slice())
            .await
            .map(|location| (location, hash))
    }
}

/* Reader */
impl BlobStorageService {
    pub async fn open_reader(&self, type_: &str, config: &str) -> IndyResult<i32> {
        let reader_config = self
            .reader_types
            .lock()
            .await
            .get(type_)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage Reader type",
                )
            })? // FIXME: Review error kind
            .open(config)
            .await?;

        let config_handle = sequence::get_next_id();

        self.reader_configs
            .lock()
            .await
            .insert(config_handle, reader_config);

        Ok(config_handle)
    }

    pub async fn open_blob(
        &self,
        config_handle: i32,
        location: &str,
        hash: &[u8],
    ) -> IndyResult<i32> {
        let reader = self
            .reader_configs
            .lock()
            .await
            .get(&config_handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage config handle",
                )
            })? // FIXME: Review error kind
            .open(hash, location)
            .await?;

        let reader_handle = sequence::get_next_id();
        self.reader_blobs.lock().unwrap().insert(reader_handle, reader);

        Ok(reader_handle)
    }

    pub fn read(&self, handle: i32, size: usize, offset: usize) -> IndyResult<Vec<u8>> {
        self.reader_blobs
            .lock().unwrap()
            .get_mut(&handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage handle",
                )
            })? // FIXME: Review error kind
            .read(size, offset)
    }

    pub async fn _verify(&self, handle: i32) -> IndyResult<bool> {
        let res = self
            .reader_blobs
            .lock().unwrap()
            .get_mut(&handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage handle",
                )
            })? // FIXME: Review error kind
            .verify()
            .await?;

        Ok(res)
    }

    pub fn close(&self, handle: i32) -> IndyResult<()> {
        self.reader_blobs
            .lock().unwrap()
            .remove(&handle)
            .ok_or_else(|| {
                err_msg(
                    IndyErrorKind::InvalidStructure,
                    "Invalid BlobStorage handle",
                )
            })? // FIXME: Review error kind
            .close()
    }
}
