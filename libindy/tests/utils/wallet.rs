use indy::api::ErrorCode;
use indy::api::wallet::*;

use utils::{callback, sequence, environment, ctypes};
use utils::inmem_wallet::InmemWallet;

use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::sync::Mutex;
use std::ptr::null;
use utils::constants::{TYPE, INMEM_TYPE, WALLET_CREDENTIALS};

use std::path::{Path, PathBuf};

use serde_json;

pub fn register_wallet_storage(xtype: &str, force_create: bool) -> Result<(), ErrorCode> {
    lazy_static! {
            static ref REGISERED_WALLETS: Mutex<HashSet<String>> = Default::default();
        }

    let mut wallets = REGISERED_WALLETS.lock().unwrap();

    if wallets.contains(xtype) & !force_create {
        // as registering of plugged wallet with
        return Ok(());
    }

    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let xxtype = CString::new(xtype).unwrap();

    let err = indy_register_wallet_storage(
        command_handle,
        xxtype.as_ptr(),
        Some(InmemWallet::create),
        Some(InmemWallet::open),
        Some(InmemWallet::close),
        Some(InmemWallet::delete),
        Some(InmemWallet::add_record),
        Some(InmemWallet::update_record_value),
        Some(InmemWallet::update_record_tags),
        Some(InmemWallet::add_record_tags),
        Some(InmemWallet::delete_record_tags),
        Some(InmemWallet::delete_record),
        Some(InmemWallet::get_record),
        Some(InmemWallet::get_record_id),
        Some(InmemWallet::get_record_type),
        Some(InmemWallet::get_record_value),
        Some(InmemWallet::get_record_tags),
        Some(InmemWallet::free_record),
        Some(InmemWallet::get_storage_metadata),
        Some(InmemWallet::set_storage_metadata),
        Some(InmemWallet::free_storage_metadata),
        Some(InmemWallet::search_records),
        Some(InmemWallet::search_all_records),
        Some(InmemWallet::get_search_total_count),
        Some(InmemWallet::fetch_search_next_record),
        Some(InmemWallet::free_search),
        cb
    );

    wallets.insert(xtype.to_string());

    super::results::result_to_empty(err, receiver)
}

pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(credentials).unwrap();

    let err =
        indy_create_wallet(command_handle,
                           config.as_ptr(),
                           credentials.as_ptr(),
                           cb);

    super::results::result_to_empty(err, receiver)
}

pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_i32();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(credentials).unwrap();

    let err =
        indy_open_wallet(command_handle,
                         config.as_ptr(),
                         credentials.as_ptr(),
                         cb);

    super::results::result_to_int(err, receiver)
}

pub fn create_and_open_wallet(storage_type: Option<&str>) -> Result<i32, ErrorCode> {
    let config = json!({
            "id": format!("default-wallet_id-{}", sequence::get_next_id()),
            "storage_type": storage_type.unwrap_or(TYPE)
        }).to_string();

    create_wallet(&config, WALLET_CREDENTIALS)?;
    open_wallet(&config, WALLET_CREDENTIALS)
}

pub fn create_and_open_default_wallet() -> Result<i32, ErrorCode> {
    let config = json!({
            "id": format!("default-wallet_id-{}", sequence::get_next_id()),
            "storage_type": TYPE
        }).to_string();

    create_wallet(&config, WALLET_CREDENTIALS)?;
    open_wallet(&config, WALLET_CREDENTIALS)
}

pub fn create_and_open_plugged_wallet() -> Result<i32, ErrorCode> {
    let config = json!({
            "id": format!("default-wallet_id-{}", sequence::get_next_id()),
            "storage_type": INMEM_TYPE
        }).to_string();

    register_wallet_storage("inmem", false).unwrap();
    create_wallet(&config, WALLET_CREDENTIALS)?;
    open_wallet(&config, WALLET_CREDENTIALS)
}

pub fn delete_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(credentials).unwrap();

    let err = indy_delete_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb);

    super::results::result_to_empty(err, receiver)
}

pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let err = indy_close_wallet(command_handle, wallet_handle, cb);

    super::results::result_to_empty(err, receiver)
}

pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();
    let export_config_json = CString::new(export_config_json).unwrap();

    let err = indy_export_wallet(command_handle, wallet_handle, export_config_json.as_ptr(), cb);

    super::results::result_to_empty(err, receiver)
}

pub fn import_wallet(config: &str, credentials: &str, import_config: &str) -> Result<(), ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let config = CString::new(config).unwrap();
    let credentials = CString::new(credentials).unwrap();
    let import_config = CString::new(import_config).unwrap();

    let err =
        indy_import_wallet(command_handle,
                           config.as_ptr(),
                           credentials.as_ptr(),
                           import_config.as_ptr(),
                           cb);

    super::results::result_to_empty(err, receiver)
}

pub fn export_wallet_path() -> PathBuf {
    environment::tmp_file_path("export_file")
}

pub fn prepare_export_wallet_config(path: &Path) -> String {
    let json = json!({
            "path": path.to_str().unwrap(),
            "key": "export_key",
        });
    serde_json::to_string(&json).unwrap()
}

pub fn generate_wallet_key(config: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec_string();

    let config = config.map(ctypes::str_to_cstring);

    let err =
        indy_generate_wallet_key(command_handle,
                                 config.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                 cb);

    super::results::result_to_string(err, receiver)
}

/* 
 * Dynamically loads the specified library and registers storage
 */
fn load_storage_library(_stg_type: &str, _library: &str, _fn_pfx: &str) {
    ()
}

/*
 * Update the given configuration string based on supplied overrides
 */
fn override_wallet_configuration(config: &str, overrides: &HashMap<String, Option<String>>) -> String {
    let v: Value = serde_json::from_str(config)?;

    match overrides.get("STG_TYPE") {
        Some(stype) => match stype {
            Some(type) => v["storage_type"] = type.clone(),
            None => ()
        },
        None => ()
    }
    match overrides.get("STG_CONFIG") {
        Some(sconfig) => match sconfig {
            Some(config) => v["storage_config"] = config.clone(),
            None => ()
        },
        None => ()
    }

    serde_json::to_string(&v);
}

/*
 * Update the given credentials string based on supplied overrides
 */
fn override_wallet_credentials(creds: &str, overrides: &HashMap<String, Option<String>>) -> String {
    let v: Value = serde_json::from_str(creds)?;

    match overrides.get("STG_CREDS") {
        Some(screds) => match screds {
            Some(creds) => v["storage_credentials"] = creds.clone(),
            None => ()
        },
        None => ()
    }

    serde_json::to_string(&v);
}

/*
 * Returns wallet storage configuation dynamically configured via environment variables:
 * STG_CONFIG - json configuration string to pass to the wallet on creation and open
 * STG_CREDS - json credentials string to pass to the wallet on creation and open
 * STG_TYPE - storage type to create
 * STG_LIB - c-callable library to load (contains a plug-in storage) 
 *             - if specified will dynamically load and register a wallet storage
 * STG_FN_PREFIX - prefix for all plug-in functions (allows standard function naming)
 */
pub fn wallet_storage_overrides() -> HashMap<String, Option<String>> {
    let mut storage_config = HashMap::new();
    let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];

    for env_var in env_vars.iter() {
        match env::var(env_var) {
            Ok(var) => storage_config.insert(env_var.to_string(), Some(var.to_string())),
            Err(_) => storage_config.insert(env_var.to_string(), None)
        };
    }

    match storage_config.get("STG_LIB") {
        Some(slibrary) => match slibrary {
            Some(library) => {
                let stg_type: String = match storage_config.get("STG_CONFIG") {
                    Some(styp) => match styp {
                        Some(typ) => typ.clone(),
                        None => "".to_string()
                    },
                    None => "".to_string()
                };
                let fn_pfx: String = match storage_config.get("STG_FN_PREFIX") {
                    Some(spfx) => match spfx {
                        Some(pfx) => pfx.clone(),
                        None => "".to_string()
                    },
                    None => "".to_string()
                };
                load_storage_library(&stg_type[..], &library[..], &fn_pfx[..])
            },
            None => ()
        },
        None => ()
    }

    storage_config
}
