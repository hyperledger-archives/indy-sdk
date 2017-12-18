extern crate libc;
extern crate indy_crypto;

use super::{Wallet, WalletType};

use api::ErrorCode;
use errors::common::CommonError;
use errors::wallet::WalletError;

use self::libc::c_char;

use std::error::Error;
use std::ffi::{CString, CStr, NulError};
use std::ptr;
use std::str::Utf8Error;

use self::indy_crypto::utils::json::JsonDecodable;

#[derive(Debug, Deserialize)]
pub struct PluggedWalletJSONValue {
    pub key: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct PluggedWalletJSONValues {
    pub values: Vec<PluggedWalletJSONValue>
}

impl<'a> JsonDecodable<'a> for PluggedWalletJSONValues {}

struct PluggedWallet {
    name: String,
    pool_name: String,
    handle: i32,
    set_handler: extern fn(handle: i32,
                           key: *const c_char,
                           value: *const c_char) -> ErrorCode,
    get_handler: extern fn(handle: i32,
                           key: *const c_char,
                           value_ptr: *mut *const c_char) -> ErrorCode,
    get_not_expired_handler: extern fn(handle: i32,
                                       key: *const c_char,
                                       value_ptr: *mut *const c_char) -> ErrorCode,
    list_handler: extern fn(handle: i32,
                            key_prefix: *const c_char,
                            values_json_ptr: *mut *const c_char) -> ErrorCode,
    close_handler: extern fn(handle: i32) -> ErrorCode,
    free_handler: extern fn(handle: i32,
                            value: *const c_char) -> ErrorCode
}

impl PluggedWallet {
    fn new(name: &str,
           pool_name: &str,
           handle: i32,
           set_handler: extern fn(xhandle: i32,
                                  key: *const c_char,
                                  value: *const c_char) -> ErrorCode,
           get_handler: extern fn(xhandle: i32,
                                  key: *const c_char,
                                  value_ptr: *mut *const c_char) -> ErrorCode,
           get_not_expired_handler: extern fn(xhandle: i32,
                                              key: *const c_char,
                                              value_ptr: *mut *const c_char) -> ErrorCode,
           list_handler: extern fn(xhandle: i32,
                                   key_prefix: *const c_char,
                                   values_json_ptr: *mut *const c_char) -> ErrorCode,
           close_handler: extern fn(xhandle: i32) -> ErrorCode,
           free_handler: extern fn(xhandle: i32,
                                   value: *const c_char) -> ErrorCode) -> PluggedWallet {
        PluggedWallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            handle: handle,
            set_handler: set_handler,
            get_handler: get_handler,
            list_handler: list_handler,
            get_not_expired_handler: get_not_expired_handler,
            close_handler: close_handler,
            free_handler: free_handler
        }
    }
}

