mod default_writer;
mod default_reader;

extern crate digest;
extern crate indy_crypto;
extern crate sha2;

use errors::common::CommonError;
use utils::sequence::SequenceUtils;

use self::digest::{FixedOutput, Input};
use self::sha2::Sha256;

use std::cell::RefCell;
use std::collections::HashMap;

trait WriterType {
    fn open(&self, config: &str) -> Result<Box<Writer>, CommonError>;
}

trait Writer {
    fn create(&self, id: i32) -> Result<Box<WritableBlob>, CommonError>;
}

trait WritableBlob {
    fn append(&mut self, bytes: &[u8]) -> Result<usize, CommonError>;
    fn finalize(&mut self, hash: &[u8]) -> Result<String, CommonError>;
}

trait ReaderType {
    fn open(&self, config: &str) -> Result<Box<Reader>, CommonError>;
}

trait Reader {
    fn open(&self, hash: &[u8], location: &str) -> Result<Box<ReadableBlob>, CommonError>;
}

trait ReadableBlob {
    fn read(&mut self, size: usize, offset: usize) -> Result<Vec<u8>, CommonError>;
    fn verify(&mut self) -> Result<bool, CommonError>;
    fn close(&self) -> Result<(), CommonError>;
}

pub struct BlobStorageService {
    writer_types: RefCell<HashMap<String, Box<WriterType>>>,
    writer_configs: RefCell<HashMap<i32, Box<Writer>>>,
    writer_blobs: RefCell<HashMap<i32, (Box<WritableBlob>, Sha256)>>,

    reader_types: RefCell<HashMap<String, Box<ReaderType>>>,
    reader_configs: RefCell<HashMap<i32, Box<Reader>>>,
    reader_blobs: RefCell<HashMap<i32, Box<ReadableBlob>>>,
}

impl BlobStorageService {
    pub fn new() -> BlobStorageService {
        let mut writer_types: HashMap<String, Box<WriterType>> = HashMap::new();
        writer_types.insert("default".to_owned(), Box::new(default_writer::DefaultWriterType::new()));
        let mut reader_types: HashMap<String, Box<ReaderType>> = HashMap::new();
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
    pub fn open_writer(&self, type_: &str, config: &str) -> Result<i32, CommonError> {
        let writer_config = self.writer_types.try_borrow()?
            .get(type_).ok_or(CommonError::InvalidStructure("Unknown BlobStorage Writer type".to_string()))?
            .open(config)?;

        let config_handle = SequenceUtils::get_next_id();
        self.writer_configs.try_borrow_mut()?.insert(config_handle, writer_config);

        Ok(config_handle)
    }

    pub fn create_blob(&self, config_handle: i32) -> Result<i32, CommonError> {
        let blob_handle = SequenceUtils::get_next_id();
        let writer = self.writer_configs.try_borrow()?
            .get(&config_handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage Writer".to_owned()))?
            .create(blob_handle)?;

        self.writer_blobs.try_borrow_mut()?.insert(blob_handle, (writer, Sha256::default()));

        Ok(blob_handle)
    }

    pub fn append(&self, handle: i32, bytes: &[u8]) -> Result<usize, CommonError> {
        let mut writers = self.writer_blobs.try_borrow_mut()?;
        let &mut (ref mut writer, ref mut hasher) = writers
            .get_mut(&handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage handle Blob to append".to_owned()))?;

        hasher.process(bytes);
        writer.append(bytes)
    }

    pub fn finalize(&self, handle: i32) -> Result<(String, Vec<u8>), CommonError> {
        let mut writers = self.writer_blobs.try_borrow_mut()?;
        let (mut writer, hasher) = writers
            .remove(&handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage handle Blob to finalize".to_owned()))?;

        let hash = hasher.fixed_result().to_vec();

        writer.finalize(hash.as_slice())
            .map(|location| (location, hash))
    }
}

/* Reader */
impl BlobStorageService {
    pub fn open_reader(&self, type_: &str, config: &str) -> Result<i32, CommonError> {
        let reader_config = self.reader_types.try_borrow()?
            .get(type_).ok_or(CommonError::InvalidStructure("Unknown BlobStorage Reader type".to_string()))?
            .open(config)?;

        let config_handle = SequenceUtils::get_next_id();
        self.reader_configs.try_borrow_mut()?.insert(config_handle, reader_config);

        Ok(config_handle)
    }

    pub fn open_blob(&self, config_handle: i32, location: &str, hash: &[u8]) -> Result<i32, CommonError> {
        let reader = self.reader_configs.try_borrow()?
            .get(&config_handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage Reader".to_string()))?
            .open(hash, location)?;

        let reader_handle = SequenceUtils::get_next_id();
        self.reader_blobs.try_borrow_mut()?.insert(reader_handle, reader);

        Ok(reader_handle)
    }

    pub fn read(&self, handle: i32, size: usize, offset: usize) -> Result<Vec<u8>, CommonError> {
        self.reader_blobs.try_borrow_mut()?
            .get_mut(&handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage handle Blob to read".to_owned()))?
            .read(size, offset)
    }

    pub fn _verify(&self, handle: i32) -> Result<bool, CommonError> {
        self.reader_blobs.try_borrow_mut()?
            .get_mut(&handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage handle Blob to verify".to_owned()))?
            .verify()
    }

    pub fn close(&self, handle: i32) -> Result<(), CommonError> {
        self.reader_blobs.try_borrow_mut()?
            .remove(&handle).ok_or(CommonError::InvalidStructure("Unknown BlobStorage handle Blob to close".to_owned()))?
            .close()
    }
}
