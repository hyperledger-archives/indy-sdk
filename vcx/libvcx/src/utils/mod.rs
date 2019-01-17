#[macro_use]
pub mod ccallback;

#[macro_use]
pub mod cstring;

#[macro_use]
pub mod version_constants;

#[macro_use]
pub mod devsetup;

pub mod error;
pub mod httpclient;
pub mod constants;
pub mod timeout;
pub mod openssl;
pub mod json;
pub mod libindy;
pub mod threadpool;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::path::PathBuf;
use std::env;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}
// allows all threads to atomically get a unique command handle
pub fn generate_command_handle() -> i32 {
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
    command_handle
}

pub fn get_temp_dir_path(filename: Option<&str>) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(filename.unwrap_or(""));
    path
}

#[macro_use]
pub mod logger;
