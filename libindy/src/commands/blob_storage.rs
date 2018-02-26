extern crate libc;
extern crate serde_json;

use errors::indy::IndyError;
use services::blob_storage::BlobStorageService;

use std::rc::Rc;

pub enum BlobStorageCommand {
    OpenReader(
        String, // reader type
        String, // reader config JSON
        String, // blob location
        Vec<u8>, // blob hash
        Box<Fn(Result<i32, IndyError>) + Send>),
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
            BlobStorageCommand::OpenReader(config, type_, location, hash, cb) => {
                cb(self.open_reader(&config, &type_, &location, hash.as_slice()));
            }
        }
    }

    fn open_reader(&self, type_: &str, config: &str, location: &str, hash: &[u8]) -> Result<i32, IndyError> {
        Ok(self.blob_storage_service.open_reader(type_, config, location, hash)?)
    }
}
