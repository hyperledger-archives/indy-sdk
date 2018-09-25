pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod ledger;
pub mod pool;
pub mod did;
pub mod wallet;
pub mod pairwise;
pub mod non_secrets;
pub mod payments;

extern crate indy_crypto;
extern crate threadpool;

use self::threadpool::ThreadPool;

use commands::anoncreds::{AnoncredsCommand, AnoncredsCommandExecutor};
use commands::blob_storage::{BlobStorageCommand, BlobStorageCommandExecutor};
use commands::crypto::{CryptoCommand, CryptoCommandExecutor};
use commands::ledger::{LedgerCommand, LedgerCommandExecutor};
use commands::pool::{PoolCommand, PoolCommandExecutor};
use commands::did::{DidCommand, DidCommandExecutor};
use commands::wallet::{WalletCommand, WalletCommandExecutor};
use commands::pairwise::{PairwiseCommand, PairwiseCommandExecutor};
use commands::non_secrets::{NonSecretsCommand, NonSecretsCommandExecutor};
use commands::payments::{PaymentsCommand, PaymentsCommandExecutor};

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;

use services::anoncreds::AnoncredsService;
use services::blob_storage::BlobStorageService;
use services::payments::PaymentsService;
use services::pool::PoolService;
use services::wallet::{WalletService, KeyDerivationData};
use services::crypto::CryptoService;
use services::ledger::LedgerService;
use domain::anoncreds::credential_definition::CredentialDefinitionData;

use indy_crypto::cl::{CredentialPrivateKey, CredentialKeyCorrectnessProof};

use std::error::Error;
use std::sync::mpsc::{Sender, channel};
use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, MutexGuard};
use std::collections::HashSet;

pub enum Command {
    Exit,
    Anoncreds(AnoncredsCommand),
    BlobStorage(BlobStorageCommand),
    Crypto(CryptoCommand),
    Ledger(LedgerCommand),
    Pool(PoolCommand),
    Did(DidCommand),
    Wallet(WalletCommand),
    Pairwise(PairwiseCommand),
    NonSecrets(NonSecretsCommand),
    Payments(PaymentsCommand),
    DeriveKey(KeyDerivationData,
              Box<Fn(Result<::utils::crypto::chacha20poly1305_ietf::Key, CommonError>) + Send>),
    CreateCredentialDefinition(HashSet<String>,
                               bool,
                               Box<Fn(Result<(CredentialDefinitionData,
                                              CredentialPrivateKey,
                                              CredentialKeyCorrectnessProof), AnoncredsError>) + Send>)
}

pub struct CommandExecutor {
    worker: Option<thread::JoinHandle<()>>,
    sender: Sender<Command>,
}

// Global (lazy inited) instance of CommandExecutor
lazy_static! {
    static ref COMMAND_EXECUTOR: Mutex<CommandExecutor> = Mutex::new(CommandExecutor::new());
}

impl CommandExecutor {
    pub fn instance<'mutex>() -> MutexGuard<'mutex, CommandExecutor> {
        COMMAND_EXECUTOR.lock().unwrap()
    }

