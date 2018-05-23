use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::ptr::null;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

use ffi::pool;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let pool_name = CString::new(pool_name).unwrap();
        let pool_config_str = pool_config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            pool::indy_create_pool_ledger_config(command_handle,
                                           pool_name.as_ptr(),
                                           if pool_config.is_some() { pool_config_str.as_ptr() } else { null() },
                                           cb)
        };

        ResultHandler::empty(err, receiver)
    }

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<IndyHandle, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let pool_name = CString::new(pool_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            pool::indy_open_pool_ledger(command_handle,
                                  pool_name.as_ptr(),
                                  if config.is_some() { config_str.as_ptr() } else { null() },
                                  cb)
        };

        ResultHandler::one(err, receiver)
    }

    #[allow(dead_code)] //TODO add refresh pool command or remove this code
    pub fn refresh(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = unsafe { pool::indy_refresh_pool_ledger(command_handle, pool_handle, cb) };

        ResultHandler::empty(err, receiver)
    }

    pub fn list() -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = unsafe { pool::indy_list_pools(command_handle, cb) };

        ResultHandler::one(err, receiver)
    }

    pub fn close(pool_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = unsafe { pool::indy_close_pool_ledger(command_handle, pool_handle, cb) };

        ResultHandler::empty(err, receiver)
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        let (receiver, cmd_id, cb) = ClosureHandler::cb_ec();

        let pool_name = CString::new(pool_name).unwrap();

        let err = unsafe { pool::indy_delete_pool_ledger_config(cmd_id, pool_name.as_ptr(), cb) };

        ResultHandler::empty(err, receiver)
    }
}

