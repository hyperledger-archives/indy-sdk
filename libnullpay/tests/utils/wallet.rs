use indy::IndyError;
use indy::wallet;
use indy::WalletHandle;
use indy::future::Future;

pub const DEFAULT_WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;

pub fn create_wallet(config: &str) -> Result<(), IndyError> {
    wallet::create_wallet(config, DEFAULT_WALLET_CREDENTIALS).wait()
}

pub fn open_wallet(config: &str) -> Result<WalletHandle, IndyError> {
    wallet::open_wallet(config, DEFAULT_WALLET_CREDENTIALS).wait()
}

pub fn create_and_open_wallet() -> Result<WalletHandle, IndyError> {
    let wallet_name = format!("default-wallet-name-{}", super::sequence::get_next_id());
    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    create_wallet(&config)?;
    open_wallet(&config)
}

pub fn close_wallet(wallet_handle: WalletHandle) -> Result<(), IndyError> {
    wallet::close_wallet(wallet_handle).wait()
}