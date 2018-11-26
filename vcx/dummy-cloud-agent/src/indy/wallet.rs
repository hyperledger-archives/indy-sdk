use futures::*;
use utils::futures::*;
use indyrs::{wallet, ErrorCode};

pub fn create_wallet(config: &str, credentials: &str) -> Box<Future<Item=(), Error=ErrorCode>> {
    wallet::create_wallet(config, credentials)
        .into_box()
}

pub fn open_wallet(config: &str, credentials: &str) -> Box<Future<Item=i32, Error=ErrorCode>> {
    wallet::open_wallet(config, credentials)
        .into_box()
}

#[allow(unused)] // TODO: Use!
pub fn close_wallet(wallet_handle: i32) -> Box<Future<Item=(), Error=ErrorCode>> {
    wallet::close_wallet(wallet_handle)
        .into_box()
}