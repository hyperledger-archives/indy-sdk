extern crate sharedlib;
extern crate base64;

use self::sharedlib::{Lib, Func, Symbol};

use indy::api::ErrorCode;
use indy::api::wallet::*;

use utils::{callback, sequence, environment, ctypes};
use utils::inmem_wallet::InmemWallet;
use utils::domain::wallet::{Config, Credentials};

use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::sync::Mutex;
use std::ptr::null;
use utils::constants::{TYPE, INMEM_TYPE, WALLET_CREDENTIALS};

use std::path::{Path, PathBuf};

use serde_json;
use serde_json::Value;


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

    let (config, credentials) = override_wallet_config_creds(config, credentials, true);

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

    let (config, credentials) = override_wallet_config_creds(config, credentials, false);

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

    let (config, credentials) = override_wallet_config_creds(config, credentials, false);

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

/* 
 * Wrapper to ensure a wallet is closed when it goes out of scope
 * (i.e. if the unit test didn't shut down cleanly)
 */
pub struct WalletHandleWrapper {
    pub handle: i32,
}
impl ::std::ops::Drop for WalletHandleWrapper {
    fn drop(&mut self) {
        // close wallet; ignore result in case we are closing it twice
        let _res = close_wallet(self.handle);
    }
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
 * Update wallet config based on supplied configuration,
 *     *only if* "storage_type" is not already provided.
 */
pub fn override_wallet_config_creds(config: &str, credentials: &str, load_dynalib: bool) -> (String, String) {
    // if storge_type is explicit then bail
    let check_config: Config = serde_json::from_str(config).unwrap();
    if let Some(_) = check_config.storage_type {
        return (config.to_owned(), credentials.to_owned());
    }

    // if no config is provided at all then bail
    let storage_config = wallet_storage_overrides();
    if !any_overrides(&storage_config) {
        return (config.to_owned(), credentials.to_owned());
    }

    // load dynamic library if requested
    if load_dynalib {
        load_storage_library_config(&storage_config).unwrap();
    }

    // update config and credentials
    let config = override_wallet_configuration(config, &storage_config);
    let credentials = override_wallet_credentials(credentials, &storage_config);

    return (config, credentials);
}

/*
 * Dynamically loads the specified library and registers storage
 */
pub fn load_storage_library(stg_type: &str, library_path: &str, fn_pfx: &str) -> Result<(), ErrorCode> {
    println!("Loading {} {} {}", stg_type, library_path, fn_pfx);
    lazy_static! {
            static ref STG_REGISERED_WALLETS: Mutex<HashMap<String, Lib>> = Default::default();
        }

    let mut wallets = STG_REGISERED_WALLETS.lock().unwrap();

    if wallets.contains_key(stg_type) {
        // as registering of plugged wallet with
        return Ok(());
    }

    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let xxtype = CString::new(stg_type).unwrap();

    let err;
    let lib;
    let lib_path = Path::new(library_path);
    unsafe {
        println!("Loading {:?}", lib_path);
        lib = match Lib::new(lib_path) {
            Ok(rlib) => {
                println!("Loaded lib");
                rlib
            },
            Err(err) => {
                println!("Load error {:?}", err);
                panic!("Load error {:?}", err)
            }
        };

        println!("Get fn pointers ...");
        let fn_create_handler: Func<WalletCreate> = lib.find_func(&format!("{}create", fn_pfx)).unwrap();
        let fn_open_handler: Func<WalletOpen> = lib.find_func(&format!("{}open", fn_pfx)).unwrap();
        let fn_close_handler: Func<WalletClose> = lib.find_func(&format!("{}close", fn_pfx)).unwrap();
        let fn_delete_handler: Func<WalletDelete> = lib.find_func(&format!("{}delete", fn_pfx)).unwrap();
        let fn_add_record_handler: Func<WalletAddRecord> = lib.find_func(&format!("{}add_record", fn_pfx)).unwrap();
        let fn_update_record_value_handler: Func<WalletUpdateRecordValue> = lib.find_func(&format!("{}update_record_value", fn_pfx)).unwrap();
        let fn_update_record_tags_handler: Func<WalletUpdateRecordTags> = lib.find_func(&format!("{}update_record_tags", fn_pfx)).unwrap();
        let fn_add_record_tags_handler: Func<WalletAddRecordTags> = lib.find_func(&format!("{}add_record_tags", fn_pfx)).unwrap();
        let fn_delete_record_tags_handler: Func<WalletDeleteRecordTags> = lib.find_func(&format!("{}delete_record_tags", fn_pfx)).unwrap();
        let fn_delete_record_handler: Func<WalletDeleteRecord> = lib.find_func(&format!("{}delete_record", fn_pfx)).unwrap();
        let fn_get_record_handler: Func<WalletGetRecord> = lib.find_func(&format!("{}get_record", fn_pfx)).unwrap();
        let fn_get_record_id_handler: Func<WalletGetRecordId> = lib.find_func(&format!("{}get_record_id", fn_pfx)).unwrap();
        let fn_get_record_type_handler: Func<WalletGetRecordType> = lib.find_func(&format!("{}get_record_type", fn_pfx)).unwrap();
        let fn_get_record_value_handler: Func<WalletGetRecordValue> = lib.find_func(&format!("{}get_record_value", fn_pfx)).unwrap();
        let fn_get_record_tags_handler: Func<WalletGetRecordTags> = lib.find_func(&format!("{}get_record_tags", fn_pfx)).unwrap();
        let fn_free_record_handler: Func<WalletFreeRecord> = lib.find_func(&format!("{}free_record", fn_pfx)).unwrap();
        let fn_get_storage_metadata_handler: Func<WalletGetStorageMetadata> = lib.find_func(&format!("{}get_storage_metadata", fn_pfx)).unwrap();
        let fn_set_storage_metadata_handler: Func<WalletSetStorageMetadata> = lib.find_func(&format!("{}set_storage_metadata", fn_pfx)).unwrap();
        let fn_free_storage_metadata_handler: Func<WalletFreeStorageMetadata> = lib.find_func(&format!("{}free_storage_metadata", fn_pfx)).unwrap();
        let fn_search_records_handler: Func<WalletSearchRecords> = lib.find_func(&format!("{}search_records", fn_pfx)).unwrap();
        let fn_search_all_records_handler: Func<WalletSearchAllRecords> = lib.find_func(&format!("{}search_all_records", fn_pfx)).unwrap();
        let fn_get_search_total_count_handler: Func<WalletGetSearchTotalCount> = lib.find_func(&format!("{}get_search_total_count", fn_pfx)).unwrap();
        let fn_fetch_search_next_record_handler: Func<WalletFetchSearchNextRecord> = lib.find_func(&format!("{}fetch_search_next_record", fn_pfx)).unwrap();
        let fn_free_search_handler: Func<WalletFreeSearch> = lib.find_func(&format!("{}free_search", fn_pfx)).unwrap();

        println!("Register wallet ...");
        err = indy_register_wallet_storage(
            command_handle,
            xxtype.as_ptr(),
            Some(fn_create_handler.get()),
            Some(fn_open_handler.get()),
            Some(fn_close_handler.get()),
            Some(fn_delete_handler.get()),
            Some(fn_add_record_handler.get()),
            Some(fn_update_record_value_handler.get()),
            Some(fn_update_record_tags_handler.get()),
            Some(fn_add_record_tags_handler.get()),
            Some(fn_delete_record_tags_handler.get()),
            Some(fn_delete_record_handler.get()),
            Some(fn_get_record_handler.get()),
            Some(fn_get_record_id_handler.get()),
            Some(fn_get_record_type_handler.get()),
            Some(fn_get_record_value_handler.get()),
            Some(fn_get_record_tags_handler.get()),
            Some(fn_free_record_handler.get()),
            Some(fn_get_storage_metadata_handler.get()),
            Some(fn_set_storage_metadata_handler.get()),
            Some(fn_free_storage_metadata_handler.get()),
            Some(fn_search_records_handler.get()),
            Some(fn_search_all_records_handler.get()),
            Some(fn_get_search_total_count_handler.get()),
            Some(fn_fetch_search_next_record_handler.get()),
            Some(fn_free_search_handler.get()),
            cb
        );
    }

    println!("Finish up ...");
    wallets.insert(stg_type.to_string(), lib);

    super::results::result_to_empty(err, receiver)
}

/*
 * Dynamically loads the specified library and registers storage, based on provided config
 */
pub fn load_storage_library_config(storage_config: &HashMap<String, Option<String>>) -> Result<(), ErrorCode> {
    match storage_config.get("STG_LIB") {
        Some(slibrary) => match slibrary {
            Some(library) => {
                let stg_type: String = match storage_config.get("STG_TYPE") {
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
            None => Ok(())
        },
        None => Ok(())
    }
}

/*
 * Update the given configuration string based on supplied overrides
 */
pub fn override_wallet_configuration(config: &str, overrides: &HashMap<String, Option<String>>) -> String {
    let mut config: Config = serde_json::from_str(config).unwrap();

    match overrides.get("STG_TYPE") {
        Some(stype) => match stype {
            Some(wtype) => {
                config.storage_type = Some(wtype.clone());
            },
            None => ()
        },
        None => ()
    }
    match overrides.get("STG_CONFIG") {
        Some(sconfig) => match sconfig {
            Some(wconfig) => {
                let v: Value = serde_json::from_str(&wconfig[..]).unwrap();
                config.storage_config = Some(v.clone());
            },
            None => ()
        },
        None => ()
    }

    serde_json::to_string(&config).unwrap()
}

/*
 * Update the given credentials string based on supplied overrides
 */
pub fn override_wallet_credentials(creds: &str, overrides: &HashMap<String, Option<String>>) -> String {
    let mut creds: Credentials = serde_json::from_str(creds).unwrap();

    match overrides.get("STG_CREDS") {
        Some(screds) => match screds {
            Some(wcreds) => {
                let v: Value = serde_json::from_str(&wcreds[..]).unwrap();
                creds.storage_credentials = Some(v.clone());
            },
            None => ()
        },
        None => ()
    }

    serde_json::to_string(&creds).unwrap()
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

    storage_config
}

pub fn any_overrides(storage_config: &HashMap<String, Option<String>>) -> bool {
    for (_key, val) in storage_config {
        if let Some(_) = val {
            return true;
        }
    }
    return false;
}
