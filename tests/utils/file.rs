use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use utils::rand;


pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn new(file_name: Option<&str>) -> io::Result<TempFile> {
        let mut path = env::temp_dir();

        if let Some(name) = file_name {
            path.push(name);
        } else {
            let name = format!("TempFile{}", rand::random_string(12));
            path.push(name);
        }

        let _file = File::create(&path)?;

        let tempfile = TempFile {
            path: path
        };

        Ok(tempfile)
    }
}

impl AsRef<Path> for TempFile {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}


#[cfg(test)]
mod test_temp_file {
    use super::*;

    #[test]
    fn temp_file() {
        let name = "test_file.txt";
        let mut path = env::temp_dir();
        path.push(name);

        {
            let _file = TempFile::new(Some(name)).unwrap();
            assert!(path.exists());
        }

        assert!(! path.exists());
    }

    #[test]
    fn write_read_temp_file() {
        const CONTENT: &str = "Writing to a temp file";

        let file: TempFile = TempFile::new(None).unwrap();

        fs::write(&file, CONTENT).unwrap();

        let contents = fs::read(&file).unwrap();

        assert_eq!(CONTENT.as_bytes(), contents.as_slice());
    }
}
