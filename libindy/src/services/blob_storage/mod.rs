pub mod default_writer;
pub mod default_reader;

extern crate digest;
extern crate indy_crypto;
extern crate sha2;

use self::digest::{FixedOutput, Input};
use self::indy_crypto::cl::RevocationTailsGenerator;
use self::indy_crypto::cl::Tail;

use errors::common::CommonError;

use std::cell::RefCell;
use std::collections::HashMap;

const TAILS_BLOB_TAG_SZ: usize = 2;
const TAIL_SIZE: usize = Tail::BYTES_REPR_SIZE;

trait WriterType {
    fn create(&self, config: &str) -> Result<Box<Writer>, CommonError>;
}

trait Writer {
    fn append(&mut self, bytes: &[u8]) -> Result<usize, CommonError>;
    fn finalize(&mut self, hash: &[u8]) -> Result<String, CommonError>;
}

trait ReaderType {
    fn open(&self, config: &str) -> Box<Reader>;
}

trait Reader {
    fn verify(&self) -> ();
    fn close(&self) -> ();
    fn read(&self, size: usize, offset: usize) -> Vec<u8>;
}

pub struct BlobStorageService {
    writer_types: RefCell<HashMap<String, Box<WriterType>>>,

    reader_types: RefCell<HashMap<String, Box<ReaderType>>>,
    readers: RefCell<HashMap<u32, Box<Reader>>>,
}

impl BlobStorageService {
    pub fn new() -> BlobStorageService {
        let mut writer_types: HashMap<String, Box<WriterType>> = HashMap::new();
        writer_types.insert("default".to_owned(), Box::new(default_writer::DefaultWriterType::new()));
        let mut reader_types: HashMap<String, Box<ReaderType>> = HashMap::new();
        reader_types.insert("default".to_owned(), Box::new(default_reader::DefaultReaderType::new()));

        BlobStorageService {
            writer_types: RefCell::new(writer_types),

            reader_types: RefCell::new(reader_types),
            readers: RefCell::new(HashMap::new()),
        }
    }
}

/* Writer part */
impl BlobStorageService {
    pub fn store_tails_from_generator(&self,
                                      type_: &str,
                                      config: &str,
                                      rtg: &mut RevocationTailsGenerator)
                                      -> Result<String, CommonError> {
        let mut tails_writer = self.writer_types.try_borrow()?
            .get(type_).unwrap().create(config)?; //FIXME UnknownType error instead of unwrap
        let mut hasher = sha2::Sha256::default();

        //FIXME store version/tag/meta at start of the Tail's BLOB

        while let Some(tail) = rtg.next()? {
            let tail_bytes = tail.to_bytes()?;
            hasher.process(tail_bytes.as_slice());
            tails_writer.append(tail_bytes.as_slice())?;
        }

        tails_writer.finalize(hasher.fixed_result().as_slice())
    }
}

/* Reader part */
impl BlobStorageService {
    pub fn read(&self, handle: u32, idx: usize) -> Tail {
        let bytes = self.readers.borrow().get(&handle).unwrap()
            .read(TAIL_SIZE, TAILS_BLOB_TAG_SZ + TAIL_SIZE * idx);
        Tail::from_bytes(bytes.as_slice()).unwrap()
    }
}
