extern crate serde_json;

use errors::anoncreds::AnoncredsError;
use errors::indy::IndyError;
use errors::common::CommonError;

use services::anoncreds::AnoncredsService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::anoncreds::types::{
    ClaimDefinition,
    ClaimDefinitionPrivate,
    ClaimJson,
    ClaimRequestJson,
    RevocationRegistry,
    RevocationRegistryPrivate,
    Schema
};
use services::anoncreds::helpers::get_composite_id;
use std::rc::Rc;
use std::collections::HashMap;
use utils::json::{JsonDecodable, JsonEncodable};
use std::cell::RefCell;

pub enum IssuerCommand {
    CreateAndStoreClaimDefinition(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        Option<String>, // signature type
        bool,
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAndStoreRevocationRegistry(
        i32, // wallet handle
        String, // issuer did
        i32, // schema seq no
        i32, // max claim num
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateClaim(
        i32, // wallet handle
        String, // claim req json
        String, // claim json
        Option<i32>, // user revoc index
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    RevokeClaim(
        i32, // wallet handle
        String, // issuer did
        i32, // schema seq no
        i32, // user revoc index
        Box<Fn(Result<String, IndyError>) + Send>),
}

pub struct IssuerCommandExecutor {
    pub anoncreds_service: Rc<AnoncredsService>,
    pub pool_service: Rc<PoolService>,
    pub wallet_service: Rc<WalletService>
}

impl IssuerCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>) -> IssuerCommandExecutor {
        IssuerCommandExecutor {
            anoncreds_service: anoncreds_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, command: IssuerCommand) {
        match command {
            IssuerCommand::CreateAndStoreClaimDefinition(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreClaim command received");
                self.create_and_store_claim_definition(wallet_handle, &issuer_did, &schema_json,
                                                       signature_type.as_ref().map(String::as_str), create_non_revoc, cb);
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(wallet_handle, issuer_did, schema_seq_no, max_claim_num, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistryRegistry command received");
                self.create_and_store_revocation_registry(wallet_handle, &issuer_did, schema_seq_no, max_claim_num, cb);
            }
            IssuerCommand::CreateClaim(wallet_handle, claim_req_json, claim_json, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                self.create_claim(wallet_handle, &claim_req_json, &claim_json,
                                  user_revoc_index, cb);
            }
            IssuerCommand::RevokeClaim(wallet_handle, issuer_did, schema_seq_no,
                                       user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "RevokeClaim command received");
                self.revoke_claim(wallet_handle, &issuer_did, schema_seq_no, user_revoc_index, cb);
            }
        };
    }

    fn create_and_store_claim_definition(&self,
                                         wallet_handle: i32,
                                         issuer_did: &str,
                                         schema_json: &str,
                                         signature_type: Option<&str>,
                                         create_non_revoc: bool,
                                         cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let result = self._create_and_store_claim_definition(wallet_handle, issuer_did, schema_json,
                                                             signature_type, create_non_revoc);
        cb(result)
    }

    fn _create_and_store_claim_definition(&self,
                                          wallet_handle: i32,
                                          issuer_did: &str,
                                          schema_json: &str,
                                          signature_type: Option<&str>,
                                          create_non_revoc: bool) -> Result<String, IndyError> {
        let schema = Schema::from_json(schema_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid schema json: {}", err.to_string())))?;

        let (claim_definition, claim_definition_private) =
            self.anoncreds_service.issuer.generate_claim_definition(issuer_did, schema.clone(), signature_type, create_non_revoc)?;

        let claim_definition_json = ClaimDefinition::to_json(&claim_definition)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim definition json: {}", err.to_string())))?;

        let claim_definition_private_json = ClaimDefinitionPrivate::to_json(&claim_definition_private)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim definition private json: {}", err.to_string())))?;

        let claim_def_id = get_composite_id(issuer_did, schema.seq_no);
        self.wallet_service.set(wallet_handle, &format!("claim_definition::{}", &claim_def_id), &claim_definition_json)?;
        self.wallet_service.set(wallet_handle, &format!("claim_definition_private::{}", &claim_def_id), &claim_definition_private_json)?;

        Ok(claim_definition_json)
    }

    fn create_and_store_revocation_registry(&self,
                                            wallet_handle: i32,
                                            issuer_did: &str,
                                            schema_seq_no: i32,
                                            max_claim_num: i32,
                                            cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let result = self._create_and_store_revocation_registry(wallet_handle, issuer_did, schema_seq_no, max_claim_num);
        cb(result)
    }

    fn _create_and_store_revocation_registry(&self,
                                             wallet_handle: i32,
                                             issuer_did: &str,
                                             schema_seq_no: i32,
                                             max_claim_num: i32) -> Result<String, IndyError> {
        let claim_def_id = get_composite_id(issuer_did, schema_seq_no);

        let claim_def_json = self.wallet_service.get(wallet_handle, &format!("claim_definition::{}", &claim_def_id))?;
        let claim_def = ClaimDefinition::from_json(&claim_def_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim definition json: {}", err.to_string())))?;

        let pk_r = claim_def.data.public_key_revocation
            .ok_or(IndyError::AnoncredsError(AnoncredsError::NotIssuedError("Revocation Public Key for this claim definition".to_string())))?;

        let (revocation_registry, revocation_registry_private) =
            self.anoncreds_service.issuer.issue_accumulator(&pk_r, max_claim_num, issuer_did, schema_seq_no)?;

        let revocation_registry_json = RevocationRegistry::to_json(&revocation_registry)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation registry: {}", err.to_string())))?;

        let revocation_registry_private_json = RevocationRegistryPrivate::to_json(&revocation_registry_private)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation registry private: {}", err.to_string())))?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", &claim_def_id), &revocation_registry_json)?;
        self.wallet_service.set(wallet_handle, &format!("revocation_registry_private::{}", &claim_def_id), &revocation_registry_private_json)?;
        // TODO: change it
        let tails_dash = serde_json::to_string(&revocation_registry_private.tails_dash)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation registry private: {}", err.to_string())))?;

        self.wallet_service.set(wallet_handle, &format!("tails"), &tails_dash)?;

        Ok(revocation_registry_json)
    }

    fn create_claim(&self,
                    wallet_handle: i32,
                    claim_req_json: &str,
                    claim_json: &str,
                    user_revoc_index: Option<i32>,
                    cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        let result = self._create_claim(wallet_handle, claim_req_json, claim_json, user_revoc_index);
        cb(result)
    }

    fn _create_claim(&self,
                     wallet_handle: i32,
                     claim_req_json: &str,
                     claim_json: &str,
                     user_revoc_index: Option<i32>) -> Result<(String, String), IndyError> {
        let claim_req_json: ClaimRequestJson = ClaimRequestJson::from_json(claim_req_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid claim_req_json: {}", err.to_string())))?;

        let claim_def_id = get_composite_id(&claim_req_json.issuer_did.clone(), claim_req_json.schema_seq_no);

        let claim_def_json = self.wallet_service.get(wallet_handle, &format!("claim_definition::{}", &claim_def_id))?;
        let claim_def_private_json = self.wallet_service.get(wallet_handle, &format!("claim_definition_private::{}", &claim_def_id))?;

        let claim_def = ClaimDefinition::from_json(&claim_def_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_def_json: {}", err.to_string())))?;

        let claim_def_private = ClaimDefinitionPrivate::from_json(&claim_def_private_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_def_private_json: {}", err.to_string())))?;

        if claim_def.data.public_key_revocation.is_some() && claim_req_json.blinded_ms.ur.is_none() {
            return Err(IndyError::AnoncredsError(AnoncredsError::NotIssuedError(
                format!("Claim_request.ur are required for this claim"))));
        }

        let (revocation_registry, revocation_registry_private,
            mut revocation_registry_json) = match claim_def.data.public_key_revocation {
            Some(_) => {
                let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", &claim_def_id))?;
                let revocation_registry_private_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_private::{}", &claim_def_id))?;

                let revocation_registry = Some(RefCell::new(RevocationRegistry::from_json(&revocation_registry_json)
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Invalid revocation_registry_json: {}", err.to_string())))?));

                let revocation_registry_private = Some(RevocationRegistryPrivate::from_json(&revocation_registry_private_json)
                    .map_err(map_err_trace!())
                    .map_err(|err| CommonError::InvalidState(format!("Invalid revocation_registry_private_json: {}", err.to_string())))?);

                (revocation_registry, revocation_registry_private, revocation_registry_json)
            }
            _ => (None, None, String::new())
        };

        let attributes: HashMap<String, Vec<String>> = serde_json::from_str(claim_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid claim_json: {}", err.to_string())))?;

        let claims = self.anoncreds_service.issuer.create_claim(&claim_def,
                                                                &claim_def_private,
                                                                &revocation_registry,
                                                                &revocation_registry_private,
                                                                &claim_req_json.blinded_ms,
                                                                &attributes,
                                                                user_revoc_index)?;

        if let Some(x) = revocation_registry {
            revocation_registry_json = RevocationRegistry::to_json(&x.borrow())
                .map_err(map_err_trace!())
                .map_err(|err| CommonError::InvalidState(format!("Invalid revocation registry: {}", err.to_string())))?;

            self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", &claim_def_id), &revocation_registry_json)?;
        }

        let claim_json = ClaimJson::new(attributes, claims, claim_def.schema_seq_no, claim_req_json.issuer_did);

        let claim_json = ClaimJson::to_json(&claim_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_json: {}", err.to_string())))?;

        Ok((revocation_registry_json, claim_json))
    }

    fn revoke_claim(&self,
                    wallet_handle: i32,
                    issuer_did: &str,
                    schema_seq_no: i32,
                    user_revoc_index: i32,
                    cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let result = self._revoke_claim(wallet_handle, issuer_did, schema_seq_no, user_revoc_index);
        cb(result)
    }

    fn _revoke_claim(&self,
                     wallet_handle: i32,
                     issuer_did: &str,
                     schema_seq_no: i32,
                     user_revoc_index: i32) -> Result<String, IndyError> {
        let revoc_reg_id = get_composite_id(&issuer_did.clone(), schema_seq_no);

        let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", &revoc_reg_id))?;
        let revocation_registry_private_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_private::{}", &revoc_reg_id))?;

        let revocation_registry = RevocationRegistry::from_json(&revocation_registry_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation_registry_json: {}", err.to_string())))?;

        let revocation_registry_private = RevocationRegistryPrivate::from_json(&revocation_registry_private_json)
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation_registry_private_json: {}", err.to_string())))?;

        let revocation_registry = RefCell::new(revocation_registry);

        self.anoncreds_service.issuer.revoke(&revocation_registry,
                                             &revocation_registry_private.tails_dash,
                                             user_revoc_index)?;

        let revoc_reg_update_json = RevocationRegistry::to_json(&revocation_registry.borrow())
            .map_err(map_err_trace!())
            .map_err(|err| CommonError::InvalidState(format!("Invalid revocation registry: {}", err.to_string())))?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", &revoc_reg_id), &revoc_reg_update_json)?;

        Ok(revoc_reg_update_json)
    }
}
