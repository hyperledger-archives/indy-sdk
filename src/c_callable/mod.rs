extern crate libc;

pub mod anoncreds;
pub mod crypto;
pub mod sovrin;
pub mod wallet;

use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};

use self::libc::{c_char, c_uchar};
use super::SovrinClient;

#[derive(Clone)]
pub struct SingletonClients {
    inner: Arc<Mutex<(HashMap<i32, SovrinClient>, i32)>>
}

pub fn get_active_clients() -> SingletonClients {
    static mut SINGLETON: *const SingletonClients = 0 as *const SingletonClients;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let singleton = SingletonClients {
                inner: Arc::new(Mutex::new((HashMap::new(), 1)))
            };
            SINGLETON = mem::transmute(Box::new(singleton));
        });
        (*SINGLETON).clone()
    }
}

#[no_mangle]
pub extern fn init_client(host_and_port: *const c_char) -> i32 {
    unimplemented!();
}

#[no_mangle]
pub extern fn release_client(client_id: i32) -> i32 {
    unimplemented!();
}

#[no_mangle]
pub extern fn free_str(c_ptr: *mut c_char) {
    unimplemented!();
}