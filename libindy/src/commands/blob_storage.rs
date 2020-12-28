use crate::services::blob_storage::BlobStorageService;
use std::sync::Arc;

use indy_api_types::errors::prelude::*;

pub enum BlobStorageCommand {
    OpenReader(
        String, // type
        String, // config
        Box<dyn Fn(IndyResult<i32 /* handle */>) + Send + Sync>),
    OpenWriter(
        String, // writer type
        String, // writer config JSON
        Box<dyn Fn(IndyResult<i32 /* handle */>) + Send + Sync>),
}

pub struct BlobStorageCommandExecutor {
    blob_storage_service: Arc<BlobStorageService>
}

impl BlobStorageCommandExecutor {
    pub fn new(blob_storage_service:Arc<BlobStorageService>) -> BlobStorageCommandExecutor {
        BlobStorageCommandExecutor {
            blob_storage_service
        }
    }

    pub async fn execute(&self, command: BlobStorageCommand) {
        match command {
            BlobStorageCommand::OpenReader(type_, config, cb) => {
                debug!("OpenReader command received");
                cb(self.open_reader(&type_, &config).await);
            }
            BlobStorageCommand::OpenWriter(writer_type, writer_config, cb) => {
                debug!("OpenWriter command received");
                cb(self.open_writer(&writer_type, &writer_config).await);
            }
        }
    }

    async fn open_reader(&self, type_: &str, config: &str) -> IndyResult<i32> {
        debug!("open_reader >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_reader(type_, config).await.map_err(IndyError::from);

        debug!("open_reader << res: {:?}", res);

        res
    }

    async fn open_writer(&self, type_: &str, config: &str) -> IndyResult<i32> {
        debug!("open_writer >>> type_: {:?}, config: {:?}", type_, config);

        let res = self.blob_storage_service.open_writer(type_, config).await.map_err(IndyError::from);

        debug!("open_writer << res: {:?}", res);

        res
    }
}
