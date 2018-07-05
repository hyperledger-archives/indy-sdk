use indy::api::ErrorCode;
use indy::api::wallet::*;

use utils::callback::CallbackUtils;
use utils::inmem_wallet::InmemWallet;
use utils::sequence::SequenceUtils;
use utils::environment::EnvironmentUtils;

use std::collections::HashSet;
use std::ffi::CString;
use std::sync::Mutex;
use utils::constants::{TYPE, INMEM_TYPE, WALLET_CREDENTIALS};

use std::path::{Path, PathBuf};

use serde_json;

pub struct WalletUtils {}

impl WalletUtils {
    pub fn register_wallet_storage(xtype: &str, force_create: bool) -> Result<(), ErrorCode> {
        lazy_static! {
            static ref REGISERED_WALLETS: Mutex<HashSet<String>> = Default::default();
        }

        let mut wallets = REGISERED_WALLETS.lock().unwrap();

        if wallets.contains(xtype) & !force_create {
            // as registering of plugged wallet with
            return Ok(());
        }

        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

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
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

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
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

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
            "id": format!("default-wallet_id-{}", SequenceUtils::get_next_id()),
            "storage_type": storage_type.unwrap_or(TYPE)
        }).to_string();

        WalletUtils::create_wallet(&config, WALLET_CREDENTIALS)?;
        WalletUtils::open_wallet(&config, WALLET_CREDENTIALS)
    }

    pub fn create_and_open_default_wallet() -> Result<i32, ErrorCode> {
        let config = json!({
            "id": format!("default-wallet_id-{}", SequenceUtils::get_next_id()),
            "storage_type": TYPE
        }).to_string();

        WalletUtils::create_wallet(&config, WALLET_CREDENTIALS)?;
        WalletUtils::open_wallet(&config, WALLET_CREDENTIALS)
    }

    pub fn create_and_open_plugged_wallet() -> Result<i32, ErrorCode> {
        let config = json!({
            "id": format!("default-wallet_id-{}", SequenceUtils::get_next_id()),
            "storage_type": INMEM_TYPE
        }).to_string();

        WalletUtils::register_wallet_storage("inmem", false).unwrap();
        WalletUtils::create_wallet(&config, WALLET_CREDENTIALS)?;
        WalletUtils::open_wallet(&config, WALLET_CREDENTIALS)
    }

    pub fn delete_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let config = CString::new(config).unwrap();
        let credentials = CString::new(credentials).unwrap();

        let err = indy_delete_wallet(command_handle, config.as_ptr(), credentials.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let err = indy_close_wallet(command_handle, wallet_handle, cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();
        let export_config_json = CString::new(export_config_json).unwrap();

        let err = indy_export_wallet(command_handle, wallet_handle, export_config_json.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

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
        EnvironmentUtils::tmp_file_path("export_file")
    }

    pub fn prepare_export_wallet_config(path: &Path) -> String {
        let json = json!({
            "path": path.to_str().unwrap(),
            "key": "export_key",
        });
        serde_json::to_string(&json).unwrap()
    }
}