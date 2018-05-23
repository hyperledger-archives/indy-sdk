use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use ffi::wallet;

pub struct Wallet {}

impl Wallet {
    pub fn create(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let pool_name = c_str!(pool_name);
        let wallet_name = c_str!(wallet_name);
        let xtype_str = opt_c_str!(xtype);
        let config_str = opt_c_str!(config);
        let credentials_str = opt_c_str!(credentials);

        let err = unsafe {
            wallet::indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               opt_c_ptr!(xtype, xtype_str),
                               opt_c_ptr!(config, config_str),
                               opt_c_ptr!(credentials, credentials_str),
                               cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn open(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let wallet_name = c_str!(wallet_name);
        let config_str = opt_c_str!(config);
        let credentials_str = opt_c_str!(credentials);

        let err = unsafe {
            wallet::indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             opt_c_ptr!(config, config_str),
                             opt_c_ptr!(credentials, credentials_str),
                             cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = unsafe { wallet::indy_list_wallets(command_handle, cb) };

        ResultHandler::one(err, receiver)
    }

    pub fn delete(wallet_name: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let wallet_name = c_str!(wallet_name);

        let err = unsafe {
            wallet::indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn close(wallet_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();


        let err = unsafe { wallet::indy_close_wallet(command_handle, wallet_handle, cb) };

        ResultHandler::empty(err, receiver)
    }
}
