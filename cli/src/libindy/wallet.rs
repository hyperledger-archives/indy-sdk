use indy::ErrorCode;
use indy::wallet;
use indy::future::Future;

pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        wallet::create_wallet(config, credentials).wait()
    }

    pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        wallet::open_wallet(config, credentials).wait()
    }

    pub fn delete_wallet(wallet_name: &str, credentials: &str) -> Result<(), ErrorCode> {
        wallet::delete_wallet(wallet_name, credentials).wait()
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        wallet::close_wallet(wallet_handle).wait()
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        wallet::export_wallet(wallet_handle, export_config_json).wait()
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config_json: &str) -> Result<(), ErrorCode> {
        wallet::import_wallet(config, credentials, import_config_json).wait()
    }
}