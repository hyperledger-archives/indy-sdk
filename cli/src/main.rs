#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linefeed;

pub mod commands;
pub mod libindy;
pub mod utils;

//use commands::CommandExecutor;

use linefeed::{Reader, ReadResult};
use linefeed::complete::PathCompleter;

fn main() {

    let mut reader = Reader::new("path-completion-demo").unwrap();
    //let mut command_executor = CommandExecutor::new();

    //reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt("path> ");
    //reader.set_completer()

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        println!("read input: {:?}", line);

        if !line.trim().is_empty() {
            //command_executor.execute(line.trim());
            reader.add_history(line);
        }
    }

    println!("Goodbye.");
}