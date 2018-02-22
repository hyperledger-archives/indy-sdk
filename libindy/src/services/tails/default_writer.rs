extern crate indy_crypto;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;

use base64;

use super::{TailsWriter, TailsWriterType};
use utils::environment::EnvironmentUtils;

use self::indy_crypto::utils::json::*;

pub struct DefaultTailsWriter {
    base_dir: PathBuf,
    uri_pattern: String,
    file: File,
}

#[derive(Serialize, Deserialize)]
struct DefaultTailsWriterConfig {
    base_dir: String,
    uri_pattern: String,
}

impl<'a> JsonDecodable<'a> for DefaultTailsWriterConfig {}

impl TailsWriterType for DefaultTailsWriterType {
    fn create(&self, config: &str) -> Box<TailsWriter> {
        let config: DefaultTailsWriterConfig = DefaultTailsWriterConfig::from_json(config).unwrap();
        let path = PathBuf::from(config.base_dir);
        let file = File::create(EnvironmentUtils::tmp_file_path("tails_tmp")).unwrap(); //TODO unique

        Box::new(DefaultTailsWriter {
            base_dir: path,
            uri_pattern: config.uri_pattern,
            file,
        })
    }
}

impl TailsWriter for DefaultTailsWriter {
    fn append(&mut self, bytes: &[u8]) -> () {
        self.file.write(bytes).unwrap();
    }

    fn finalize(&mut self, hash: &[u8]) -> String {
        self.file.flush().unwrap();
        let mut path = self.base_dir.clone();
        path.push(base64::encode(hash));

        fs::rename(EnvironmentUtils::tmp_file_path("tails_tmp"), &path).unwrap();

        path.to_str().unwrap().to_owned()
    }
}

pub struct DefaultTailsWriterType {}

impl DefaultTailsWriterType {
    pub fn new() -> Self {
        DefaultTailsWriterType {}
    }
}
