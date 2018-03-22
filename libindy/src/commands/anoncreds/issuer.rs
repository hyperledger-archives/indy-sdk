extern crate base64;
extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;

use services::anoncreds::AnoncredsService;
use services::anoncreds::helpers::build_id;
use services::anoncreds::constants::*;
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
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    CreateAndStoreCredentialDefinition(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        String, // tag
        Option<String>, // type
        String, // config_json
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    CreateAndStoreRevocationRegistry(
        i32, // wallet handle
        String, // issuer did
        Option<String>, // type
        String, // tag
        String, // credential definition id
        String, // config
        i32, // tails writer handle
        Box<Fn(Result<(String, String, String), IndyError>) + Send>),
    CreateCredentialOffer(
        i32, // wallet handle
        String, // credential definition id
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateCredential(
        i32, // wallet handle
        String, // credential offer json
        String, // credential request json
        String, // credential values json
        Option<String>, // revocation registry id
        Option<i32>, // blob storage reader config handle
        Box<Fn(Result<(String, Option<String>, Option<String>), IndyError>) + Send>),
    RevokeCredential(
        i32, // wallet handle
        i32, // blob storage reader config handle
        String, //revocation revoc id
        String, //credential revoc id
        Box<Fn(Result<String, IndyError>) + Send>),
    RecoverCredential(
        i32, // wallet handle
        i32, // blob storage reader config handle
        String, //revocation revoc id
        String, //credential revoc id
        Box<Fn(Result<String, IndyError>) + Send>),
    MergeRevocationRegistryDeltas(
        String, //revocation registry delta json
        String, //other revocation registry delta json
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
            IssuerCommand::CreateSchema(issuer_did, name, version, attrs, cb) => {
                trace!(target: "issuer_command_executor", "CreateSchema command received");
                cb(self.create_schema(&issuer_did, &name, &version, &attrs));
            }
            IssuerCommand::CreateAndStoreCredentialDefinition(wallet_handle, issuer_did, schema_json, tag, type_, config_json, cb) => {
                trace!(target: "issuer_command_executor", "CreateAndStoreCredentialDefinition command received");
                cb(self.create_and_store_credential_definition(wallet_handle, &issuer_did, &schema_json, &tag,
                                                               type_.as_ref().map(String::as_str), &config_json));
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(wallet_handle, issuer_did, type_, tag, cred_def_id, config_json,
                                                            tails_writer_handle, cb) => {
                trace!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistryRegistry command received");
                cb(self.create_and_store_revocation_registry(wallet_handle,
                                                             &issuer_did,
                                                             type_.as_ref().map(String::as_str),
                                                             &tag,
                                                             &cred_def_id,
                                                             &config_json,
                                                             tails_writer_handle));
            }
            IssuerCommand::CreateCredentialOffer(wallet_handle, cred_def_id, cb) => {
                trace!(target: "issuer_command_executor", "CreateCredentialOffer command received");
                cb(self.create_credential_offer(wallet_handle, &cred_def_id));
            }
            IssuerCommand::CreateCredential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb) => {
                info!(target: "issuer_command_executor", "CreateCredential command received");
                cb(self.new_credential(wallet_handle, &cred_offer_json, &cred_req_json, &cred_values_json, rev_reg_id.as_ref().map(String::as_str), blob_storage_reader_handle));
            }
            IssuerCommand::RevokeCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb) => {
                trace!(target: "issuer_command_executor", "RevokeCredential command received");
                cb(self.revoke_credential(wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_revoc_id));
            }
            IssuerCommand::RecoverCredential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id, cb) => {
                trace!(target: "issuer_command_executor", "RecoverCredential command received");
                cb(self.recovery_credential(wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_revoc_id));
            }
            IssuerCommand::MergeRevocationRegistryDeltas(rev_reg_delta_json, other_rev_reg_delta_json, cb) => {
                trace!(target: "issuer_command_executor", "MergeRevocationRegistryDeltas command received");
                cb(self.merge_revocation_registry_deltas(&rev_reg_delta_json, &other_rev_reg_delta_json));
            }
        };
    }

    fn create_schema(&self,
                     issuer_did: &str,
                     name: &str,
                     version: &str,
                     attrs: &str) -> Result<(String, String), IndyError> {
        trace!("create_schema >>> issuer_did: {:?}, name: {:?}, version: {:?}, attrs: {:?}", issuer_did, name, version, attrs);

        self.crypto_service.validate_did(issuer_did)?;

        let attrs: HashSet<String> = serde_json::from_str(attrs)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize AttributeNames: {:?}", err)))?;

        if attrs.is_empty() {
            return Err(IndyError::CommonError(CommonError::InvalidStructure(format!("List of Schema attributes is empty"))));
        }

        let schema_id = build_id(issuer_did, SCHEMA_MARKER, None, name, version);

        let schema = Schema {
            id: schema_id.clone(),
            name: name.to_string(),
            version: version.to_string(),
            attr_names: attrs,
        };

        let schema_json = schema.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize Schema: {:?}", err)))?;

        trace!("create_schema <<< schema_id: {:?}, schema_json: {:?}", schema_id, schema_json);

        Ok((schema_id, schema_json))
    }

    fn create_and_store_credential_definition(&self,
                                              wallet_handle: i32,
                                              issuer_did: &str,
                                              schema_json: &str,
                                              tag: &str,
                                              type_: Option<&str>,
                                              config_json: &str) -> Result<(String, String), IndyError> {
        trace!("create_and_store_credential_definition >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, tag: {:?}, \
              type_: {:?}, config_json: {:?}", wallet_handle, issuer_did, schema_json, tag, type_, config_json);

        self.crypto_service.validate_did(issuer_did)?;

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Schema: {:?}", err)))?;

        let cred_def_config: CredentialDefinitionConfig = CredentialDefinitionConfig::from_json(config_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialDefinitionConfig: {:?}", err)))?;

        let signature_type = match type_ {
            Some(type_) =>
                SignatureTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize SignatureType: {:?}", err)))?,
            None => SignatureTypes::CL,
        };

        let cred_def_id = build_id(issuer_did, CRED_DEF_MARKER, Some(&schema.id), signature_type.to_str(), tag);

        if self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", cred_def_id)).is_ok() {
            return Err(IndyError::AnoncredsError(AnoncredsError::ClaimDefAlreadyExists(format!("CredentialDefinition for cred_def_id: {:?} already exists", cred_def_id))));
        };

        let (credential_definition_value, credential_priv_key, credential_key_correctness_proof) =
            self.anoncreds_service.issuer.new_credential_definition(issuer_did, &schema, cred_def_config.support_revocation)?;

        let credential_definition = CredentialDefinition {
            id: cred_def_id.clone(),
            schema_id: schema.id,
            signature_type,
            tag: tag.to_string(),
            value: credential_definition_value
        };

        let credential_definition_json = self.wallet_service.set_object(wallet_handle,
                                                                        &format!("credential_definition::{}", cred_def_id),
                                                                        &credential_definition,
                                                                        "CredentialDefinition")?;
        self.wallet_service.set_object(wallet_handle,
                                       &format!("credential_private_key::{}", cred_def_id),
                                       &credential_priv_key,
                                       "CredentialPrivateKey")?;
        self.wallet_service.set_object(wallet_handle,
                                       &format!("credential_key_correctness_proof::{}", cred_def_id),
                                       &credential_key_correctness_proof,
                                       "CredentialKeyCorrectnessProof")?;

        trace!("create_and_store_credential_definition <<< cred_def_id: {:?}, credential_definition_json: {:?}", cred_def_id, credential_definition_json);

        Ok((cred_def_id, credential_definition_json))
    }

    fn create_and_store_revocation_registry(&self,
                                            wallet_handle: i32,
                                            issuer_did: &str,
                                            type_: Option<&str>,
                                            tag: &str,
                                            cred_def_id: &str,
                                            config_json: &str,
                                            tails_writer_handle: i32) -> Result<(String, String, String), IndyError> {
        trace!("create_and_store_revocation_registry >>> wallet_handle: {:?}, issuer_did: {:?}, type_: {:?}, tag: {:?}, cred_def_id: {:?}, config_json: {:?}, \
               tails_handle: {:?}",
               wallet_handle, issuer_did, type_, tag, cred_def_id, config_json, tails_writer_handle);

        let rev_reg_config: RevocationRegistryConfig = RevocationRegistryConfig::from_json(config_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryConfig: {:?}", err)))?;

        let rev_reg_type = match type_ {
            Some(type_) =>
                RevocationRegistryTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryType: {:?}", err)))?,
            None => RevocationRegistryTypes::CL_ACCUM,
        };

        let issuance_type = match rev_reg_config.issuance_type {
            Some(type_) =>
                IssuanceTypes::from_json(&format!("\"{}\"", type_))
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize IssuanceType: {:?}", err)))?,
            None => IssuanceTypes::ISSUANCE_ON_DEMAND,
        };

        let max_cred_num = rev_reg_config.max_cred_num.unwrap_or(100000);

        let rev_reg_id = build_id(issuer_did, REV_REG_MARKER, Some(cred_def_id), rev_reg_type.to_str(), tag);

        let credential_def: CredentialDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("credential_definition::{}", &cred_def_id), "CredentialDefinition", &mut String::new())?;

        let (revocation_public_keys, revocation_key_private, revocation_registry, mut revocation_tails_generator) =
            self.anoncreds_service.issuer.new_revocation_registry(&credential_def,
                                                                  max_cred_num,
                                                                  issuance_type.to_bool(),
                                                                  issuer_did)?;

        let (tails_location, tails_hash) =
            store_tails_from_generator(self.blob_storage_service.clone(), tails_writer_handle, &mut revocation_tails_generator)?;

        let revocation_registry_definition_value = RevocationRegistryDefinitionValue {
            max_cred_num,
            issuance_type: issuance_type.clone(),
            public_keys: revocation_public_keys,
            tails_location,
            tails_hash,
        };

        let revocation_registry_definition = RevocationRegistryDefinition {
            id: rev_reg_id.clone(),
            type_: rev_reg_type,
            tag: tag.to_string(),
            cred_def_id: cred_def_id.to_string(),
            value: revocation_registry_definition_value
        };

        let revocation_registry_definition_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), &revocation_registry_definition, "RevocationRegistryDefinition")?;
        let revocation_registry_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), &revocation_registry, "RevocationRegistry")?;
        let revocation_tails_generator_json =
            self.wallet_service.set_object(wallet_handle, &format!("revocation_tails_generator::{}", rev_reg_id), &revocation_tails_generator, "RevocationTailsGenerator")?;

        self.wallet_service.set_object(wallet_handle, &format!("revocation_key_private::{}", rev_reg_id), &revocation_key_private, "RevocationKeyPrivate")?;

        match issuance_type {
            IssuanceTypes::ISSUANCE_BY_DEFAULT =>
                self.wallet_service.set(wallet_handle, &format!("revoked_credential_ids::{}", rev_reg_id), "[]")?,
            IssuanceTypes::ISSUANCE_ON_DEMAND =>
                self.wallet_service.set(wallet_handle, &format!("issued_credential_ids::{}", rev_reg_id), "[]")?
        };
        self.wallet_service.set(wallet_handle, &format!("current_credential_id::{}", rev_reg_id), "0")?;

        trace!("create_and_store_revocation_registry <<< rev_reg_id: {:?}, revocation_registry_definition_json: {:?}, revocation_registry_json: {:?}",
               rev_reg_id, revocation_registry_definition_json, revocation_registry_json);

        Ok((rev_reg_id, revocation_registry_definition_json, revocation_registry_json))
    }

    fn create_credential_offer(&self,
                               wallet_handle: i32,
                               cred_def_id: &str) -> Result<String, IndyError> {
        trace!("create_credential_offer >>> wallet_handle: {:?}, cred_def_id: {:?}", wallet_handle, cred_def_id);

        let key_correctness_proof: CredentialKeyCorrectnessProof =
            self.wallet_service.get_object(wallet_handle, &format!("credential_key_correctness_proof::{}", cred_def_id), "CredentialKeyCorrectnessProof", &mut String::new())?;

        let nonce = new_nonce()
            .map_err(|err| IndyError::AnoncredsError(AnoncredsError::from(err)))?;

        let credential_offer = CredentialOffer {
            cred_def_id: cred_def_id.to_string(),
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
                      cred_offer_json: &str,
                      cred_req_json: &str,
                      cred_values_json: &str,
                      rev_reg_id: Option<&str>,
                      blob_storage_reader_handle: Option<i32>) -> Result<(String, Option<String>, Option<String>), IndyError> {
        trace!("new_credential >>> wallet_handle: {:?}, cred_offer_json: {:?}, cred_req_json: {:?}, cred_values_json: {:?}, rev_reg_id: {:?}, blob_storage_reader_handle: {:?}",
               wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle);

        let credential_offer: CredentialOffer = CredentialOffer::from_json(cred_offer_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialOffer: {:?}", err)))?;

        let credential_request: CredentialRequest = CredentialRequest::from_json(cred_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialRequest: {:?}", err)))?;

        let credential_values: HashMap<String, AttributeValues> = serde_json::from_str(cred_values_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialValues: {:?}", err)))?;

        let credential_def: CredentialDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("credential_definition::{}", credential_request.cred_def_id), "CredentialDefinition", &mut String::new())?;

        let credential_priv_key: CredentialPrivateKey =
            self.wallet_service.get_object(wallet_handle, &format!("credential_private_key::{}", credential_request.cred_def_id), "CredentialPrivateKey", &mut String::new())?;

        let (rev_reg_def, mut rev_reg,
            rev_key_priv, sdk_tails_accessor, cred_rev_id) = match rev_reg_id {
            Some(ref r_reg_id) => {
                let rev_reg_def: RevocationRegistryDefinition =
                    self.wallet_service.get_object(wallet_handle, &format!("revocation_registry_definition::{}", r_reg_id), "RevocationRegistryDefinition", &mut String::new())?;

                let rev_reg: RevocationRegistry =
                    self.wallet_service.get_object(wallet_handle, &format!("revocation_registry::{}", r_reg_id), "RevocationRegistry", &mut String::new())?;

                let rev_key_priv: RevocationKeyPrivate =
                    self.wallet_service.get_object(wallet_handle, &format!("revocation_key_private::{}", r_reg_id), "RevocationKeyPrivate", &mut String::new())?;

                let current_id = self.wallet_service.get(wallet_handle, &format!("current_credential_id::{}", r_reg_id))?;

                let cred_rev_id = 1 + IssuerCommandExecutor::parse_rev_index(&current_id)?;

                if cred_rev_id > rev_reg_def.value.max_cred_num {
                    return Err(IndyError::AnoncredsError(AnoncredsError::AccumulatorIsFull(format!("RevocationRegistryAccumulator is full"))));
                }

                let blob_storage_reader_handle = blob_storage_reader_handle
                    .ok_or(IndyError::CommonError(CommonError::InvalidStructure(format!("TailsReaderHandle not found"))))?;

                let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                               blob_storage_reader_handle,
                                                               &rev_reg_def)?;

                (Some(rev_reg_def), Some(rev_reg), Some(rev_key_priv), Some(sdk_tails_accessor), Some(cred_rev_id))
            }
            None => (None, None, None, None, None)
        };

        let (credential_signature, signature_correctness_proof, rev_reg_delta) =
            self.anoncreds_service.issuer.new_credential(&credential_def,
                                                         &credential_priv_key,
                                                         &credential_offer.nonce,
                                                         &credential_request,
                                                         &credential_values,
                                                         cred_rev_id,
                                                         rev_reg_def.as_ref(),
                                                         rev_reg.as_mut(),
                                                         rev_key_priv.as_ref(),
                                                         sdk_tails_accessor.as_ref())?;

        let (witness, issued, revoked) = if let (&Some(ref r_reg_def), &Some(ref r_reg), &Some(ref r_reg_id), &Some(ref rev_tails_accessor), &Some(ref cred_rev_id)) =
        (&rev_reg_def, &rev_reg, &rev_reg_id, &sdk_tails_accessor, &cred_rev_id) {
            let (issued, revoked) = match r_reg_def.value.issuance_type {
                IssuanceTypes::ISSUANCE_ON_DEMAND => {
                    let mut issued = self.get_ids_list(wallet_handle, &format!("issued_credential_ids::{}", r_reg_id))?;
                    issued.insert(cred_rev_id.to_string());

                    let issued = issued.iter().map(|id| IssuerCommandExecutor::parse_rev_index(id).unwrap().clone()).collect::<HashSet<u32>>();
                    let revoked = HashSet::new();
                    (issued, revoked)
                }
                IssuanceTypes::ISSUANCE_BY_DEFAULT => {
                    let mut revoked = self.get_ids_list(wallet_handle, &format!("revoked_credential_ids::{}", r_reg_id))?;
                    revoked.insert(cred_rev_id.to_string());

                    let revoked = revoked.iter().map(|id| IssuerCommandExecutor::parse_rev_index(id).unwrap().clone()).collect::<HashSet<u32>>();
                    let issued = (1..r_reg_def.value.max_cred_num + 1).collect::<HashSet<u32>>().difference(&revoked).cloned().collect::<HashSet<u32>>();
                    (issued, revoked)
                }
            };

            let rev_reg_delta = RevocationRegistryDelta::from_parts(r_reg, &issued, &revoked)
                .map_err(|err| IndyError::CommonError(CommonError::from(err)))?;

            let witness = Some(Witness::new(cred_rev_id.clone(), r_reg_def.value.max_cred_num, &rev_reg_delta, rev_tails_accessor)
                .map_err(|err| IndyError::CommonError(CommonError::from(err)))?);

            let issued = serde_json::to_string(&issued.iter().map(|index| index.to_string()).collect::<HashSet<String>>())
                .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of Revocation Ids: {:?}", err)))?;

            let revoked = serde_json::to_string(&revoked.iter().map(|index| index.to_string()).collect::<HashSet<String>>())
                .map_err(|err| CommonError::InvalidState(format!("Cannot serialize list of Revocation Ids: {:?}", err)))?;

            (witness, issued, revoked)
        } else {
            (None, String::new(), String::new())
        };


        let credential = Credential {
            cred_def_id: credential_request.cred_def_id.clone(),
            rev_reg_id: rev_reg_id.map(String::from),
            values: credential_values,
            signature: credential_signature,
            signature_correctness_proof,
            rev_reg,
            witness
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

        let cred_rev_id = cred_rev_id.map(|rev_id| rev_id.to_string());

        if let (Some(r_reg_def), Some(r_reg), Some(r_reg_id)) = (rev_reg_def, credential.rev_reg, rev_reg_id) {
            self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", r_reg_id), &r_reg, "RevocationRegistry")?;
            match r_reg_def.value.issuance_type {
                IssuanceTypes::ISSUANCE_ON_DEMAND =>
                    self.wallet_service.set(wallet_handle, &format!("issued_credential_ids::{}", r_reg_id), &issued)?,
                IssuanceTypes::ISSUANCE_BY_DEFAULT =>
                    self.wallet_service.set(wallet_handle, &format!("revoked_credential_ids::{}", r_reg_id), &revoked)?
            }
        };

        trace!("new_credential <<< credential_json: {:?}, cred_rev_id: {:?}, rev_reg_delta_json: {:?}", credential_json, cred_rev_id, rev_reg_delta_json);

        Ok((credential_json, cred_rev_id, rev_reg_delta_json))
    }

    fn parse_rev_index(cred_rev_id: &str) -> Result<u32, IndyError> {
        Ok(cred_rev_id.parse::<u32>()
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot parse CredentialRevocationIndex: {}", err)))?)
    }

    fn get_ids_list(&self, wallet_handle: i32, key: &str) -> Result<HashSet<String>, IndyError> {
        let ids = self.wallet_service.get(wallet_handle, key)?;
        Ok(serde_json::from_str::<HashSet<String>>(&ids)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize list of Revocation Ids: {:?}", err)))?)
    }

    fn revoke_credential(&self,
                         wallet_handle: i32,
                         blob_storage_reader_handle: i32,
                         rev_reg_id: &str,
                         cred_revoc_id: &str) -> Result<String, IndyError> {
        trace!("revoke_credential >>> wallet_handle: {:?}, blob_storage_reader_handle:  {:?}, rev_reg_id: {:?}, cred_revoc_id: {:?}",
               wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id);

        let rev_idx = cred_revoc_id.parse::<u32>()
            .map_err(|_| CommonError::InvalidStructure(format!("Cannot parse CredentialRevocationIndex: {}", cred_revoc_id)))?;

        let revocation_registry_definition: RevocationRegistryDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), "RevocationRegistryDefinition", &mut String::new())?;

        let mut revocation_registry: RevocationRegistry =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), "RevocationRegistry", &mut String::new())?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revocation_registry_definition)?;

        let revocation_registry_delta =
            self.anoncreds_service.issuer.revoke(&mut revocation_registry, revocation_registry_definition.value.max_cred_num, rev_idx, &sdk_tails_accessor)?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?;

        match revocation_registry_definition.value.issuance_type {
            IssuanceTypes::ISSUANCE_ON_DEMAND => {
                let mut issued = self.get_ids_list(wallet_handle, &format!("issued_credential_ids::{}", rev_reg_id))?;

                if !issued.remove(cred_revoc_id) {
                    return Err(IndyError::AnoncredsError(AnoncredsError::InvalidUserRevocIndex(format!("Revocation id: {:?} not found in RevocationRegistry", cred_revoc_id))));
                };
                self.wallet_service.set(wallet_handle, &format!("issued_credential_ids::{}", rev_reg_id), &serde_json::to_string(&issued).unwrap())?;
            }
            IssuanceTypes::ISSUANCE_BY_DEFAULT => {
                let mut revoked = self.get_ids_list(wallet_handle, &format!("revoked_credential_ids::{}", rev_reg_id))?;

                if revoked.contains(&cred_revoc_id.to_string()) {
                    return Err(IndyError::AnoncredsError(AnoncredsError::InvalidUserRevocIndex(format!("Revocation id: {:?} not found in RevocationRegistry", cred_revoc_id))));
                }

                revoked.insert(cred_revoc_id.to_string());
                self.wallet_service.set(wallet_handle, &format!("revoked_credential_ids::{}", rev_reg_id), &serde_json::to_string(&revoked).unwrap())?;
            }
        };

        self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), &revocation_registry, "RevocationRegistry")?;

        trace!("revoke_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }

    fn recovery_credential(&self,
                           wallet_handle: i32,
                           blob_storage_reader_handle: i32,
                           rev_reg_id: &str,
                           cred_revoc_id: &str) -> Result<String, IndyError> {
        trace!("recovery_credential >>> wallet_handle: {:?}, blob_storage_reader_handle: {:?}, rev_reg_id: {:?}, cred_revoc_id: {:?}",
               wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id);

        let rev_idx = cred_revoc_id.parse::<u32>()
            .map_err(|_| CommonError::InvalidStructure(format!("Cannot parse CredentialRevocationIndex: {}", cred_revoc_id)))?;

        let revocation_registry_definition: RevocationRegistryDefinition =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry_definition::{}", rev_reg_id), "RevocationRegistryDefinition", &mut String::new())?;

        let mut revocation_registry: RevocationRegistry =
            self.wallet_service.get_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), "RevocationRegistry", &mut String::new())?;

        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(),
                                                       blob_storage_reader_handle,
                                                       &revocation_registry_definition)?;

        let revocation_registry_delta =
            self.anoncreds_service.issuer.recovery(&mut revocation_registry, revocation_registry_definition.value.max_cred_num, rev_idx, &sdk_tails_accessor)?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?;

        match revocation_registry_definition.value.issuance_type {
            IssuanceTypes::ISSUANCE_ON_DEMAND => {
                let mut issued = self.get_ids_list(wallet_handle, &format!("issued_credential_ids::{}", rev_reg_id))?;

                if issued.contains(&cred_revoc_id.to_string()) {
                    return Err(IndyError::AnoncredsError(AnoncredsError::InvalidUserRevocIndex(format!("Revocation id: {:?} not found in RevocationRegistry", cred_revoc_id))));
                }

                issued.insert(cred_revoc_id.to_string());
                self.wallet_service.set(wallet_handle, &format!("issued_credential_ids::{}", rev_reg_id), &format!("{:?}", issued))?;
            }
            IssuanceTypes::ISSUANCE_BY_DEFAULT => {
                let mut revoked = self.get_ids_list(wallet_handle, &format!("revoked_credential_ids::{}", rev_reg_id))?;

                if !revoked.remove(cred_revoc_id) {
                    return Err(IndyError::AnoncredsError(AnoncredsError::InvalidUserRevocIndex(format!("Revocation id: {:?} not found in RevocationRegistry", cred_revoc_id))));
                }

                self.wallet_service.set(wallet_handle, &format!("revoked_credential_ids::{}", rev_reg_id), &format!("{:?}", revoked))?;
            }
        };

        self.wallet_service.set_object(wallet_handle, &format!("revocation_registry::{}", rev_reg_id), &revocation_registry, "RevocationRegistry")?;

        trace!("recovery_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }

    fn merge_revocation_registry_deltas(&self,
                                        rev_reg_delta_json: &str,
                                        other_rev_reg_delta_json: &str) -> Result<String, IndyError> {
        trace!("merge_revocation_registry_deltas >>> rev_reg_delta_json: {:?}, other_rev_reg_delta_json: {:?}", rev_reg_delta_json, other_rev_reg_delta_json);

        let mut rev_reg_delta: RevocationRegistryDelta = RevocationRegistryDelta::from_json(rev_reg_delta_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize RevocationRegistryDelta: {:?}", err)))?;

        let other_rev_reg_delta: RevocationRegistryDelta = RevocationRegistryDelta::from_json(other_rev_reg_delta_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize other RevocationRegistryDelta: {:?}", err)))?;

        rev_reg_delta.merge(&other_rev_reg_delta)
            .map_err(|err| IndyError::CommonError(CommonError::from(err)))?;

        let merged_rev_reg_delta_json = rev_reg_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta: {:?}", err)))?;

        trace!("merge_revocation_registry_deltas <<< merged_rev_reg_delta: {:?}", merged_rev_reg_delta_json);

        Ok(merged_rev_reg_delta_json)
    }
}
