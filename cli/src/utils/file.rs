use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;
use std::path::PathBuf;
use std::fs::{File, DirBuilder, OpenOptions};

pub fn read_file(file: &str) -> Result<String, String> {
    let mut file = File::open(file)
        .map_err(error_err!())
        .map_err(|_| format!("Can't read the file"))?;

    let content = {
        let mut s = String::new();
        file.read_to_string(&mut s)
            .map_err(error_err!())
            .map_err(|err| format!("Can't read the file: {}", err))?;
        s
    };

    Ok(content)
}

pub fn read_lines_from_file(file: PathBuf) -> Result<impl Iterator<Item=Result<String, ::std::io::Error>>, String> {
    let file = File::open(file).map_err(error_err!()).map_err(|_| format!("Can't read the file"))?;
    let lines = BufReader::new(file).lines();
    Ok(lines)
}

pub fn create_path(file: &PathBuf) -> Result<(), String> {
    let path = PathBuf::from(&file);

    if let Some(parent_path) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(error_err!())
            .map_err(|err| format!("Can't create the file: {}", err))?;
    }

    Ok(())
}

pub fn open_file_for_writing(path: &PathBuf)  -> Result<File, String> {
    create_path(&path)?;

    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .map_err(error_err!())
        .map_err(|err| format!("Can't open the file: {}", err))
}

pub fn write_file(file: &str, content: &str) -> Result<(), String> {
    let path = PathBuf::from(&file);

    let mut file = open_file_for_writing(&path)?;

    file
        .write_all(content.as_bytes())
        .map_err(error_err!())
        .map_err(|err| format!("Can't write content: \"{}\" to the file: {}", content, err))
}

pub fn write_lines_to_file<'a, I>(file: PathBuf, content: I) -> Result<(), String> where I: IntoIterator<Item=&'a str> {
    let mut file = open_file_for_writing(&file)?;

    for line in content {
        writeln!(file, "{}", line)
            .map_err(error_err!())
            .map_err(|err| format!("Can't write line: \"{}\" to the file: {}", line, err))
            .ok();
    }

    Ok(())
}