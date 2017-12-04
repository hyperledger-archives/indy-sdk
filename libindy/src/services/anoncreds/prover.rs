extern crate indy_crypto;

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::types::*;
use std::collections::{HashMap, HashSet};
use services::anoncreds::types::{ClaimInfo, RequestedClaims, ProofRequest, PredicateInfo, Identifier};

use self::indy_crypto::cl::*;
use self::indy_crypto::cl::prover;
use services::anoncreds::helpers::*;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }

    pub fn new_master_secret(&self) -> Result<MasterSecret, CommonError> {
        Ok(prover::Prover::new_master_secret()?)
    }

    pub fn new_claim_request(&self, issuer_pub_key: &IssuerPublicKey, master_secret: &MasterSecret, claim_offer: &ClaimOffer,
                             prover_did: &str) -> Result<(ClaimRequest, MasterSecretBlindingData), CommonError> {
        info!(target: "services/anoncreds/prover", "new_claim_request >>> issuer_pub_key: {:?}, master_secret: {:?}, prover_did: {:?}",
              issuer_pub_key, master_secret, prover_did);

        let (blinded_ms, master_secret_blinding_data) = prover::Prover::blind_master_secret(issuer_pub_key, master_secret)?;

        let claim_request = ClaimRequest {
            prover_did: prover_did.to_owned(),
            issuer_did: claim_offer.issuer_did.clone(),
            schema_seq_no: claim_offer.schema_seq_no,
            blinded_ms
        };

        info!(target: "services/anoncreds/prover", "new_claim_request <<< claim_request: {:?}, master_secret_blinding_data: {:?}",
              claim_request, master_secret_blinding_data);

        Ok((claim_request, master_secret_blinding_data))
    }

    pub fn process_claim(&self, claim: &mut Claim, master_secret_blinding_data: &MasterSecretBlindingData,
                         pub_key: &IssuerPublicKey, rev_reg_pub: Option<&RevocationRegistryPublic>) -> Result<(), CommonError> {
        info!(target: "services/anoncreds/prover", "process_claim >>> claim: {:?}, master_secret_blinding_data: {:?}, pub_key: {:?}, rev_reg_pub: {:?}",
              claim, master_secret_blinding_data, pub_key, rev_reg_pub);

        prover::Prover::process_claim_signature(&mut claim.signature,
                                                &master_secret_blinding_data,
                                                &pub_key,
                                                rev_reg_pub)?;

        info!(target: "services/anoncreds/prover", "process_claim <<<");

        Ok(())
    }

    pub fn get_claims_for_proof_req(&self, proof_request: &ProofRequest, claims: &Vec<ClaimInfo>) -> Result<ClaimsForProofRequest, CommonError> {
        info!(target: "services/anoncreds/prover", "get_claims_for_proof_req >>> proof_request: {:?}, claims: {:?}", proof_request, claims);

        let mut found_attributes: HashMap<String, Vec<ClaimInfo>> = HashMap::new();
        let mut found_predicates: HashMap<String, Vec<ClaimInfo>> = HashMap::new();

        for (attr_id, requested_attr) in &proof_request.requested_attrs {
            let mut claims_for_attribute: Vec<ClaimInfo> = Vec::new();

            for claim in claims {
                if claim.values.contains_key(&requested_attr.name) &&
                    if requested_attr.schema_seq_no.is_some() { claim.schema_seq_no == requested_attr.schema_seq_no.unwrap() } else { true } &&
                    if requested_attr.issuer_did.is_some() { claim.issuer_did == requested_attr.issuer_did.clone().unwrap() } else { true } {
                    claims_for_attribute.push(claim.clone());
                }
            }
            found_attributes.insert(attr_id.clone(), claims_for_attribute);
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let mut claims_for_predicate: Vec<ClaimInfo> = Vec::new();

            for claim in claims {
                if let Some(attribute_value) = claim.values.get(&requested_predicate.attr_name) {
                    if Prover::_attribute_satisfy_predicate(&requested_predicate, attribute_value)? &&
                        if requested_predicate.schema_seq_no.is_some() { claim.schema_seq_no == requested_predicate.schema_seq_no.unwrap() } else { true } &&
                        if requested_predicate.issuer_did.is_some() { claim.issuer_did == requested_predicate.issuer_did.clone().unwrap() } else { true } {
                        claims_for_predicate.push(claim.clone());
                    }
                }
            }
            found_predicates.insert(predicate_id.clone(), claims_for_predicate);
        }

        let claims_for_proof_requerst = ClaimsForProofRequest {
            attrs: found_attributes,
            predicates: found_predicates
        };


        info!(target: "services/anoncreds/prover", "get_claims_for_proof_req <<< claims_for_proof_requerst: {:?}", claims_for_proof_requerst);
        Ok(claims_for_proof_requerst)
    }

    pub fn create_proof(&self,
                        claims: &HashMap<String, Claim>,
                        proof_req: &ProofRequest,
                        schemas: &HashMap<String, Schema>,
                        claim_defs: &HashMap<String, ClaimDefinition>,
                        revoc_regs: &HashMap<String, RevocationRegistry>,
                        requested_claims: &RequestedClaims,
                        master_secret: &MasterSecret) -> Result<FullProof, AnoncredsError> {
        info!(target: "services/anoncreds/prover", "create_proof >>> claims: {:?}, proof_req: {:?}, schemas: {:?}, claim_defs: {:?}, revoc_regs: {:?}, \
                       requested_claims: {:?}, master_secret: {:?}", claims, proof_req, schemas, claim_defs, revoc_regs, requested_claims, master_secret);

        let mut proof_builder = prover::Prover::new_proof_builder()?;

        let mut identifiers: HashSet<Identifier> = HashSet::new();

        for (claim_id, claim) in claims {
            let schema = schemas.get(claim_id.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let claim_definition = claim_defs.get(claim_id.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Claim definition not found")))?;
            let revocation_registry = revoc_regs.get(claim_id.as_str());

            let attrs_for_claim = Prover::_get_revealed_attributes_for_claim(claim_id.as_str(), requested_claims, proof_req)?;
            let predicates_for_claim = Prover::_get_predicates_for_claim(claim_id.as_str(), requested_claims, proof_req)?;

            let claim_schema = build_claim_schema(&schema.data.attr_names)?;
            let claim_values = build_claim_values(&claim.values)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_claim, &predicates_for_claim)?;

            proof_builder.add_sub_proof_request(claim_id.as_str(),
                                                &sub_proof_request,
                                                &claim_schema,
                                                &claim.signature,
                                                &claim_values,
                                                &claim_definition.data,
                                                revocation_registry.map(|rev_reg| &rev_reg.data).clone())?;

            identifiers.insert(Identifier {
                schema_seq_no: claim.schema_seq_no,
                issuer_did: claim.issuer_did.clone()
            });
        }

        let proof = proof_builder.finalize(&proof_req.nonce, &master_secret)?;

        let (revealed_attrs, unrevealed_attrs) =
            Prover::_split_attributes(&proof_req, requested_claims, claims)?;

        let requested_proof = RequestedProof {
            self_attested_attrs: requested_claims.self_attested_attributes.clone(),
            revealed_attrs,
            unrevealed_attrs,
            predicates: requested_claims.requested_predicates.clone()
        };

        let full_proof = FullProof {
            proof,
            requested_proof,
            identifiers
        };

        info!(target: "services/anoncreds/prover", "create_proof <<< full_proof: {:?}", full_proof);

        Ok(full_proof)
    }

    fn _attribute_satisfy_predicate(predicate: &PredicateInfo, attribute_value: &String) -> Result<bool, CommonError> {
        info!(target: "services/anoncreds/prover", "_attribute_satisfy_predicate >>> predicate: {:?}, attribute_value: {:?}", predicate, attribute_value);

        let res = match predicate.p_type {
            PredicateType::GE => Ok({
                let attribute_value = attribute_value.parse::<i32>()
                    .map_err(|err| CommonError::InvalidStructure(format!("Ivalid format of predicate attribute: {}", attribute_value)))?;
                attribute_value >= predicate.value
            })
        };

        info!(target: "services/anoncreds/prover", "_attribute_satisfy_predicate <<< res: {:?}", res);

        res
    }

    fn _get_revealed_attributes_for_claim(claim_id: &str, requested_claims: &RequestedClaims, proof_req: &ProofRequest) -> Result<Vec<String>, CommonError> {
        info!(target: "services/anoncreds/prover", "_get_revealed_attributes_for_claim >>> claim_id: {:?}, requested_claims: {:?}, proof_req: {:?}",
              claim_id, requested_claims, proof_req);

        let mut revealed_attrs_for_claim: Vec<String> = Vec::new();

        for (attr_uuid, &(ref requested_claim_id, ref revealed)) in &requested_claims.requested_attrs {
            if claim_id.eq(requested_claim_id) && revealed.clone() {
                if let Some(attr) = proof_req.requested_attrs.get(attr_uuid) {
                    revealed_attrs_for_claim.push(attr.name.clone());
                }
            }
        }

        info!(target: "services/anoncreds/prover", "_get_revealed_attributes_for_claim <<< revealed_attrs_for_claim: {:?}", revealed_attrs_for_claim);

        Ok(revealed_attrs_for_claim)
    }

    fn _get_predicates_for_claim(claim_id: &str, requested_claims: &RequestedClaims, proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!(target: "services/anoncreds/prover", "_get_predicates_for_claim >>> claim_id: {:?}, requested_claims: {:?}, proof_req: {:?}",
              claim_id, requested_claims, proof_req);

        let mut predicates_for_claim: Vec<PredicateInfo> = Vec::new();

        for (predicate_uuid, requested_claim_id) in &requested_claims.requested_predicates {
            if claim_id.eq(requested_claim_id) {
                if let Some(predicate) = proof_req.requested_predicates.get(predicate_uuid) {
                    predicates_for_claim.push(predicate.clone());
                }
            }
        }

        info!(target: "services/anoncreds/prover", "_get_predicates_for_claim <<< predicates_for_claim: {:?}", predicates_for_claim);

        Ok(predicates_for_claim)
    }


    pub fn _split_attributes(proof_req: &ProofRequest, requested_claims: &RequestedClaims,
                             claims: &HashMap<String, Claim>) -> Result<(HashMap<String, (String, String, String)>, HashMap<String, String>), CommonError> {
        info!(target: "services/anoncreds/prover", "_split_attributes >>> proof_req: {:?}, requested_claims: {:?}, claims: {:?}",
              proof_req, requested_claims, claims);

        let mut revealed_attrs: HashMap<String, (String, String, String)> = HashMap::new();
        let mut unrevealed_attrs: HashMap<String, String> = HashMap::new();

        for (attr_uuid, &(ref claim_id, ref revealed)) in &requested_claims.requested_attrs {
            let claim = claims.get(claim_id)
                .ok_or(CommonError::InvalidStructure(format!("Claim not found")))?;

            let attribute = proof_req.requested_attrs.get(attr_uuid)
                .ok_or(CommonError::InvalidStructure(format!("Attribute not found")))?;

            if revealed.clone() {
                let attribute_values = claim.values.get(&attribute.name)
                    .ok_or(CommonError::InvalidStructure(format!("Attributes for claim {} not found", claim_id)))?;

                let value = attribute_values.get(0)
                    .ok_or(CommonError::InvalidStructure(format!("Raw value not found")))?;

                let encoded_value = attribute_values.get(1)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

                revealed_attrs.insert(attr_uuid.clone(), (claim_id.clone(), value.clone(), encoded_value.clone()));
            } else {
                unrevealed_attrs.insert(attr_uuid.clone(), claim_id.clone());
            }
        }

        info!(target: "services/anoncreds/prover", "_split_attributes <<< revealed_attrs: {:?}, unrevealed_attrs: {:?}", revealed_attrs, unrevealed_attrs);

        Ok((revealed_attrs, unrevealed_attrs))
    }
}