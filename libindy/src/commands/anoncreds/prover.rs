extern crate serde_json;
extern crate uuid;
extern crate indy_crypto;

use self::uuid::Uuid;
use errors::common::CommonError;
use errors::indy::IndyError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::AnoncredsService;
use utils::crypto::bn::BigNumber;
use utils::json::{JsonDecodable, JsonEncodable};
use services::wallet::WalletService;
use std::rc::Rc;
use services::anoncreds::helpers::get_composite_id;
use services::anoncreds::types::{ClaimDefinition, Schema, RevocationRegistry, ClaimOfferFilter,
                                 ClaimInfoFilter, ProofJson, ClaimInfo, ProofClaimsJson,
                                 ProofRequestJson, RequestedClaimsJson, ClaimJson, ClaimOffer,
                                 ClaimInitData, RevocationClaimInitData, ClaimRequestJson};
use services::anoncreds::constants::MASTER_SECRET_WALLET_KEY_PREFIX;

use commands::authz::AuthzCommandExecutor;

use std::collections::HashMap;
use std::cell::RefCell;
use utils::crypto::base58::Base58;
use self::indy_crypto::pair::PointG2;

pub enum ProverCommand {
    StoreClaimOffer(
        i32, // wallet handle
        String, // claim offer json
        Box<Fn(Result<(), IndyError>) + Send>
    ),
    GetClaimOffers(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, IndyError>) + Send>
    ),
    CreateMasterSecret(
        i32, // wallet handle
        String, // master secret name
        Box<Fn(Result<(), IndyError>) + Send>
    ),
    CreateAndStoreClaimRequest(
        i32, // wallet handle
        String, // prover_did
        String, // claim offer json
        String, // claim def json
        String, // master secret name
        Option<String>, // policy address name
        Box<Fn(Result<String, IndyError>) + Send>
    ),
    StoreClaim(
        i32, // wallet handle
        String, // claims json
        Box<Fn(Result<(), IndyError>) + Send>
    ),
    GetClaims(
        i32, // wallet handle
        String, // filter json
        Box<Fn(Result<String, IndyError>) + Send>
    ),
    GetClaimsForProofReq(
        i32, // wallet handle
        String, // proof request json
        Box<Fn(Result<String, IndyError>) + Send>
    ),
    CreateProof(
        i32, // wallet handle
        String, // proof request json
        String, // requested claims json
        String, // schemas json
        String, // master secret name,
        String, // policy address
        String, // agent_verkey
        String, // claim defs json
        String, // revoc regs json
        Box<Fn(Result<String, IndyError>) + Send>
    ),
}

pub struct ProverCommandExecutor {
    anoncreds_service: Rc<AnoncredsService>,
    wallet_service: Rc<WalletService>,
}

impl ProverCommandExecutor {
    pub fn new(
        anoncreds_service: Rc<AnoncredsService>,
        wallet_service: Rc<WalletService>,
    ) -> ProverCommandExecutor {
        ProverCommandExecutor {
            anoncreds_service,
            wallet_service,
        }
    }

