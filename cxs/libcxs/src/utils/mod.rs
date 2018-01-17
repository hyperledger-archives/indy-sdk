#[macro_use]
pub mod ccallback;

#[macro_use]
pub mod cstring;

pub mod wallet;
pub mod error;
pub mod httpclient;
pub mod callback;
pub mod crypto;
pub mod signus;
pub mod constants;
pub mod timeout;
pub mod issuer_claim;
pub mod openssl;
pub mod json;
pub mod libindy;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}
// allows all threads to atomically get a unique command handle
pub fn generate_command_handle() -> i32 {
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
    command_handle
}

#[macro_use]
pub mod logger;
