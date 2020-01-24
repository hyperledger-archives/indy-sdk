use futures::*;
use utils::futures::*;
use indyrs::{wallet, IndyError, WalletHandle};

pub fn create_wallet(config: &str, credentials: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    wallet::create_wallet(config, credentials)
        .into_box()
}

pub fn open_wallet(config: &str, credentials: &str) -> Box<dyn Future<Item=WalletHandle, Error=IndyError>> {
    wallet::open_wallet(config, credentials)
        .into_box()
}

#[allow(unused)] // TODO: Use!
pub fn close_wallet(wallet_handle: WalletHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    wallet::close_wallet(wallet_handle)
        .into_box()
}