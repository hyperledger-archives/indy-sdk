use indy::IndyError;
use indy::wallet;
use indy::future::Future;

pub const DEFAULT_WALLET_CREDENTIALS: &'static str = r#"{"key":"key"}"#;

pub fn create_wallet(config: &str) -> Result<(), IndyError> {
    wallet::create_wallet(config, DEFAULT_WALLET_CREDENTIALS).wait()
}

pub fn open_wallet(config: &str) -> Result<i32, IndyError> {
    wallet::open_wallet(config, DEFAULT_WALLET_CREDENTIALS).wait()
}

pub fn create_and_open_wallet() -> Result<i32, IndyError> {
    let wallet_name = format!("default-wallet-name-{}", super::sequence::get_next_id());
    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    create_wallet(&config)?;
    open_wallet(&config)
}

pub fn close_wallet(wallet_handle: i32) -> Result<(), IndyError> {
    wallet::close_wallet(wallet_handle).wait()
}