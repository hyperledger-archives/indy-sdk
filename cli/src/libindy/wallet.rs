use indy::ErrorCode;
use indy::wallet::{Wallet as IndyWallet};
use indy::future::Future;

pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        IndyWallet::create(config, credentials).wait()
    }

    pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        IndyWallet::open(config, credentials).wait()
    }

    pub fn delete_wallet(wallet_name: &str, credentials: &str) -> Result<(), ErrorCode> {
        IndyWallet::delete(wallet_name, credentials).wait()
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        IndyWallet::close(wallet_handle).wait()
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        IndyWallet::export(wallet_handle, export_config_json).wait()
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config_json: &str) -> Result<(), ErrorCode> {
        IndyWallet::import(config, credentials, import_config_json).wait()
    }
}