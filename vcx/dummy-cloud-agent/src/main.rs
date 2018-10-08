// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate failure;

extern crate futures;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate pretty_env_logger as env_logger;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate rmp_serde;

#[cfg(test)]
extern crate tokio_core;

#[macro_use]
pub(crate) mod utils;

pub(crate) mod actors;
pub(crate) mod domain;
pub(crate) mod endpoints;
pub(crate) mod indy;

use actix::prelude::*;
use actix_web::{http, middleware, server, App};
use actix_web::server::Server;
use actors::forward_agent::ForwardAgent;
use domain::config::{AgentConfig, Config, ServerConfig};
use failure::Error;
use futures::*;
use endpoints::AppState;
use std::env;
use std::fs::File;

fn main() {
    env_logger::init();
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
    let config = match File::open(config_path) {
        Ok(config) => config,
        Err(err) => return println!("Can't open config file {}\nError: {}", config_path, err),
    };

    let Config { agent: agent_config, server: server_config } = match serde_json::from_reader(config) {
        Ok(config) => config,
        Err(err) => return println!("Can't parse config file {}\nError: {}", config_path, err)
    };

    let sys = actix::System::new("dummy-agent");

    Arbiter::spawn(
        _start_forward_agent(agent_config)
            .and_then(|forward_agent| _start_server(server_config, forward_agent))
            .map(|_| ()) // TODO: Expose server addr for graceful shutdown support
            .map_err(|err| panic!("Can't start Dummy Agent: {}!", err)));

    let _ = sys.run();
}

fn _start_forward_agent(config: AgentConfig) -> ResponseFuture<Addr<ForwardAgent>, Error> {
    let res = ForwardAgent::new(config)
        .and_then(|forward_agent| {
            let res = forward_agent.start();
            info!("Dummy Agent started!");
            future::ok(res)
        });

    Box::new(res)
}

fn _start_server(config: ServerConfig, forward_agent: Addr<ForwardAgent>) -> ResponseFuture<Addr<Server>, Error> {
    let mut server = server::new(move || {
        _start_app(forward_agent.clone())
    });

    for address in config.addresses {
        server = server
            .bind(address)
            .expect("Can't bind to address!");
    }

    let res = server.start();
    info!("Server started!");

    Box::new(future::ok(res))
}

fn _start_app(forward_agent: Addr<ForwardAgent>) -> App<AppState> {
    let res = App::with_state(AppState { forward_agent })
        .middleware(middleware::Logger::default()) // enable logger
        .resource("/forward_agent", |r| r.method(http::Method::GET).with(endpoints::get))
        .resource("/forward_agent/msg", |r| r.method(http::Method::GET).with(endpoints::post_msg));

    info!("App started!");
    res
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