extern crate serde_json;

pub mod common;
pub mod did;
pub mod pool;
pub mod wallet;
pub mod ledger;

use command_executor::{CommandContext, CommandParams};

use std;

pub fn get_str_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<&'a str, ()> {
    match params.get(name) {
        Some(v) if v == "" => {
            println_err!("Required \"{}\" parameter is empty.", name);
            Err(())
        }
        Some(v) => Ok(v.as_str()),
        None => {
            println_err!("No required \"{}\" parameter present.", name);
            Err(())
        }
    }
}

pub fn get_opt_str_param<'a>(key: &'a str, params: &'a CommandParams) -> Result<Option<&'a str>, ()> {
    match params.get(key) {
        Some(v) if v == "" => {
            println_err!("Optional parameter \"{}\" is empty.", key);
            Err(())
        }
        Some(v) => Ok(Some(v.as_str())),
        None => Ok(None)
    }
}

pub fn get_opt_empty_str_param<'a>(key: &'a str, params: &'a CommandParams) -> Result<Option<&'a str>, ()> {
    Ok(params.get(key).map(|v| v.as_str()))
}

pub fn _get_int_param<T>(name: &str, params: &CommandParams) -> Result<T, ()>
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

pub fn get_opt_number_param<T>(key: &str, params: &CommandParams) -> Result<Option<T>, ()>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<T>().map_err(|err|
            println_err!("Can't parse integer parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_bool_param(key: &str, params: &CommandParams) -> Result<bool, ()> {
    match params.get(key) {
        Some(value) => Ok(value.parse::<bool>().map_err(|err|
            println_err!("Can't parse bool parameter \"{}\": err {}", key, err))?),
        None => {
            println_err!("No required \"{}\" parameter present", key);
            Err(())
        }
    }
}

pub fn get_opt_bool_param(key: &str, params: &CommandParams) -> Result<Option<bool>, ()> {
    match params.get(key) {
        Some(value) => Ok(Some(value.parse::<bool>().map_err(|err|
            println_err!("Can't parse bool parameter \"{}\": err {}", key, err))?)),
        None => Ok(None)
    }
}

pub fn get_str_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Vec<&'a str>, ()> {
    match params.get(name) {
        Some(v) => Ok(v.split(",").collect::<Vec<&'a str>>()),
        None => Err(println_err!("No required \"{}\" parameter present", name))
    }
}

pub fn get_opt_str_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Option<Vec<&'a str>>, ()> {
    match params.get(name) {
        Some(v) =>
            if v.is_empty() {
                Ok(Some(Vec::<&'a str>::new()))
            } else {
                Ok(Some(v.split(",").collect::<Vec<&'a str>>()))
            },
        None => Ok(None)
    }
}

pub fn get_object_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<serde_json::Value, ()> {
    match params.get(name) {
        Some(v) => Ok(serde_json::from_str(v).map_err(|err|
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

pub fn ensure_opened_wallet(ctx: &CommandContext) -> Result<(i32, String), ()> {
    let handle = ctx.get_int_value("OPENED_WALLET_HANDLE");
    let name = ctx.get_string_value("OPENED_WALLET_NAME");

    match (handle, name) {
        (Some(handle), Some(name)) => Ok((handle, name)),
        _ => Err(println_err!("There is no opened wallet now"))
    }
}

pub fn get_opened_wallet(ctx: &CommandContext) -> Option<(i32, String)> {
    let handle = ctx.get_int_value("OPENED_WALLET_HANDLE");
    let name = ctx.get_string_value("OPENED_WALLET_NAME");

    if let (Some(handle), Some(name)) = (handle, name) {
        Some((handle, name))
    } else {
        None
    }
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

pub fn ensure_connected_pool(ctx: &CommandContext) -> Result<(i32, String), ()> {
    let handle = ctx.get_int_value("CONNECTED_POOL_HANDLE");
    let name = ctx.get_string_value("CONNECTED_POOL_NAME");

    match (handle, name) {
        (Some(handle), Some(name)) => Ok((handle, name)),
        _ => Err(println_err!("There is no opened pool now"))
    }
}

pub fn get_connected_pool(ctx: &CommandContext) -> Option<(i32, String)> {
    let handle = ctx.get_int_value("CONNECTED_POOL_HANDLE");
    let name = ctx.get_string_value("CONNECTED_POOL_NAME");

    if let (Some(handle), Some(name)) = (handle, name) {
        Some((handle, name))
    } else {
        None
    }
}

pub fn set_connected_pool(ctx: &CommandContext, value: Option<(i32, String)>) {
    ctx.set_int_value("CONNECTED_POOL_HANDLE", value.as_ref().map(|value| value.0.to_owned()));
    ctx.set_string_value("CONNECTED_POOL_NAME", value.as_ref().map(|value| value.1.to_owned()));
    ctx.set_sub_prompt(1, value.map(|value| format!("pool({})", value.1)));
}