use futures::*;
use super::IndyError;
use utils::futures::*;
use indyrs::wallet::Wallet as wallet;

pub fn create_wallet(config: &str, credentials: &str) -> Box<Future<Item=(), Error=IndyError>> {
    wallet::create(config, credentials)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

pub fn open_wallet(config: &str, credentials: &str) -> Box<Future<Item=i32, Error=IndyError>> {
    wallet::open(config, credentials)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}

#[allow(unused)] // TODO: Use!
pub fn close_wallet(wallet_handle: i32) -> Box<Future<Item=(), Error=IndyError>> {
    wallet::close(wallet_handle)
        .map_err(|err| IndyError::from_err_code(err as i32))
        .into_box()
}