mod issuer;

use commands::anoncreds::issuer::{IssuerCommand, IssuerCommandExecutor};

use errors::anoncreds::AnoncredsError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum AnoncredsCommand {
    Issuer(IssuerCommand)
}

pub struct AnoncredsCommandExecutor {
    issuer_command_cxecutor: IssuerCommandExecutor
}

impl AnoncredsCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> AnoncredsCommandExecutor {
        AnoncredsCommandExecutor {
            issuer_command_cxecutor: IssuerCommandExecutor::new(
                crypto_service, pool_service, wallet_service)
        }
    }

    pub fn execute(&self, command: AnoncredsCommand) {
        match command {
            AnoncredsCommand::Issuer(cmd) => {
                info!(target: "anoncreds_command_executor", "Issuer command received");
                self.issuer_command_cxecutor.execute(cmd);
            }
        };
    }
}