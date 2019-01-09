use services::blob_storage::BlobStorageService;
use std::rc::Rc;

use errors::prelude::*;

pub enum BlobStorageCommand {
    OpenReader(
        String, // type
        String, // config
        Box<Fn(IndyResult<i32 /* handle */>) + Send>),
    OpenWriter(
        String, // writer type
        String, // writer config JSON
        Box<Fn(IndyResult<i32 /* handle */>) + Send>),
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
                info!("OpenReader command received");
                cb(self.open_reader(&type_, &config));
            }
            BlobStorageCommand::OpenWriter(writer_type, writer_config, cb) => {
                info!("OpenWriter command received");
                cb(self.open_writer(&writer_type, &writer_config));
            }
        }
    }

    fn open_reader(&self, type_: &str, config: &str) -> IndyResult<i32> {
        debug!("open_reader >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_reader(type_, config).map_err(IndyError::from);

        debug!("open_reader << res: {:?}", res);

        res
    }

    fn open_writer(&self, type_: &str, config: &str) -> IndyResult<i32> {
        debug!("open_writer >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_writer(type_, config).map_err(IndyError::from);

        debug!("open_writer << res: {:?}", res);

        res
    }
}
