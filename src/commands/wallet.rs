use errors::wallet::WalletError;
use services::wallet::WalletService;
use services::wallet::{Wallet, AnoncredsWallet, IdentityWallet};
use std::rc::Rc;


pub enum WalletCommand {
    Set(Vec<String>, String, Box<Fn(Result<(), WalletError>) + Send>),
    Get(Vec<String>, Box<Fn(Result<Option<String>, WalletError>) + Send>),
    AnoncredsWalletCommand(AnoncredsWalletCommand),
    IdentityWalletCommand(IdentityWalletCommand)
}

pub enum AnoncredsWalletCommand {
    GetMasterSecret(String, String, String, Box<Fn(Result<Option<String>, WalletError>) + Send>)
}

pub enum IdentityWalletCommand {
    GetKeyByDid(String, Box<Fn(Result<Option<String>, WalletError>) + Send>)
}

pub struct WalletCommandExecutor {
    wallet_service: Rc<WalletService>
}

impl WalletCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
            wallet_service: wallet_service
        }
    }

    pub fn execute(&self, command: WalletCommand) {
        match command {
            WalletCommand::Set(keys, value, cb) => {
                info!(target: "wallet_command_executor", "Set command received");
                let result = self.set(&keys, &value);
                cb(result);
            },
            WalletCommand::Get(keys, cb) => {
                info!(target: "wallet_command_executor", "Get command received");
                let result = self.get(&keys);
                cb(result);
            },
            WalletCommand::AnoncredsWalletCommand(AnoncredsWalletCommand::GetMasterSecret(did, schema, pk, cb)) => {
                info!(target: "wallet_command_executor", "Get command received");
                let result = self.get_master_secret(&did, &schema, &pk);
                cb(result);
            },
            WalletCommand::IdentityWalletCommand(IdentityWalletCommand::GetKeyByDid(did, cb)) => {
                info!(target: "wallet_command_executor", "Get command received");
                let result = self.get_key_by_did(&did);
                cb(result);
            }
        };
    }

    fn set(&self, keys: &Vec<String>, value: &String) -> Result<(), WalletError> {
        let vector_links: Vec<&String> = keys.iter().map(|s| s).collect();

        self.wallet_service.set(vector_links.as_slice(), value)
    }

    fn get(&self, keys: &Vec<String>) -> Result<Option<String>, WalletError> {
        let vector_links: Vec<&String> = keys.iter().map(|s| s).collect();

        self.wallet_service.get(vector_links.as_slice())
    }

    fn get_master_secret(&self, did: &String, schema: &String, pk: &String) -> Result<Option<String>, WalletError> {
        self.wallet_service.get_master_secret(did, schema, pk)
    }

    fn get_key_by_did(&self, did: &String) -> Result<Option<String>, WalletError> {
        self.wallet_service.get_key_by_did(did)
    }
}