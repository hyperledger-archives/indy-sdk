use std::env;
use std::fs;
use std::fs::{File};
use std::io;
use std::path::{Path, PathBuf};
use utils::rand;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn new(file_name: Option<&str>) -> io::Result<TempFile> {
        let path = generate_temp_path(file_name, "TempFile_");

        let _file = File::create(&path)?;

        Ok(TempFile {
            path: path
        })
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

pub struct TempDir {
    path: PathBuf
}

impl TempDir {
    pub fn new(dir_name: Option<&str>) -> io::Result<TempDir> {
        let path = generate_temp_path(dir_name, "TempDir_");

        fs::create_dir(&path)?;

        Ok(TempDir {
            path: path
        })
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

fn generate_temp_path(name: Option<&str>, prefix: &str) -> PathBuf {
    let mut path = env::temp_dir();

    if let Some(name) = name {
        path.push(name);
    } else {
        let name = format!("{}{}", prefix, rand::random_string(12));
        path.push(name);
    }

    path
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

    #[test]
    fn temp_dir() {
        let name = "test_dir";
        let mut path = env::temp_dir();
        path.push(name);

        {
            let _dir = TempDir::new(Some(name)).unwrap();
            assert!(path.exists());
        }

        assert!(! path.exists());
    }

}
