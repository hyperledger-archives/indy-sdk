extern crate serde_json;

use errors::anoncreds::AnoncredsError;
use errors::crypto::CryptoError;
use errors::wallet::WalletError;

use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::anoncreds::types::{
    Accumulator,
    AccumulatorPublicKey,
    AccumulatorSecretKey,
    PublicKey,
    RevocationPublicKey,
    RevocationSecretKey,
    Schema,
    SecretKey
};
use services::crypto::wrappers::pair::{PointG1};
use std::cell::RefCell;
use std::collections::HashMap;
use types::claim_definition::ClaimDefinition;
use utils::json::{JsonEncodable, JsonDecodable};

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
        String, // issuer did
        i32, // claim def seq no
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
    pub crypto_service: Rc<CryptoService>,
    pub pool_service: Rc<PoolService>,
    pub wallet_service: Rc<WalletService>
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
            IssuerCommand::CreateClaim(wallet_handle, claim_req_json, claim_json, issuer_did,
                                       claim_def_seq_no, revoc_reg_seq_no, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                self.create_and_store_claim(wallet_handle, &claim_req_json, &claim_json, &issuer_did,
                                            claim_def_seq_no, revoc_reg_seq_no, user_revoc_index, cb);
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
                             cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let schema: Schema = Schema::decode(schema_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let ((pk, sk), (pkr, skr)) =
                        self.crypto_service.anoncreds.issuer.generate_keys(schema)
                            .map_err(|err| AnoncredsError::CryptoError(CryptoError::BackendError(err.to_string())))?;

                    let pk_json = PublicKey::encode(&pk)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let sk_json = SecretKey::encode(&sk)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let pkr_json = RevocationPublicKey::encode(&pkr)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let skr_json = RevocationSecretKey::encode(&skr)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    wallet.set(&format!("public_key {}", &issuer_did), &pk_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("secret_key {}", &issuer_did), &sk_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("public_key_revocation {}", &issuer_did), &pkr_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("secret_key_revocation {}", &issuer_did), &skr_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    let claim_def = ClaimDefinition {
                        public_key: pk_json,
                        schema: schema_json.to_string(),
                        signature_type: signature_type.unwrap_or("CL").to_string()
                    };

                    let claim_def_json = ClaimDefinition::encode(&claim_def)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    Ok((claim_def_json, "".to_string())) //TODO unique ID
                });

        match result {
            Ok((claim_def_json, id)) => cb(Ok((claim_def_json, id))),
            Err(err) => cb(Err(err))
        }
    }

    fn create_and_store_revocation(&self,
                                   wallet_handle: i32,
                                   issuer_did: &str,
                                   claim_def_seq_no: i32,
                                   max_claim_num: i32,
                                   cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let pkr_json = wallet.get(&format!("public_key_revocation {}", &issuer_did))
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    let pkr = RevocationPublicKey::decode(&pkr_json)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let (acc, tails, acc_pk, acc_sk) =
                        self.crypto_service.anoncreds.issuer.issue_accumulator(&pkr, claim_def_seq_no, max_claim_num)
                            .map_err(|err| AnoncredsError::CryptoError(CryptoError::BackendError(err.to_string())))?;

                    let acc_json = Accumulator::encode(&acc)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let tails_json = serde_json::to_string(&tails)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let acc_pk_json = AccumulatorPublicKey::encode(&acc_pk)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;
                    let acc_sk_json = AccumulatorSecretKey::encode(&acc_sk)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    wallet.set(&format!("accumulator {}", &issuer_did), &acc_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("tails {}", &issuer_did), &tails_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("accumulator_pk {}", &issuer_did), &acc_pk_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    wallet.set(&format!("accumulator_sk {}", &issuer_did), &acc_sk_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    Ok((acc_json, "".to_string()))
                });

        match result {
            Ok((revoc_registry_json, unique_number)) => cb(Ok((revoc_registry_json, unique_number))),
            Err(err) => cb(Err(err))
        }
    }

    fn create_and_store_claim(&self,
                              wallet_handle: i32,
                              claim_req_json: &str,
                              claim_json: &str,
                              issuer_did: &str,
                              claim_def_seq_no: i32,
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
        let result =
            self.wallet_service.wallets.borrow().get(&wallet_handle)
                .ok_or_else(|| AnoncredsError::WalletError(WalletError::InvalidHandle(format!("{}", wallet_handle))))
                .and_then(|wallet| {
                    let acc_json = RefCell::new(Rewallet.get(&format!("accumulator {}", &issuer_did))
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?);

                    let tails: HashMap<i32, PointG1> = serde_json::to_string(&tails)
                        .map_err(|err| AnoncredsError::CryptoError(CryptoError::InvalidStructure(err.to_string())))?;

                    let (acc, timestamp) =
                        self.crypto_service.anoncreds.issuer.revoke(&acc_json, &tails, &user_revoc_index)
                            .map_err(|err| AnoncredsError::CryptoError(CryptoError::BackendError(err.to_string())))?;


                    wallet.set(&format!("accumulator_sk {}", &issuer_did), &acc_sk_json)
                        .map_err(|err| AnoncredsError::WalletError(WalletError::BackendError(err.to_string())))?;

                    Ok((acc_json, "".to_string()))
                });

        match result {
            Ok(revoc_registry_json) => cb(Ok(revoc_registry_json)),
            Err(err) => cb(Err(err))
        }
    }
}