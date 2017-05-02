extern crate serde_json;
extern crate uuid;

use self::uuid::Uuid;

use errors::anoncreds::AnoncredsError;
use services::crypto::CryptoService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::anoncreds::types::{
    ClaimDefinition,
    ClaimDefinitionPrivate,
    ClaimJson,
    ClaimRequestJson,
    RevocationRegistry,
    RevocationRegistryPrivate,
    Schema,
    ClaimRequestJson,
    ClaimJson
};
use std::rc::Rc;
use std::collections::HashMap;
use utils::json::{JsonDecodable, JsonEncodable};
use std::cell::RefCell;

pub enum IssuerCommand {
    CreateAndStoreClaimDefinition(
        i32, // wallet handle
        String, // schema json
        Option<String>, // signature type
        bool,
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    CreateAndStoreRevocationRegistry(
        i32, // wallet handle
        i32, // claim def seq no
        i32, // max claim num
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    CreateClaim(
        i32, // wallet handle
        String, // claim req json
        String, // claim json
        i32, // revoc reg seq no
        Option<i32>, // user revoc index
        Box<Fn(Result<(String, String), AnoncredsError>) + Send>),
    RevokeClaim(
        i32, // wallet handle
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
            IssuerCommand::CreateAndStoreClaimDefinition(wallet_handle, schema_json, signature_type, create_non_revoc, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreClaim command received");
                self.create_and_store_claim_definition(wallet_handle, &schema_json,
                                                       signature_type.as_ref().map(String::as_str), create_non_revoc, cb);
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(wallet_handle, claim_def_seq_no, max_claim_num, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistryRegistry command received");
                self.create_and_store_revocation_registry(wallet_handle, claim_def_seq_no, max_claim_num, cb);
            }
            IssuerCommand::CreateClaim(wallet_handle, claim_req_json, claim_json, revoc_reg_seq_no, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                self.create_and_store_claim(wallet_handle, &claim_req_json, &claim_json,
                                            revoc_reg_seq_no, user_revoc_index, cb);
            }
            IssuerCommand::RevokeClaim(wallet_handle, claim_def_seq_no, revoc_reg_seq_no,
                                       user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "RevokeClaim command received");
                self.revoke_claim(wallet_handle, claim_def_seq_no, revoc_reg_seq_no, user_revoc_index, cb);
            }
        };
    }

    fn create_and_store_claim_definition(&self,
                                         wallet_handle: i32,
                                         schema_json: &str,
                                         signature_type: Option<&str>,
                                         create_non_revoc: bool,
                                         cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        let result = self._create_and_store_claim_definition(wallet_handle, schema_json,
                                                             signature_type, create_non_revoc);
        cb(result)
    }

    fn _create_and_store_claim_definition(&self,
                                          wallet_handle: i32,
                                          schema_json: &str,
                                          signature_type: Option<&str>, create_non_revoc: bool) -> Result<(String, String), AnoncredsError> {
        let schema = Schema::from_str(schema_json)?;

        let (claim_definition, claim_definition_private) =
            self.crypto_service.anoncreds.issuer.generate_keys(schema, signature_type, create_non_revoc)?;

        let claim_definition_json = ClaimDefinition::to_string(&claim_definition)?;
        let claim_definition_private_json = ClaimDefinitionPrivate::to_string(&claim_definition_private)?;

        let uuid = Uuid::new_v4().to_string();

        self.wallet_service.set(wallet_handle, &format!("claim_definition::{}", &uuid), &claim_definition_json)?;
        self.wallet_service.set(wallet_handle, &format!("claim_definition_private::{}", &uuid), &claim_definition_private_json)?;

        Ok((claim_definition_json, uuid))
    }

    fn create_and_store_revocation_registry(&self,
                                            wallet_handle: i32,
                                            claim_def_seq_no: i32,
                                            max_claim_num: i32,
                                            cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        let result = self._create_and_store_revocation_registry(wallet_handle, claim_def_seq_no, max_claim_num);
        cb(result)
    }

    fn _create_and_store_revocation_registry(&self,
                                             wallet_handle: i32,
                                             claim_def_seq_no: i32,
                                             max_claim_num: i32) -> Result<(String, String), AnoncredsError> {
        let claim_def_uuid = self.wallet_service.get(wallet_handle, &format!("claim_definition_uuid::{}", &claim_def_seq_no))?;
        let claim_def_json = self.wallet_service.get(wallet_handle, &format!("claim_definition::{}", &claim_def_uuid))?;
        let claim_def = ClaimDefinition::from_str(&claim_def_json)?;

        let pk_r = claim_def.public_key_revocation.ok_or(AnoncredsError::NotIssuedError("Revocation Public Key for this claim definition".to_string()))?;

        let (revocation_registry, revocation_registry_private) =
            self.crypto_service.anoncreds.issuer.issue_accumulator(&pk_r, max_claim_num, claim_def_seq_no)?;

        let uuid = Uuid::new_v4().to_string();

        let revocation_registry_json = RevocationRegistry::to_string(&revocation_registry)?;
        let revocation_registry_private_json = RevocationRegistryPrivate::to_string(&revocation_registry_private)?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", &uuid), &revocation_registry_json)?;
        self.wallet_service.set(wallet_handle, &format!("revocation_registry_private::{}", &uuid), &revocation_registry_private_json)?;

        Ok((revocation_registry_json, uuid))
    }

    fn create_and_store_claim(&self,
                              wallet_handle: i32,
                              claim_req_json: &str,
                              claim_json: &str,
                              revoc_reg_seq_no: i32,
                              user_revoc_index: Option<i32>,
                              cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        let result = self._create_and_store_claim(wallet_handle, claim_req_json, claim_json, revoc_reg_seq_no, user_revoc_index);
        cb(result)
    }

    fn _create_and_store_claim(&self,
                               wallet_handle: i32,
                               claim_req_json: &str,
                               claim_json: &str,
                               revoc_reg_seq_no: i32,
                               user_revoc_index: Option<i32>) -> Result<(String, String), AnoncredsError> {
        let claim_req_json: ClaimRequestJson = serde_json::from_str(claim_req_json)?;

        let claim_def_uuid = self.wallet_service.get(wallet_handle, &format!("claim_definition_uuid::{}", &claim_req_json.claim_def_seq_no))?;
        let claim_def_json = self.wallet_service.get(wallet_handle, &format!("claim_definition::{}", &claim_def_uuid))?;
        let claim_def_private_json = self.wallet_service.get(wallet_handle, &format!("claim_definition_private::{}", &claim_def_uuid))?;

        let claim_def = ClaimDefinition::from_str(&claim_def_json)?;
        let claim_def_private = ClaimDefinitionPrivate::from_str(&claim_def_private_json)?;

        let revocation_registry_uuid = self.wallet_service.get(wallet_handle, &format!("revocation_registry_uuid::{}", &revoc_reg_seq_no))?;
        let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", &revocation_registry_uuid))?;
        let revocation_registry_private_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_private::{}", &revocation_registry_uuid))?;

        let revocation_registry = RevocationRegistry::from_str(&revocation_registry_json)?;
        let revocation_registry_private = RevocationRegistryPrivate::from_str(&revocation_registry_private_json)?;

        let attributes: HashMap<String, Vec<String>> = serde_json::from_str(claim_json)?;

        let revocation_registry = RefCell::new(revocation_registry);

        let claims = self.crypto_service.anoncreds.issuer.create_claim(
            claim_def,
            claim_def_private,
            &revocation_registry,
            &revocation_registry_private,
            &claim_req_json.blinded_ms,
            &attributes,
            user_revoc_index
        )?;

        let revocation_registry_json = RevocationRegistry::to_string(&revocation_registry.borrow())?;

        let claim_json = ClaimJson::new(attributes, claim_req_json.claim_def_seq_no, revoc_reg_seq_no, claims);
        let claim_json = ClaimJson::to_string(&claim_json)?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry_uuid::{}", &revocation_registry_uuid), &revocation_registry_json)?;

        Ok((revocation_registry_json, claim_json))
    }

    fn revoke_claim(&self,
                    wallet_handle: i32,
                    claim_def_seq_no: i32,
                    revoc_reg_seq_no: i32,
                    user_revoc_index: i32,
                    cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        let result = self._revoke_claim(wallet_handle, claim_def_seq_no, revoc_reg_seq_no, user_revoc_index);
        cb(result)
    }

    fn _revoke_claim(&self,
                     wallet_handle: i32,
                     claim_def_seq_no: i32,
                     revoc_reg_seq_no: i32,
                     user_revoc_index: i32) -> Result<String, AnoncredsError> {
        let revocation_registry_uuid = self.wallet_service.get(wallet_handle, &format!("revocation_registry_uuid::{}", &revoc_reg_seq_no))?;
        let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", &revocation_registry_uuid))?;
        let revocation_registry_private_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_private::{}", &revocation_registry_uuid))?;

        let revocation_registry = RevocationRegistry::from_str(&revocation_registry_json)?;
        let revocation_registry_private = RevocationRegistryPrivate::from_str(&revocation_registry_private_json)?;

        let revocation_registry = RefCell::new(revocation_registry);

        self.crypto_service.anoncreds.issuer.revoke(
            &revocation_registry,
            &revocation_registry_private.tails,
            user_revoc_index
        )?;

        let revoc_reg_update_json = RevocationRegistry::to_string(&revocation_registry.borrow())?;
        self.wallet_service.set(wallet_handle, &format!("revocation_registry_uuid::{}", &revocation_registry_uuid), &revoc_reg_update_json)?;

        Ok(revoc_reg_update_json)
    }
}