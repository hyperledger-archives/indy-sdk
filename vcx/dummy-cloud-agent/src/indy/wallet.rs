extern crate libc;
extern crate libloading;

use futures::*;
use utils::futures::*;
use indyrs::{wallet, IndyError, ErrorCode};
use std::ffi::CString;
use self::libc::c_char;

pub fn create_wallet(config: &str, credentials: &str) -> Box<Future<Item=(), Error=IndyError>> {
    wallet::create_wallet(config, credentials)
        .into_box()
}

pub fn open_wallet(config: &str, credentials: &str) -> Box<Future<Item=i32, Error=IndyError>> {
    wallet::open_wallet(config, credentials)
        .into_box()
}

#[allow(unused)] // TODO: Use!
pub fn close_wallet(wallet_handle: i32) -> Box<Future<Item=(), Error=IndyError>> {
    wallet::close_wallet(wallet_handle)
        .into_box()
}



pub fn get_postgres_storage_plugin() -> String {
    let os = os_type::current_platform();
    let osfile = match os.os_type {
        os_type::OSType::OSX => "/usr/local/lib/libindystrgpostgres.dylib",
        _ => "/usr/lib/libindystrgpostgres.so"
    };
    return osfile.to_owned();
}

#[cfg(all(unix, test))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::os::unix::Library::open(Some(library), ::libc::RTLD_NOW | ::libc::RTLD_NODELETE)
        .map(libloading::Library::from)
}

#[cfg(any(not(unix), not(test)))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::Library::new(library)
}

pub fn load_storage_library(library: &str, initializer: &str, storage_config: &str, storage_credentials: &str){
    debug!("Loading storage plugin '{:}' as dynamic library.", library);
    let lib_res = _load_lib(library);
    match lib_res {
        Ok(lib) => {
            unsafe {
                debug!("Storage library '{:}' loaded. Resolving its init function '{:}'.", library, initializer);
                let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(initializer.as_bytes()).unwrap();

                match init_func() {
                    ErrorCode::Success => {}
                    _ => panic!("Failed to resolve init function '{:}' for storage library '{:}'.", initializer, library)
                }
                let init_storage_func: libloading::Symbol<unsafe extern fn(config: *const c_char, credentials: *const c_char) -> ErrorCode> = lib.get("init_storagetype".as_bytes()).unwrap();

                let init_config = CString::new(storage_config).expect("CString::new failed");
                let init_credentials = CString::new(storage_credentials).expect("CString::new failed");
                let err = init_storage_func(init_config.as_ptr(), init_credentials.as_ptr());
//
                if err != ErrorCode::Success {
                    panic!("Failed to initialize storage library '{:}' using init function '{:}'", library, initializer);
                }
                debug!("Successfully initialized storage library '{:}'.", library);
            }
        }
        Err(_) => panic!("Plugin {:} failed loading.", library)
    };
}