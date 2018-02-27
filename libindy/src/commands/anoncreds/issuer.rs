extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;

use services::anoncreds::AnoncredsService;
use services::blob_storage::BlobStorageService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use services::anoncreds::types::*;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use super::tails::{SDKTailsAccessor, store_tails_from_generator};

pub enum IssuerCommand {
    CreateSchema(
        String, // issuer did
        String, // name
        String, // version
        String, // attribute names
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAndStoreCredentialDefinition(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        String, // tag
        Option<String>, // type
        bool, // support revocation
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAndStoreRevocationRegistry(
        i32, // wallet handle
        String, // issuer did
        Option<String>, // type
        String, // tag
        String, // credential definition id
        Option<String>, // issuance type
        u32, // max credential num
        Option<String>, // tails writer type
        String, // tails writer config
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    CreateCredentialOffer(
        i32, // wallet handle
        String, // credential definition id
        Option<String>, // revocation registry id
        String, // prover did
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateClaim(
        i32, // wallet handle
        String, // credential req json
        String, // credential json
        Option<i32>, // tails reader handle
        Option<u32>, // user revoc index
        Box<Fn(Result<(Option<String>, String), IndyError>) + Send>),
    RevokeClaim(
        i32, // wallet handle
        i32, // tails reader handle
        String, // revocation registry id
        u32, // user revoc index
        Box<Fn(Result<String, IndyError>) + Send>),
    RecoverClaim(
        i32, // wallet handle
        i32, // tails reader handle
        String, // revocation registry id
        u32, // user revoc index
        Box<Fn(Result<String, IndyError>) + Send>)
}

pub struct IssuerCommandExecutor {
    pub anoncreds_service: Rc<AnoncredsService>,
    pub blob_storage_service: Rc<BlobStorageService>,
    pub pool_service: Rc<PoolService>,
    pub wallet_service: Rc<WalletService>,
    pub crypto_service: Rc<CryptoService>
}

impl IssuerCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               blob_storage_service: Rc<BlobStorageService>,
               wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>) -> IssuerCommandExecutor {
        IssuerCommandExecutor {
            anoncreds_service,
            pool_service,
            blob_storage_service,
            wallet_service,
            crypto_service,
        }
    }

    pub fn execute(&self, command: IssuerCommand) {
        match command {
            IssuerCommand::CreateSchema(issuer_did, name, version, attr_names, cb) => {
                trace!(target: "issuer_command_executor", "CreateSchema command received");
                cb(self.create_schema(&issuer_did, &name, &version, &attr_names));
            }
            IssuerCommand::CreateAndStoreCredentialDefinition(wallet_handle, issuer_did, schema_json, tag, type_, create_non_revoc, cb) => {
                trace!(target: "issuer_command_executor", "CreateAndStoreClaimDef command received");
                cb(self.create_and_store_credential_definition(wallet_handle, &issuer_did, &schema_json, &tag,
                                                               type_.as_ref().map(String::as_str), create_non_revoc));
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(wallet_handle, issuer_did, type_, tag, cred_def_id, issuance_type, max_cred_num, tails_writer_type, tails_writer_config, cb) => {
                trace!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistryRegistry command received");
                cb(self.create_and_store_revocation_registry(wallet_handle,
                                                             &issuer_did,
                                                             type_.as_ref().map(String::as_str),
                                                             &tag,
                                                             &cred_def_id,
                                                             issuance_type.as_ref().map(String::as_str),
                                                             max_cred_num,
                                                             tails_writer_type.as_ref().map(String::as_str),
                                                             &tails_writer_config));
            }
            IssuerCommand::CreateCredentialOffer(wallet_handle, cred_def_id, rev_reg_id, prover_did, cb) => {
                trace!(target: "issuer_command_executor", "CreateCredentialOffer command received");
                cb(self.create_credential_offer(wallet_handle, &cred_def_id, rev_reg_id.as_ref().map(String::as_str), &prover_did));
            }
            IssuerCommand::CreateClaim(wallet_handle, credential_req_json, credential_json, tails_reader_handle, rev_idx, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                cb(self.new_credential(wallet_handle, &credential_req_json, &credential_json, tails_reader_handle, rev_idx));
            }
            IssuerCommand::RevokeClaim(wallet_handle, tails_reader_handle, rev_reg_id, user_revoc_index, cb) => {
                trace!(target: "issuer_command_executor", "RevokeClaim command received");
                cb(self.revoke_credential(wallet_handle, tails_reader_handle, &rev_reg_id, user_revoc_index));
            }
            IssuerCommand::RecoverClaim(wallet_handle, tails_reader_handle, rev_reg_id, user_revoc_index, cb) => {
                trace!(target: "issuer_command_executor", "RecoverClaim command received");
                cb(self.recovery_credential(wallet_handle, tails_reader_handle, &rev_reg_id, user_revoc_index));
            }
        };
    }

    fn create_schema(&self,
                     issuer_did: &str,
                     name: &str,
                     version: &str,
                     attr_names: &str) -> Result<String, IndyError> {
        trace!("create_schema >>> issuer_did: {:?}, name: {:?}, version: {:?}, attr_names: {:?}", issuer_did, name, version, attr_names);

        self.crypto_service.validate_did(issuer_did)?;

        let attr_names: HashSet<String> = serde_json::from_str(attr_names)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize AttributeNames: {:?}", err)))?;

        const MARKER: &'static str = "1";

        let id = IssuerCommandExecutor::build_id(issuer_did, MARKER, None, name, version);

        let schema = Schema {
            id,
            name: name.to_string(),
            version: version.to_string(),
            attr_names,
        };

        let schema_json = schema.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize Schema: {:?}", err)))?;

        trace!("create_schema <<< schema_json: {:?}", schema_json);

        Ok(schema_json)
    }

    fn create_and_store_credential_definition(&self,
                                              wallet_handle: i32,
                                              issuer_did: &str,
                                              schema_json: &str,
                                              tag: &str,
                                              type_: Option<&str>,
                                              support_revocation: bool) -> Result<String, IndyError> {
        trace!("create_and_store_credential_definition >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, tag: {:?}, \
              type_: {:?}, support_revocation: {:?}", wallet_handle, issuer_did, schema_json, tag, type_, support_revocation);

        self.crypto_service.validate_did(issuer_did)?;

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Schema: {:?}", err)))?;

        let signature_type = match type_ {
            Some(type_) =>
                SignatureTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize SignatureType: {:?}", err)))?,
            None => SignatureTypes::CL,
        };

        const MARKER: &'static str = "2";

        let id = IssuerCommandExecutor::build_id(issuer_did, MARKER, Some(&schema.id), signature_type.to_str(), tag);

        if self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", id)).is_ok() {
            return Err(IndyError::AnoncredsError(AnoncredsError::ClaimDefAlreadyExists(format!("CredentialDefinition for id: {:?} already exists", id))));
        };

        let (credential_definition_value, credential_priv_key, credential_key_correctness_proof) =
            self.anoncreds_service.issuer.new_credential_definition(issuer_did, &schema, support_revocation)?;

        let credential_definition = CredentialDefinition {
            id: id.clone(),
            schema_id: schema.id,
            signature_type,
            tag: tag.to_string(),
            value: credential_definition_value
        };

        let credential_definition_json = self.wallet_service.set_object(wallet_handle,
                                                                        &format!("credential_definition::{}", id),
                                                                        &credential_definition,
                                                                        "CredentialDefinition")?;
        self.wallet_service.set_object(wallet_handle,
                                       &format!("credential_private_key::{}", id),
                                       &credential_priv_key,
                                       "CredentialPrivateKey")?;
        self.wallet_service.set_object(wallet_handle,
                                       &format!("credential_key_correctness_proof::{}", id),
                                       &credential_key_correctness_proof,
                                       "credential_key_correctness_proof_json")?;

        trace!("create_and_store_credential_definition <<< credential_definition_json: {:?}", credential_definition_json);

        Ok(credential_definition_json)
    }

    fn create_and_store_revocation_registry(&self,
                                            wallet_handle: i32,
                                            issuer_did: &str,
                                            type_: Option<&str>,
                                            tag: &str,
                                            cred_def_id: &str,
                                            issuance_type: Option<&str>,
                                            max_cred_num: u32,
                                            tails_writer_type: Option<&str>,
                                            tails_writer_config: &str) -> Result<(String, String), IndyError> {
        trace!("create_and_store_revocation_registry >>> wallet_handle: {:?}, type_: {:?}, tag: {:?}, cred_def_id: {:?}, issuance_type: {:?}, \
               max_cred_num: {:?}, tails_writer_type: {:?}, tails_writer_config: {:?}",
               wallet_handle, type_, tag, cred_def_id, issuance_type, max_cred_num, tails_writer_type, tails_writer_config);

        let rev_reg_type = match type_ {
            Some(type_) =>
                RevocationRegistryTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryTypes: {:?}", err)))?,
            None => RevocationRegistryTypes::CL_ACCUM,
        };

        let issuance_type = match issuance_type {
            Some(type_) =>
                IssuanceTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize IssuanceTypes: {:?}", err)))?,
            None => IssuanceTypes::ISSUANCE_BY_DEFAULT,
        };

        let tails_writer_type = tails_writer_type.unwrap_or("default");

        const MARKER: &'static str = "3";

        let id = IssuerCommandExecutor::build_id(issuer_did, MARKER, Some(cred_def_id), rev_reg_type.to_str(), tag);

        let credential_def: CredentialDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("credential_definition::{}", &cred_def_id), "CredentialDefinition", &mut String::new())?;

        let (revocation_public_keys, revocation_key_private, revocation_registry, mut revocation_tails_generator) =
            self.anoncreds_service.issuer.new_revocation_registry(&credential_def,
                                                                  max_cred_num,
                                                                  issuance_type.to_bool(),
                                                                  issuer_did)?;

        let (tails_location, tails_hash) =
            store_tails_from_generator(self.blob_storage_service.clone(), tails_writer_type, tails_writer_config, &mut revocation_tails_generator)?;

        let revocation_registry_definition_value = RevocationRegistryDefinitionValue {
            max_cred_num,
            issuance_type,
            public_keys: revocation_public_keys,
            tails_location,
            tails_hash,
        };

        let revocation_registry_definition = RevocationRegistryDefinition {
            id: id.clone(),
            type_: rev_reg_type,
            tag: tag.to_string(),
            cred_def_id: cred_def_id.to_string(),
            value: revocation_registry_definition_value
        };

        let revocation_registry_definition_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry_definition::{}", id), &revocation_registry_definition, "RevocationRegistryDefinitionValue")?;
        let revocation_registry_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", id), &revocation_registry, "RevocationRegistry")?;
        let revocation_tails_generator_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_tails_generator::{}", id), &revocation_tails_generator, "RevocationTailsGenerator")?;
        self.wallet_service.set_object(wallet_handle, &format!("revocation_key_private::{}", id), &revocation_key_private, "RevocationKeyPrivate")?;

        trace!("create_and_store_revocation_registry <<< revocation_registry_definition_json: {:?}, revocation_registry_json: {:?}",
               revocation_registry_definition_json, revocation_registry_json);

        Ok((revocation_registry_definition_json, revocation_registry_json))
    }

    fn create_credential_offer(&self,
                               wallet_handle: i32,
                               cred_def_id: &str,
                               rev_reg_id: Option<&str>,
                               prover_did: &str) -> Result<String, IndyError> {
        trace!("create_credential_offer >>> wallet_handle: {:?}, cred_def_id: {:?}, prover_did: {:?}",
               wallet_handle, cred_def_id, prover_did);

        self.crypto_service.validate_did(prover_did)?;

        let key_correctness_proof: CredentialKeyCorrectnessProof =
            self.wallet_service.get_object(wallet_handle, &format!("credential_key_correctness_proof::{}", cred_def_id), "CredentialKeyCorrectnessProof", &mut String::new())?;

        let nonce = new_nonce()
            .map_err(|err| IndyError::AnoncredsError(AnoncredsError::from(err)))?;

        self.wallet_service.set_object(wallet_handle, &format!("master_secret_blinding_nonce::{}::{}", cred_def_id, prover_did), &nonce, "Nonce")?;

        let credential_offer = CredentialOffer {
            cred_def_id: cred_def_id.to_string(),
            rev_reg_id: rev_reg_id.as_ref().map(|s| s.to_string()),
            key_correctness_proof,
            nonce
        };

        let credential_offer_json = credential_offer.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialOffer: {:?}", err)))?;

        trace!("create_credential_offer <<< credential_offer_json: {:?}", credential_offer_json);

        Ok(credential_offer_json)
    }

    fn new_credential(&self,
                      wallet_handle: i32,
                      credential_req_json: &str,
                      credential_json: &str,
                      tails_reader_handle: Option<i32>,
                      rev_idx: Option<u32>) -> Result<(Option<String>, String), IndyError> {
        trace!("new_credential >>> wallet_handle: {:?}, tails_reader_handle: {:?}, credential_req_json: {:?}, credential_json: {:?}, rev_idx: {:?}",
               wallet_handle, tails_reader_handle, credential_req_json, credential_json, rev_idx);

        let credential_request: CredentialRequest = CredentialRequest::from_json(credential_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialRequest: {:?}", err)))?;

        let credential_def: CredentialDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("credential_definition::{}", credential_request.cred_def_id), "CredentialDefinition", &mut String::new())?;

        let credential_priv_key: CredentialPrivateKey =
            self.wallet_service.get_object(wallet_handle, &format!("credential_private_key::{}", credential_request.cred_def_id), "CredentialPrivateKey", &mut String::new())?;

        let master_secret_blinding_nonce: Nonce =
            self.wallet_service.get_object(wallet_handle, &format!("master_secret_blinding_nonce::{}::{}", credential_request.cred_def_id, credential_request.prover_did), "Nonce", &mut String::new())?;

        let credential_values: HashMap<String, Vec<String>> = serde_json::from_str(credential_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialValues: {:?}", err)))?;

        let (rev_reg_def, mut rev_reg,
            rev_key_priv, sdk_tails_accessor) = match credential_request.rev_reg_id.as_ref() {
            Some(ref rev_reg_id) => {
                let rev_reg_def: Option<RevocationRegistryDefinitionValue> =
                    self.wallet_service.get_opt_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), "RevocationRegistryDefinitionValue", &mut String::new())?;

                let rev_reg: Option<RevocationRegistry> =
                    self.wallet_service.get_opt_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), "RevocationRegistry", &mut String::new())?;

                let rev_key_priv: Option<RevocationKeyPrivate> =
                    self.wallet_service.get_opt_object(wallet_handle, &format!("revocation_key_private::{}", rev_reg_id), "RevocationKeyPrivate", &mut String::new())?;

                let sdk_tails_accessor = match tails_reader_handle {
                    Some(handle) => Some(SDKTailsAccessor::new(self.blob_storage_service.clone(), handle)),
                    None => None
                };
                (rev_reg_def, rev_reg, rev_key_priv, sdk_tails_accessor)
            }
            None => (None, None, None, None)
        };

        let (credential_signature, signature_correctness_proof, rev_reg_delta) =
            self.anoncreds_service.issuer.new_credential(&credential_def,
                                                         &credential_priv_key,
                                                         &master_secret_blinding_nonce,
                                                         &credential_request,
                                                         &credential_values,
                                                         rev_idx,
                                                         rev_reg_def.as_ref(),
                                                         rev_reg.as_mut(),
                                                         rev_key_priv.as_ref(),
                                                         sdk_tails_accessor.as_ref())?;

        if let Some(r_reg) = rev_reg {
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", credential_request.rev_reg_id.clone().unwrap_or(String::new())), &r_reg, "RevocationRegistry")?;
        }

        let credential = Credential {
            values: credential_values,
            signature: credential_signature,
            signature_correctness_proof,
            schema_id: credential_def.schema_id.clone(),
            cred_def_id: credential_request.cred_def_id.clone(),
            rev_reg_id: credential_request.rev_reg_id.clone()
        };

        let credential_json = credential.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize Credential: {:?}", err)))?;

        let rev_reg_delta_json = match rev_reg_delta {
            Some(r_reg_delta) => {
                Some(r_reg_delta.to_json()
                    .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?)
            }
            None => None
        };

        trace!("new_credential <<< rev_reg_delta_json: {:?}, credential_json: {:?}", rev_reg_delta_json, credential_json);

        Ok((rev_reg_delta_json, credential_json))
    }

    fn revoke_credential(&self,
                         wallet_handle: i32,
                         tails_reader_handle: i32,
                         rev_reg_id: &str,
                         rev_idx: u32) -> Result<String, IndyError> {
        trace!("revoke_credential >>> wallet_handle: {:?}, rev_reg_id:  {:?}, rev_idx: {:?}", wallet_handle, rev_reg_id, rev_idx);

        let revocation_registry_definition: RevocationRegistryDefinitionValue =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), "RevocationRegistryDefinitionValue", &mut String::new())?;

        let mut revocation_registry: RevocationRegistry =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), "RevocationRegistry", &mut String::new())?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(), tails_reader_handle);

        let revocation_registry_delta =
            self.anoncreds_service.issuer.revoke(&mut revocation_registry, revocation_registry_definition.max_cred_num, rev_idx, &sdk_tails_accessor)?;

        self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), &revocation_registry, "RevocationRegistry")?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?;

        trace!("revoke_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }

    fn recovery_credential(&self,
                           wallet_handle: i32,
                           tails_reader_handle: i32,
                           rev_reg_id: &str,
                           rev_idx: u32) -> Result<String, IndyError> {
        trace!("recovery_credential >>> wallet_handle: {:?}, rev_reg_id: {:?}, rev_idx: {:?}", wallet_handle, rev_reg_id, rev_idx);

        let revocation_registry_definition: RevocationRegistryDefinitionValue =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), "RevocationRegistryDefinitionValue", &mut String::new())?;

        let mut revocation_registry: RevocationRegistry =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), "RevocationRegistry", &mut String::new())?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(), tails_reader_handle);

        let revocation_registry_delta =
            self.anoncreds_service.issuer.recovery(&mut revocation_registry, revocation_registry_definition.max_cred_num, rev_idx, &sdk_tails_accessor)?;

        self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), &revocation_registry, "RevocationRegistry")?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?;

        trace!("recovery_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }

    fn build_id(identifier: &str, marker: &str, related_entity_id: Option<&str>, word1: &str, word2: &str) -> String {
        format!("{}{}{}{}{}", identifier, marker, related_entity_id.unwrap_or(""), word1, word2)
    }
}
