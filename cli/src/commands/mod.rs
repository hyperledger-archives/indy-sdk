extern crate serde_json;
extern crate regex;

pub mod common;
pub mod did;
pub mod pool;
pub mod wallet;
pub mod ledger;
pub mod payment_address;

use self::regex::Regex;

use crate::command_executor::{CommandContext, CommandParams};
use indy::{ErrorCode, IndyError, WalletHandle, PoolHandle};

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

pub fn get_number_param<T>(key: &str, params: &CommandParams) -> Result<T, ()>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    match params.get(key) {
        Some(value) => value.parse::<T>().map_err(|err|
            println_err!("Can't parse number parameter \"{}\": value: \"{}\", err \"{}\"", key, value, err)),
        None => {
            println_err!("No required \"{}\" parameter present", key);
            Err(())
        }
    }
}

pub fn get_opt_number_param<T>(key: &str, params: &CommandParams) -> Result<Option<T>, ()>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<T>().map_err(|err|
            println_err!("Can't parse number parameter \"{}\": value: \"{}\", err \"{}\"", key, value, err))?),
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
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        },
        Some(v) if v.is_empty() => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        },
        Some(v) => Ok(v.split(',').collect::<Vec<&'a str>>())
    }
}

pub fn get_opt_str_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Option<Vec<&'a str>>, ()> {
    match params.get(name) {
        Some(v) =>
            if v.is_empty() {
                Ok(Some(Vec::<&'a str>::new()))
            } else {
                Ok(Some(v.split(',').collect::<Vec<&'a str>>()))
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

pub fn get_opt_object_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Option<serde_json::Value>, ()> {
    match params.get(name) {
        Some(_) => Ok(Some(get_object_param(name, params)?)),
        None => Ok(None)
    }
}

fn extract_array_tuples(param: &str) -> Vec<String> {
    let re = Regex::new(r#"\(([^\(\)]+)\),?"#).unwrap();
    re.captures_iter(param).map(|c| c[1].to_string()).collect::<Vec<String>>()
}

pub fn get_str_tuple_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Vec<String>, ()> {
    match params.get(name) {
        Some(v) if !v.is_empty() => {
            let tuples = extract_array_tuples(v);
            if tuples.is_empty() {
                println_err!("Parameter \"{}\" has invalid format", name);
                Err(())
            } else {
                Ok(tuples)
            }
        }
        _ => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_number_tuple_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Vec<u64>, ()> {
    match params.get(name) {
        Some(v) if !v.is_empty() => {
            let tuples: Vec<&str> = v.split(",").collect();
            if tuples.is_empty() {
                println_err!("Parameter \"{}\" has invalid format", name);
                Err(())
            } else {
                let mut result: Vec<u64> = Vec::new();
                for item in tuples {
                    println!("{:?}",item);
                    result.push(item.parse::<u64>().map_err(|err|
                        println_err!("Can't parse number parameter \"{}\": value: \"{}\", err \"{}\"", name, item, err))?);
                }
                Ok(result)
            }
        }
        _ => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_str_tuple_array_param<'a>(name: &'a str, params: &'a CommandParams) -> Result<Option<Vec<String>>, ()> {
    match params.get(name) {
        Some(v) =>
            if v.is_empty() {
                Ok(Some(Vec::<String>::new()))
            } else {
                Ok(Some(extract_array_tuples(v)))
            },
        None => Ok(None)
    }
}

pub fn ensure_active_did(ctx: &CommandContext) -> Result<String, ()> {
    match ctx.get_string_value("ACTIVE_DID") {
        Some(did) => Ok(did),
        None => {
            println_err!("There is no active did");
            Err(())
        }
    }
}

pub fn get_active_did(ctx: &CommandContext) -> Option<String> {
    ctx.get_string_value("ACTIVE_DID")
}

pub fn set_active_did(ctx: &CommandContext, did: Option<String>) {
    ctx.set_string_value("ACTIVE_DID", did.clone());
    ctx.set_sub_prompt(3, did.map(|did| format!("did({}...{})", &did[..3], &did[did.len() - 3..])));
}

pub fn ensure_opened_wallet_handle(ctx: &CommandContext) -> Result<WalletHandle, ()> {
    match ctx.get_int_value("OPENED_WALLET_HANDLE") {
        Some(wallet_handle) => Ok(WalletHandle(wallet_handle)),
        None => {
            println_err!("There is no opened wallet now");
            Err(())
        }
    }
}

pub fn ensure_opened_wallet(ctx: &CommandContext) -> Result<(WalletHandle, String), ()> {
    let handle = ctx.get_int_value("OPENED_WALLET_HANDLE");
    let name = ctx.get_string_value("OPENED_WALLET_NAME");

    match (handle, name) {
        (Some(handle), Some(name)) => Ok((WalletHandle(handle), name)),
        _ => {
            println_err!("There is no opened wallet now");
            Err(())
        }
    }
}

pub fn get_opened_wallet(ctx: &CommandContext) -> Option<(WalletHandle, String)> {
    let handle = ctx.get_int_value("OPENED_WALLET_HANDLE");
    let name = ctx.get_string_value("OPENED_WALLET_NAME");

    if let (Some(handle), Some(name)) = (handle, name) {
        Some((WalletHandle(handle), name))
    } else {
        None
    }
}

pub fn get_opened_wallet_handle(ctx: &CommandContext) -> Option<WalletHandle> {
    ctx.get_int_value("OPENED_WALLET_HANDLE").map(|val| WalletHandle(val))
}

pub fn set_opened_wallet(ctx: &CommandContext, value: Option<(WalletHandle, String)>) {
    match value {
        Some((wallet_handle, wallet_name)) => {
            ctx.set_int_value("OPENED_WALLET_HANDLE", Some(wallet_handle.0));
            ctx.set_string_value("OPENED_WALLET_NAME", Some(wallet_name.to_owned()));
            ctx.set_sub_prompt(2, Some(wallet_name));
        },
        None => {
            ctx.set_int_value("OPENED_WALLET_HANDLE", None);
            ctx.set_string_value("OPENED_WALLET_NAME", None);
            ctx.set_sub_prompt(2, None);
        }
    }
}

pub fn ensure_connected_pool_handle(ctx: &CommandContext) -> Result<PoolHandle, ()> {
    match ctx.get_int_value("CONNECTED_POOL_HANDLE") {
        Some(pool_handle) => Ok(pool_handle),
        None => {
            println_err!("There is no opened pool now");
            Err(())
        }
    }
}

pub fn ensure_connected_pool(ctx: &CommandContext) -> Result<(PoolHandle, String), ()> {
    let handle = ctx.get_int_value("CONNECTED_POOL_HANDLE");
    let name = ctx.get_string_value("CONNECTED_POOL_NAME");

    match (handle, name) {
        (Some(handle), Some(name)) => Ok((handle, name)),
        _ => {
            println_err!("There is no opened pool now");
            Err(())
        }
    }
}

pub fn get_connected_pool(ctx: &CommandContext) -> Option<(PoolHandle, String)> {
    let handle = ctx.get_int_value("CONNECTED_POOL_HANDLE");
    let name = ctx.get_string_value("CONNECTED_POOL_NAME");

    if let (Some(handle), Some(name)) = (handle, name) {
        Some((handle, name))
    } else {
        None
    }
}

pub fn set_connected_pool(ctx: &CommandContext, value: Option<(PoolHandle, String)>) {
    ctx.set_int_value("CONNECTED_POOL_HANDLE", value.as_ref().map(|value| value.0.to_owned()));
    ctx.set_string_value("CONNECTED_POOL_NAME", value.as_ref().map(|value| value.1.to_owned()));
    ctx.set_sub_prompt(1, value.map(|value| format!("pool({})", value.1)));
}


pub fn set_transaction(ctx: &CommandContext, request: Option<String>) {
    ctx.set_string_value("LEDGER_TRANSACTION", request.clone());
}

pub fn get_transaction(ctx: &CommandContext) -> Option<String> {
    ctx.get_string_value("LEDGER_TRANSACTION")
}

pub fn ensure_set_transaction(ctx: &CommandContext) -> Result<String, ()> {
    match ctx.get_string_value("LEDGER_TRANSACTION") {
        Some(transaction) => Ok(transaction),
        None => {
            println_err!("There is no transaction stored into context");
            Err(())
        }
    }
}

pub fn set_transaction_author_info(ctx: &CommandContext, value: Option<(String, String, u64)>) {
    ctx.set_string_value("AGREEMENT_TEXT", value.as_ref().map(|value| value.0.to_owned()));
    ctx.set_string_value("AGREEMENT_VERSION", value.as_ref().map(|value| value.1.to_owned()));
    ctx.set_uint_value("AGREEMENT_TIME_OF_ACCEPTANCE", value.as_ref().map(|value| value.2));
}

pub fn get_transaction_author_info(ctx: &CommandContext) -> Option<(String, String, String, u64)> {
    let text = ctx.get_string_value("AGREEMENT_TEXT");
    let version = ctx.get_string_value("AGREEMENT_VERSION");
    let acc_mech_type = ctx.get_taa_acceptance_mechanism();
    let time_of_acceptance = ctx.get_uint_value("AGREEMENT_TIME_OF_ACCEPTANCE");

    if let (Some(text), Some(version), Some(time_of_acceptance)) = (text, version, time_of_acceptance) {
        Some((text, version, acc_mech_type, time_of_acceptance))
    } else {
        None
    }
}

const DEFAULT_POOL_PROTOCOL_VERSION: usize = 2;

pub fn set_pool_protocol_version(ctx: &CommandContext, protocol_version: usize) {
    ctx.set_uint_value("POOL_PROTOCOL_VERSION", Some(protocol_version as u64));
}

pub fn get_pool_protocol_version(ctx: &CommandContext) -> usize {
    match ctx.get_uint_value("POOL_PROTOCOL_VERSION") {
        Some(protocol_version) => protocol_version as usize,
        None => DEFAULT_POOL_PROTOCOL_VERSION
    }
}

pub fn handle_indy_error(err: IndyError, submitter_did: Option<&str>, pool_name: Option<&str>, wallet_name: Option<&str>) {
    match err.error_code {
        ErrorCode::WalletAlreadyExistsError => println_err!("Wallet \"{}\" already exists", wallet_name.unwrap_or("")),
        ErrorCode::WalletInvalidHandle => println_err!("Wallet: \"{}\" not found", wallet_name.unwrap_or("")),
        ErrorCode::WalletItemNotFound => println_err!("Submitter DID: \"{}\" not found", submitter_did.unwrap_or("")),
        ErrorCode::WalletIncompatiblePoolError =>
            println_err!("Wallet \"{}\" is incompatible with pool \"{}\".", wallet_name.unwrap_or(""), pool_name.unwrap_or("")),
        ErrorCode::PoolLedgerInvalidPoolHandle => println_err!("Pool: \"{}\" not found", pool_name.unwrap_or("")),
        ErrorCode::PoolLedgerNotCreatedError => println_err!("Pool \"{}\" does not exist.", pool_name.unwrap_or("")),
        ErrorCode::PoolLedgerTerminated => println_err!("Pool \"{}\" does not exist.", pool_name.unwrap_or("")),
        ErrorCode::PoolLedgerTimeout => println_err!("Transaction response has not been received"),
        ErrorCode::DidAlreadyExistsError => println_err!("Did already exists"),
        _ => println_err!("{}", err.message),
    }
}

#[cfg(test)]
use crate::libindy::ledger::Ledger;

#[cfg(test)]
pub fn submit_retry<F, T, E>(ctx: &CommandContext, request: &str, parser: F) -> Result<(), ()>
    where F: Fn(&str) -> Result<T, E> {
    const SUBMIT_RETRY_CNT: usize = 3;
    const SUBMIT_TIMEOUT_SEC: u64 = 2;

    let pool_handle = ensure_connected_pool_handle(ctx).unwrap();

    for _ in 0..SUBMIT_RETRY_CNT {
        let response = Ledger::submit_request(pool_handle, request).unwrap();
        if parser(&response).is_ok() {
            return Ok(());
        }
        ::std::thread::sleep(::std::time::Duration::from_secs(SUBMIT_TIMEOUT_SEC));
    }

    return Err(());
}

#[cfg(test)]
use crate::utils::test::TestUtils;

#[cfg(test)]
fn setup() -> CommandContext {
    TestUtils::cleanup_storage();
    CommandContext::new()
}

#[cfg(test)]
fn setup_with_wallet() -> CommandContext {
    let ctx = setup();
    wallet::tests::create_and_open_wallet(&ctx);
    ctx
}

#[cfg(test)]
fn setup_with_wallet_and_pool() -> CommandContext {
    let ctx = setup();
    wallet::tests::create_and_open_wallet(&ctx);
    pool::tests::create_and_connect_pool(&ctx);
    ctx
}

#[cfg(test)]
#[cfg(feature = "nullpay_plugin")]
fn setup_with_wallet_and_pool_and_payment_plugin() -> CommandContext {
    let ctx = setup_with_wallet_and_pool();
    common::tests::load_null_payment_plugin(&ctx);
    ctx
}

#[cfg(test)]
fn tear_down_with_wallet_and_pool(ctx: &CommandContext) {
    wallet::tests::close_and_delete_wallet(&ctx);
    pool::tests::disconnect_and_delete_pool(&ctx);
    tear_down();
}

#[cfg(test)]
fn tear_down_with_wallet(ctx: &CommandContext) {
    wallet::tests::close_and_delete_wallet(&ctx);
    tear_down();
}

#[cfg(test)]
fn tear_down() {
    TestUtils::cleanup_storage();
}
