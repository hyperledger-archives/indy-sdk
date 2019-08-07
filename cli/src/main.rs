#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate atty;
extern crate ansi_term;
extern crate unescape;
#[cfg(test)]
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
extern crate log4rs;
extern crate indyrs as indy;

#[macro_use]
mod utils;
mod command_executor;
#[macro_use]
mod commands;
mod libindy;

use command_executor::CommandExecutor;

use commands::{common, did, ledger, pool, wallet, payment_address};
use utils::history;

use linefeed::{Reader, ReadResult, Terminal, Signal};
use linefeed::complete::{Completer, Completion};

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

fn main() {
    #[cfg(target_os = "windows")]
    let _ = ansi_term::enable_ansi_support().is_ok();

    let mut args = env::args();
    args.next(); // skip library

    let command_executor = build_executor();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return _print_help(),
            "--config" => {
                let file = unwrap_or_return!(args.next(), println_err!("CLI configuration file is not specified"));

                match CliConfig::read_from_file(&file)
                    .and_then(|config| config.handle(&command_executor)) {
                    Ok(()) => {}
                    Err(err) => return println_err!("{}", err)
                }
            }
            "--logger-config" => {
                let file = unwrap_or_return!(args.next(), println_err!("Logger config file is not specified"));
                match utils::logger::IndyCliLogger::init(&file) {
                    Ok(()) => println_succ!("Logger has been initialized according to the config file: \"{}\"", file),
                    Err(err) => return println_err!("{}", err)
                }
            }
            "--plugins" => {
                let plugins = unwrap_or_return!(args.next(), println_err!("Plugins are not specified"));
                _load_plugins(&command_executor, &plugins)
            }
            _ if args.len() == 0 => {
                execute_batch(&command_executor, Some(&arg));

                if command_executor.ctx().is_exit() {
                    return;
                }
            },
            _ => {
                println_err!("Unknown option");
                return _print_help();
            }
        }
    }
    execute_stdin(command_executor);
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CliConfig {
    pub logger_config: Option<String>,
    pub plugins: Option<String>,
    pub taa_acceptance_mechanism: Option<String>,
}

impl CliConfig {
    fn read_from_file(file: &str) -> Result<CliConfig, String> {
        let content = utils::file::read_file(file)?;
        let config: CliConfig = serde_json::from_str(&content)
            .map_err(|err| format!("Invalid CLI configuration file: {:?}", err))?;
        Ok(config)
    }

    fn handle(&self, command_executor: &CommandExecutor) -> Result<(), String> {
        if let Some(ref logger_config) = self.logger_config {
            utils::logger::IndyCliLogger::init(logger_config)?;
            println_succ!("Logger has been initialized according to the config file: \"{}\"", logger_config);
        }
        if let Some(ref plugins) = self.plugins {
            _load_plugins(&command_executor, plugins);
        }
        if let Some(ref taa_acceptance_mechanism) = self.taa_acceptance_mechanism {
            command_executor.ctx().set_taa_acceptance_mechanism(taa_acceptance_mechanism);
            println_succ!("\"{}\" is used as transaction author agreement acceptance mechanism", taa_acceptance_mechanism);
        }
        Ok(())
    }
}

fn build_executor() -> CommandExecutor {
    CommandExecutor::build()
        .add_command(common::about_command::new())
        .add_command(common::exit_command::new())
        .add_command(common::prompt_command::new())
        .add_command(common::show_command::new())
        .add_command(common::load_plugin_command::new())
        .add_command(common::init_logger_command::new())
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
        .add_command(pool::refresh_command::new())
        .add_command(pool::list_command::new())
        .add_command(pool::disconnect_command::new())
        .add_command(pool::delete_command::new())
        .add_command(pool::show_taa_command::new())
        .finalize_group()
        .add_group(wallet::group::new())
        .add_command(wallet::create_command::new())
        .add_command(wallet::attach_command::new())
        .add_command(wallet::open_command::new())
        .add_command(wallet::list_command::new())
        .add_command(wallet::close_command::new())
        .add_command(wallet::delete_command::new())
        .add_command(wallet::detach_command::new())
        .add_command(wallet::export_command::new())
        .add_command(wallet::import_command::new())
        .finalize_group()
        .add_group(ledger::group::new())
        .add_command(ledger::nym_command::new())
        .add_command(ledger::get_nym_command::new())
        .add_command(ledger::attrib_command::new())
        .add_command(ledger::get_attrib_command::new())
        .add_command(ledger::schema_command::new())
        .add_command(ledger::get_schema_command::new())
        .add_command(ledger::get_validator_info_command::new())
        .add_command(ledger::cred_def_command::new())
        .add_command(ledger::get_cred_def_command::new())
        .add_command(ledger::node_command::new())
        .add_command(ledger::pool_config_command::new())
        .add_command(ledger::pool_restart_command::new())
        .add_command(ledger::pool_upgrade_command::new())
        .add_command(ledger::custom_command::new())
        .add_command(ledger::get_payment_sources_command::new())
        .add_command(ledger::payment_command::new())
        .add_command(ledger::get_fees_command::new())
        .add_command(ledger::mint_prepare_command::new())
        .add_command(ledger::set_fees_prepare_command::new())
        .add_command(ledger::verify_payment_receipt_command::new())
        .add_command(ledger::sign_multi_command::new())
        .add_command(ledger::auth_rule_command::new())
        .add_command(ledger::auth_rules_command::new())
        .add_command(ledger::get_auth_rule_command::new())
        .add_command(ledger::save_transaction_command::new())
        .add_command(ledger::load_transaction_command::new())
        .add_command(ledger::taa_command::new())
        .add_command(ledger::aml_command::new())
        .add_command(ledger::endorse_transaction_command::new())
        .finalize_group()
        .add_group(payment_address::group::new())
        .add_command(payment_address::create_command::new())
        .add_command(payment_address::list_command::new())
        .add_command(payment_address::sign_command::new())
        .add_command(payment_address::verify_command::new())
        .finalize_group()
        .finalize()
}

