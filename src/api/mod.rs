extern crate libc;

pub mod anoncreds;
pub mod ledger;
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

/// Refreshes a local copy of a pool ledger associated with the given genesis transactions file.
/// If there is not copy for this genesis transaction file, then a new one will be created.
/// The method should be called at the very first connection to a ledger, as well as when the ledger needs to be updated.
/// The method can not be called for another genesis transaction during the library run,
/// that is a library can work only with one pool ledger per run.
/// No other calls to ledger should be done until the method completes.
///
/// #Params
/// genesis_txn: (optional) a path to genesis transaction file. If NULL, then a default one will be created.
/// config: (optional) pool ledger configuration json.
///
/// #Returns
/// pool ledger handle
///
/// #Errors
/// PoolLedgerAlreadyInitializedError
/// PoolLedgerError
/// IOError
/// LedgerConsensusError
/// LedgerInvalidDataError
#[no_mangle]
pub extern fn refresh_pool_ledger(genesis_txn: *const c_char, config: *const c_char,
                                cb: extern fn(xcommand_handle: i32, err: i32) -> i32 {
    unimplemented!();
}

/// Creates a new wallet with a given name. The method is synchronous.
/// The wallet name must be unique.
///
/// #Params
/// wallet_name: a name for the new wallet (must be unique)
/// config: (optional) config; may contain a name for the external implementation of the wallet
///
/// #Returns
/// error code
///
/// #Errors
/// DuplicateNameError
/// IOError
#[no_mangle]
pub extern fn create_wallet(wallet_name: *const c_char, config: *const c_char) -> i32 {
    unimplemented!();
}

/// Creates a new session. A session is associated with a wallet and a pool ledger.
/// The call si synchronous.
///
/// #Params
/// wallet_name: a wallet's name (must be created by create_wallet)
/// config: (optional) session config
///
/// #Returns
/// session handle
///
/// #Errors
/// WalletNotFoundError
/// PoolNotInitializedError
#[no_mangle]
pub extern fn open_session(wallet_name (required), config (optional)) -> i32 {
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

/// Closes a session. The call is synchronous.
///
/// #Params
/// session_handle: session handler (created by open_session).
///
/// #Returns
/// error code
///
/// #Errors
#[no_mangle]
pub extern fn close_session(session_handle: i32) -> i32 {
    let s = get_active_clients();
    let ref mut clients: HashMap<i32, CommandExecutor> = (*s.inner.lock().unwrap()).0;

    if clients.contains_key(&client_id) {
        clients.remove(&client_id);
        0
    } else {
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn ledger_client_can_be_created() {
        let empty = CString::new("").unwrap();
        init_client(empty.as_ptr());
    }

    #[test]
    fn ledger_client_can_be_created_and_freed() {
        let empty = CString::new("").unwrap();
        let id = init_client(empty.as_ptr());
        let other_id = id + 1;
        assert_eq!(0, release_client(id));
        assert_eq!(-1, release_client(other_id));
        //TODO create more complex example: use different threads
    }

//        TODO: check memory consumption
//        #[test]
//        fn ledger_client_no_leak() {
//            let empty = CString::new("").unwrap();
//            for i in 1..1000000 {
//                let id = init_client(empty.as_ptr());
//                assert_eq!(0, release_client(id));
//            }
//        }
}
