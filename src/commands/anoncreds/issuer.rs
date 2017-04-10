use errors::anoncreds::AnoncredsError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum IssuerCommand {
    CreateAndStoreKeys(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        String, // signature type
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>)
}

pub struct IssuerCommandExecutor {
    crypto_service: Rc<CryptoService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>
}

impl IssuerCommandExecutor {
    pub fn new(crypto_service: Rc<CryptoService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> IssuerCommandExecutor {
        IssuerCommandExecutor {
            crypto_service: crypto_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: IssuerCommand) {
        match command {
            IssuerCommand::CreateAndStoreKeys(wallet_handle, issuer_did, schema_json, signature_type, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreKeys command received");
                self.create_and_store_keys(wallet_handle, &issuer_did, &schema_json, &signature_type, cb);
            }
        };
    }

    fn create_and_store_keys(&self,
                             wallet_handle: i32,
                             issuer_did: &str,
                             schema_json: &str,
                             signature_type: &str,
                             cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        unimplemented!();
    }
}