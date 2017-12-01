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

use std::rc::Rc;

fn main() {    

    let mut reader = Reader::new("indy-cli").unwrap();
    //let mut command_executor = CommandExecutor::new();

    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt("indy> ");

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        println!("read input: {:?}", line);

        if !line.trim().is_empty() {
            //command_executor.execute(line.trim());
            reader.add_history(line);
        }
    }

    println!("Goodbye.");
}