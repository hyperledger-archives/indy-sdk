extern crate indy_crypto;

pub mod issuer;
pub mod prover;
pub mod verifier;

use commands::anoncreds::issuer::{IssuerCommand, IssuerCommandExecutor};
use commands::anoncreds::prover::{ProverCommand, ProverCommandExecutor};
use commands::anoncreds::verifier::{VerifierCommand, VerifierCommandExecutor};

use services::anoncreds::AnoncredsService;
use services::blob_storage::BlobStorageService;
use services::pool::PoolService;
use services::wallet::WalletService;

use self::indy_crypto::cl::{Tail, RevocationTailsAccessor};
use self::indy_crypto::errors::IndyCryptoError;

use std::rc::Rc;

pub enum AnoncredsCommand {
    Issuer(IssuerCommand),
    Prover(ProverCommand),
    Verifier(VerifierCommand),
}

pub struct AnoncredsCommandExecutor {
    issuer_command_cxecutor: IssuerCommandExecutor,
    prover_command_cxecutor: ProverCommandExecutor,
    verifier_command_cxecutor: VerifierCommandExecutor
}

impl AnoncredsCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               blob_storage_service: Rc<BlobStorageService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> AnoncredsCommandExecutor {
        AnoncredsCommandExecutor {
            issuer_command_cxecutor: IssuerCommandExecutor::new(
                anoncreds_service.clone(), pool_service.clone(), blob_storage_service.clone(), wallet_service.clone()),
            prover_command_cxecutor: ProverCommandExecutor::new(
                anoncreds_service.clone(), wallet_service.clone()),
            verifier_command_cxecutor: VerifierCommandExecutor::new(
                anoncreds_service.clone()),
        }
    }

    pub fn execute(&self, command: AnoncredsCommand) {
        match command {
            AnoncredsCommand::Issuer(cmd) => {
                info!(target: "anoncreds_command_executor", "Issuer command received");
                self.issuer_command_cxecutor.execute(cmd);
            }
            AnoncredsCommand::Prover(cmd) => {
                info!(target: "anoncreds_command_executor", "Prover command received");
                self.prover_command_cxecutor.execute(cmd);
            }
            AnoncredsCommand::Verifier(cmd) => {
                info!(target: "anoncreds_command_executor", "Verifier command received");
                self.verifier_command_cxecutor.execute(cmd);
            }
        };
    }
}

pub struct SDKTailsAccessor {
    tails_service: Rc<BlobStorageService>,
    tails_reader_handle: u32,
}

impl SDKTailsAccessor {
    pub fn new(tails_service: Rc<BlobStorageService>, tails_reader_handle: u32) -> SDKTailsAccessor {
        SDKTailsAccessor {
            tails_service,
            tails_reader_handle
        }
    }
}

impl RevocationTailsAccessor for SDKTailsAccessor {
    fn access_tail(&self, tail_id: u32, accessor: &mut FnMut(&Tail)) -> Result<(), IndyCryptoError> {
        let tail = self.tails_service.read(self.tails_reader_handle, tail_id as usize);
        accessor(&tail);
        Ok(())
    }
}
