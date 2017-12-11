extern crate serde_json;

pub mod common;
pub mod did;
pub mod pool;
pub mod wallet;
pub mod ledger;

use command_executor::CommandContext;

use std::collections::HashMap;
use std;

pub fn get_str_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<&'a str, ()> {
    match params.get(name) {
        Some(v) => Ok(*v),
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_str_param<'a>(key: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<&'a str>, ()> {
    Ok(params.get(key).map(|v| *v))
}

pub fn get_int_param<T>(name: &str, params: &HashMap<&'static str, &str>) -> Result<T, ()>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    match params.get(name) {
        Some(v) => {
            Ok(v.parse::<T>().map_err(|err|
                println_err!("Can't parse integer parameter \"{}\": err {}", name, err))?)
        }
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_int_param<T>(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<T>, ()>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<T>().map_err(|err|
            println_err!("Can't parse integer parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_opt_bool_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<bool>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<bool>().map_err(|err|
            println_err!("Can't parse bool parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_str_array_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Vec<&'a str>, ()> {
    match params.get(name) {
        Some(v) => Ok(v.split(",").collect::<Vec<&'a str>>()),
        None => Err(println_err!("No required \"{}\" parameter present", name))
    }
}

pub fn get_opt_str_array_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<Vec<&'a str>>, ()> {
    match params.get(name) {
        Some(v) => Ok(Some(v.split(",").collect::<Vec<&'a str>>())),
        None => Ok(None)
    }
}

pub fn get_object_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<serde_json::Value, ()> {
    match params.get(name) {
        Some(v) => Ok(serde_json::from_str(*v).map_err(|err|
            println_err!("Can't parse object parameter \"{}\": err {}", name, err))?),
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn ensure_active_did(ctx: &CommandContext) -> Result<String, ()> {
    match ctx.get_string_value("ACTIVE_DID") {
        Some(did) => Ok(did),
        None => Err(println_err!("There is no active did"))
    }
}

pub fn get_active_did(ctx: &CommandContext) -> Option<String> {
    ctx.get_string_value("ACTIVE_DID")
}

pub fn set_active_did(ctx: &CommandContext, did: Option<String>) {
    ctx.set_string_value("ACTIVE_DID", did.clone());
    ctx.set_sub_prompt(3, did.map(|did| format!("did({}...{})", &did[..3], &did[did.len() - 3..])));
}

pub fn ensure_opened_wallet_handle(ctx: &CommandContext) -> Result<i32, ()> {
    match ctx.get_int_value("OPENED_WALLET_HANDLE") {
        Some(wallet_handle) => Ok(wallet_handle),
        None => Err(println_err!("There is no opened wallet now"))
    }
}

pub fn get_opened_wallet(ctx: &CommandContext) -> Option<(i32, String)> {
    if let Some(handle) = ctx.get_int_value("OPENED_WALLET_HANDLE") {
        if let Some(name) = ctx.get_string_value("OPENED_WALLET_NAME") {
            return Some((handle, name));
        }
    }

    None
}

pub fn set_opened_wallet(ctx: &CommandContext, value: Option<(i32, String)>) {
    ctx.set_int_value("OPENED_WALLET_HANDLE", value.as_ref().map(|value| value.0.to_owned()));
    ctx.set_string_value("OPENED_WALLET_NAME", value.as_ref().map(|value| value.1.to_owned()));
    ctx.set_sub_prompt(2, value.map(|value| format!("wallet({})", value.1)));
}

pub fn ensure_connected_pool_handle(ctx: &CommandContext) -> Result<i32, ()> {
    match ctx.get_int_value("CONNECTED_POOL_HANDLE") {
        Some(pool_handle) => Ok(pool_handle),
        None => Err(println_err!("There is no opened pool now"))
    }
}

pub fn get_connected_pool(ctx: &CommandContext) -> Option<(i32, String)> {
    if let Some(handle) = ctx.get_int_value("CONNECTED_POOL_HANDLE") {
        if let Some(name) = ctx.get_string_value("CONNECTED_POOL_NAME") {
            return Some((handle, name));
        }
    }

    None
}

pub fn set_connected_pool(ctx: &CommandContext, value: Option<(i32, String)>) {
    ctx.set_int_value("CONNECTED_POOL_HANDLE", value.as_ref().map(|value| value.0.to_owned()));
    ctx.set_string_value("CONNECTED_POOL_NAME", value.as_ref().map(|value| value.1.to_owned()));
    ctx.set_sub_prompt(1, value.map(|value| format!("pool({})", value.1)));
}