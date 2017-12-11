extern crate ansi_term;
extern crate unescape;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate linefeed;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate prettytable;

#[macro_use]
mod utils;
mod command_executor;
#[macro_use]
mod commands;
mod libindy;

use command_executor::CommandExecutor;

use commands::{common, did, ledger, pool, wallet};

use linefeed::{Reader, ReadResult};
use linefeed::complete::PathCompleter;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

fn main() {
    utils::logger::LoggerUtils::init();

    let command_executor = build_executor();

    if env::args().len() == 1 {
        execute_stdin(command_executor);
    } else {
        let mut args = env::args();
        args.next(); //skip 0 param
        execute_batch(command_executor, Some(&args.next().unwrap()))
    }
}

fn build_executor() -> CommandExecutor {
    CommandExecutor::build()
        .add_command(common::AboutCommand::new())
        .add_command(common::ExitCommand::new())
        .add_command(common::PromptCommand::new())
        .add_command(common::ShowCommand::new())
        .add_group(did::Group::new())
        .add_command(did::NewCommand::new())
        .add_command(did::UseCommand::new())
        .add_command(did::RotateKeyCommand::new())
        .add_command(did::ListCommand::new())
        .finalize_group()
        .add_group(pool::Group::new())
        .add_command(pool::CreateCommand::new())
        .add_command(pool::ConnectCommand::new())
        .add_command(pool::ListCommand::new())
        .add_command(pool::DisconnectCommand::new())
        .add_command(pool::DeleteCommand::new())
        .finalize_group()
        .add_group(wallet::Group::new())
        .add_command(wallet::CreateCommand::new())
        .add_command(wallet::OpenCommand::new())
        .add_command(wallet::ListCommand::new())
        .add_command(wallet::CloseCommand::new())
        .add_command(wallet::DeleteCommand::new())
        .finalize_group()
        .add_group(ledger::Group::new())
        .add_command(ledger::NymCommand::new())
        .add_command(ledger::GetNymCommand::new())
        .add_command(ledger::AttribCommand::new())
        .add_command(ledger::GetAttribCommand::new())
        .add_command(ledger::SchemaCommand::new())
        .add_command(ledger::GetSchemaCommand::new())
        .add_command(ledger::ClaimDefCommand::new())
        .add_command(ledger::GetClaimDefCommand::new())
        .add_command(ledger::NodeCommand::new())
        .add_command(ledger::CustomCommand::new())
        .finalize_group()
        .finalize()
}

fn execute_stdin(command_executor: CommandExecutor) {
    #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support().is_ok();

    match Reader::new("indy-cli") {
        Ok(reader) => execute_interactive(command_executor, reader),
        Err(_) => execute_batch(command_executor, None),
    }
}

fn execute_interactive<T>(command_executor: CommandExecutor, mut reader: Reader<T>)
    where T: linefeed::Terminal {
    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt(&command_executor.ctx().get_prompt());

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if line.trim().is_empty() {
            continue;
        }

        command_executor.execute(&line).is_ok();
        reader.add_history(line);
        reader.set_prompt(&command_executor.ctx().get_prompt());

        if command_executor.ctx().is_exit() {
            break;
        }
    }
}

fn execute_batch(command_executor: CommandExecutor, script_path: Option<&str>) {
    if let Some(script_path) = script_path {
        let file = match File::open(script_path) {
            Ok(file) => file,
            Err(err) => return println_err!("Can't open script file {}\nError: {}", script_path, err),
        };
        _iter_batch(command_executor, BufReader::new(file));
    } else {
        let stdin = std::io::stdin();
        _iter_batch(command_executor, stdin.lock());
    };
}

fn _iter_batch<T>(command_executor: CommandExecutor, reader: T) where T: std::io::BufRead {
    let mut line_num = 1;
    for line in reader.lines() {
        let line = if let Ok(line) = line { line } else {
            return println_err!("Can't parse line #{}", line_num);
        };
        println!("{}", line);
        let (line, force) = if line.starts_with("-") {
            (line[1..].as_ref(), true)
        } else {
            (line[0..].as_ref(), false)
        };
        if command_executor.execute(line).is_err() && !force {
            return println_err!("Batch execution failed at line #{}", line_num);
        }
        println!();
        line_num += 1;
    }
}
