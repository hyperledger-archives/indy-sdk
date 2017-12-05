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
mod application_context;
mod command_executor;
mod commands;
mod indy_context;
mod libindy;

use command_executor::CommandExecutor;
use commands::{common, pool, wallet};
use indy_context::IndyContext;
use libindy::IndyHandle;

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
        .add_command(Box::new(common::AboutCommand::new()))
        .add_command(Box::new(common::ShowCommand::new()))
        .add_group(Box::new(wallet::Group::new()))
        .add_command(Box::new(wallet::CreateCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::OpenCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::ListCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::CloseCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::DeleteCommand::new(indy_context.clone())))
        .finalize_group()
        .add_group(Box::new(pool::Group::new()))
        .add_command(Box::new(pool::CreateCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::ConnectCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::ListCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::DisconnectCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::DeleteCommand::new(indy_context.clone())))
        .finalize_group()
        .finalize()
}

fn execute_interactive(command_executor: CommandExecutor) {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().is_ok();

    let mut reader = Reader::new("indy-cli").unwrap();
    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt("indy> ");

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if !line.trim().is_empty() {
            command_executor.execute(&line).is_ok();
            reader.add_history(line);
        }
    }

    println!("\nGoodbye.");
}

fn execute_batch(_command_executor: CommandExecutor, _script_path: &str) {
    unimplemented!()
}
