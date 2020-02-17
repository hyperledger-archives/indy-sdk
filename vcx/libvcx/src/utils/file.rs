use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder, OpenOptions};
use error::prelude::*;

pub fn read_file<P: AsRef<Path>>(file: P) -> VcxResult<String> {
    let mut file = File::open(file)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidConfiguration, format!("Cannot read file: {:?}", err)))?;

    let content = {
        let mut s = String::new();
        file.read_to_string(&mut s)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidConfiguration, format!("Cannot read file: {:?}", err)))?;
        s
    };

    Ok(content)
}

pub fn write_file<P: AsRef<Path>>(file: P, content: &str) -> VcxResult<()> where P: std::convert::AsRef<std::ffi::OsStr> {
    let path = PathBuf::from(&file);

    if let Some(parent_path) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::UnknownError, format!("Can't create the file: {}", err)))?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::UnknownError, format!("Can't open the file: {}", err)))?;

    file
        .write_all(content.as_bytes())
        .map_err(|err| VcxError::from_msg(VcxErrorKind::UnknownError, format!("Can't write content: \"{}\" to the file: {}", content, err)))?;

    file.flush()
        .map_err(|err| VcxError::from_msg(VcxErrorKind::UnknownError, format!("Can't write content: \"{}\" to the file: {}", content, err)))?;

    file.sync_data()
        .map_err(|err| VcxError::from_msg(VcxErrorKind::UnknownError, format!("Can't write content: \"{}\" to the file: {}", content, err)))
}