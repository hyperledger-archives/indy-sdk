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
        Option<String>, // signature type
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    CreateAndStoreRevocation(
        i32, // wallet handle
        String, // issuer did
        i32, // claim def seq no
        i32, // max claim num
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    CreateClaim(
        i32, // wallet handle
        String, // claim req json
        String, // claim json
        i32, // revoc reg seq no
        i32, // user revoc index
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    RevokeClaim(
        i32, // wallet handle
        String, // issuer did
        i32, // claim def seq no
        i32, // revoc reg seq no
        i32, // user revoc index
        Box<Fn(Result<String, AnoncredsError>) + Send>),
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
                info!(target: "issuer_command_executor", "CreateAndStoreClaim command received");
                self.create_and_store_keys(wallet_handle, &issuer_did, &schema_json, signature_type.as_ref().map(String::as_str), cb);
            },
            IssuerCommand::CreateAndStoreRevocation(wallet_handle, issuer_did, claim_def_seq_no, max_claim_num, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistry command received");
                self.create_and_store_revocation(wallet_handle, &issuer_did, claim_def_seq_no, max_claim_num, cb);
            },
            IssuerCommand::CreateClaim(wallet_handle, claim_req_json, claim_json,
                                       revoc_reg_seq_no, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                self.create_and_store_claim(wallet_handle, &claim_req_json, &claim_json,
                                            revoc_reg_seq_no, user_revoc_index, cb);
            },
            IssuerCommand::RevokeClaim(wallet_handle, issuer_did, claim_def_seq_no, revoc_reg_seq_no,
                                       user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "RevokeClaim command received");
                self.revoke_claim(wallet_handle, &issuer_did, claim_def_seq_no, revoc_reg_seq_no, user_revoc_index, cb);
            }
        };
    }

    fn create_and_store_keys(&self,
                             wallet_handle: i32,
                             issuer_did: &str,
                             schema_json: &str,
                             signature_type: Option<&str>,
                             cb: Box<Fn(Result<(String, i32), AnoncredsError>) + Send>) {
        cb(Ok(("".to_string(), "".to_string())));
    }

    fn create_and_store_revocation(&self,
                                   wallet_handle: i32,
                                   issuer_did: &str,
                                   claim_def_seq_no: i32,
                                   max_claim_num: i32,
                                   cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        cb(Ok(("".to_string(), "".to_string())));
    }

    fn create_and_store_claim(&self,
                              wallet_handle: i32,
                              claim_req_json: &str,
                              claim_json: &str,
                              revoc_reg_seq_no: i32,
                              user_revoc_index: i32,
                              cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        cb(Ok(("".to_string(), "".to_string())));
    }

    fn revoke_claim(&self,
                    wallet_handle: i32,
                    issuer_did: &str,
                    claim_def_seq_no: i32,
                    revoc_reg_seq_no: i32,
                    user_revoc_index: i32,
                    cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        cb(Ok("".to_string()));
    }
}