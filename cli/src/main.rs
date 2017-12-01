#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linefeed;

pub mod commands;
pub mod libindy;
pub mod utils;

use commands::CommandExecutor;
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
        console_mod_start(command_executor);
    } else {
        unimplemented!("Batch mod");
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

fn console_mod_start(command_executor: CommandExecutor) {
    let mut reader = Reader::new("indy-cli").unwrap();
    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt("indy> ");

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if !line.trim().is_empty() {
            command_executor.execute(line.trim());
            reader.add_history(line);
        }
    }

    println!("\nGoodbye.");
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
}
