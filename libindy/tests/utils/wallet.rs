use indy::api::ErrorCode;
use indy::api::wallet::*;

use utils::callback::CallbackUtils;
use utils::inmem_wallet::InmemWallet;
use utils::sequence::SequenceUtils;

use std::collections::HashSet;
use std::ffi::CString;
use std::ptr::null;
use std::sync::Mutex;
use utils::constants::DEFAULT_WALLET_CREDENTIALS;

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

    pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(wallet_name).unwrap();
        let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = CString::new(credentials.unwrap_or(DEFAULT_WALLET_CREDENTIALS)).unwrap();

        let err =
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               credentials_str.as_ptr(),
                               cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn open_wallet(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

        let wallet_name = CString::new(wallet_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = CString::new(credentials.unwrap_or(DEFAULT_WALLET_CREDENTIALS)).unwrap();

        let err =
            indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             credentials_str.as_ptr(),
                             cb);

        super::results::result_to_int(err, receiver)
    }

    pub fn create_and_open_wallet(pool_name: &str, xtype: Option<&str>) -> Result<i32, ErrorCode> {
        let wallet_name = format!("default-wallet-name-{}", SequenceUtils::get_next_id());

        WalletUtils::create_wallet(pool_name, &wallet_name, xtype, None, None)?;
        WalletUtils::open_wallet(&wallet_name, None, None)
    }

    pub fn create_and_open_plugged_wallet(pool_name: &str) -> Result<i32, ErrorCode> {
        let wallet_name = format!("default-wallet-name-{}", SequenceUtils::get_next_id());

        WalletUtils::register_wallet_storage("inmem", false).unwrap();
        WalletUtils::create_wallet(pool_name, &wallet_name, Some("inmem"), None, None).unwrap();
        WalletUtils::open_wallet(&wallet_name, None, None)
    }

    pub fn delete_wallet(wallet_name: &str, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let wallet_name = CString::new(wallet_name).unwrap();
        let credentials_str = CString::new(credentials.unwrap_or(DEFAULT_WALLET_CREDENTIALS)).unwrap();

        let err = indy_delete_wallet(command_handle, wallet_name.as_ptr(), credentials_str.as_ptr(), cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let err = indy_close_wallet(command_handle, wallet_handle, cb);

        super::results::result_to_empty(err, receiver)
    }
}