impl Wallet for PluggedWallet {
    fn set(&self, key: &str, value: &str) -> Result<(), WalletError> {
        let key = CString::new(key)?;
        let value = CString::new(value)?;

        let err = (self.set_handler)(self.handle,
                                     key.as_ptr(),
                                     value.as_ptr());

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn get(&self, key: &str) -> Result<String, WalletError> {
        let key = CString::new(key)?;
        let mut value_ptr: *const c_char = ptr::null_mut();

        let err = (self.get_handler)(self.handle,
                                     key.as_ptr(),
                                     &mut value_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let result = unsafe {
            CStr::from_ptr(value_ptr).to_str()?.to_string()
        };

        let err = (self.free_handler)(self.handle, value_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(result)
    }

    fn list(&self, key_prefix: &str) -> Result<Vec<(String, String)>, WalletError> {
        let key_prefix = CString::new(key_prefix)?;
        let mut values_json_ptr: *const c_char = ptr::null_mut();

        let err = (self.list_handler)(self.handle,
                                      key_prefix.as_ptr(),
                                      &mut values_json_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let values_json = unsafe {
            CStr::from_ptr(values_json_ptr).to_str()?.to_string()
        };

        let err = (self.free_handler)(self.handle, values_json_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }
        
        let result = PluggedWalletJSONValues::from_json(values_json.as_str()).map_err(map_err_trace!())?
            .values
            .iter()
            .map(|value| (value.key.clone(), value.value.clone()))
            .collect();

        Ok(result)
    }

    fn get_not_expired(&self, key: &str) -> Result<String, WalletError> {
        let key = CString::new(key)?;
        let mut value_ptr: *const c_char = ptr::null_mut();

        let err = (self.get_not_expired_handler)(self.handle,
                                                 key.as_ptr(),
                                                 &mut value_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        let result = unsafe {
            CStr::from_ptr(value_ptr).to_str()?.to_string()
        };

        let err = (self.free_handler)(self.handle, value_ptr);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(result)
    }

    fn close(&self) -> Result<(), WalletError> {
        let err = (self.close_handler)(self.handle);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct PluggedWalletType {
    create_handler: extern fn(name: *const c_char,
                              config: *const c_char,
                              credentials: *const c_char) -> ErrorCode,
    open_handler: extern fn(name: *const c_char,
                            config: *const c_char,
                            runtime_config: *const c_char,
                            credentials: *const c_char,
                            handle: *mut i32) -> ErrorCode,
    set_handler: extern fn(handle: i32,
                           key: *const c_char,
                           value: *const c_char) -> ErrorCode,
    get_handler: extern fn(handle: i32,
                           key: *const c_char,
                           value_ptr: *mut *const c_char) -> ErrorCode,
    get_not_expired_handler: extern fn(handle: i32,
                                       key: *const c_char,
                                       value_ptr: *mut *const c_char) -> ErrorCode,
    list_handler: extern fn(handle: i32,
                            key_prefix: *const c_char,
                            values_json_ptr: *mut *const c_char) -> ErrorCode,
    close_handler: extern fn(handle: i32) -> ErrorCode,
    delete_handler: extern fn(name: *const c_char,
                              config: *const c_char,
                              credentials: *const c_char) -> ErrorCode,
    free_handler: extern fn(xhandle: i32,
                            value: *const c_char) -> ErrorCode,
}

impl PluggedWalletType {
    pub fn new(create_handler: extern fn(name: *const c_char,
                                         config: *const c_char,
                                         credentials: *const c_char) -> ErrorCode,
               open_handler: extern fn(name: *const c_char,
                                       config: *const c_char,
                                       runtime_config: *const c_char,
                                       credentials: *const c_char,
                                       handle: *mut i32) -> ErrorCode,
               set_handler: extern fn(handle: i32,
                                      key: *const c_char,
                                      value: *const c_char) -> ErrorCode,
               get_handler: extern fn(handle: i32,
                                      key: *const c_char,
                                      value_ptr: *mut *const c_char) -> ErrorCode,
               get_not_expired_handler: extern fn(handle: i32,
                                                  key: *const c_char,
                                                  value_ptr: *mut *const c_char) -> ErrorCode,
               list_handler: extern fn(handle: i32,
                                       key_prefix: *const c_char,
                                       values_json_ptr: *mut *const c_char) -> ErrorCode,
               close_handler: extern fn(handle: i32) -> ErrorCode,
               delete_handler: extern fn(name: *const c_char,
                                         config: *const c_char,
                                         credentials: *const c_char) -> ErrorCode,
               free_handler: extern fn(xhandle: i32,
                                       value: *const c_char) -> ErrorCode) -> PluggedWalletType {
        PluggedWalletType {
            create_handler: create_handler,
            open_handler: open_handler,
            set_handler: set_handler,
            get_handler: get_handler,
            get_not_expired_handler: get_not_expired_handler,
            list_handler: list_handler,
            close_handler: close_handler,
            delete_handler: delete_handler,
            free_handler: free_handler
        }
    }
}

impl WalletType for PluggedWalletType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError> {
        let name = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.create_handler)(name.as_ptr(),
                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                        credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn delete(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError> {
        let name = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.delete_handler)(name.as_ptr(),
                                        config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                        credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()));

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(())
    }

    fn open(&self, name: &str, pool_name: &str, config: Option<&str>, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError> {
        let mut handle: i32 = 0;
        let cname = CString::new(name)?;

        let config = match config {
            Some(config) => Some(CString::new(config)?),
            None => None
        };

        let runtime_config = match runtime_config {
            Some(runtime_config) => Some(CString::new(runtime_config)?),
            None => None
        };

        let credentials = match credentials {
            Some(credentials) => Some(CString::new(credentials)?),
            None => None
        };

        let err = (self.open_handler)(cname.as_ptr(),
                                      config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      runtime_config.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      credentials.as_ref().map_or(ptr::null(), |x| x.as_ptr()),
                                      &mut handle);

        if err != ErrorCode::Success {
            return Err(WalletError::PluggedWallerError(err));
        }

        Ok(Box::new(
            PluggedWallet::new(
                name,
                pool_name,
                handle,
                self.set_handler,
                self.get_handler,
                self.get_not_expired_handler,
                self.list_handler,
                self.close_handler,
                self.free_handler)))
    }
}


impl From<NulError> for WalletError {
    fn from(err: NulError) -> WalletError {
        WalletError::CommonError(CommonError::InvalidState(format!("Null symbols in wallet keys or values: {}", err.description())))
    }
}

impl From<Utf8Error> for WalletError {
    fn from(err: Utf8Error) -> WalletError {
        WalletError::CommonError(CommonError::InvalidState(format!("Incorrect utf8 symbols in wallet keys or values: {}", err.description())))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;

    use std::time::Duration;
    use std::thread;

    #[test]
    fn plugged_wallet_type_new_works() {
        InmemWallet::cleanup();

        PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        InmemWallet::cleanup();
    }


    #[test]
    fn plugged_wallet_type_create_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        wallet_type.create("wallet1", None, None).unwrap();

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_type_create_works_for_twice() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        wallet_type.create("wallet1", None, None).unwrap();

        let res = wallet_type.create("wallet1", None, None);
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::CommonInvalidState)), res);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_type_delete_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.delete("wallet1", None, None).unwrap();
        wallet_type.create("wallet1", None, None).unwrap();

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_type_open_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_set_get_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );

        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        wallet.set("key1", "value1").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_set_get_works_for_reopen() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();

        {
            let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
            wallet.set("key1", "value1").unwrap();
        }

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_get_works_for_unknown() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        let value = wallet.get("key1");
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_set_get_works_for_update() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        wallet.set("key1", "value1").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        wallet.set("key1", "value2").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value2", value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_set_get_not_expired_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, Some("{\"freshness_time\": 1}"), None).unwrap();
        wallet.set("key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(2, 0));

        let value = wallet.get_not_expired("key1");
        assert_match!(Err(WalletError::PluggedWallerError(ErrorCode::WalletNotFoundError)), value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_list_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        wallet.set("key1::subkey1", "value1").unwrap();
        wallet.set("key1::subkey2", "value2").unwrap();

        let mut key_values = wallet.list("key1::").unwrap();
        key_values.sort();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_get_pool_name_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        assert_eq!(wallet.get_pool_name(), "pool1");

        InmemWallet::cleanup();
    }

    #[test]
    fn plugged_wallet_get_name_works() {
        InmemWallet::cleanup();

        let wallet_type = PluggedWalletType::new(
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::set,
            InmemWallet::get,
            InmemWallet::get_not_expired,
            InmemWallet::list,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::free
        );
        wallet_type.create("wallet1", None, None).unwrap();

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        assert_eq!(wallet.get_name(), "wallet1");

        InmemWallet::cleanup();
    }
}