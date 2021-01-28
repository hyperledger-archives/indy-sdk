use std::path::PathBuf;

use async_std::{fs, fs::File, prelude::*};
use async_trait::async_trait;
use indy_api_types::errors::prelude::*;
use rust_base58::ToBase58;
use serde_json;

use super::{WritableBlob, Writer, WriterType};

use crate::utils::environment;

#[allow(dead_code)]
pub struct DefaultWriter {
    base_dir: PathBuf,
    uri_pattern: String,
    file: File,
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct DefaultWriterConfig {
    base_dir: String,
    uri_pattern: String,
}

#[async_trait]
impl WriterType for DefaultWriterType {
    async fn open(&self, config: &str) -> IndyResult<Box<dyn Writer>> {
        let config: DefaultWriterConfig = serde_json::from_str(config).to_indy(
            IndyErrorKind::InvalidStructure,
            "Can't deserialize DefaultWriterConfig",
        )?;

        Ok(Box::new(config))
    }
}

#[async_trait]
impl Writer for DefaultWriterConfig {
    async fn create(&self, id: i32) -> IndyResult<Box<dyn WritableBlob>> {
        let path = PathBuf::from(&self.base_dir);

        fs::DirBuilder::new()
            .recursive(true)
            .create(tmp_storage_file(id).parent().unwrap())
            .await?;

        let file = File::create(tmp_storage_file(id))
            .await
            .map_err(map_err_trace!())?;

        Ok(Box::new(DefaultWriter {
            base_dir: path,
            uri_pattern: self.uri_pattern.clone(),
            file,
            id,
        }))
    }
}

#[async_trait]
impl WritableBlob for DefaultWriter {
    async fn append(&mut self, bytes: &[u8]) -> IndyResult<usize> {
        trace!("append >>>");

        let res = self.file.write_all(bytes).await.map_err(map_err_trace!())?;

        let res = bytes.len();
        trace!("append <<< {}", res);
        Ok(res)
    }

    async fn finalize(&mut self, hash: &[u8]) -> IndyResult<String> {
        trace!("finalize >>>");

        self.file.flush().await.map_err(map_err_trace!())?;
        self.file.sync_all().await.map_err(map_err_trace!())?;

        let mut path = self.base_dir.clone();
        path.push(hash.to_base58());

        fs::DirBuilder::new()
            .recursive(true)
            .create(path.parent().unwrap())
            .await
            .map_err(map_err_trace!(format!("path: {:?}", path)))?;

        fs::copy(&tmp_storage_file(self.id), &path).await.map_err(map_err_trace!())?; //FIXME

        fs::remove_file(&tmp_storage_file(self.id)).await.map_err(map_err_trace!())?;

        let res = path.to_str().unwrap().to_owned();

        trace!("finalize <<< {}", res);
        Ok(res)
    }
}

fn tmp_storage_file(id: i32) -> PathBuf {
    environment::tmp_file_path(&format!("def_storage_tmp_{}", id))
}

pub struct DefaultWriterType {}

impl DefaultWriterType {
    pub fn new() -> Self {
        DefaultWriterType {}
    }
}
