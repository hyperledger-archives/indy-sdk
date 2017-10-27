#[macro_use]
mod utils;

pub mod agent;
#[allow(unused_variables)] /* FIXME */
pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pool;
pub mod signus;
pub mod wallet;
pub mod pairwise;

use commands::agent::{AgentCommand, AgentCommandExecutor};
use commands::anoncreds::{AnoncredsCommand, AnoncredsCommandExecutor};
use commands::crypto::{CryptoCommand, CryptoCommandExecutor};
use commands::ledger::{LedgerCommand, LedgerCommandExecutor};
use commands::pool::{PoolCommand, PoolCommandExecutor};
use commands::signus::{SignusCommand, SignusCommandExecutor};
use commands::wallet::{WalletCommand, WalletCommandExecutor};
use commands::pairwise::{PairwiseCommand, PairwiseCommandExecutor};

use errors::common::CommonError;

use services::anoncreds::AnoncredsService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
use services::ledger::LedgerService;

use std::error::Error;
use std::sync::mpsc::{Sender, channel};
use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, MutexGuard};

pub enum Command {
    Exit,
    Agent(AgentCommand),
    Anoncreds(AnoncredsCommand),
    Crypto(CryptoCommand),
    Ledger(LedgerCommand),
    Pool(PoolCommand),
    Signus(SignusCommand),
    Wallet(WalletCommand),
    Pairwise(PairwiseCommand)
}

pub struct CommandExecutor {
    worker: Option<thread::JoinHandle<()>>,
    sender: Sender<Command>
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
        ::utils::logger::LoggerUtils::init();
        let (sender, receiver) = channel();

        CommandExecutor {
            sender: sender,
            worker: Some(thread::spawn(move || {
                info!(target: "command_executor", "Worker thread started");

                let anoncreds_service = Rc::new(AnoncredsService::new());
                let pool_service = Rc::new(PoolService::new());
                let wallet_service = Rc::new(WalletService::new());
                let signus_service = Rc::new(SignusService::new());
                let ledger_service = Rc::new(LedgerService::new());

                let agent_command_executor = AgentCommandExecutor::new(signus_service.clone(), wallet_service.clone());
                let anoncreds_command_executor = AnoncredsCommandExecutor::new(anoncreds_service.clone(), pool_service.clone(), wallet_service.clone());
                let crypto_command_executor = CryptoCommandExecutor::new(wallet_service.clone(), signus_service.clone());
                let ledger_command_executor = LedgerCommandExecutor::new(pool_service.clone(), signus_service.clone(), wallet_service.clone(), ledger_service.clone());
                let pool_command_executor = PoolCommandExecutor::new(pool_service.clone());
                let signus_command_executor = SignusCommandExecutor::new(pool_service.clone(), wallet_service.clone(), signus_service.clone(), ledger_service.clone());
                let wallet_command_executor = WalletCommandExecutor::new(wallet_service.clone());
                let pairwise_command_executor = PairwiseCommandExecutor::new(wallet_service.clone());

                loop {
                    match receiver.recv() {
                        Ok(Command::Agent(cmd)) => {
                            info!("AgentCommand command received");
                            agent_command_executor.execute(cmd);
                        }
                        Ok(Command::Anoncreds(cmd)) => {
                            info!("AnoncredsCommand command received");
                            anoncreds_command_executor.execute(cmd);
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
                        Ok(Command::Signus(cmd)) => {
                            info!("SignusCommand command received");
                            signus_command_executor.execute(cmd);
                        }
                        Ok(Command::Wallet(cmd)) => {
                            info!("WalletCommand command received");
                            wallet_command_executor.execute(cmd);
                        }
                        Ok(Command::Pairwise(cmd)) => {
                            info!("PairwiseCommand command received");
                            pairwise_command_executor.execute(cmd);
                        }
                        Ok(Command::Exit) => {
                            info!("Exit command received");
                            break
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
    #[allow(unused_variables)]
    fn command_executor_can_be_created() {
        let command_executor = CommandExecutor::new();
        assert!(true, "No crashes on CommandExecutor::new");
    }

    #[test]
    fn command_executor_can_be_dropped() {
        #[allow(unused_variables)]
        fn drop_test() {
            let command_executor = CommandExecutor::new();
        }

        drop_test();
        assert!(true, "No crashes on CommandExecutor::drop");
    }

    #[test]
    #[allow(unused_variables)]
    fn command_executor_can_get_instance() {
        let ref command_executor: CommandExecutor = *CommandExecutor::instance();
        // Deadlock if another one instance will be requested (try to uncomment the next line)
        // let ref other_ce: CommandExecutor = *CommandExecutor::instance();
    }
}
