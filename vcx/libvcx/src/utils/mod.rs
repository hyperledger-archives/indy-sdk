#[macro_use]
mod ccallback;

#[macro_use]
pub mod cstring;

#[macro_use]
pub mod version_constants;

#[macro_use]
#[cfg(test)]
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

#[cfg(test)]
macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

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
pub mod qualifier;
pub mod file;
pub mod option_util;
pub mod agent_info;

#[cfg(test)]
pub mod plugins;

#[macro_use]
pub mod logger;

use std::path::PathBuf;
use std::env;

pub fn get_temp_dir_path(filename: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(filename);
    path
}
