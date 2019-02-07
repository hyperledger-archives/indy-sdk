// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate actix;
extern crate actix_web;
extern crate bytes;
#[cfg(test)]
extern crate dirs;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate pretty_env_logger as env_logger;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[cfg(test)]
extern crate tokio_core;
extern crate base64;
extern crate rand;
extern crate hyper;
extern crate indyrs;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use actix::prelude::*;
use actors::forward_agent::ForwardAgent;
use domain::config::Config;
use domain::protocol_type::ProtocolType;
use failure::*;
use futures::*;
use std::env;
use std::fs::File;

#[macro_use]
pub(crate) mod utils;

pub(crate) mod actors;
pub(crate) mod app;
pub(crate) mod domain;
pub(crate) mod indy;
pub(crate) mod server;

fn main() {
    indy::logger::set_default_logger(None)
        .expect("Can't init indy logger");

    env_logger::try_init()
        .expect("Can't init env logger");

    let mut args = env::args();
    args.next(); // skip app name

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return _print_help(),
            _ if args.len() == 0 => return _start(&arg),
            _ => {
                println!("Unknown option");
                return _print_help();
            }
        }
    }

    _print_help();
}

fn _start(config_path: &str) {
    info!("Starting Indy Dummy Agent with config: {}", config_path);

    let Config {
        app: app_config,
        forward_agent: forward_agent_config,
        server: server_config,
        wallet_storage: wallet_storage_config,
        protocol_type: protocol_type_config,
    } = File::open(config_path)
        .context("Can't open config file")
        .and_then(|reader| serde_json::from_reader(reader)
            .context("Can't parse config file"))
        .expect("Invalid configuration file");

    let sys = actix::System::new("indy-dummy-agent");

    Arbiter::spawn_fn(move || {
        info!("Starting Forward Agent with config: {:?}", forward_agent_config);

        ProtocolType::set(protocol_type_config);

        ForwardAgent::create_or_restore(forward_agent_config, wallet_storage_config)
            .map(move |forward_agent| {
                info!("Forward Agent started");
                info!("Starting Server with config: {:?}", server_config);

                server::start(server_config, move || {
                    info!("Starting App with config: {:?}", app_config);
                    app::new(app_config.clone(), forward_agent.clone())
                });

                info!("Server started");
            })
            .map(|_| ()) // TODO: Expose server addr for graceful shutdown support
            .map_err(|err| panic!("Can't start Indy Dummy Agent: {}!", err))
    });

    let _ = sys.run();
}

fn _print_help() {
    println!("Hyperledger Indy Dummy Agent");
    println!("\tUsage:");
    println!("\t\tindy-dummy-agent <path-to-config-file>");
    println!("Options:");
    println!("\t-h | --help Print help. Usage:");
    println!("\tUsage:");
    println!("\t\tindy-dummy-agent -h");
    println!("\t\tindy-dummy-agent --help");
    println!();
}