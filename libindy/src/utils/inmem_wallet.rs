extern crate libc;
extern crate time;
extern crate indy_crypto;

use api::ErrorCode;
use utils::cstring::CStringUtils;
use utils::sequence::SequenceUtils;

use self::libc::c_char;
use self::time::Timespec;

use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;
use std::ops::Sub;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

#[derive(Deserialize)]
struct InmemWalletRuntimeConfig {
    freshness_time: i64
}

impl<'a> JsonDecodable<'a> for InmemWalletRuntimeConfig {}

impl Default for InmemWalletRuntimeConfig {
    fn default() -> Self {
        InmemWalletRuntimeConfig { freshness_time: 1000 }
    }
}

#[derive(Debug)]
struct InmemWalletContext {
    name: String,
    freshness_time: i64
}

#[derive(Debug)]
struct InmemWalletRecord {
    key: String,
    value: String,
    time_created: Timespec
}

#[derive(Debug, Serialize)]
pub struct InmemWalletJSONValue {
    pub key: String,
    pub value: String
}

#[derive(Debug, Serialize)]
pub struct InmemWalletJSONValues {
    pub values: Vec<InmemWalletJSONValue>
}

impl JsonEncodable for InmemWalletJSONValues {}

lazy_static! {
    static ref INMEM_WALLETS: Mutex<HashMap<String, HashMap<String, InmemWalletRecord>>> = Default::default();
}

lazy_static! {
    static ref INMEM_WALLET_HANDLES: Mutex<HashMap<i32, InmemWalletContext>> = Default::default();
}

pub struct InmemWallet {}

impl InmemWallet {
    pub extern "C" fn create(name: *const c_char,
                             _: *const c_char,
                             _: *const c_char) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if wallets.contains_key(&name) {
            // Invalid state as "already exists" case must be checked on service layer
            return ErrorCode::CommonInvalidState;
        }
        wallets.insert(name.clone(), HashMap::new());
        ErrorCode::Success
    }

    pub extern "C" fn open(name: *const c_char,
                           _: *const c_char,
                           runtime_config: *const c_char,
                           _: *const c_char,
                           handle: *mut i32) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);
        check_useful_opt_c_str!(runtime_config, ErrorCode::CommonInvalidStructure);

        let runtime_config = match runtime_config {
            Some(config) => InmemWalletRuntimeConfig::from_json(config.as_str()).unwrap(), // FIXME: parse error!!!
            None => InmemWalletRuntimeConfig::default()
        };

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&name) {
            return ErrorCode::CommonInvalidState;
        }

        let mut handles = INMEM_WALLET_HANDLES.lock().unwrap();
        let xhandle = SequenceUtils::get_next_id();
        handles.insert(xhandle, InmemWalletContext {
            name: name,
            freshness_time: runtime_config.freshness_time
        });

        unsafe { *handle = xhandle };
        ErrorCode::Success
    }

    pub extern "C" fn set(xhandle: i32,
                          key: *const c_char,
                          value: *const c_char) -> ErrorCode {
        check_useful_c_str!(key, ErrorCode::CommonInvalidStructure);
        check_useful_c_str!(value, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_WALLET_HANDLES.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get_mut(&wallet_context.name).unwrap();

        wallet.insert(key.clone(), InmemWalletRecord {
            key: key,
            value: value,
            time_created: time::get_time()
        });
        ErrorCode::Success
    }

    pub extern "C" fn get(xhandle: i32,
                          key: *const c_char,
                          value_ptr: *mut *const c_char) -> ErrorCode {
        check_useful_c_str!(key, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_WALLET_HANDLES.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

        if !wallet.contains_key(&key) {
            return ErrorCode::WalletNotFoundError;
        }

        let ref value = wallet.get(&key).unwrap().value;
        unsafe { *value_ptr = CString::new(value.as_str()).unwrap().into_raw(); }
        ErrorCode::Success
    }

    pub extern "C" fn get_not_expired(xhandle: i32,
                                      key: *const c_char,
                                      value_ptr: *mut *const c_char) -> ErrorCode {
        check_useful_c_str!(key, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_WALLET_HANDLES.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();

        if !wallet.contains_key(&key) {
            return ErrorCode::WalletNotFoundError;
        }

        let ref record = wallet.get(&key).unwrap();

        if time::get_time().sub(record.time_created).num_seconds() > wallet_context.freshness_time {
            return ErrorCode::WalletNotFoundError;
        }

        unsafe { *value_ptr = CString::new(record.value.as_str()).unwrap().into_raw(); }
        ErrorCode::Success
    }

    pub extern "C" fn list(xhandle: i32,
                           key_prefix: *const c_char,
                           values_json_ptr: *mut *const c_char) -> ErrorCode {
        check_useful_c_str!(key_prefix, ErrorCode::CommonInvalidStructure);

        let handles = INMEM_WALLET_HANDLES.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet_context = handles.get(&xhandle).unwrap();

        let wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&wallet_context.name) {
            return ErrorCode::CommonInvalidState;
        }

        let wallet = wallets.get(&wallet_context.name).unwrap();


        let values = InmemWalletJSONValues {
            values: wallet
                .keys()
                .filter(|&ref key| key.starts_with(&key_prefix))
                .map(|&ref key| InmemWalletJSONValue {
                    key: key.clone(),
                    value: wallet.get(key).unwrap().value.clone()
                })
                .collect()
        }
            .to_json()
            .unwrap();

        unsafe { *values_json_ptr = CString::new(values.as_str()).unwrap().into_raw(); }
        ErrorCode::Success
    }

    pub extern "C" fn close(xhandle: i32) -> ErrorCode {
        let mut handles = INMEM_WALLET_HANDLES.lock().unwrap();

        if !handles.contains_key(&xhandle) {
            return ErrorCode::CommonInvalidState;
        }

        handles.remove(&xhandle);
        ErrorCode::Success
    }

    pub extern "C" fn delete(name: *const c_char,
                             _: *const c_char,
                             _: *const c_char) -> ErrorCode {
        check_useful_c_str!(name, ErrorCode::CommonInvalidStructure);

        let mut wallets = INMEM_WALLETS.lock().unwrap();

        if !wallets.contains_key(&name) {
            return ErrorCode::CommonInvalidState;
        }

        wallets.remove(&name);
        ErrorCode::Success
    }

    pub extern "C" fn free(_: i32,
                           value: *const c_char) -> ErrorCode {
        unsafe { CString::from_raw(value as *mut c_char); }
        ErrorCode::Success
    }

    pub fn cleanup() {
        let mut wallets = INMEM_WALLETS.lock().unwrap();
        wallets.clear();

        let mut handles = INMEM_WALLET_HANDLES.lock().unwrap();
        handles.clear();
    }
}
