extern crate indy_crypto;
extern crate rust_base58;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;

use self::rust_base58::ToBase58;

use super::{WritableBlob, Writer, WriterType};
use errors::common::CommonError;
use utils::environment::EnvironmentUtils;

use self::indy_crypto::utils::json::JsonDecodable;

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

impl<'a> JsonDecodable<'a> for DefaultWriterConfig {}

impl WriterType for DefaultWriterType {
    fn open(&self, config: &str) -> Result<Box<Writer>, CommonError> {
        let config: DefaultWriterConfig = DefaultWriterConfig::from_json(config)
            .map_err(map_err_trace!())?;
        Ok(Box::new(config))
    }
}

impl Writer for DefaultWriterConfig {
    fn create(&self, id: i32) -> Result<Box<WritableBlob>, CommonError> {
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
    fn append(&mut self, bytes: &[u8]) -> Result<usize, CommonError> {
        trace!("append >>>");

        let res = self.file.write(bytes)
            .map_err(map_err_trace!())?;

        trace!("append <<< {}", res);
        Ok(res)
    }

    fn finalize(&mut self, hash: &[u8]) -> Result<String, CommonError> {
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
    EnvironmentUtils::tmp_file_path(&format!("def_storage_tmp_{}", id))
}

pub struct DefaultWriterType {}

impl DefaultWriterType {
    pub fn new() -> Self {
        DefaultWriterType {}
    }
}
