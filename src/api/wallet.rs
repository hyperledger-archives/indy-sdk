use commands::{Command, CommandExecutor};
use commands::wallet::WalletCommand;
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

    /// Set or update Wallet record.
    ///
    /// #Params
    /// collection: Name of collection that identifies entity.
    /// key: First part of (key, subkey) pair that identifies entity.
    /// sub_key: Second part of (key, subkey) pair that identifies entity.
    /// value: Wallet record value to set or update.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No result
    ///
    /// #Errors
    /// No method specific errors.
    /// See `WallerError` docs for common errors description.
    pub fn set(&self, collection: &str, key: &str, sub_key: &str, value: &str, cb: Box<Fn(Result<(), WalletError>) + Send>) {
        unimplemented!();
    }

    /// Get Wallet record.
    ///
    /// #Params
    /// collection: Name of collection that identifies entity.
    /// key: First part of (key, subkey) pair that identifies entity.
    /// sub_key: Second part of (key, subkey) pair that identifies entity.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// None if no value was set for this keys
    /// Value of corresponded Wallet record otherwise.
    ///
    /// #Errors
    /// WalletError::NotFound - If no corresponded Wallet record found.
    /// See `WallerError` docs for common errors description.
    pub fn get(&self, collection: &str, key: &str, sub_key: &str, cb: Box<Fn(Result<Option<String>, WalletError>) + Send>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_api_can_be_created() {
        let wallet_api = WalletAPI::new(Arc::new(CommandExecutor::new()));
        assert! (true, "No crashes on WalletAPI::new");
    }

    #[test]
    fn wallet_api_can_be_dropped() {
        fn drop_test() {
            let wallet_api = WalletAPI::new(Arc::new(CommandExecutor::new()));
        }

        drop_test();
        assert! (true, "No crashes on WalletAPI::drop");
    }
}