#[macro_use]
pub mod ccallback;

#[macro_use]
pub mod cstring;

#[macro_use]
pub mod version_constants;

#[macro_use]
pub mod devsetup;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ $val }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ "_" }};
}

pub mod error;
pub mod httpclient;
pub mod constants;
pub mod timeout;
pub mod openssl;
pub mod json;
pub mod libindy;
pub mod threadpool;
pub mod uuid;
pub mod author_agreement;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::PathBuf;
use std::env;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = AtomicUsize::new(1);
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
