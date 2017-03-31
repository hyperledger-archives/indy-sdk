extern crate libc;

pub mod anoncreds;
pub mod sovrin;
pub mod wallet;

use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};

use self::libc::{c_char, c_uchar};
use commands::CommandExecutor;

#[derive(Clone)]
pub struct SingletonClients {
    inner: Arc<Mutex<(HashMap<i32, CommandExecutor>, i32)>>
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
    let s = get_active_clients();
    let (ref mut clients, mut cl_id): (HashMap<i32, CommandExecutor>, i32) = *s.inner.lock().unwrap();

    while clients.contains_key(&cl_id) {
        cl_id += 1;
        if cl_id < 0 {
            cl_id = 1;
        }
    }

    clients.insert(cl_id, CommandExecutor::new());

    cl_id
}

#[no_mangle]
pub extern fn release_client(client_id: i32) -> i32 {
    let s = get_active_clients();
    let ref mut clients: HashMap<i32, CommandExecutor> = (*s.inner.lock().unwrap()).0;

    if clients.contains_key(&client_id) {
        clients.remove(&client_id);
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern fn free_str(c_ptr: *mut c_char) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn sovrin_client_can_be_created() {
        let empty = CString::new("").unwrap();
        init_client(empty.as_ptr());
    }

    #[test]
    fn sovrin_client_can_be_created_and_freed() {
        let empty = CString::new("").unwrap();
        let id = init_client(empty.as_ptr());
        let other_id = id + 1;
        assert_eq!(0, release_client(id));
        assert_eq!(-1, release_client(other_id));
        //TODO create more complex example: use different threads
    }

//        TODO: check memory consumption
//        #[test]
//        fn sovrin_client_no_leak() {
//            let empty = CString::new("").unwrap();
//            for i in 1..1000000 {
//                let id = init_client(empty.as_ptr());
//                assert_eq!(0, release_client(id));
//            }
//        }
}