    pub fn execute(&self, command: ProverCommand) {
        match command {
            ProverCommand::StoreClaimOffer(wallet_handle, claim_offer_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaimOffer command received");
                self.store_claim_offer(wallet_handle, &claim_offer_json, cb);
            }
            ProverCommand::GetClaimOffers(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimOffers command received");
                self.get_claim_offers(wallet_handle, &filter_json, cb);
            }
            ProverCommand::CreateMasterSecret(wallet_handle, master_secret_name, cb) => {
                info!(target: "prover_command_executor", "CreateMasterSecret command received");
                self.create_master_secret(wallet_handle, &master_secret_name, cb);
            }
            ProverCommand::CreateAndStoreClaimRequest(wallet_handle,
                                                      prover_did,
                                                      claim_offer_json,
                                                      claim_def_json,
                                                      master_secret_name,
                                                      policy_address_name,
                                                      cb) => {
                info!(target: "prover_command_executor", "CreateAndStoreClaimRequest command received");
                self.create_and_store_claim_request(
                    wallet_handle,
                    &prover_did,
                    &claim_offer_json,
                    &claim_def_json,
                    &master_secret_name,
                    policy_address_name.as_ref().map(String::as_str),
                    cb,
                );
            }
            ProverCommand::StoreClaim(wallet_handle, claims_json, cb) => {
                info!(target: "prover_command_executor", "StoreClaim command received");
                self.store_claim(wallet_handle, &claims_json, cb);
            }
            ProverCommand::GetClaims(wallet_handle, filter_json, cb) => {
                info!(target: "prover_command_executor", "GetClaims command received");
                self.get_claims(wallet_handle, &filter_json, cb);
            }
            ProverCommand::GetClaimsForProofReq(wallet_handle, proof_req_json, cb) => {
                info!(target: "prover_command_executor", "GetClaimsForProofReq command received");
                self.get_claims_for_proof_req(wallet_handle, &proof_req_json, cb);
            }
            ProverCommand::CreateProof(wallet_handle,
                                       proof_req_json,
                                       requested_claims_json,
                                       schemas_jsons,
                                       master_secret_name,
                                       policy_address,
                                       agent_verkey,
                                       claim_def_jsons,
                                       revoc_regs_jsons,
                                       cb) => {
                info!(target: "prover_command_executor", "CreateProof command received");
                self.create_proof(
                    wallet_handle,
                    &proof_req_json,
                    &requested_claims_json,
                    &schemas_jsons,
                    &master_secret_name,
                    &policy_address,
                    &agent_verkey,
                    &claim_def_jsons,
                    &revoc_regs_jsons,
                    cb,
                );
            }
        };
    }

    fn store_claim_offer(
        &self,
        wallet_handle: i32,
        claim_offer_json: &str,
        cb: Box<Fn(Result<(), IndyError>) + Send>,
    ) {
        cb(self._store_claim_offer(wallet_handle, claim_offer_json));
    }

