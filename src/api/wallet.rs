use commands::{Command, CommandExecutor};
use errors::wallet::WalletError;

use std::sync::Arc;

pub struct WalletAPI {
    command_executor: Arc<CommandExecutor>,
}

impl WalletAPI {
    /// Constructs a new `WalletAPI`.
    ///
    /// #Params
    /// command_executor: Reference to `CommandExecutor` instance.
    ///
    pub fn new(command_executor: Arc<CommandExecutor>) -> WalletAPI {
        WalletAPI { command_executor: command_executor }
    }

    /// Set or update Wallet record identified by keys list.
    ///
    /// #Params
    /// keys: List of keys that identify Wallet record.
    /// value: Wallet record value to set or update.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No result
    ///
    /// #Errors
    /// No method specific errors.
    /// See `WallerError` docs for common errors description.
    pub fn set(keys: &[&str], value: &str, cb: Box<Fn(Result<(), WalletError>) + Send>) {
        unimplemented!();
    }

    /// Get Wallet record identified by keys list.
    ///
    /// #Params
    /// keys: List of keys that identify Wallet record.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Value of corresponded Wallet record.
    ///
    /// #Errors
    /// WalletError::NotFound - If no corresponded Wallet record found.
    /// See `WallerError` docs for common errors description.
    pub fn get(keys: &[&str], cb: Box<Fn(Result<String, WalletError>) + Send>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_api_can_be_created() {
        let sovrin_api = WalletAPI::new(Arc::new(CommandExecutor::new()));
        assert! (true, "No crashes on WalletAPI::new");
    }

    #[test]
    fn wallet_api_can_be_dropped() {
        fn drop_test() {
            let sovrin_api = WalletAPI::new(Arc::new(CommandExecutor::new()));
        }

        drop_test();
        assert! (true, "No crashes on WalletAPI::drop");
    }
}