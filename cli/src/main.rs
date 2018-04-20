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

use linefeed::{Reader, ReadResult, Terminal};
use linefeed::complete::{Completer, Completion};

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

fn main() {
    utils::logger::LoggerUtils::init();

    if env::args().find(|a| a == "-h" || a == "--help").is_some() {
        return _print_help();
    }

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
        .add_command(common::about_command::new())
        .add_command(common::exit_command::new())
        .add_command(common::prompt_command::new())
        .add_command(common::show_command::new())
        .add_group(did::group::new())
        .add_command(did::new_command::new())
        .add_command(did::import_command::new())
        .add_command(did::use_command::new())
        .add_command(did::rotate_key_command::new())
        .add_command(did::list_command::new())
        .finalize_group()
        .add_group(pool::group::new())
        .add_command(pool::create_command::new())
        .add_command(pool::connect_command::new())
        .add_command(pool::list_command::new())
        .add_command(pool::disconnect_command::new())
        .add_command(pool::delete_command::new())
        .finalize_group()
        .add_group(wallet::group::new())
        .add_command(wallet::create_command::new())
        .add_command(wallet::open_command::new())
        .add_command(wallet::list_command::new())
        .add_command(wallet::close_command::new())
        .add_command(wallet::delete_command::new())
        .finalize_group()
        .add_group(ledger::group::new())
        .add_command(ledger::nym_command::new())
        .add_command(ledger::get_nym_command::new())
        .add_command(ledger::attrib_command::new())
        .add_command(ledger::get_attrib_command::new())
        .add_command(ledger::schema_command::new())
        .add_command(ledger::get_schema_command::new())
        .add_command(ledger::cred_def_command::new())
        .add_command(ledger::get_cred_def_command::new())
        .add_command(ledger::node_command::new())
        .add_command(ledger::pool_config_command::new())
        .add_command(ledger::pool_restart_command::new())
        .add_command(ledger::pool_upgrade_command::new())
        .add_command(ledger::custom_command::new())
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
    where T: Terminal {
    let command_executor = Rc::new(command_executor);
    reader.set_completer(command_executor.clone());
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

fn _print_help() {
    println_acc!("Hyperledger Indy CLI");
    println!();
    println_acc!("CLI supports 2 execution modes:");
    println_acc!("\tInteractive - reads commands from terminal. To start just run indy-cli without params.");
    println_acc!("\tUsage: indy-cli");
    println!();
    println_acc!("\tBatch - all commands will be read from text file or pipe and executed in series.");
    println_acc!("\tUsage: indy-cli <path-to-text-file>");
    println!();
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

impl<Term: Terminal> Completer<Term> for CommandExecutor {
    fn complete(&self, word: &str, reader: &Reader<Term>,
                start: usize, end: usize) -> Option<Vec<Completion>> {
        Some(self
            .complete(reader.buffer(),
                      word,
                      start,
                      end)
            .into_iter()
            .map(|c| Completion {
                completion: c.0,
                display: None,
                suffix: linefeed::Suffix::Some(c.1),
            })
            .collect())
    }
}
