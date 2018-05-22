use super::ErrorCode;

use std::ffi::CString;
use std::ptr::null;
use utils;
use indy::pool;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();
        let pool_config_str = CString::new(pool_config).unwrap();

        let err = unsafe {
            pool::indy_create_pool_ledger_config(command_handle,
                                           pool_name.as_ptr(),
                                           pool_config_str.as_ptr(),
                                           cb)
        };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_i32();

        let pool_name = CString::new(pool_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            pool::indy_open_pool_ledger(command_handle,
                                  pool_name.as_ptr(),
                                  if config.is_some() { config_str.as_ptr() } else { null() },
                                  cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    #[allow(dead_code)] //TODO add refresh pool command or remove this code
    pub fn refresh(pool_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let err = unsafe { pool::indy_refresh_pool_ledger(command_handle, pool_handle, cb) };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let err = unsafe { pool::indy_list_pools(command_handle, cb) };

        utils::results::result_to_one(err, receiver)
    }

    pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec();

        let err = unsafe { pool::indy_close_pool_ledger(command_handle, pool_handle, cb) };

        utils::results::result_to_empty(err, receiver)
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (receiver, cmd_id, cb) = utils::callbacks::_closure_to_cb_ec();

        let pool_name = CString::new(pool_name).unwrap();

        let err = unsafe { pool::indy_delete_pool_ledger_config(cmd_id, pool_name.as_ptr(), cb) };

        utils::results::result_to_empty(err, receiver)
    }
}

