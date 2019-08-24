use std::cell::RefCell;
use std::collections::HashMap;

use errors::prelude::*;
use utils::sequence;

use sha2::Sha256;
use sha2::digest::{FixedOutput, Input};

mod default_writer;
mod default_reader;

trait WriterType {
    fn open(&self, config: &str) -> IndyResult<Box<dyn Writer>>;
}

trait Writer {
    fn create(&self, id: i32) -> IndyResult<Box<dyn WritableBlob>>;
}

trait WritableBlob {
    fn append(&mut self, bytes: &[u8]) -> IndyResult<usize>;
    fn finalize(&mut self, hash: &[u8]) -> IndyResult<String>;
}

trait ReaderType {
    fn open(&self, config: &str) -> IndyResult<Box<dyn Reader>>;
}

trait Reader {
    fn open(&self, hash: &[u8], location: &str) -> IndyResult<Box<dyn ReadableBlob>>;
}

trait ReadableBlob {
    fn read(&mut self, size: usize, offset: usize) -> IndyResult<Vec<u8>>;
    fn verify(&mut self) -> IndyResult<bool>;
    fn close(&self) -> IndyResult<()>;
}

pub struct BlobStorageService {
    writer_types: RefCell<HashMap<String, Box<dyn WriterType>>>,
    writer_configs: RefCell<HashMap<i32, Box<dyn Writer>>>,
    writer_blobs: RefCell<HashMap<i32, (Box<dyn WritableBlob>, Sha256)>>,

    reader_types: RefCell<HashMap<String, Box<dyn ReaderType>>>,
    reader_configs: RefCell<HashMap<i32, Box<dyn Reader>>>,
    reader_blobs: RefCell<HashMap<i32, Box<dyn ReadableBlob>>>,
}

impl BlobStorageService {
    pub fn new() -> BlobStorageService {
        let mut writer_types: HashMap<String, Box<dyn WriterType>> = HashMap::new();
        writer_types.insert("default".to_owned(), Box::new(default_writer::DefaultWriterType::new()));
        let mut reader_types: HashMap<String, Box<dyn ReaderType>> = HashMap::new();
        reader_types.insert("default".to_owned(), Box::new(default_reader::DefaultReaderType::new()));

        BlobStorageService {
            writer_types: RefCell::new(writer_types),
            writer_configs: RefCell::new(HashMap::new()),
            writer_blobs: RefCell::new(HashMap::new()),

            reader_types: RefCell::new(reader_types),
            reader_configs: RefCell::new(HashMap::new()),
            reader_blobs: RefCell::new(HashMap::new()),
        }
    }
}

/* Writer */
impl BlobStorageService {
    pub fn open_writer(&self, type_: &str, config: &str) -> IndyResult<i32> {
        let writer_config = self.writer_types.try_borrow()?
            .get(type_).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Unknown BlobStorage Writer type"))?
            .open(config)?;

        let config_handle = sequence::get_next_id();
        self.writer_configs.try_borrow_mut()?.insert(config_handle, writer_config);

        Ok(config_handle)
    }

    pub fn create_blob(&self, config_handle: i32) -> IndyResult<i32> {
        let blob_handle = sequence::get_next_id();
        let writer = self.writer_configs.try_borrow()?
            .get(&config_handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage config handle"))? // FIXME: Review error kind
            .create(blob_handle)?;

        self.writer_blobs.try_borrow_mut()?.insert(blob_handle, (writer, Sha256::default()));

        Ok(blob_handle)
    }

    pub fn append(&self, handle: i32, bytes: &[u8]) -> IndyResult<usize> {
        let mut writers = self.writer_blobs.try_borrow_mut()?;
        let &mut (ref mut writer, ref mut hasher) = writers
            .get_mut(&handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage handle"))?; // FIXME: Review error kind

        hasher.input(bytes);
        writer.append(bytes)
    }

    pub fn finalize(&self, handle: i32) -> IndyResult<(String, Vec<u8>)> {
        let mut writers = self.writer_blobs.try_borrow_mut()?;
        let (mut writer, hasher) = writers
            .remove(&handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage handle"))?; // FIXME: Review error kind

        let hash = hasher.fixed_result().to_vec();

        writer.finalize(hash.as_slice())
            .map(|location| (location, hash))
    }
}

/* Reader */
impl BlobStorageService {
    pub fn open_reader(&self, type_: &str, config: &str) -> IndyResult<i32> {
        let reader_config = self.reader_types.try_borrow()?
            .get(type_).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage Reader type"))? // FIXME: Review error kind
            .open(config)?;

        let config_handle = sequence::get_next_id();
        self.reader_configs.try_borrow_mut()?.insert(config_handle, reader_config);

        Ok(config_handle)
    }

    pub fn open_blob(&self, config_handle: i32, location: &str, hash: &[u8]) -> IndyResult<i32> {
        let reader = self.reader_configs.try_borrow()?
            .get(&config_handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage config handle"))? // FIXME: Review error kind
            .open(hash, location)?;

        let reader_handle = sequence::get_next_id();
        self.reader_blobs.try_borrow_mut()?.insert(reader_handle, reader);

        Ok(reader_handle)
    }

    pub fn read(&self, handle: i32, size: usize, offset: usize) -> IndyResult<Vec<u8>> {
        self.reader_blobs.try_borrow_mut()?
            .get_mut(&handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage handle"))? // FIXME: Review error kind
            .read(size, offset)
    }

    pub fn _verify(&self, handle: i32) -> IndyResult<bool> {
        self.reader_blobs.try_borrow_mut()?
            .get_mut(&handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage handle"))? // FIXME: Review error kind
            .verify()
    }

    pub fn close(&self, handle: i32) -> IndyResult<()> {
        self.reader_blobs.try_borrow_mut()?
            .remove(&handle).ok_or_else(|| err_msg(IndyErrorKind::InvalidStructure, "Invalid BlobStorage handle"))? // FIXME: Review error kind
            .close()
    }
}
