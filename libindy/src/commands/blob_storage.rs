use std::sync::Arc;

use indy_api_types::errors::prelude::*;

use crate::services::blob_storage::BlobStorageService;

pub(crate) struct BlobStorageCommandExecutor {
    blob_storage_service: Arc<BlobStorageService>,
}

impl BlobStorageCommandExecutor {
    pub(crate) fn new(blob_storage_service: Arc<BlobStorageService>) -> BlobStorageCommandExecutor {
        BlobStorageCommandExecutor {
            blob_storage_service,
        }
    }

    pub(crate) async fn open_reader(&self, type_: String, config: String) -> IndyResult<i32> {
        debug!("open_reader > type_ {:?} config {:?}", type_, config);

        let handle = self
            .blob_storage_service
            .open_reader(&type_, &config)
            .await?;

        let res = Ok(handle);
        debug!("open_reader < {:?}", res);
        res
    }

    pub(crate) async fn open_writer(&self, type_: String, config: String) -> IndyResult<i32> {
        debug!("open_writer > type_ {:?} config {:?}", type_, config);

        let handle = self
            .blob_storage_service
            .open_writer(&type_, &config)
            .await?;

        let res = Ok(handle);
        debug!("open_writer < {:?}", res);
        res
    }
}