    fn _store_claim_offer(
        &self,
        wallet_handle: i32,
        claim_offer_json: &str,
    ) -> Result<(), IndyError> {
        let uuid = Uuid::new_v4().to_string();

        let claim_offer: ClaimOffer = ClaimOffer::from_json(claim_offer_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid claim_offer_json: {}", err.to_string()),
                )
            })?;

        Base58::decode(&claim_offer.issuer_did)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!("Invalid issuer did: {}", err.to_string()))
            })?;

        self.wallet_service.set(
            wallet_handle,
            &format!("claim_offer_json::{}", &uuid),
            &claim_offer_json,
        )?;

        Ok(())
    }

    fn get_claim_offers(
        &self,
        wallet_handle: i32,
        filter_json: &str,
        cb: Box<Fn(Result<String, IndyError>) + Send>,
    ) {
        cb(self._get_claim_offers(wallet_handle, filter_json));
    }

    fn _get_claim_offers(
        &self,
        wallet_handle: i32,
        filter_json: &str,
    ) -> Result<String, IndyError> {
        let claim_offer_jsons: Vec<(String, String)> =
            self.wallet_service.list(
                wallet_handle,
                &format!("claim_offer_json::"),
            )?;

        let mut claim_offers: Vec<ClaimOffer> = Vec::new();

        for &(ref uuid, ref claim_offer_json) in claim_offer_jsons.iter() {
            claim_offers.push(ClaimOffer::from_json(claim_offer_json)
                .map_err(map_err_trace!())
                .map_err(|err| {
                    CommonError::InvalidState(
                        format!("Invalid claim_offer_jsons: {}", err.to_string()),
                    )
                })?);
        }

        let filter = ClaimOfferFilter::from_json(filter_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid claim_def_json: {}", err.to_string()),
                )
            })?;

        claim_offers.retain(move |claim_offer| {
            let mut condition = true;
            if let Some(ref issuer_did) = filter.issuer_did {
                condition = condition && claim_offer.issuer_did == issuer_did.clone();
            }
            if let Some(ref schema_seq_no) = filter.schema_seq_no {
                condition = condition && claim_offer.schema_seq_no == schema_seq_no.clone();
            }
            condition
        });

        let claim_offers_json = serde_json::to_string(&claim_offers)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid claim_offers: {}", err.to_string()))
            })?;

        Ok(claim_offers_json)
    }

    fn create_master_secret(
        &self,
        wallet_handle: i32,
        master_secret_name: &str,
        cb: Box<Fn(Result<(), IndyError>) + Send>,
    ) {
        cb(self._create_master_secret(
            wallet_handle,
            master_secret_name,
        ))
    }

    fn _create_master_secret(
        &self,
        wallet_handle: i32,
        master_secret_name: &str,
    ) -> Result<(), IndyError> {
        if self.wallet_service
            .get(
                wallet_handle,
                &ProverCommandExecutor::_master_secret_name_to_wallet_key(master_secret_name),
            )
            .is_ok()
        {
            return Err(IndyError::AnoncredsError(
                AnoncredsError::MasterSecretDuplicateNameError(format!(
                    "Master Secret already exists {}",
                    master_secret_name
                )),
            ));
        };

        let master_secret = self.anoncreds_service.prover.generate_master_secret()?;

        self.wallet_service.set(
            wallet_handle,
            &ProverCommandExecutor::_master_secret_name_to_wallet_key(master_secret_name),
            &master_secret.to_dec()?,
        )?;

        Ok(())
    }

    fn create_and_store_claim_request(
        &self,
        wallet_handle: i32,
        prover_did: &str,
        claim_offer_json: &str,
        claim_def_json: &str,
        master_secret_name: &str,
        policy_address: Option<&str>,
        cb: Box<Fn(Result<String, IndyError>) + Send>,
    ) {
        cb(self._create_and_store_claim_request(
            wallet_handle,
            prover_did,
            claim_offer_json,
            claim_def_json,
            master_secret_name,
            policy_address,
        ))
    }

    fn _create_and_store_claim_request(
        &self,
        wallet_handle: i32,
        prover_did: &str,
        claim_offer_json: &str,
        claim_def_json: &str,
        master_secret_name: &str,
        policy_address_str: Option<&str>,
    ) -> Result<String, IndyError> {
        let master_secret_str = self.wallet_service.get(
            wallet_handle,
            &ProverCommandExecutor::_master_secret_name_to_wallet_key(master_secret_name),
        )?;

        let master_secret = BigNumber::from_dec(&master_secret_str)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid master_secret_str: {}", err.to_string()))
            })?;

        let policy_address = match policy_address_str {
            Some(name) => {
                // Making sure policy address in wallet
                self.wallet_service.get(
                    wallet_handle,
                    &AuthzCommandExecutor::_policy_addr_to_wallet_key(name.to_string()),
                )?;
                Some(BigNumber::from_dec(&name)
                    .map_err(map_err_trace!())
                    .map_err(|err| {
                        CommonError::InvalidState(
                            format!("Invalid policy_address_str: {}", err.to_string()),
                        )
                    })?)
            }
            None => None,
        };

        let claim_def: ClaimDefinition = ClaimDefinition::from_json(&claim_def_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid claim_def_json: {}", err.to_string()),
                )
            })?;

        let claim_offer: ClaimOffer = ClaimOffer::from_json(&claim_offer_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid claim_offer_json: {}", err.to_string()),
                )
            })?;

        Base58::decode(&prover_did)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!("Invalid prover did: {}", err.to_string()))
            })?;


        if claim_def.issuer_did != claim_offer.issuer_did {
            return Err(IndyError::CommonError(
                CommonError::InvalidStructure(format!(
                    "ClaimOffer issuer_did {} does not correspond to ClaimDef issuer_did {:?}",
                    claim_offer.issuer_did,
                    claim_def.issuer_did
                )),
            ));
        }

        if claim_def.schema_seq_no != claim_offer.schema_seq_no {
            return Err(IndyError::CommonError(
                CommonError::InvalidStructure(format!(
                    "ClaimOffer schema_seq_no {} does not correspond to ClaimDef schema_seq_no{}",
                    claim_offer.schema_seq_no,
                    claim_def.schema_seq_no
                )),
            ));
        }

        let (claim_request, primary_claim_init_data, revocation_claim_init_data) =
            self.anoncreds_service.prover.create_claim_request(
                claim_def.data.public_key,
                claim_def
                    .data
                    .public_key_revocation,
                master_secret,
                policy_address,
                prover_did,
            )?;

        let claim_def_id =
            get_composite_id(&claim_offer.issuer_did.clone(), claim_offer.schema_seq_no);
        self.wallet_service.set(
            wallet_handle,
            &format!(
                "claim_definition::{}",
                &claim_def_id
            ),
            &claim_def_json,
        )?;

        let primary_claim_init_data_json = ClaimInitData::to_json(&primary_claim_init_data)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!(
                    "Invalid primary_claim_init_data: {}",
                    err.to_string()
                ))
            })?;

        self.wallet_service.set(
            wallet_handle,
            &format!(
                "primary_claim_init_data::{}",
                &claim_def_id
            ),
            &primary_claim_init_data_json,
        )?;

        if let Some(data) = revocation_claim_init_data {
            let revocation_claim_init_data_json = RevocationClaimInitData::to_json(&data)
                .map_err(map_err_trace!())
                .map_err(|err| {
                    CommonError::InvalidState(format!("Invalid data: {}", err.to_string()))
                })?;

            self.wallet_service.set(
                wallet_handle,
                &format!(
                    "revocation_claim_init_data::{}",
                    &claim_def_id
                ),
                &revocation_claim_init_data_json,
            )?;
        }

        let claim_request = ClaimRequestJson::new(
            claim_request,
            claim_offer.issuer_did,
            claim_offer.schema_seq_no,
        );
        let claim_request_json = ClaimRequestJson::to_json(&claim_request)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid claim_request: {}", err.to_string()))
            })?;

        Ok(claim_request_json)
    }

    fn store_claim(
        &self,
        wallet_handle: i32,
        claims_json: &str,
        cb: Box<Fn(Result<(), IndyError>) + Send>,
    ) {
        cb(self._store_claim(wallet_handle, claims_json));
    }

    fn _store_claim(&self, wallet_handle: i32, claims_json: &str) -> Result<(), IndyError> {
        let claim_json = ClaimJson::from_json(&claims_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!("Invalid claim_json: {}", err.to_string()))
            })?;

        let revoc_reg_id =
            get_composite_id(&claim_json.issuer_did.clone(), claim_json.schema_seq_no);

        let (revocation_registry, revocation_claim_init_data) = match claim_json
            .signature
            .non_revocation_claim {
            Some(_) => {
                let revocation_registry_json = self.wallet_service.get(
                    wallet_handle,
                    &format!(
                        "revocation_registry::{}",
                        &revoc_reg_id
                    ),
                )?;

                let revocation_registry = RevocationRegistry::from_json(&revocation_registry_json)
                    .map_err(map_err_trace!())
                    .map_err(|err| {
                        CommonError::InvalidState(format!(
                            "Invalid revocation_registry_json: {}",
                            err.to_string()
                        ))
                    })?;

                let revocation_claim_init_data_json = self.wallet_service.get(
                    wallet_handle,
                    &format!(
                        "revocation_claim_init_data::{}",
                        &revoc_reg_id
                    ),
                )?;
                let revocation_claim_init_data = RevocationClaimInitData::from_json(
                    &revocation_claim_init_data_json,
                ).map_err(map_err_trace!())
                    .map_err(|err| {
                        CommonError::InvalidState(format!(
                            "Invalid revocation_claim_init_data_json: {}",
                            err.to_string()
                        ))
                    })?;

                (Some(revocation_registry), Some(revocation_claim_init_data))
            }
            _ => (None, None),
        };

        let primary_claim_init_data_json = self.wallet_service.get(
            wallet_handle,
            &format!(
                "primary_claim_init_data::{}",
                &revoc_reg_id
            ),
        )?;
        let primary_claim_init_data = ClaimInitData::from_json(&primary_claim_init_data_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!(
                    "Invalid primary_claim_init_data_json: {}",
                    err.to_string()
                ))
            })?;

        let claim_def_json = self.wallet_service.get(
            wallet_handle,
            &format!(
                "claim_definition::{}",
                &revoc_reg_id
            ),
        )?;
        let claim_def = ClaimDefinition::from_json(&claim_def_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid claim_def_json: {}", err.to_string()))
            })?;

        let claim_json = RefCell::new(claim_json);

        self.anoncreds_service.prover.process_claim(
            &claim_json,
            primary_claim_init_data,
            revocation_claim_init_data,
            claim_def
                .data
                .public_key_revocation,
            &revocation_registry,
        )?;

        let claim = ClaimJson::to_json(&claim_json.borrow())
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid claim_json: {}", err.to_string()))
            })?;

        let uuid = Uuid::new_v4().to_string();
        self.wallet_service.set(
            wallet_handle,
            &format!("claim::{}", &uuid),
            &claim,
        )?;

        Ok(())
    }

    fn get_claims(
        &self,
        wallet_handle: i32,
        filter_json: &str,
        cb: Box<Fn(Result<String, IndyError>) + Send>,
    ) {
        let result = self._get_claims(wallet_handle, filter_json);
        cb(result)
    }

    fn _get_claims(&self, wallet_handle: i32, filter_json: &str) -> Result<String, IndyError> {
        let claims: Vec<(String, String)> =
            self.wallet_service.list(wallet_handle, &format!("claim::"))?;
        let mut claims_info: Vec<ClaimInfo> = ProverCommandExecutor::get_all_claims(claims)?;

        let filter = ClaimInfoFilter::from_json(filter_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!("Invalid filter_json: {}", err.to_string()))
            })?;

        claims_info.retain(move |claim_info| {
            let mut condition = true;

            if let Some(schema_seq_no) = filter.schema_seq_no {
                condition = condition && claim_info.schema_seq_no == schema_seq_no;
            }

            if let Some(issuer_did) = filter.issuer_did.clone() {
                condition = condition && claim_info.issuer_did == issuer_did;
            }

            condition
        });

        let claims_info_json = serde_json::to_string(&claims_info)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid claim_info: {}", err.to_string()))
            })?;

        Ok(claims_info_json)
    }

    fn get_all_claims(claims: Vec<(String, String)>) -> Result<Vec<ClaimInfo>, IndyError> {
        let mut claims_info: Vec<ClaimInfo> = Vec::new();

        for &(ref uuid, ref claim) in claims.iter() {
            let claim_json: ClaimJson = ClaimJson::from_json(claim)
                .map_err(map_err_trace!())
                .map_err(|err| {
                    CommonError::InvalidState(format!("Invalid claim: {}", err.to_string()))
                })?;

            let mut attrs: HashMap<String, String> = HashMap::new();

            for (attr, values) in claim_json.claim {
                attrs.insert(attr.clone(), values[0].clone());
            }

            claims_info.push(ClaimInfo::new(
                uuid.clone(),
                attrs,
                claim_json.schema_seq_no.clone(),
                claim_json.issuer_did.clone(),
            ));
        }

        Ok(claims_info)
    }

    fn get_claims_for_proof_req(
        &self,
        wallet_handle: i32,
        proof_req_json: &str,
        cb: Box<Fn(Result<String, IndyError>) + Send>,
    ) {
        let result = self._get_claims_for_proof_req(wallet_handle, proof_req_json);
        cb(result)
    }

    fn _get_claims_for_proof_req(
        &self,
        wallet_handle: i32,
        proof_req_json: &str,
    ) -> Result<String, IndyError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_json(proof_req_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid proof_req_json: {}", err.to_string()),
                )
            })?;

        let claims: Vec<(String, String)> =
            self.wallet_service.list(wallet_handle, &format!("claim::"))?;
        let claims_info: Vec<ClaimInfo> = ProverCommandExecutor::get_all_claims(claims)?;

        let (attributes, predicates) = self.anoncreds_service.prover.find_claims(
            proof_req.requested_attrs,
            proof_req.requested_predicates,
            claims_info,
        )?;

        let proof_claims = ProofClaimsJson::new(attributes, predicates);

        let proof_claims_json = ProofClaimsJson::to_json(&proof_claims)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid proof_claims: {}", err.to_string()))
            })?;

        Ok(proof_claims_json)
    }
    fn create_proof(
        &self,
        wallet_handle: i32,
        proof_req_json: &str,
        requested_claims_json: &str,
        schemas_jsons: &str,
        master_secret_name: &str,
        policy_address: &str,
        agent_verkey: &str,
        claim_def_jsons: &str,
        revoc_regs_jsons: &str,
        cb: Box<Fn(Result<String, IndyError>) + Send>,
    ) {
        let result = self._create_proof(
            wallet_handle,
            proof_req_json,
            requested_claims_json,
            schemas_jsons,
            master_secret_name,
            policy_address,
            agent_verkey,
            claim_def_jsons,
            revoc_regs_jsons,
        );
        cb(result)
    }

    fn _create_proof(
        &self,
        wallet_handle: i32,
        proof_req_json: &str,
        requested_claims_json: &str,
        schemas_jsons: &str,
        master_secret_name: &str,
        policy_address: &str,
        agent_verkey: &str,
        claim_def_jsons: &str,
        revoc_regs_jsons: &str,
    ) -> Result<String, IndyError> {
        let proof_req: ProofRequestJson = ProofRequestJson::from_json(proof_req_json)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid proof_req_json: {}", err.to_string()),
                )
            })?;

        let schemas: HashMap<String, Schema> = serde_json::from_str(schemas_jsons)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!("Invalid schemas_jsons: {}", err.to_string()))
            })?;

        let claim_defs: HashMap<String, ClaimDefinition> = serde_json::from_str(claim_def_jsons)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid claim_def_jsons: {}", err.to_string()),
                )
            })?;

        let revoc_regs: HashMap<String, RevocationRegistry> = serde_json::from_str(
            revoc_regs_jsons,
        ).map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(
                    format!("Invalid revoc_regs_jsons: {}", err.to_string()),
                )
            })?;

        let requested_claims: RequestedClaimsJson = RequestedClaimsJson::from_json(
            requested_claims_json,
        ).map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidStructure(format!(
                    "Invalid requested_claims_json: {}",
                    err.to_string()
                ))
            })?;

        let mut claims: HashMap<String, ClaimJson> = HashMap::new();

        for claim_uuid in claim_defs.keys() {
            let claim_json = self.wallet_service.get(wallet_handle, &claim_uuid)?;
            let claim = ClaimJson::from_json(&claim_json)
                .map_err(map_err_trace!())
                .map_err(|err| {
                    CommonError::InvalidState(format!("Invalid claim_json: {}", err.to_string()))
                })?;

            claims.insert(claim_uuid.clone(), claim);
        }

        let ms = self.wallet_service.get(
            wallet_handle,
            &ProverCommandExecutor::_master_secret_name_to_wallet_key(master_secret_name),
        )?;

        let ms: BigNumber = BigNumber::from_dec(&ms)?;

        let policy_agent = AuthzCommandExecutor::get_policy_agent_from_wallet(
            &self.wallet_service, wallet_handle, policy_address.to_string(), agent_verkey.to_string())?;
        let policy_address: BigNumber = BigNumber::from_dec(&policy_address)?;

        let mut tails: HashMap<i32, PointG2> = HashMap::new();
        if revoc_regs.len() > 0 {
            // TODO: need to change
            let tails_json = self.wallet_service.get(wallet_handle, &format!("tails"))?;
            tails = serde_json::from_str(&tails_json)
                .map_err(map_err_trace!())
                .map_err(|err| {
                    CommonError::InvalidState(format!("Invalid tails_json: {}", err.to_string()))
                })?;
        }

        let proof_claims = self.anoncreds_service.prover.create_proof(
            claims,
            &proof_req,
            &schemas,
            &claim_defs,
            &revoc_regs,
            &requested_claims,
            &ms,
            &policy_address,
            policy_agent,
            &tails,
        )?;

        let proof_claims_json = ProofJson::to_json(&proof_claims)
            .map_err(map_err_trace!())
            .map_err(|err| {
                CommonError::InvalidState(format!("Invalid proof_claims: {}", err.to_string()))
            })?;

        Ok(proof_claims_json)
    }

    fn _master_secret_name_to_wallet_key(master_secret_name: &str) -> String {
        format!(
            "{}::{}",
            MASTER_SECRET_WALLET_KEY_PREFIX,
            master_secret_name
        )
    }
}