fn execute_stdin(command_executor: CommandExecutor) {
    match Reader::new("indy-cli") {
        Ok(reader) => execute_interactive(command_executor, reader),
        Err(_) => execute_batch(&command_executor, None),
    }
}

fn execute_interactive<T>(command_executor: CommandExecutor, mut reader: Reader<T>)
    where T: Terminal {
    let command_executor = Rc::new(command_executor);
    reader.set_completer(command_executor.clone());
    reader.set_prompt(&command_executor.ctx().get_prompt());
    history::load(&mut reader).ok();

    while let Ok(read_result) = reader.read_line() {
        match read_result {
            ReadResult::Input(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let _ = command_executor.execute(&line).is_ok();
                reader.add_history(line.to_string());
                reader.set_prompt(&command_executor.ctx().get_prompt());

                if command_executor.ctx().is_exit() {
                    history::persist(&reader).ok();
                    break;
                }
            },
            ReadResult::Eof | ReadResult::Signal(Signal::Quit) | ReadResult::Signal(Signal::Break)| ReadResult::Signal(Signal::Interrupt) => {
                history::persist(&reader).ok();
                break;
            },
            _ => {break}
        }
    }
}

fn execute_batch(command_executor: &CommandExecutor, script_path: Option<&str>) {
    command_executor.ctx().set_batch_mode();
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
    command_executor.ctx().set_not_batch_mode();
}

fn _load_plugins(command_executor: &CommandExecutor, plugins_str: &str) {
    for plugin in plugins_str.split(',') {
        let parts: Vec<&str> = plugin.split(':').collect::<Vec<&str>>();

        let name = unwrap_or_return!(parts.get(0), println_err!("Plugin Name not found in {}", plugin));
        let init_func = unwrap_or_return!(parts.get(1), println_err!("Plugin Init function not found in {}", plugin));

        common::load_plugin(command_executor.ctx(), name, init_func).ok();
    }
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
    println_acc!("Options:");
    println_acc!("\tLoad plugins in Libindy.");
    println_acc!("\tUsage: indy-cli --plugins <lib-1-name>:<init-func-1-name>,...,<lib-n-name>:<init-func-n-name>");
    println!();
    println_acc!("\tInit logger according to a config file. \n\tIndy Cli uses `log4rs` logging framework: https://crates.io/crates/log4rs");
    println_acc!("\tUsage: indy-cli --logger-config <path-to-config-file>");
    println!();
    println_acc!("\tUse config file for CLI initialization. A config file can contain the following fields:");
    println_acc!("\t\tplugins - a list of plugins to load in Libindy (is equal to usage of \"--plugins\" option).");
    println_acc!("\t\tloggerConfig - path to a logger config file (is equal to usage of \"--logger-config\" option).");
    println_acc!("\t\ttaaAcceptanceMechanism - transaction author agreement acceptance mechanism to use for sending write transactions to the Ledger.");
    println_acc!("\tUsage: indy-cli --config <path-to-config-json-file>");
    println!();
}

fn _iter_batch<T>(command_executor: &CommandExecutor, reader: T) where T: std::io::BufRead {
    let mut line_num = 1;
    for line in reader.lines() {
        let line = if let Ok(line) = line { line } else {
            return println_err!("Can't parse line #{}", line_num);
        };

        if line.starts_with('#') || line.is_empty() {
            // Skip blank lines and lines starting with #
            continue;
        }

        println!("{}", line);
        let (line, force) = if line.starts_with('-') {
            (line[1..].as_ref(), true)
        } else {
            (line[0..].as_ref(), false)
        };
        if command_executor.execute(line).is_err() && !force {
            return println_err!("Batch execution failed at line #{}", line_num);
        }
        println!();
        line_num += 1;

        if command_executor.ctx().is_exit() {
            break;
        }
    }
}

impl<Term: Terminal> Completer<Term> for CommandExecutor {
    fn complete(&self, word: &str, reader: &Reader<Term>,
                _start: usize, _end: usize) -> Option<Vec<Completion>> {
        Some(self
            .complete(reader.buffer(),
                      word,
                      reader.cursor())
            .into_iter()
            .map(|c| Completion {
                completion: c.0,
                display: None,
                suffix: linefeed::Suffix::Some(c.1),
            })
            .collect())
    }
}
