use errors::signus::SignusError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum SignusCommand {
    CreateAndStoreMyDid(
        i32, // wallet handle
        String, // did json
        Box<Fn(Result<(String, String, String), SignusError>) + Send>)
}

pub struct SignusCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

}

impl SignusCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> SignusCommandExecutor {
        SignusCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: SignusCommand) {
        match command {
            SignusCommand::CreateAndStoreMyDid(walled_handle, did_json, cb) => {
                info!(target: "signus_command_executor", "CreateAndStoreMyDid command received");
                self.create_and_store_my_did(walled_handle, &did_json, cb);
            }
        };
    }

    fn create_and_store_my_did(&self,
                               walled_handle: i32,
                               did_json: &str,
                               cb: Box<Fn(Result<(String, String, String), SignusError>) + Send>) {
        unimplemented!();
    }
}