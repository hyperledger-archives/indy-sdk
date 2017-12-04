#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linefeed;
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod command_executor;
pub mod commands;
pub mod libindy;
pub mod utils;

use command_executor::CommandExecutor;
use commands::wallet;
use libindy::IndyHandle;

use linefeed::{Reader, ReadResult};
use linefeed::complete::PathCompleter;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[derive(Debug)]
pub struct IndyContext {
    cur_wallet: RefCell<Option<(String, IndyHandle)>>,
}

fn main() {
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
        .add_command(Box::new(wallet::OpenCommand::new(indy_context)))
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

impl IndyContext {
    pub fn new() -> IndyContext {
        IndyContext {
            cur_wallet: RefCell::new(None),
        }
    }

    pub fn set_current_wallet(&self, wallet_name: &str, wallet_handle: IndyHandle) {
        *self.cur_wallet.borrow_mut() = Some((wallet_name.to_string(), wallet_handle));
    }

    pub fn reset_current_wallet(&self) {
        *self.cur_wallet.borrow_mut() = None;
    }

    pub fn get_current_wallet_handle(&self) -> Option<IndyHandle> {
        self.cur_wallet.borrow().as_ref().map(|&(_, handle)| handle)
    }
}
