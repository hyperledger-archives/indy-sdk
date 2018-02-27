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
            BlobStorageCommand::OpenReader(type_, config, location, hash, cb) => {
                cb(self.open_reader(&type_, &config, &location, &hash));
            }
        }
    }

    fn open_reader(&self, type_: &str, config: &str, location: &str, hash: &str) -> Result<i32, IndyError> {
        let hash: Vec<u8> = base64::decode(&hash)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode hash from base64 {}", err)))?;

        Ok(self.blob_storage_service.open_reader(type_, config, location, &hash)?)
    }
}
