extern crate ansi_term;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linefeed;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;
mod command_executor;
mod commands;
mod indy_context;
mod libindy;

use command_executor::CommandExecutor;
use commands::wallet;
use indy_context::IndyContext;

use linefeed::{Reader, ReadResult};
use linefeed::complete::PathCompleter;

use std::env;
use std::rc::Rc;

fn main() {
    //FIXME move from SDK utils::logger::LoggerUtils::init();

    let command_executor = build_executor();

    if env::args().len() == 1 {
        execute_interactive(command_executor);
    } else {
        execute_batch(command_executor, &env::args().next().unwrap())
    }
}

fn build_executor() -> CommandExecutor {
    let indy_context = Rc::new(IndyContext::new());

    CommandExecutor::build()
        .add_group(Box::new(wallet::Group::new()))
        .add_command(Box::new(wallet::CreateCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::OpenCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::CloseCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::DeleteCommand::new(indy_context.clone())))
        .finalize_group()
        .finalize()
}

fn execute_interactive(command_executor: CommandExecutor) {
    let mut reader = Reader::new("indy-cli").unwrap();
    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt("indy> ");

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if command_executor.execute(&line).is_ok() {
            reader.add_history(line);
        }
    }

    println!("\nGoodbye.");
}

fn execute_batch(_command_executor: CommandExecutor, _script_path: &str) {
    unimplemented!()
}
