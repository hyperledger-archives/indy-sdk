use errors::ledger::LedgerError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum LedgerCommand {
    PublishTx(
        i32, // pool handle
        String, // tx_json
        Box<Fn(Result<String, LedgerError>) + Send>),
    PublishTxAck(
        i32, // command handle
        Result<String, LedgerError>)
}

pub struct LedgerCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

}

impl LedgerCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: LedgerCommand) {
        match command {
            LedgerCommand::PublishTx(handle, tx_json, cb) => {
                info!(target: "ledger_command_executor", "PublishTx command received");
                self.publish_tx(handle, &tx_json, cb);
            },
            LedgerCommand::PublishTxAck(handle, result) => {
                info!(target: "ledger_command_executor", "PublishTxAck command received");
                self.publish_tx_ack(handle, result);
            }
        };
    }

    fn publish_tx(&self,
                  handle: i32,
                  tx_json: &str,
                  cb: Box<Fn(Result<String, LedgerError>) + Send>) {
        // TODO: FIXME: In real implementation publish_tx will save callback in context
        // TODO: FIXME: and send message to pool thread. Callback will be called on
        // TODO: FIXME: receiving of ack command.
        unimplemented!()
    }

    fn publish_tx_ack(&self, handle: i32, result: Result<String, LedgerError>) {
        // cb(result)
        unimplemented!()
    }
}