use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde_json;

use errors::prelude::*;
use utils::environment;

use super::{WritableBlob, Writer, WriterType};

use rust_base58::ToBase58;

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

impl WriterType for DefaultWriterType {
    fn open(&self, config: &str) -> IndyResult<Box<Writer>> {
        let config: DefaultWriterConfig = serde_json::from_str(config)
            .to_indy(IndyErrorKind::InvalidStructure, "Can't deserialize DefaultWriterConfig")?;

        Ok(Box::new(config))
    }
}

impl Writer for DefaultWriterConfig {
    fn create(&self, id: i32) -> IndyResult<Box<WritableBlob>> {
        let path = PathBuf::from(&self.base_dir);

        fs::DirBuilder::new()
            .recursive(true)
            .create(tmp_storage_file(id).parent().unwrap())?;

        let file = File::create(tmp_storage_file(id))
            .map_err(map_err_trace!())?;

        Ok(Box::new(DefaultWriter {
            base_dir: path,
            uri_pattern: self.uri_pattern.clone(),
            file,
            id,
        }))
    }
}

impl WritableBlob for DefaultWriter {
    fn append(&mut self, bytes: &[u8]) -> IndyResult<usize> {
        trace!("append >>>");

        let res = self.file.write(bytes)
            .map_err(map_err_trace!())?;

        trace!("append <<< {}", res);
        Ok(res)
    }

    fn finalize(&mut self, hash: &[u8]) -> IndyResult<String> {
        trace!("finalize >>>");

        self.file.flush().map_err(map_err_trace!())?;
        self.file.sync_all().map_err(map_err_trace!())?;

        let mut path = self.base_dir.clone();
        path.push(hash.to_base58());

        fs::DirBuilder::new()
            .recursive(true)
            .create(path.parent().unwrap())
            .map_err(map_err_trace!(format!("path: {:?}", path)))?;

        fs::copy(&tmp_storage_file(self.id), &path)
            .map_err(map_err_trace!())?; //FIXME

        fs::remove_file(&tmp_storage_file(self.id))
            .map_err(map_err_trace!())?;

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
