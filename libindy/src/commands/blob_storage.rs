extern crate libc;
extern crate serde_json;
extern crate base64;

use errors::indy::IndyError;
use services::blob_storage::BlobStorageService;

use std::rc::Rc;

pub enum BlobStorageCommand {
    OpenReader(
        String, // type
        String, // config
        Box<Fn(Result<i32 /* handle */, IndyError>) + Send>),
    OpenWriter(
        String, // writer type
        String, // writer config JSON
        Box<Fn(Result<i32 /* handle */, IndyError>) + Send>),
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
            BlobStorageCommand::OpenReader(type_, config, cb) => {
                cb(self.open_reader(&type_, &config));
            }
            BlobStorageCommand::OpenWriter(writer_type, writer_config, cb) => {
                cb(self.open_writer(&writer_type, &writer_config));
            }
        }
    }

    fn open_reader(&self, type_: &str, config: &str) -> Result<i32, IndyError> {
        trace!("open_reader >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_reader(type_, config).map_err(IndyError::from);

        trace!("open_reader << res: {:?}", res);

        res
    }

    fn open_writer(&self, type_: &str, config: &str) -> Result<i32, IndyError> {
        trace!("open_writer >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_writer(type_, config).map_err(IndyError::from);

        trace!("open_writer << res: {:?}", res);

        res
    }
}
