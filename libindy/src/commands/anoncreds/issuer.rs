extern crate serde_json;
extern crate indy_crypto;

use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;

use services::anoncreds::AnoncredsService;
use services::blob_storage::BlobStorageService;
use services::pool::PoolService;
use services::wallet::WalletService;
use services::anoncreds::types::*;
use services::anoncreds::helpers::get_composite_id;
use std::rc::Rc;
use std::collections::HashMap;
use utils::crypto::base58::Base58;
use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use super::tails::SDKTailsAccessor;

#[allow(dead_code)] //FIXME
pub enum IssuerCommand {
    CreateAndStoreCredentialDefinition(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        Option<String>, // signature type
        bool, // support revocation
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAndStoreRevocationRegistry(
        i32, // wallet handle
        i32, // tails writer config handle
        String, // schema json
        String, // issuer did
        u32, // max credential num
        bool, // issuance by default
        Box<Fn(Result<(String, String, String), IndyError>) + Send>),
    CreateCredentialOffer(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        String, // prover did
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateClaim(
        i32, // wallet handle
        String, // credential req json
        String, // credential json
        Option<u32>, // user revoc index
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    RevokeClaim(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        u32, // user revoc index
        Box<Fn(Result<String, IndyError>) + Send>),
    RecoverClaim(
        i32, // wallet handle
        String, // issuer did
        String, // schema json
        u32, // user revoc index
        Box<Fn(Result<String, IndyError>) + Send>)
}

pub struct IssuerCommandExecutor {
    pub anoncreds_service: Rc<AnoncredsService>,
    pub blob_storage_service: Rc<BlobStorageService>,
    pub pool_service: Rc<PoolService>,
    pub wallet_service: Rc<WalletService>
}

impl IssuerCommandExecutor {
    pub fn new(anoncreds_service: Rc<AnoncredsService>,
               pool_service: Rc<PoolService>,
               blob_storage_service: Rc<BlobStorageService>,
               wallet_service: Rc<WalletService>) -> IssuerCommandExecutor {
        IssuerCommandExecutor {
            anoncreds_service,
            pool_service,
            blob_storage_service,
            wallet_service,
        }
    }

    pub fn execute(&self, command: IssuerCommand) {
        match command {
            IssuerCommand::CreateAndStoreCredentialDefinition(wallet_handle, issuer_did, schema_json, signature_type, create_non_revoc, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreClaimDef command received");
                cb(self.create_and_store_credential_definition(wallet_handle, &issuer_did, &schema_json,
                                                               signature_type.as_ref().map(String::as_str), create_non_revoc));
            }
            IssuerCommand::CreateAndStoreRevocationRegistry(wallet_handle, tails_writer_config_handle, issuer_did, schema_json, max_cred_num, issuance_by_default, cb) => {
                info!(target: "issuer_command_executor", "CreateAndStoreRevocationRegistryRegistry command received");
                cb(self.create_and_store_revocation_registry(wallet_handle, tails_writer_config_handle, &issuer_did, &schema_json, max_cred_num, issuance_by_default));
            }
            IssuerCommand::CreateClaim(wallet_handle, credential_req_json, credential_json, rev_idx, cb) => {
                info!(target: "issuer_command_executor", "CreateClaim command received");
                cb(self.new_credential(wallet_handle, &credential_req_json, &credential_json, rev_idx));
            }
            IssuerCommand::CreateCredentialOffer(wallet_handle, schema_json, issuer_did, prover_did, cb) => {
                info!(target: "issuer_command_executor", "CreateCredentialOffer command received");
                cb(self.create_credential_offer(wallet_handle, &schema_json, &issuer_did, &prover_did));
            }
            IssuerCommand::RevokeClaim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "RevokeClaim command received");
                cb(self.revoke_credential(wallet_handle, &issuer_did, &schema_json, user_revoc_index));
            }
            IssuerCommand::RecoverClaim(wallet_handle, issuer_did, schema_json, user_revoc_index, cb) => {
                info!(target: "issuer_command_executor", "RecoverClaim command received");
                cb(self.recovery_credential(wallet_handle, &issuer_did, &schema_json, user_revoc_index));
            }
        };
    }

    fn create_and_store_credential_definition(&self,
                                              wallet_handle: i32,
                                              issuer_did: &str,
                                              schema_json: &str,
                                              signature_type: Option<&str>,
                                              support_revocation: bool) -> Result<String, IndyError> {
        info!("create_and_store_credential_definition >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, \
              signature_type: {:?}, support_revocation: {:?}", wallet_handle, issuer_did, schema_json, signature_type, support_revocation);

        Base58::decode(&issuer_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid issuer did: {:?}", err)))?;

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize Schema: {:?}", err)))?;

        let id = get_composite_id(issuer_did, &schema.schema_key());

        if self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", id)).is_ok() {
            return Err(IndyError::AnoncredsError(AnoncredsError::ClaimDefAlreadyExists(format!("CredentialDefinition for key: {:?} already exists", id))));
        };

        let (credential_definition, credential_priv_key, credential_key_correctness_proof) =
            self.anoncreds_service.issuer.new_credential_definition(issuer_did, &schema, signature_type, support_revocation)?;

        let credential_definition_json = credential_definition.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialDefinition: {:?}", err)))?;

        let credential_priv_key_json = credential_priv_key.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialPrivateKey: {:?}", err)))?;

        let credential_key_correctness_proof_json = credential_key_correctness_proof.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize CredentialKeyCorrectnessProof: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("credential_definition::{}", id), &credential_definition_json)?;
        self.wallet_service.set(wallet_handle, &format!("credential_private_key::{}", id), &credential_priv_key_json)?;
        self.wallet_service.set(wallet_handle, &format!("credential_key_correctness_proof::{}", id), &credential_key_correctness_proof_json)?;

        info!("create_and_store_credential_definition <<< credential_definition_json: {:?}", credential_definition_json);

        Ok(credential_definition_json)
    }

