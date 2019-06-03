use std::io::{Read, Write};
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

pub fn write_file(file: &str, content: &str) -> Result<(), String> {
    let path = PathBuf::from(&file);

    if let Some(parent_path) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(error_err!())
            .map_err(|err| format!("Can't create the file: {}", err))?;
    }

    let mut file =
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file)
            .map_err(error_err!())
            .map_err(|err| format!("Can't open the file: {}", err))?;

    file
        .write_all(content.as_bytes())
        .map_err(error_err!())
        .map_err(|err| format!("Can't write content: \"{}\" to the file: {}", content, err))
}