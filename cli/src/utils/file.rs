use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;
use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder, OpenOptions};

pub fn read_file<P: AsRef<Path>>(file: P) -> Result<String, String> {
    let mut file = File::open(file)
        .map_err(error_err!())
        .map_err(|_| "Can't read the file".to_string())?;

    let content = {
        let mut s = String::new();
        file.read_to_string(&mut s)
            .map_err(error_err!())
            .map_err(|err| format!("Can't read the file: {}", err))?;
        s
    };

    Ok(content)
}

pub fn read_lines_from_file<P: AsRef<Path>>(file: P) -> Result<impl Iterator<Item=Result<String, ::std::io::Error>>, String> {
    let file = File::open(file)
        .map_err(error_err!())
        .map_err(|_| "Can't read the file".to_string())?;

    let lines = BufReader::new(file).lines();
    Ok(lines)
}

pub fn write_file<P: AsRef<Path>>(file: P, content: &str) -> Result<(), String> where P: std::convert::AsRef<std::ffi::OsStr> {
    let path = PathBuf::from(&file);

    if let Some(parent_path) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(error_err!())
            .map_err(|err| format!("Can't create the file: {}", err))?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(error_err!())
        .map_err(|err| format!("Can't open the file: {}", err))?;

    file.write_all(content.as_bytes())
        .map_err(|err| format!("Can't write content: \"{}\" to the file: {}", content, err))?;

    file.flush()
        .map_err(|err| format!("Can't write content: \"{}\" to the file: {}", content, err))
}