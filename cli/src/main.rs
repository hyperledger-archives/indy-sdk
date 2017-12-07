extern crate ansi_term;
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

#[macro_use]
mod utils;
mod application_context;
mod command_executor;
#[macro_use]
mod commands;
mod indy_context;
mod libindy;

use application_context::ApplicationContext;
use command_executor::CommandExecutor;

use commands::{common, did, pool, wallet, ledger};
use indy_context::IndyContext;

use linefeed::{Reader, ReadResult};
use linefeed::complete::PathCompleter;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

fn main() {
    utils::logger::LoggerUtils::init();

    let application_context = Rc::new(ApplicationContext::new());
    let indy_context = Rc::new(IndyContext::new());

    let command_executor = build_executor(application_context.clone(), indy_context);

    if env::args().len() == 1 {
        execute_interactive(command_executor, application_context);
    } else {
        let mut args = env::args();
        args.next(); //skip 0 param
        execute_batch(command_executor, &args.next().unwrap())
    }
}

fn build_executor(application_context: Rc<ApplicationContext>,
                  indy_context: Rc<IndyContext>) -> CommandExecutor {
    CommandExecutor::build()
        .add_command(Box::new(common::AboutCommand::new()))
        .add_command(Box::new(common::ExitCommand::new(application_context.clone())))
        .add_command(Box::new(common::PromptCommand::new(application_context.clone())))
        .add_command(Box::new(common::ShowCommand::new()))
        .add_group(Box::new(did::Group::new()))
        .add_command(Box::new(did::NewCommand::new(indy_context.clone())))
        .add_command(Box::new(did::UseCommand::new(indy_context.clone())))
        .add_command(Box::new(did::RotateKeyCommand::new(indy_context.clone())))
        .add_command(Box::new(did::ListCommand::new(indy_context.clone())))
        .finalize_group()
        .add_group(Box::new(pool::Group::new()))
        .add_command(Box::new(pool::CreateCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::ConnectCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::ListCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::DisconnectCommand::new(indy_context.clone())))
        .add_command(Box::new(pool::DeleteCommand::new(indy_context.clone())))
        .finalize_group()
        .add_group(Box::new(wallet::Group::new()))
        .add_command(Box::new(wallet::CreateCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::OpenCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::ListCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::CloseCommand::new(indy_context.clone())))
        .add_command(Box::new(wallet::DeleteCommand::new(indy_context.clone())))
        .finalize_group()
        .add_group(Box::new(ledger::Group::new()))
        .add_command(Box::new(ledger::SendNymCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::GetNymCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::SendAttribCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::GetAttribCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::SendSchemaCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::GetSchemaCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::SendClaimDefCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::GetClaimDefCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::SendNodeCommand::new(indy_context.clone())))
        .add_command(Box::new(ledger::SenCustomCommand::new(indy_context.clone())))
        .finalize_group()
        .finalize()
}

fn execute_interactive(command_executor: CommandExecutor,
                       application_context: Rc<ApplicationContext>) {
    #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support().is_ok();

    let mut reader = Reader::new("indy-cli").unwrap();
    reader.set_completer(Rc::new(PathCompleter));
    reader.set_prompt(&application_context.get_prompt());

    while let Ok(ReadResult::Input(line)) = reader.read_line() {
        if line.trim().is_empty() {
            continue;
        }

        command_executor.execute(&line).is_ok();
        reader.add_history(line);
        reader.set_prompt(&application_context.get_prompt());

        if application_context.is_exit() {
            break;
        }
    }
}

fn execute_batch(command_executor: CommandExecutor, script_path: &str) {
    let file = match File::open(script_path) {
        Ok(file) => file,
        Err(err) => return println_err!("Can't open script file {}\nError: {}", script_path, err),
    };
    let reader = BufReader::new(file);

    let mut line_nym = 1;
    for line in reader.lines() {
        let line = if let Ok(line) = line { line } else {
            return println_err!("Can't parse line #{}", line_nym);
        };
        println!("{}", line);
        let (line, force) = if line.starts_with("-") {
            (line[1..].as_ref(), true)
        } else {
            (line[0..].as_ref(), false)
        };
        if command_executor.execute(line).is_err() && !force {
            return println_err!("Batch execution failed at line #{}", line_nym);
        }
        println!();
        line_nym += 1;
    }
}
