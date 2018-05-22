use super::ErrorCode;

use std::ffi::CString;
use std::ptr::null;
use utils;
use indy::wallet;

pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(wallet_name).unwrap();
        let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            wallet::indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                               cb)
        };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn open_wallet(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_i32();

        let wallet_name = CString::new(wallet_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            wallet::indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                             cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn list_wallets() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let err = unsafe { wallet::indy_list_wallets(command_handle, cb) };

        utils::results::result_to_one(err, receiver)
    }

    pub fn delete_wallet(wallet_name: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let wallet_name = CString::new(wallet_name).unwrap();

        let err = unsafe {
            wallet::indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               cb)
        };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();


        let err = unsafe { wallet::indy_close_wallet(command_handle, wallet_handle, cb) };

        utils::results::result_to_empty(err, receiver)
    }
}