    fn create_and_store_revocation_registry(&self,
                                            wallet_handle: i32,
                                            tails_writer_config_handle: i32,
                                            issuer_did: &str,
                                            schema_json: &str,
                                            max_cred_num: u32,
                                            issuance_by_default: bool) -> Result<(String, String, String), IndyError> {
        info!("create_and_store_revocation_registry >>> wallet_handle: {:?}, tails_writer_config_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, \
               max_cred_num: {:?}", wallet_handle, tails_writer_config_handle, issuer_did, schema_json, max_cred_num);

        Base58::decode(&issuer_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Issuer DID is invalid: {:?}", err)))?;

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Schema: {:?}", err)))?;

        let id = get_composite_id(issuer_did, &schema.schema_key());

        let credential_def_json = self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", &id))?;
        let credential_def: CredentialDefinition = CredentialDefinition::from_json(&credential_def_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize credential definition: {:?}", err)))?;

        let (revocation_registry_definition, revocation_key_private, revocation_registry, revocation_tails_generator) =
            self.anoncreds_service.issuer.new_revocation_registry(&credential_def,
                                                                  max_cred_num,
                                                                  issuance_by_default,
                                                                  issuer_did,
                                                                  schema.seq_no)?;

        let revocation_registry_definition_json = revocation_registry_definition.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistryDefinition: {:?}", err)))?;

        let revocation_registry_json = revocation_registry.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistry: {:?}", err)))?;

        let revocation_key_private_json = revocation_key_private.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationKeyPrivate: {:?}", err)))?;

        let revocation_tails_generator_json = revocation_tails_generator.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationTailsGenerator: {:?}", err)))?;

        // TODO: store revocation registry using unique identifier(https://jira.hyperledger.org/browse/IS-514).
        self.wallet_service.set(wallet_handle, &format!("revocation_registry_definition::{}", id), &revocation_registry_definition_json)?;
        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", id), &revocation_registry_json)?;
        self.wallet_service.set(wallet_handle, &format!("revocation_key_private::{}", id), &revocation_key_private_json)?;
        self.wallet_service.set(wallet_handle, &format!("revocation_tails_generator::{}", id), &revocation_tails_generator_json)?;

        // TODO: decide about tails storing
        info!("create_and_store_revocation_registry <<< revocation_registry_definition_json: {:?}, revocation_registry_json: {:?}, revocation_tails_generator_json: {:?}",
              revocation_registry_definition_json, revocation_registry_json, revocation_tails_generator_json);

        Ok((revocation_registry_definition_json, revocation_registry_json, revocation_tails_generator_json))
    }

    fn create_credential_offer(&self,
                               wallet_handle: i32,
                               schema_json: &str,
                               issuer_did: &str,
                               prover_did: &str) -> Result<String, IndyError> {
        info!("create_credential_offer >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, prover_did: {:?}",
              wallet_handle, issuer_did, schema_json, prover_did);

        Base58::decode(&issuer_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid issuer did: {:?}", err)))?;

        Base58::decode(&prover_did)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid prover did: {:?}", err)))?;

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid schema json: {}", err.to_string())))?;
        let schema_key = SchemaKey { name: schema.data.name.clone(), version: schema.data.version.clone(), did: schema.dest.clone() };

        let id = get_composite_id(issuer_did, &schema_key);

        self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", id))?;

        let key_correctness_proof_json = self.wallet_service.get(wallet_handle, &format!("key_correctness_proof::{}", id))?;
        let key_correctness_proof = CredentialKeyCorrectnessProof::from_json(&key_correctness_proof_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize key correctness proof: {:?}", err)))?;

        let nonce = new_nonce()
            .map_err(|err| IndyError::AnoncredsError(AnoncredsError::from(err)))?;

        let nonce_json = nonce.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize credential offer: {:?}", err)))?;

        let credential_offer = CredentialOffer {
            issuer_did: issuer_did.to_string(),
            schema_key,
            key_correctness_proof,
            nonce
        };

        let credential_offer_json = credential_offer.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize credential offer: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("nonce::{}::{}", id, prover_did), &nonce_json)?;

        info!("create_credential_offer <<< credential_offer_json: {:?}", credential_offer_json);

        Ok(credential_offer_json)
    }

    fn new_credential(&self,
                      wallet_handle: i32,
                      credential_req_json: &str,
                      credential_json: &str,
                      rev_idx: Option<u32>) -> Result<(String, String), IndyError> {
        info!("new_credential >>> wallet_handle: {:?}, credential_req_json: {:?}, credential_json: {:?}, rev_idx: {:?}",
              wallet_handle, credential_req_json, credential_json, rev_idx);

        let credential_request: CredentialRequest = CredentialRequest::from_json(credential_req_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize credential request: {:?}", err)))?;

        let id = get_composite_id(&credential_request.issuer_did, &credential_request.schema_key);

        let credential_def_json = self.wallet_service.get(wallet_handle, &format!("credential_definition::{}", id))?;
        let credential_def: CredentialDefinition = CredentialDefinition::from_json(&credential_def_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize credential definition: {:?}", err)))?;

        let credential_priv_key_json = self.wallet_service.get(wallet_handle, &format!("credential_private_key::{}", id))?;
        let credential_priv_key = CredentialPrivateKey::from_json(&credential_priv_key_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize credential definition private key: {:?}", err)))?;

        let master_secret_blinding_nonce_json =
            self.wallet_service.get(wallet_handle, &format!("master_secret_blinding_nonce::{}::{}", id, credential_request.prover_did))?;
        let master_secret_blinding_nonce = Nonce::from_json(&master_secret_blinding_nonce_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannot deserialize master_secret_blinding_nonce: {:?}", err)))?;

        let credential_values: HashMap<String, Vec<String>> = serde_json::from_str(credential_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannon deserialize CredentialValues: {:?}", err)))?;

        let rev_reg_def = match self.wallet_service.get(wallet_handle, &format!("revocation_registry_definition::{}", id)) {
            Ok(rev_reg_def_json) =>
                Some(RevocationRegistryDefinition::from_json(&rev_reg_def_json)
                    .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationRegistryDefinition: {:?}", err)))?),
            Err(_) => None
        };

        let mut rev_reg = match self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", id)) {
            Ok(rev_reg_json) =>
                Some(RevocationRegistry::from_json(&rev_reg_json)
                    .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationRegistry: {:?}", err)))?),
            Err(_) => None
        };

        let rev_key_priv = match self.wallet_service.get(wallet_handle, &format!("revocation_registry_private::{}", id)) {
            Ok(rev_key_priv_json) =>
                Some(RevocationKeyPrivate::from_json(&rev_key_priv_json)
                    .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationKeyPrivate: {:?}", err)))?),
            Err(_) => None
        };

        let simple_tails_accessor = match self.wallet_service.get(wallet_handle, &format!("revocation_tails_generator::{}", id)) {
            Ok(revocation_tails_generator_json) => {
                let mut rev_tails_generator = RevocationTailsGenerator::from_json(&revocation_tails_generator_json)
                    .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationTailsGenerator: {:?}", err)))?;
                Some(SimpleTailsAccessor::new(&mut rev_tails_generator).unwrap()) // TODO FIX ITs
            }
            Err(_) => None
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
                                                         simple_tails_accessor.as_ref())?;

        let revocation_registry_json = match rev_reg {
            Some(r_reg) => {
                let rev_reg_json = r_reg.to_json()
                    .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistry: {:?}", err)))?;

                self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", &id), &rev_reg_json)?;
                rev_reg_json
            }
            None => String::new()
        };

        let credential = Credential {
            values: credential_values,
            signature: credential_signature,
            signature_correctness_proof,
            schema_key: credential_request.schema_key,
            issuer_did: credential_request.issuer_did,
            rev_reg_seq_no: None // TODO: How Issuer gets rev_reg_seq_no
        };

        let credential_json = credential.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize Credential: {:?}", err)))?;

        let rev_reg_delta_json = match rev_reg_delta {
            // TODO return Option!
            Some(r_reg_delta) => {
                r_reg_delta.to_json()
                    .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistryDelta: {:?}", err)))?
            }
            None => String::new()
        };

        info!("new_credential <<< rev_reg_delta_json: {:?}, credential_json: {:?}", rev_reg_delta_json, credential_json);

        Ok((rev_reg_delta_json, credential_json))
    }

    fn revoke_credential(&self,
                         wallet_handle: i32,
                         issuer_did: &str,
                         schema_json: &str,
                         user_revoc_index: u32) -> Result<String, IndyError> {
        info!("revoke_credential >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, user_revoc_index: {:?}",
              wallet_handle, issuer_did, schema_json, user_revoc_index);

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid schema json: {}", err.to_string())))?;
        let schema_key = SchemaKey { name: schema.data.name.clone(), version: schema.data.version.clone(), did: schema.dest.clone() };

        let id = get_composite_id(issuer_did, &schema_key);

        let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", id))?;
        let mut revocation_registry = RevocationRegistry::from_json(&revocation_registry_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize revocation registry: {:?}", err)))?;

        let revocation_registry_definition_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_definition::{}", id))?;
        let revocation_registry_definition = RevocationRegistryDefinition::from_json(&revocation_registry_definition_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize  RevocationRegistryDefinition: {:?}", err)))?;

        /*
        let revocation_tails_generator_json = self.wallet_service.get(wallet_handle, &format!("revocation_tails_generator::{}", id))?;
        let mut rev_tails_generator = RevocationTailsGenerator::from_json(&revocation_tails_generator_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationTailsGenerator: {:?}", err)))?;
        let simple_tails_accessor = SimpleTailsAccessor::new(&mut rev_tails_generator).unwrap(); // TODO
        */
        let tails_reader_handle = 0; //FIXME should be param
        let sdk_tails_accessor = SDKTailsAccessor::new(self.blob_storage_service.clone(), tails_reader_handle);

        let revocation_registry_delta =
            self.anoncreds_service.issuer.revoke(&mut revocation_registry, revocation_registry_definition.max_cred_num, user_revoc_index, &sdk_tails_accessor)?;

        let revocation_registry_updated_json = revocation_registry.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistry: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", id), &revocation_registry_updated_json)?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistryDelta: {:?}", err)))?;

        info!("revoke_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }

    fn recovery_credential(&self,
                           wallet_handle: i32,
                           issuer_did: &str,
                           schema_json: &str,
                           user_revoc_index: u32) -> Result<String, IndyError> {
        info!("recovery_credential >>> wallet_handle: {:?}, issuer_did: {:?}, schema_json: {:?}, user_revoc_index: {:?}",
              wallet_handle, issuer_did, schema_json, user_revoc_index);

        let schema: Schema = Schema::from_json(schema_json)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid schema json: {}", err.to_string())))?;
        let schema_key = SchemaKey { name: schema.data.name.clone(), version: schema.data.version.clone(), did: schema.dest.clone() };

        let id = get_composite_id(issuer_did, &schema_key);

        let revocation_registry_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry::{}", id))?;
        let mut revocation_registry = RevocationRegistry::from_json(&revocation_registry_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize revocation registry: {:?}", err)))?;

        let revocation_registry_definition_json = self.wallet_service.get(wallet_handle, &format!("revocation_registry_definition::{}", id))?;
        let revocation_registry_definition = RevocationRegistryDefinition::from_json(&revocation_registry_definition_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize  RevocationRegistryDefinition: {:?}", err)))?;

        let revocation_tails_generator_json = self.wallet_service.get(wallet_handle, &format!("revocation_tails_generator::{}", id))?;
        let mut rev_tails_generator = RevocationTailsGenerator::from_json(&revocation_tails_generator_json)
            .map_err(|err| CommonError::InvalidState(format!("Cannon deserialize RevocationTailsGenerator: {:?}", err)))?;
        let simple_tails_accessor = SimpleTailsAccessor::new(&mut rev_tails_generator).unwrap(); // TODO

        let revocation_registry_delta =
            self.anoncreds_service.issuer.recovery(&mut revocation_registry, revocation_registry_definition.max_cred_num, user_revoc_index, &simple_tails_accessor)?;

        let revocation_registry_updated_json = revocation_registry.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistry: {:?}", err)))?;

        self.wallet_service.set(wallet_handle, &format!("revocation_registry::{}", id), &revocation_registry_updated_json)?;

        let revocation_registry_delta_json = revocation_registry_delta.to_json()
            .map_err(|err| CommonError::InvalidState(format!("Cannon serialize RevocationRegistryDelta: {:?}", err)))?;

        info!("recovery_credential <<< revocation_registry_delta_json: {:?}", revocation_registry_delta_json);

        Ok(revocation_registry_delta_json)
    }
}