    fn new() -> CommandExecutor {
        ::utils::logger::init();
        let (sender, receiver) = channel();
        let threadpool = ThreadPool::new(4);

        CommandExecutor {
            sender,
            worker: Some(thread::spawn(move || {
                info!(target: "command_executor", "Worker thread started");

                let anoncreds_service = Rc::new(AnoncredsService::new());
                let blob_storage_service = Rc::new(BlobStorageService::new());
                let crypto_service = Rc::new(CryptoService::new());
                let ledger_service = Rc::new(LedgerService::new());
                let payments_service = Rc::new(PaymentsService::new());
                let pool_service = Rc::new(PoolService::new());
                let wallet_service = Rc::new(WalletService::new());

                let anoncreds_command_executor = AnoncredsCommandExecutor::new(anoncreds_service.clone(), blob_storage_service.clone(), pool_service.clone(), wallet_service.clone(), crypto_service.clone());
                let crypto_command_executor = CryptoCommandExecutor::new(wallet_service.clone(), crypto_service.clone());
                let ledger_command_executor = LedgerCommandExecutor::new(pool_service.clone(), crypto_service.clone(), wallet_service.clone(), ledger_service.clone());
                let pool_command_executor = PoolCommandExecutor::new(pool_service.clone());
                let did_command_executor = DidCommandExecutor::new(wallet_service.clone(), crypto_service.clone(), ledger_service.clone());
                let wallet_command_executor = WalletCommandExecutor::new(wallet_service.clone(), crypto_service.clone());
                let pairwise_command_executor = PairwiseCommandExecutor::new(wallet_service.clone());
                let blob_storage_command_executor = BlobStorageCommandExecutor::new(blob_storage_service.clone());
                let non_secret_command_executor = NonSecretsCommandExecutor::new(wallet_service.clone());
                let payments_command_executor = PaymentsCommandExecutor::new(payments_service.clone(), wallet_service.clone(), crypto_service.clone());

                loop {
                    match receiver.recv() {
                        Ok(Command::Anoncreds(cmd)) => {
                            info!("AnoncredsCommand command received");
                            anoncreds_command_executor.execute(cmd);
                        }
                        Ok(Command::BlobStorage(cmd)) => {
                            info!("BlobStorageCommand command received");
                            blob_storage_command_executor.execute(cmd);
                        }
                        Ok(Command::Crypto(cmd)) => {
                            info!("CryptoCommand command received");
                            crypto_command_executor.execute(cmd);
                        }
                        Ok(Command::Ledger(cmd)) => {
                            info!("LedgerCommand command received");
                            ledger_command_executor.execute(cmd);
                        }
                        Ok(Command::Pool(cmd)) => {
                            info!("PoolCommand command received");
                            pool_command_executor.execute(cmd);
                        }
                        Ok(Command::Did(cmd)) => {
                            info!("DidCommand command received");
                            did_command_executor.execute(cmd);
                        }
                        Ok(Command::Wallet(cmd)) => {
                            info!("WalletCommand command received");
                            wallet_command_executor.execute(cmd);
                        }
                        Ok(Command::Pairwise(cmd)) => {
                            info!("PairwiseCommand command received");
                            pairwise_command_executor.execute(cmd);
                        }
                        Ok(Command::NonSecrets(cmd)) => {
                            info!("NonSecretCommand command received");
                            non_secret_command_executor.execute(cmd);
                        }
                        Ok(Command::Payments(cmd)) => {
                            info!("PaymentsCommand command received");
                            payments_command_executor.execute(cmd);
                        }
                        Ok(Command::Exit) => {
                            info!("Exit command received");
                            break
                        }
                        Ok(Command::DeriveKey(key_data, cb)) => {
                            threadpool.execute(move || cb(key_data.calc_master_key()));
                        }
                        Ok(Command::CreateCredentialDefinition(attr_names, support_revocation, cb)) => {
                            threadpool.execute(move || cb(::services::anoncreds::issuer::Issuer::new_credential_definition(&attr_names, support_revocation)));
                        }
                        Err(err) => {
                            error!("Failed to get command!");
                            panic!("Failed to get command! {:?}", err)
                        }
                    }
                }
            }))
        }
    }

    pub fn send(&self, cmd: Command) -> Result<(), CommonError> {
        self.sender.send(cmd).map_err(|err|
            CommonError::InvalidState(err.description().to_string()))
    }
}

impl Drop for CommandExecutor {
    fn drop(&mut self) {
        info!(target: "command_executor", "Drop started");
        self.send(Command::Exit).unwrap();
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join().unwrap();
        info!(target: "command_executor", "Drop finished");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_executor_can_be_created() {
        let _command_executor = CommandExecutor::new();
        assert!(true, "No crashes on CommandExecutor::new");
    }

    #[test]
    fn command_executor_can_be_dropped() {
        fn drop_test() {
            let _command_executor = CommandExecutor::new();
        }

        drop_test();
        assert!(true, "No crashes on CommandExecutor::drop");
    }

    #[test]
    fn command_executor_can_get_instance() {
        let ref _command_executor: CommandExecutor = *CommandExecutor::instance();
        // Deadlock if another one instance will be requested (try to uncomment the next line)
        // let ref other_ce: CommandExecutor = *CommandExecutor::instance();
    }
}
