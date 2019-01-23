use futures::*;
use utils::futures::*;
use indyrs::{wallet, IndyError};

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