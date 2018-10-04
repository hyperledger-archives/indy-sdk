// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate error_chain;

extern crate futures as futures_rs;

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
pub(crate) mod errors;

#[macro_use]
pub(crate) mod futures;

#[macro_use]
pub(crate) mod utils;

pub(crate) mod actors;
pub(crate) mod domain;
pub(crate) mod endpoints;
pub(crate) mod indy;

use actix::prelude::*;
use actix_web::{http, middleware, server, App};
use actix_web::server::Server;
use actors::agency::Agency;
use domain::config::{AgencyConfig, Config, ServerConfig};
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

    let Config { agency: agency_config, server: server_config } = match serde_json::from_reader(config) {
        Ok(config) => config,
        Err(err) => return println!("Can't parse config file {}\nError: {}", config_path, err)
    };

    let sys = actix::System::new("dummy-agency");

    Arbiter::spawn(
        _start_agency(agency_config)
            .and_then(|agency| _start_server(server_config, agency))
            .map(|_| ()) // TODO: Expose server addr for graceful shutdown support
            .map_err(|err| panic!("Can't start agency: {}!", err)));

    let _ = sys.run();
}

fn _start_agency(config: AgencyConfig) -> BoxedFuture<Addr<Agency>> {
    let res = Agency::new(config)
        .and_then(|agency| {
            let res = agency.start();
            info!("Agency started!");
            f_ok(res)
        });

    Box::new(res)
}

fn _start_server(config: ServerConfig, agency: Addr<Agency>) -> BoxedFuture<Addr<Server>> {
    let mut server = server::new(move || {
        _start_app(agency.clone())
    });

    for address in config.addresses {
        server = server
            .bind(address)
            .expect("Can't bind to address!");
    }

    let res = server.start();
    info!("Server started!");
    f_ok(res)
}

fn _start_app(agency: Addr<Agency>) -> App<AppState> {
    let res = App::with_state(AppState { agency })
        .middleware(middleware::Logger::default()) // enable logger
        .resource("/agency", |r| r.method(http::Method::GET).with(endpoints::get))
        .resource("/agency/msg", |r| r.method(http::Method::POST).with(endpoints::post_msg));

    info!("App started!");
    res
}

fn _print_help() {
    println!("Hyperledger Indy Dummy Agency");
    println!("\tUsage:");
    println!("\t\tindy-dummy-agency <path-to-config-file>");
    println!("Options:");
    println!("\t-h | --help Print help. Usage:");
    println!("\tUsage:");
    println!("\t\tindy-dummy-agency -h");
    println!("\t\tindy-dummy-agency --help");
    println!();
}