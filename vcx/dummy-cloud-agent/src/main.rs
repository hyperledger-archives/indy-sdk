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
extern crate libc;
extern crate libloading;

use actix::prelude::*;
use actors::forward_agent::ForwardAgent;
use domain::config::{Config, WalletStorageConfig};
use domain::protocol_type::ProtocolType;
use failure::*;
use std::env;
use std::fs::File;
use actors::admin::Admin;
use app::start_app_server;
use app_admin::start_app_admin_server;
use indy::wallet_plugin::{load_storage_library, serialize_storage_plugin_configuration, finish_loading_postgres};

#[macro_use]
pub(crate) mod utils;

pub(crate) mod actors;
pub(crate) mod app;
pub(crate) mod app_admin;
pub(crate) mod domain;
pub(crate) mod indy;

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


fn _init_wallet(wallet_storage_config: &WalletStorageConfig) -> Result<(), String> {
    match wallet_storage_config.xtype.as_ref() {
        Some(wallet_type) => {
            let (plugin_library_path_serialized,
                plugin_init_function_serialized,
                storage_config_serialized,
                storage_credentials_serialized)
                = serialize_storage_plugin_configuration(wallet_type,
                                                         &wallet_storage_config.config,
                                                         &wallet_storage_config.credentials,
                                                         &wallet_storage_config.plugin_library_path,
                                                         &wallet_storage_config.plugin_init_function)?;
            let lib= load_storage_library(&plugin_library_path_serialized, &plugin_init_function_serialized)?;
            if wallet_type == "postgres_storage" {
                finish_loading_postgres(lib, &storage_config_serialized, &storage_credentials_serialized)?;
            }
            info!("Successfully loaded wallet plugin {}.", wallet_type);
            Ok(())
        }
        None => {
            info!("Using default builtin IndySDK wallets.");
            Ok(())
        }
    }
}

fn _start(config_path: &str) {
    info!("Starting Indy Dummy Agent with config: {}", config_path);
    let Config {
        app: app_config,
        forward_agent: forward_agent_config,
        server: server_config,
        wallet_storage: wallet_storage_config,
        protocol_type: protocol_type_config,
        indy_runtime,
        server_admin: server_admin_config
    } = File::open(config_path)
        .context("Can't open config file")
        .and_then(|reader| serde_json::from_reader(reader)
            .context("Can't parse config file"))
        .expect("Invalid configuration file");

    match indy_runtime {
        Some(x) => {
            let runtime_config_str = serde_json::to_string(&x)
                .expect("Failed to re-serialize indy_runtime.");
            info!("Setting indy runtime configuration: {}", &runtime_config_str);
            indyrs::set_runtime_config(&runtime_config_str);
        }
        None => {
            info!("Will use IndySDK default number of threads for expensive crypto.");
        }
    }

    match _init_wallet(&wallet_storage_config) {
        Err(err) => panic!("Failed to load and initialize storage library. {:}", err),
        Ok(()) => {}
    }

    let sys = actix::System::new("indy-dummy-agent");

    Arbiter::spawn_fn(move || {
        info!("Starting Forward Agent with config: {:?}", forward_agent_config);

        ProtocolType::set(protocol_type_config);

        let admin = match &server_admin_config {
            Some(server_admin_config) if server_admin_config.enabled => {
                let admin = Admin::create();
                start_app_admin_server(server_admin_config, admin.clone());
                Some(admin)
            },
            _ => None
        };
        ForwardAgent::create_or_restore(forward_agent_config, wallet_storage_config, admin.clone())
            .map(move |forward_agent| {
                start_app_server(server_config, app_config, forward_agent, admin)
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


