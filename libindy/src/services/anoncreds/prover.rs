extern crate indy_crypto;

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::types::*;
use std::collections::{HashMap, HashSet};
use services::anoncreds::types::{ClaimInfo, RequestedClaims, ProofRequest, PredicateInfo, Identifier};

use self::indy_crypto::cl::*;
use self::indy_crypto::cl::prover::Prover as CryptoProver;
use services::anoncreds::helpers::*;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }

    pub fn new_master_secret(&self) -> Result<MasterSecret, CommonError> {
        Ok(CryptoProver::new_master_secret()?)
    }

    pub fn new_claim_request(&self, claim_def_data: &ClaimDefinitionData, master_secret: &MasterSecret, claim_offer: &ClaimOffer,
                             prover_did: &str) -> Result<(ClaimRequest, MasterSecretBlindingData), CommonError> {
        info!("new_claim_request >>> claim_def_data: {:?}, master_secret: {:?}, prover_did: {:?}",
              claim_def_data, master_secret, prover_did);

        let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_def_data.primary, claim_def_data.revocation.as_ref())?;
        let (blinded_ms, master_secret_blinding_data) = CryptoProver::blind_master_secret(&issuer_pub_key, master_secret)?;

        let claim_request = ClaimRequest {
            prover_did: prover_did.to_owned(),
            issuer_did: claim_offer.issuer_did.clone(),
            schema_seq_no: claim_offer.schema_seq_no,
            blinded_ms
        };

        info!("new_claim_request <<< claim_request: {:?}, master_secret_blinding_data: {:?}",
              claim_request, master_secret_blinding_data);

        Ok((claim_request, master_secret_blinding_data))
    }

    pub fn process_claim(&self, claim: &mut Claim, master_secret_blinding_data: &MasterSecretBlindingData,
                         claim_def_data: &ClaimDefinitionData, rev_reg_pub: Option<&RevocationRegistryPublic>) -> Result<(), CommonError> {
        info!("process_claim >>> claim: {:?}, master_secret_blinding_data: {:?}, claim_def_data: {:?}, rev_reg_pub: {:?}",
              claim, master_secret_blinding_data, claim_def_data, rev_reg_pub);

        let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_def_data.primary, claim_def_data.revocation.as_ref())?;
        CryptoProver::process_claim_signature(&mut claim.signature,
                                              &master_secret_blinding_data,
                                              &issuer_pub_key,
                                              rev_reg_pub)?;

        info!("process_claim <<<");

        Ok(())
    }

    pub fn get_claims_for_proof_req(&self, proof_request: &ProofRequest, claims: &Vec<ClaimInfo>) -> Result<ClaimsForProofRequest, CommonError> {
        info!("get_claims_for_proof_req >>> proof_request: {:?}, claims: {:?}", proof_request, claims);

        let mut found_attributes: HashMap<String, Vec<ClaimInfo>> = HashMap::new();
        let mut found_predicates: HashMap<String, Vec<ClaimInfo>> = HashMap::new();

        for (attr_id, requested_attr) in &proof_request.requested_attrs {
            let mut claims_for_attribute: Vec<ClaimInfo> = Vec::new();

            for claim in claims {
                let mut satisfy = claim.attrs.contains_key(&requested_attr.name);

                satisfy = satisfy && Prover::_claim_satisfy_restrictions(claim, &requested_attr.restrictions);

                if satisfy { claims_for_attribute.push(claim.clone()); }
            }

            found_attributes.insert(attr_id.clone(), claims_for_attribute);
        }

        for (predicate_id, requested_predicate) in &proof_request.requested_predicates {
            let mut claims_for_predicate: Vec<ClaimInfo> = Vec::new();

            for claim in claims {
                let mut satisfy = match claim.attrs.get(&requested_predicate.attr_name) {
                    Some(attribute_value) => Prover::_attribute_satisfy_predicate(&requested_predicate, attribute_value)?,
                    None => false
                };

                satisfy = satisfy && Prover::_claim_satisfy_restrictions(claim, &requested_predicate.restrictions);

                if satisfy { claims_for_predicate.push(claim.clone()); }
            }

            found_predicates.insert(predicate_id.clone(), claims_for_predicate);
        }

        let claims_for_proof_requerst = ClaimsForProofRequest {
            attrs: found_attributes,
            predicates: found_predicates
        };


        info!("get_claims_for_proof_req <<< claims_for_proof_requerst: {:?}", claims_for_proof_requerst);
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
        info!("create_proof >>> claims: {:?}, proof_req: {:?}, schemas: {:?}, claim_defs: {:?}, revoc_regs: {:?}, \
                       requested_claims: {:?}, master_secret: {:?}", claims, proof_req, schemas, claim_defs, revoc_regs, requested_claims, master_secret);

        let mut proof_builder = CryptoProver::new_proof_builder()?;

        let mut identifiers: HashSet<Identifier> = HashSet::new();

        for (referent, claim) in claims {
            let schema = schemas.get(referent.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let claim_definition = claim_defs.get(referent.as_str())
                .ok_or(CommonError::InvalidStructure(format!("Claim definition not found")))?;
            let issuer_pub_key = IssuerPublicKey::build_from_parts(&claim_definition.data.primary, claim_definition.data.revocation.as_ref())?;

            let revocation_registry = revoc_regs.get(referent.as_str());

            let attrs_for_claim = Prover::_get_revealed_attributes_for_claim(referent.as_str(), requested_claims, proof_req)?;
            let predicates_for_claim = Prover::_get_predicates_for_claim(referent.as_str(), requested_claims, proof_req)?;

            let claim_schema = build_claim_schema(&schema.data.attr_names)?;
            let claim_values = build_claim_values(&claim.values)?;
            let sub_proof_request = build_sub_proof_request(&attrs_for_claim, &predicates_for_claim)?;

            proof_builder.add_sub_proof_request(referent.as_str(),
                                                &sub_proof_request,
                                                &claim_schema,
                                                &claim.signature,
                                                &claim_values,
                                                &issuer_pub_key,
                                                revocation_registry.map(|rev_reg| &rev_reg.data).clone())?;

            identifiers.insert(Identifier {
                schema_seq_no: claim.schema_seq_no,
                issuer_did: claim.issuer_did.clone(),
                rev_reg_seq_no: claim.rev_reg_seq_no.clone()
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

        info!("create_proof <<< full_proof: {:?}", full_proof);

        Ok(full_proof)
    }

    fn _claim_satisfy_restrictions(claim: &ClaimInfo, restrictions: &Option<Vec<Filter>>) -> bool {
        info!("_claim_satisfy_restrictions >>> claim: {:?}, restrictions: {:?}", claim, restrictions);

        let res = match restrictions {
            &Some(ref restrictions) => restrictions.iter().any(|restriction|
                Prover::_claim_satisfy_restriction(claim, &restriction)),
            &None => true
        };

        info!("_claim_satisfy_restrictions <<< res: {:?}", res);

        res
    }

    fn _claim_satisfy_restriction(claim: &ClaimInfo, restriction: &Filter) -> bool {
        info!("_claim_satisfy_restriction >>> claim: {:?}, restriction: {:?}", claim, restriction);

        let mut res = true;

        if let Some(schema_seq_no) = restriction.schema_seq_no {
            res = res && claim.schema_seq_no == schema_seq_no;
        }

        if let Some(issuer_did) = restriction.issuer_did.clone() {
            res = res && claim.issuer_did == issuer_did;
        }

        info!("_claim_satisfy_restriction >>> res: {:?}", res);

        res
    }

    fn _attribute_satisfy_predicate(predicate: &PredicateInfo, attribute_value: &String) -> Result<bool, CommonError> {
        info!("_attribute_satisfy_predicate >>> predicate: {:?}, attribute_value: {:?}", predicate, attribute_value);

        let res = match predicate.p_type.as_str() {
            ">=" => Ok({
                let attribute_value = attribute_value.parse::<i32>()
                    .map_err(|err| CommonError::InvalidStructure(format!("Ivalid format of predicate attribute: {}", attribute_value)))?;
                attribute_value >= predicate.value
            }),
            _ => return Err(CommonError::InvalidStructure(format!("Invalid predicate type: {:?}", predicate.p_type)))
        };

        info!("_attribute_satisfy_predicate <<< res: {:?}", res);

        res
    }

    fn _get_revealed_attributes_for_claim(referent: &str, requested_claims: &RequestedClaims, proof_req: &ProofRequest) -> Result<Vec<String>, CommonError> {
        info!("_get_revealed_attributes_for_claim >>> referent: {:?}, requested_claims: {:?}, proof_req: {:?}",
              referent, requested_claims, proof_req);

        let mut revealed_attrs_for_claim: Vec<String> = Vec::new();

        for (attr_referent, &(ref requested_referent, ref revealed)) in &requested_claims.requested_attrs {
            if referent.eq(requested_referent) && revealed.clone() {
                if let Some(attr) = proof_req.requested_attrs.get(attr_referent) {
                    revealed_attrs_for_claim.push(attr.name.clone());
                }
            }
        }

        info!("_get_revealed_attributes_for_claim <<< revealed_attrs_for_claim: {:?}", revealed_attrs_for_claim);

        Ok(revealed_attrs_for_claim)
    }

    fn _get_predicates_for_claim(referent: &str, requested_claims: &RequestedClaims, proof_req: &ProofRequest) -> Result<Vec<PredicateInfo>, CommonError> {
        info!("_get_predicates_for_claim >>> referent: {:?}, requested_claims: {:?}, proof_req: {:?}",
              referent, requested_claims, proof_req);

        let mut predicates_for_claim: Vec<PredicateInfo> = Vec::new();

        for (predicate_referent, requested_referent) in &requested_claims.requested_predicates {
            if referent.eq(requested_referent) {
                if let Some(predicate) = proof_req.requested_predicates.get(predicate_referent) {
                    predicates_for_claim.push(predicate.clone());
                }
            }
        }

        info!("_get_predicates_for_claim <<< predicates_for_claim: {:?}", predicates_for_claim);

        Ok(predicates_for_claim)
    }


    pub fn _split_attributes(proof_req: &ProofRequest, requested_claims: &RequestedClaims,
                             claims: &HashMap<String, Claim>) -> Result<(HashMap<String, (String, String, String)>, HashMap<String, String>), CommonError> {
        info!("_split_attributes >>> proof_req: {:?}, requested_claims: {:?}, claims: {:?}",
              proof_req, requested_claims, claims);

        let mut revealed_attrs: HashMap<String, (String, String, String)> = HashMap::new();
        let mut unrevealed_attrs: HashMap<String, String> = HashMap::new();

        for (attr_referent, &(ref referent, ref revealed)) in &requested_claims.requested_attrs {
            let claim = claims.get(referent)
                .ok_or(CommonError::InvalidStructure(format!("Claim not found")))?;

            let attribute = proof_req.requested_attrs.get(attr_referent)
                .ok_or(CommonError::InvalidStructure(format!("Attribute not found")))?;

            if revealed.clone() {
                let attribute_values = claim.values.get(&attribute.name)
                    .ok_or(CommonError::InvalidStructure(format!("Attributes for claim {} not found", referent)))?;

                let value = attribute_values.get(0)
                    .ok_or(CommonError::InvalidStructure(format!("Raw value not found")))?;

                let encoded_value = attribute_values.get(1)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

                revealed_attrs.insert(attr_referent.clone(), (referent.clone(), value.clone(), encoded_value.clone()));
            } else {
                unrevealed_attrs.insert(attr_referent.clone(), referent.clone());
            }
        }

        info!("_split_attributes <<< revealed_attrs: {:?}, unrevealed_attrs: {:?}", revealed_attrs, unrevealed_attrs);

        Ok((revealed_attrs, unrevealed_attrs))
    }
}