extern crate libc;
extern crate serde_json;
extern crate base64;

use errors::indy::IndyError;
use errors::common::CommonError;
use services::blob_storage::BlobStorageService;

use std::rc::Rc;

pub enum BlobStorageCommand {
    OpenReader(
        String, // reader type
        String, // reader config JSON
        String, // blob location
        String, // blob hash
        Box<Fn(Result<i32 /* handle */, IndyError>) + Send>),
    Read(
        i32, // reader handle
        u64, // size
        u64, // offset
        Box<Fn(Result<Vec<u8> /* data */, IndyError>) + Send>),
    CloseReader(
        i32, // reader handle
        Box<Fn(Result<(), IndyError>) + Send>),
    CreateWriter(
        String, // writer type
        String, // writer config JSON
        Box<Fn(Result<i32 /* handle */, IndyError>) + Send>),
    AppendToWriter(
        i32, // writer handle
        Vec<u8>, // data to append
        Box<Fn(Result<usize /* written bytes */, IndyError>) + Send>),
    FinaliseWriter(
        i32, // writer handle
        Box<Fn(Result<(String /* location */, Vec<u8> /* hash */), IndyError>) + Send>),
}

pub struct BlobStorageCommandExecutor {
    blob_storage_service: Rc<BlobStorageService>
}

impl BlobStorageCommandExecutor {
    pub fn new(blob_storage_service: Rc<BlobStorageService>) -> BlobStorageCommandExecutor {
        BlobStorageCommandExecutor {
            blob_storage_service
        }
    }

    pub fn execute(&self, command: BlobStorageCommand) {
        match command {
            BlobStorageCommand::OpenReader(type_, config, location, hash, cb) => {
                cb(self.open_reader(&type_, &config, &location, &hash));
            }
            BlobStorageCommand::Read(reader_handle, size, offset, cb) => {
                cb(self.read(reader_handle, size, offset));
            }
            BlobStorageCommand::CloseReader(reader_handle, cb) => {
                cb(self.close_reader(reader_handle));
            }
            BlobStorageCommand::CreateWriter(writer_type, writer_config, cb) => {
                cb(self.create_writer(&writer_type, &writer_config));
            }
            BlobStorageCommand::AppendToWriter(writer_handle, data, cb) => {
                cb(self.append_to_writer(writer_handle, data.as_slice()));
            }
            BlobStorageCommand::FinaliseWriter(writer_handle, cb) => {
                cb(self.finalise_writer(writer_handle));
            }
        }
    }

    fn open_reader(&self, type_: &str, config: &str, location: &str, hash: &str) -> Result<i32, IndyError> {
     trace!("open_reader >>> type_: {:?}, config: {:?}, location: {:?}, hash: {:?}", type_, config, location, hash);

        let hash: Vec<u8> = base64::decode(&hash)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode hash from base64 {}", err)))?;

        let res = self.blob_storage_service.open_reader(type_, config, location, &hash).map_err(IndyError::from);

        trace!("open_reader << res: {:?}", res);

        res

    }

    fn read(&self, handle: i32, size: u64, offset: u64) -> Result<Vec<u8>, IndyError> {
        self.blob_storage_service.read(handle, size as usize, offset as usize).map_err(IndyError::from) //FIXME resolve cast
    }

    fn close_reader(&self, handle: i32) -> Result<(), IndyError> {
        self.blob_storage_service.close(handle).map_err(IndyError::from)
    }

    fn create_writer(&self, type_: &str, config: &str) -> Result<i32, IndyError> {
        self.blob_storage_service.create_writer(type_, config).map_err(IndyError::from)
    }

    fn append_to_writer(&self, handle: i32, data: &[u8]) -> Result<usize, IndyError> {
        self.blob_storage_service.append(handle, data).map_err(IndyError::from)
    }

    fn finalise_writer(&self, handle: i32) -> Result<(String, Vec<u8>), IndyError> {
        self.blob_storage_service.finalize(handle).map_err(IndyError::from)
    }
}
