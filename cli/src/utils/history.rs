use linefeed::{Reader, Terminal};

use crate::utils::environment::EnvironmentUtils;
use crate::utils::file::{read_lines_from_file, write_file};


const HISTORY_SIZE: usize = 100;
const SECRET_DATA: [&str; 2] = [" seed=", " key="];

pub fn load<T>(reader: &mut Reader<T>) -> Result<(), String> where T: Terminal {
    reader.set_history_size(HISTORY_SIZE);

    let path = EnvironmentUtils::history_file_path();

    for line in read_lines_from_file(path)? {
        if let Ok(line) = line {
            reader.add_history(line)
        }
    }
    Ok(())
}

pub fn persist<T>(reader: &Reader<T>) -> Result<(), String> where T: Terminal {
    let content =
        reader
            .history()
            .filter(|record|
                !SECRET_DATA.iter().any(|secret_word| record.contains(secret_word))
            )
            .collect::<Vec<&str>>()
            .join("\n");

    let path = EnvironmentUtils::history_file_path();
    write_file(&path, &content)
}
