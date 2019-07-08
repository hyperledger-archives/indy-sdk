use linefeed::{Reader, Terminal};

use utils::environment::EnvironmentUtils;
use utils::file::{read_lines_from_file, write_lines_to_file};


const HISTORY_SIZE: usize = 100;

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
    let content = reader.history().collect::<std::collections::HashSet<&str>>();
    let path = EnvironmentUtils::history_file_path();
    write_lines_to_file(path, content)
}