extern crate indy_crypto;

use errors::common::CommonError;
use errors::anoncreds::AnoncredsError;
use services::anoncreds::constants::*;
use services::anoncreds::types::*;
use services::anoncreds::helpers::*;
use services::anoncreds::verifier::Verifier;
use services::anoncreds::issuer::Issuer;
use utils::crypto::bn::BigNumber;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use services::anoncreds::types::{AttributeInfo, ClaimInfo, RequestedClaimsJson, ProofRequestJson};
use std::iter::FromIterator;

use self::indy_crypto::pair::{GroupOrderElement, PointG1, PointG2, Pair};
use self::indy_crypto::cl::prover::Prover as CryptoProver;

use services::anoncreds::converters::*;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }

    pub fn generate_master_secret(&self) -> Result<BigNumber, CommonError> {
        BigNumber::rand(LARGE_MASTER_SECRET)
    }

    pub fn create_claim_request(&self, pk: PublicKey, pkr: Option<RevocationPublicKey>, ms: BigNumber,
                                policy_address: Option<BigNumber>, prover_did: &str) -> Result<(ClaimRequest, ClaimInitData, Option<RevocationClaimInitData>), CommonError> {
        info!(target: "anoncreds_service", "Prover create claim request -> start");
        let pub_key = PublicKey_to_CredentialPublicKey(&pk)?;

        let primary_blinded_credential_secrets =
            CryptoProver::_generate_primary_blinded_credential_secrets(&pub_key.p_key, &gen_hidden_CredentialValues(&ms, policy_address)?)?;

        let primary_claim_init_data = PrimaryBlindedCredentialSecretsFactors_to_ClaimInitData(primary_blinded_credential_secrets)?;

        let revocation_claim_init_data = match pkr {
            Some(pk_r) => Some(Prover::_generate_revocation_claim_init_data(&pk_r)?),
            _ => None
        };

        info!(target: "anoncreds_service", "Prover create claim request -> done");
        Ok((
            ClaimRequest::new(prover_did.to_string(),
                              primary_claim_init_data.u.clone()?,
                              revocation_claim_init_data.clone().map(|ref d| d.u)),
            primary_claim_init_data,
            revocation_claim_init_data
        ))
    }

    fn _gen_primary_claim_init_data(public_key: &PublicKey, ms: &BigNumber) -> Result<ClaimInitData, CommonError> {
        let mut ctx = BigNumber::new_context()?;
        let v_prime = BigNumber::rand(LARGE_VPRIME)?;

        let u = public_key.s
            .mod_exp(&v_prime, &public_key.n, Some(&mut ctx))?
            .mul(
                &public_key.rms.mod_exp(&ms, &public_key.n, Some(&mut ctx))?,
                None
            )?
            .modulus(&public_key.n, Some(&mut ctx))?;

        Ok(ClaimInitData::new(u, v_prime))
    }

    fn _generate_revocation_claim_init_data(pkr: &RevocationPublicKey) -> Result<RevocationClaimInitData, CommonError> {
        let vr_prime = GroupOrderElement::new()?;
        let ur = pkr.h2.mul(&vr_prime)?;
        Ok(RevocationClaimInitData::new(ur, vr_prime))
    }

    pub fn process_claim(&self, claim_json: &RefCell<ClaimJson>, primary_claim_init_data: ClaimInitData,
                         revocation_claim_init_data: Option<RevocationClaimInitData>,
                         pkr: Option<RevocationPublicKey>, revoc_reg: &Option<RevocationRegistry>)
                         -> Result<(), CommonError> {
        info!(target: "anoncreds_service", "Prover process received claim -> start");

        Prover::_init_primary_claim(claim_json, &primary_claim_init_data.v_prime)?;

        if let Some(ref non_revocation_claim) = claim_json.borrow().signature.non_revocation_claim {
            Prover::_init_non_revocation_claim(non_revocation_claim,
                                               &revocation_claim_init_data.
                                                   ok_or(CommonError::InvalidStructure("Field v_prime not found".to_string()))?.v_prime,
                                               &pkr
                                                   .ok_or(CommonError::InvalidStructure("Field pkr not found".to_string()))?,
                                               &revoc_reg.clone()
                                                   .ok_or(CommonError::InvalidStructure("Field revoc_reg not found".to_string()))?.accumulator,
                                               &revoc_reg.clone()
                                                   .ok_or(CommonError::InvalidStructure("Field revoc_reg not found".to_string()))?.acc_pk)?;
        }
        info!(target: "anoncreds_service", "Prover process received claim -> done");

        Ok(())
    }

    pub fn _init_primary_claim(claim: &RefCell<ClaimJson>, v_prime: &BigNumber) -> Result<(), CommonError> {
        let ref mut primary_claim = claim.borrow_mut().signature.primary_claim;
        primary_claim.v = v_prime.add(&primary_claim.v)?;
        Ok(())
    }

    pub fn _init_non_revocation_claim(claim: &RefCell<NonRevocationClaim>, v_prime: &GroupOrderElement,
                                      pkr: &RevocationPublicKey, acc: &Accumulator, acc_pk: &AccumulatorPublicKey)
                                      -> Result<(), CommonError> {
        let mut claim_mut = claim.borrow_mut();
        let m2 = BigNumber::from_bytes(&claim_mut.m2.to_bytes()?)?;
        claim_mut.vr_prime_prime = v_prime.add_mod(&claim_mut.vr_prime_prime)?;
        Prover::_test_witness_credential(&claim_mut, pkr, acc, acc_pk, &m2)?;
        Ok(())
    }

    pub fn _test_witness_credential(claim: &NonRevocationClaim, pkr: &RevocationPublicKey, acc: &Accumulator,
                                    acc_pk: &AccumulatorPublicKey, context_attribute: &BigNumber) -> Result<(), CommonError> {
        let z_calc = Pair::pair(&claim.witness.g_i, &acc.acc)?
            .mul(&Pair::pair(&pkr.g, &claim.witness.omega)?.inverse()?)?;
        if z_calc != acc_pk.z {
            return Err(CommonError::InvalidStructure("issuer is sending incorrect data".to_string()));
        }
        let pair_gg_calc = Pair::pair(&pkr.pk.add(&claim.g_i)?, &claim.witness.sigma_i)?;
        let pair_gg = Pair::pair(&pkr.g, &pkr.g_dash)?;
        if pair_gg_calc != pair_gg {
            return Err(CommonError::InvalidStructure("issuer is sending incorrect data".to_string()));
        }

        let m2 = GroupOrderElement::from_bytes(&context_attribute.to_bytes()?)?;

        let pair_h1 = Pair::pair(&claim.sigma, &pkr.y.add(&pkr.h_cap.mul(&claim.c)?)?)?;
        let pair_h2 = Pair::pair(
            &pkr.h0
                .add(&pkr.h1.mul(&m2)?)?
                .add(&pkr.h2.mul(&claim.vr_prime_prime)?)?
                .add(&claim.g_i)?,
            &pkr.h_cap
        )?;
        if pair_h1 != pair_h2 {
            return Err(CommonError::InvalidStructure("issuer is sending incorrect data".to_string()));
        }

        Ok(())
    }

    pub fn find_claims(&self, requested_attrs: HashMap<String, AttributeInfo>, requested_predicates: HashMap<String, Predicate>,
                       claims: Vec<ClaimInfo>)
                       -> Result<(HashMap<String, Vec<ClaimInfo>>, HashMap<String, Vec<ClaimInfo>>), CommonError> {
        info!(target: "anoncreds_service", "Prover find claims for proof request -> start");

        let mut found_attributes: HashMap<String, Vec<ClaimInfo>> = HashMap::new();
        let mut found_predicates: HashMap<String, Vec<ClaimInfo>> = HashMap::new();

        for (uuid, attribute_info) in requested_attrs {
            let claims_for_attribute: Vec<ClaimInfo> =
                claims.iter().cloned()
                    .filter(|claim|
                        claim.attrs.contains_key(&attribute_info.name) &&
                            if attribute_info.schema_seq_no.is_some() { claim.schema_seq_no == attribute_info.schema_seq_no.unwrap() } else { true } &&
                            if attribute_info.issuer_did.is_some() { claim.issuer_did == attribute_info.issuer_did.clone().unwrap() } else { true })
                    .collect();

            found_attributes.insert(uuid, claims_for_attribute);
        }

        for (uuid, predicate) in requested_predicates {
            let mut claims_for_predicate: Vec<ClaimInfo> = Vec::new();

            for claim in claims.iter() {
                if let Some(attribute_value) = claim.attrs.get(&predicate.attr_name) {
                    if Prover::_attribute_satisfy_predicate(&predicate, attribute_value)? &&
                        if predicate.schema_seq_no.is_some() { claim.schema_seq_no == predicate.schema_seq_no.unwrap() } else { true } &&
                        if predicate.issuer_did.is_some() { claim.issuer_did == predicate.issuer_did.clone().unwrap() } else { true } {
                        claims_for_predicate.push(claim.clone());
                    }
                }
            }
            found_predicates.insert(uuid, claims_for_predicate);
        }

        info!(target: "anoncreds_service", "Prover find claims for proof request -> done");
        Ok((found_attributes, found_predicates))
    }

    fn _attribute_satisfy_predicate(predicate: &Predicate, attribute_value: &String) -> Result<bool, CommonError> {
        match predicate.p_type {
            PredicateType::GE => Ok({
                let attribute_value = attribute_value.parse::<i32>()
                    .map_err(|err|
                        CommonError::InvalidStructure(
                            format!("Ivalid format of predicate attribute: {}", attribute_value)))?;
                attribute_value >= predicate.value
            })
        }
    }

    fn _prepare_proof_claims(proof_req: &ProofRequestJson,
                             schemas: &HashMap<String, Schema>,
                             claim_defs: &HashMap<String, ClaimDefinition>,
                             revoc_regs: &HashMap<String, RevocationRegistry>,
                             requested_claims: &RequestedClaimsJson,
                             claims: HashMap<String, ClaimJson>) -> Result<HashMap<String, ProofClaims>, CommonError> {
        let mut proof_claims: HashMap<String, ProofClaims> = HashMap::new();

        for (claim_uuid, claim) in claims {
            let schema = schemas.get(&claim_uuid)
                .ok_or(CommonError::InvalidStructure(format!("Schema not found")))?;
            let claim_definition = claim_defs.get(&claim_uuid)
                .ok_or(CommonError::InvalidStructure(format!("Claim definition not found")))?;
            let revocation_registry = revoc_regs.get(&claim_uuid);

            let mut predicates_for_claim: Vec<Predicate> = Vec::new();

            for (predicate_uuid, claim_uuid_for_predicate) in &requested_claims.requested_predicates {
                if claim_uuid_for_predicate.clone() == claim_uuid {
                    let predicate = proof_req.requested_predicates.get(predicate_uuid)
                        .ok_or(CommonError::InvalidStructure(format!("Predicate not found")))?;

                    predicates_for_claim.push(predicate.clone());
                }
            }

            let mut revealed_attrs_for_claim: Vec<String> = Vec::new();
            let mut unrevealed_attrs_for_claim: Vec<String> = Vec::new();

            for (attr_uuid, &(ref claim_uuid_for_attr, ref revealed)) in &requested_claims.requested_attrs {
                if claim_uuid_for_attr.clone() == claim_uuid.clone() {
                    let attr = proof_req.requested_attrs.get(attr_uuid)
                        .ok_or(CommonError::InvalidStructure(format!("Attribute not found")))?;

                    if revealed.clone() {
                        revealed_attrs_for_claim.push(attr.name.clone());
                    } else {
                        unrevealed_attrs_for_claim.push(attr.name.clone());
                    }
                }
            }

            let proof_claim = ProofClaims::new(claim,
                                               schema.clone(),
                                               claim_definition.clone()?,
                                               revocation_registry.map(|r| r.clone()),
                                               predicates_for_claim,
                                               revealed_attrs_for_claim,
                                               unrevealed_attrs_for_claim);

            proof_claims.insert(claim_uuid.clone(), proof_claim);
        }
        Ok(proof_claims)
    }

    pub fn _split_attributes(proof_req: &ProofRequestJson,
                             requested_claims: &RequestedClaimsJson,
                             attributes: &HashMap<String, HashMap<String, Vec<String>>>)
                             -> Result<(HashMap<String, (String, String, String)>, HashMap<String, String>), CommonError> {
        let mut revealed_attrs: HashMap<String, (String, String, String)> = HashMap::new();
        let mut unrevealed_attrs: HashMap<String, String> = HashMap::new();

        for (attr_uuid, &(ref claim_uuid, ref revealed)) in &requested_claims.requested_attrs {
            let attribute = proof_req.requested_attrs.get(attr_uuid)
                .ok_or(CommonError::InvalidStructure(format!("Attribute not found")))?;

            if revealed.clone() {
                let attribute = attributes.get(claim_uuid)
                    .ok_or(CommonError::InvalidStructure(format!("Attributes for claim {} not found", claim_uuid)))?
                    .get(&attribute.name).unwrap();

                let value = attribute.get(0)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

                let encoded_value = attribute.get(1)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

                revealed_attrs.insert(attr_uuid.clone(), (claim_uuid.clone(), value.clone(), encoded_value.clone()));
            } else {
                unrevealed_attrs.insert(attr_uuid.clone(), claim_uuid.clone());
            }
        }

        Ok((revealed_attrs, unrevealed_attrs))
    }

    pub fn create_proof(&self,
                        claims: HashMap<String, ClaimJson>,
                        proof_req: &ProofRequestJson,
                        schemas: &HashMap<String, Schema>,
                        claim_defs: &HashMap<String, ClaimDefinition>,
                        revoc_regs: &HashMap<String, RevocationRegistry>,
                        requested_claims: &RequestedClaimsJson,
                        ms: &BigNumber,
                        tails: &HashMap<i32, PointG2>)
                        -> Result<ProofJson, AnoncredsError> {
        info!(target: "anoncreds_service", "Prover create proof -> start");

//        let proof_builder = CryptoProver::new_proof_builder()?;

        let proof_claims = Prover::_prepare_proof_claims(proof_req,
                                                         schemas,
                                                         claim_defs,
                                                         revoc_regs,
                                                         requested_claims,
                                                         claims)?;

        let m1_tilde = BigNumber::rand(LARGE_M2_TILDE)?;

        let mut init_proofs: HashMap<String, InitProof> = HashMap::new();
        let mut c_list: Vec<Vec<u8>> = Vec::new();
        let mut tau_list: Vec<Vec<u8>> = Vec::new();

        for (proof_claim_uuid, proof_claim) in &proof_claims {
            let mut non_revoc_init_proof = None;
            let mut m2_tilde: Option<BigNumber> = None;

            if let Some(ref non_revocation_claim) = proof_claim.claim_json.signature.non_revocation_claim.clone() {
                let proof = Prover::_init_non_revocation_proof(non_revocation_claim,
                                                               &proof_claim.revocation_registry.clone()
                                                                   .ok_or(CommonError::InvalidStructure("Revocation registry not found".to_string()))?
                                                                   .accumulator,
                                                               &proof_claim.claim_definition.data.public_key_revocation.clone()
                                                                   .ok_or(CommonError::InvalidStructure("Field public_key_revocation not found".to_string()))?,
                                                               tails)?;

                c_list.extend_from_slice(&proof.as_c_list()?);
                tau_list.extend_from_slice(&proof.as_tau_list()?);
                m2_tilde = Some(group_element_to_bignum(&proof.tau_list_params.m2)?);
                non_revoc_init_proof = Some(proof);
            }

            let primary_init_proof = Prover::_init_proof(&proof_claim.claim_definition.data.public_key,
                                                         &proof_claim.schema,
                                                         &proof_claim.claim_json.signature.primary_claim,
                                                         &proof_claim.claim_json.claim,
                                                         &proof_claim.revealed_attrs,
                                                         &proof_claim.predicates,
                                                         &m1_tilde,
                                                         m2_tilde)?;

            c_list.extend_from_slice(&primary_init_proof.as_c_list()?);
            tau_list.extend_from_slice(&primary_init_proof.as_tau_list()?);

            let init_proof = InitProof::new(primary_init_proof, non_revoc_init_proof);

            init_proofs.insert(proof_claim_uuid.clone(), init_proof);
        }

        let mut values: Vec<Vec<u8>> = Vec::new();
        values.extend_from_slice(&tau_list);
        values.extend_from_slice(&c_list);
        values.push(proof_req.nonce.to_bytes()?);

        let c_h = get_hash_as_int(&mut values)?;

        let mut proofs: HashMap<String, ClaimProof> = HashMap::new();
        let mut attributes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        for (proof_claim_uuid, init_proof) in init_proofs.iter() {
            let proof_claim = proof_claims.get(proof_claim_uuid)
                .ok_or(CommonError::InvalidStructure(format!("Claim not found")))?;

            let mut non_revoc_proof: Option<NonRevocProof> = None;
            if let Some(ref non_revoc_init_proof) = init_proof.non_revoc_init_proof {
                non_revoc_proof = Some(Prover::_finalize_non_revocation_proof(&non_revoc_init_proof,
                                                                              &c_h)?);
            }

            let primary_proof = Prover::_finalize_proof(&ms,
                                                        &init_proof.primary_init_proof,
                                                        &c_h,
                                                        &proof_claim.claim_json.claim,
                                                        &proof_claim.revealed_attrs)?;

            let proof = Proof {
                primary_proof: primary_proof,
                non_revoc_proof: non_revoc_proof
            };

            let claim_proof = ClaimProof::new(proof,
                                              proof_claim.claim_json.schema_seq_no,
                                              proof_claim.claim_json.issuer_did.clone());

            proofs.insert(proof_claim_uuid.clone(), claim_proof);
            attributes.insert(proof_claim_uuid.clone(), proof_claim.claim_json.claim.clone());
        }

        let aggregated_proof = AggregatedProof::new(c_h, c_list);

        let (revealed_attrs, unrevealed_attrs) = Prover::_split_attributes(&proof_req, requested_claims, &attributes)?;

        let requested_proof = RequestedProofJson::new(revealed_attrs,
                                                      unrevealed_attrs,
                                                      requested_claims.self_attested_attributes.clone(),
                                                      requested_claims.requested_predicates.clone()
        );

        info!(target: "anoncreds_service", "Prover create proof -> done");
        Ok(ProofJson::new(proofs, aggregated_proof, requested_proof))
    }

    fn _init_proof(pk: &PublicKey, schema: &Schema, c1: &PrimaryClaim, attributes: &HashMap<String, Vec<String>>,
                   revealed_attrs: &Vec<String>, predicates: &Vec<Predicate>, m1_t: &BigNumber,
                   m2_t: Option<BigNumber>) -> Result<PrimaryInitProof, CommonError> {
        info!(target: "anoncreds_service", "Prover init primary proof -> start");
        let eq_proof = Prover::_init_eq_proof(&pk, schema, c1, revealed_attrs, m1_t, m2_t)?;


        let mut ge_proofs: Vec<PrimaryPredicateGEInitProof> = Vec::new();

        for predicate in predicates.iter() {
            let ge_proof = Prover::_init_ge_proof(&pk, &eq_proof.mtilde, attributes, predicate)?;
            ge_proofs.push(ge_proof);
        }

        info!(target: "anoncreds_service", "Prover init primary proof -> done");
        Ok(PrimaryInitProof::new(eq_proof, ge_proofs))
    }

    fn _init_non_revocation_proof(claim: &RefCell<NonRevocationClaim>, accum: &Accumulator,
                                  pkr: &RevocationPublicKey, tails: &HashMap<i32, PointG2>)
                                  -> Result<NonRevocInitProof, AnoncredsError> {
        info!(target: "anoncreds_service", "Prover init non-revocation proof -> start");
        Prover::_update_non_revocation_claim(claim, accum, tails)?;

        let c_list_params = Prover::_gen_c_list_params(&claim)?;
        let proof_c_list = Prover::_create_c_list_values(&claim, &c_list_params, &pkr)?;

        let tau_list_params = Prover::_gen_tau_list_params()?;
        let proof_tau_list = Issuer::_create_tau_list_values(&pkr, &accum, &tau_list_params, &proof_c_list)?;

        info!(target: "anoncreds_service", "Prover init non-revocation proof -> done");
        Ok(NonRevocInitProof::new(c_list_params, tau_list_params, proof_c_list, proof_tau_list))
    }

    fn _update_non_revocation_claim(claim: &RefCell<NonRevocationClaim>,
                                    accum: &Accumulator, tails: &HashMap<i32, PointG2>)
                                    -> Result<(), AnoncredsError> {
        if !accum.v.contains(&claim.borrow().i) {
            return Err(AnoncredsError::ClaimRevoked("Can not update Witness. Claim revoked.".to_string()));
        }

        if claim.borrow().witness.v != accum.v {
            let mut mut_claim = claim.borrow_mut();

            let v_old_minus_new: HashSet<i32> =
                mut_claim.witness.v.difference(&accum.v).cloned().collect();
            let v_new_minus_old: HashSet<i32> =
                accum.v.difference(&mut_claim.witness.v).cloned().collect();
            let mut omega_denom = PointG2::new_inf()?;
            for j in v_old_minus_new.iter() {
                omega_denom = omega_denom.add(
                    tails.get(&(accum.max_claim_num + 1 - j + mut_claim.i))
                        .ok_or(CommonError::InvalidStructure(format!("Key not found {} in tails", accum.max_claim_num + 1 - j + mut_claim.i)))?)?;
            }
            let mut omega_num = PointG2::new_inf()?;
            let mut new_omega: PointG2 = mut_claim.witness.omega.clone();
            for j in v_old_minus_new.iter() {
                omega_num = omega_num.add(
                    tails.get(&(accum.max_claim_num + 1 - j + mut_claim.i))
                        .ok_or(CommonError::InvalidStructure(format!("Key not found {} in tails", accum.max_claim_num + 1 - j + mut_claim.i)))?)?;
                new_omega = new_omega.add(
                    &omega_num.sub(&omega_denom)?
                )?;
            }

            mut_claim.witness.v = accum.v.clone();
            mut_claim.witness.omega = new_omega;
        }

        Ok(())
    }

    fn _init_eq_proof(pk: &PublicKey, schema: &Schema, c1: &PrimaryClaim, revealed_attrs: &Vec<String>,
                      m1_tilde: &BigNumber, m2_t: Option<BigNumber>) -> Result<PrimaryEqualInitProof, CommonError> {
        let mut ctx = BigNumber::new_context()?;

        let m2_tilde = m2_t.unwrap_or(BigNumber::rand(LARGE_MVECT)?);

        let r = BigNumber::rand(LARGE_VPRIME)?;
        let etilde = BigNumber::rand(LARGE_ETILDE)?;
        let vtilde = BigNumber::rand(LARGE_VTILDE)?;

        let unrevealed_attrs: Vec<String> =
            schema.data.attr_names
                .difference(&HashSet::from_iter(revealed_attrs.iter().cloned()))
                .map(|attr| attr.clone())
                .collect::<Vec<String>>();

        let mtilde = get_mtilde(&unrevealed_attrs)?;

        let aprime = pk.s
            .mod_exp(&r, &pk.n, Some(&mut ctx))?
            .mul(&c1.a, Some(&mut ctx))?
            .modulus(&pk.n, Some(&mut ctx))?;

        let large_e_start = BigNumber::from_dec(&LARGE_E_START.to_string())?;

        let vprime = c1.v.sub(
            &c1.e.mul(&r, Some(&mut ctx))?
        )?;

        let eprime = c1.e.sub(
            &BigNumber::from_dec("2")?.exp(&large_e_start, Some(&mut ctx))?
        )?;

        let t = Verifier::calc_teq(
            &pk, &aprime, &etilde, &vtilde, &mtilde, &m1_tilde, &m2_tilde, &unrevealed_attrs)?;

        Ok(
            PrimaryEqualInitProof::new(
                aprime, t, etilde, eprime, vtilde, vprime, mtilde,
                m1_tilde.clone()?, m2_tilde, c1.m2.clone()?
            )
        )
    }

    fn _init_ge_proof(pk: &PublicKey, mtilde: &HashMap<String, BigNumber>,
                      encoded_attributes: &HashMap<String, Vec<String>>, predicate: &Predicate)
                      -> Result<PrimaryPredicateGEInitProof, CommonError> {
        let mut ctx = BigNumber::new_context()?;
        let (k, value) = (&predicate.attr_name, predicate.value);

        let attr_value = encoded_attributes.get(&k[..])
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in c1.encoded_attributes", k)))?
            .get(0)
            .ok_or(CommonError::InvalidStructure(format!("Value not found in c1.encoded_attributes")))?
            .parse::<i32>()
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Value by key '{}' has invalid format", k)))?;

        let delta: i32 = attr_value - value;

        if delta < 0 {
            return Err(CommonError::InvalidStructure("Predicate is not satisfied".to_string()));
        }

        let u = four_squares(delta)?;

        let mut r: HashMap<String, BigNumber> = HashMap::new();
        let mut t: HashMap<String, BigNumber> = HashMap::new();
        let mut c_list: Vec<BigNumber> = Vec::new();

        for i in 0..ITERATION {
            let cur_u = u.get(&i.to_string())
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in u1", i)))?;

            let cur_r = BigNumber::rand(LARGE_VPRIME)?;

            let cut_t = pk.z
                .mod_exp(&cur_u, &pk.n, Some(&mut ctx))?
                .mul(
                    &pk.s.mod_exp(&cur_r, &pk.n, Some(&mut ctx))?,
                    Some(&mut ctx)
                )?
                .modulus(&pk.n, Some(&mut ctx))?;

            r.insert(i.to_string(), cur_r);
            t.insert(i.to_string(), cut_t.clone()?);
            c_list.push(cut_t)
        }

        let r_delta = BigNumber::rand(LARGE_VPRIME)?;

        let t_delta = pk.z
            .mod_exp(&BigNumber::from_dec(&delta.to_string())?, &pk.n, Some(&mut ctx))?
            .mul(
                &pk.s.mod_exp(&r_delta, &pk.n, Some(&mut ctx))?,
                Some(&mut ctx)
            )?
            .modulus(&pk.n, Some(&mut ctx))?;

        r.insert("DELTA".to_string(), r_delta);
        t.insert("DELTA".to_string(), t_delta.clone()?);
        c_list.push(t_delta);

        let mut utilde: HashMap<String, BigNumber> = HashMap::new();
        let mut rtilde: HashMap<String, BigNumber> = HashMap::new();

        for i in 0..ITERATION {
            utilde.insert(i.to_string(), BigNumber::rand(LARGE_UTILDE)?);
            rtilde.insert(i.to_string(), BigNumber::rand(LARGE_RTILDE)?);
        }

        rtilde.insert("DELTA".to_string(), BigNumber::rand(LARGE_RTILDE)?);
        let alphatilde = BigNumber::rand(LARGE_ALPHATILDE)?;

        let mj = mtilde.get(&k[..])
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in eq_proof.mtilde", k)))?;

        let tau_list = Verifier::calc_tge(&pk, &utilde, &rtilde, &mj, &alphatilde, &t)?;

        Ok(PrimaryPredicateGEInitProof::new(
            c_list, tau_list, u, utilde, r, rtilde, alphatilde, predicate.clone(), t
        ))
    }

    fn _finalize_eq_proof(ms: &BigNumber, init_proof: &PrimaryEqualInitProof, c_h: &BigNumber,
                          encoded_attributes: &HashMap<String, Vec<String>>, revealed_attrs: &Vec<String>)
                          -> Result<PrimaryEqualProof, CommonError> {
        info!(target: "anoncreds_service", "Prover finalize primary proof -> start");
        let mut ctx = BigNumber::new_context()?;

        let keys_hash_set: HashSet<String> = HashSet::from_iter(encoded_attributes.keys().cloned());
        let unrevealed_attrs: Vec<String> =
            keys_hash_set
                .difference(&HashSet::from_iter(revealed_attrs.iter().cloned()))
                .map(|attr| attr.clone())
                .collect::<Vec<String>>();


        let e = c_h
            .mul(&init_proof.eprime, Some(&mut ctx))?
            .add(&init_proof.etilde)?;

        let v = c_h
            .mul(&init_proof.vprime, Some(&mut ctx))?
            .add(&init_proof.vtilde)?;

        let mut m: HashMap<String, BigNumber> = HashMap::new();

        for k in unrevealed_attrs.iter() {
            let cur_mtilde = init_proof.mtilde.get(k)
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.mtilde", k)))?;
            let cur_val = encoded_attributes.get(k)
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_prook.c1", k)))?
                .get(1)
                .ok_or(CommonError::InvalidStructure(format!("Encoded Value not found in init_prook.c1")))?;

            let val = c_h
                .mul(&BigNumber::from_dec(cur_val)?,
                     Some(&mut ctx))?
                .add(&cur_mtilde)?;

            m.insert(k.clone(), val);
        }

        let m1 = c_h
            .mul(&ms, Some(&mut ctx))?
            .add(&init_proof.m1_tilde)?;

        let m2 = c_h
            .mul(&init_proof.m2, Some(&mut ctx))?
            .add(&init_proof.m2_tilde)?;


        let mut revealed_attrs_with_values: HashMap<String, String> = HashMap::new();

        for attr in revealed_attrs.iter() {
            revealed_attrs_with_values.insert(
                attr.clone(),
                encoded_attributes
                    .get(attr)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?
                    .get(1)
                    .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?
                    .clone(),
            );
        }

        info!(target: "anoncreds_service", "Prover finalize primary proof -> done");

        Ok(PrimaryEqualProof::new(
            revealed_attrs_with_values, init_proof.a_prime.clone()?, e, v, m, m1, m2
        ))
    }

    fn _finalize_ge_proof(c_h: &BigNumber, init_proof: &PrimaryPredicateGEInitProof,
                          eq_proof: &PrimaryEqualProof) -> Result<PrimaryPredicateGEProof, CommonError> {
        let mut ctx = BigNumber::new_context()?;
        let mut u: HashMap<String, BigNumber> = HashMap::new();
        let mut r: HashMap<String, BigNumber> = HashMap::new();
        let mut urproduct = BigNumber::new()?;

        for i in 0..ITERATION {
            let cur_utilde = init_proof.u_tilde.get(&i.to_string())
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.u_tilde", i)))?;
            let cur_u = init_proof.u.get(&i.to_string())
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.u", i)))?;
            let cur_rtilde = init_proof.r_tilde.get(&i.to_string())
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r_tilde", i)))?;
            let cur_r = init_proof.r.get(&i.to_string())
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", i)))?;

            let new_u: BigNumber = c_h
                .mul(&cur_u, Some(&mut ctx))?
                .add(&cur_utilde)?;
            let new_r: BigNumber = c_h
                .mul(&cur_r, Some(&mut ctx))?
                .add(&cur_rtilde)?;

            u.insert(i.to_string(), new_u);
            r.insert(i.to_string(), new_r);

            urproduct = cur_u
                .mul(&cur_r, Some(&mut ctx))?
                .add(&urproduct)?;

            let cur_rtilde_delta = init_proof.r_tilde.get("DELTA")
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r_tilde", "DELTA")))?;
            let cur_r_delta = init_proof.r.get("DELTA")
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", "DELTA")))?;

            let new_delta = c_h
                .mul(&cur_r_delta, Some(&mut ctx))?
                .add(&cur_rtilde_delta)?;

            r.insert("DELTA".to_string(), new_delta);
        }

        let r_delta = init_proof.r.get("DELTA")
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", "DELTA")))?;

        let alpha = r_delta
            .sub(&urproduct)?
            .mul(&c_h, Some(&mut ctx))?
            .add(&init_proof.alpha_tilde)?;

        let mj = eq_proof.m.get(&init_proof.predicate.attr_name)
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in eq_proof.m", init_proof.predicate.attr_name)))?;

        Ok(PrimaryPredicateGEProof::new(
            u, r, mj.clone()?, alpha, clone_bignum_map(&init_proof.t)?, init_proof.predicate.clone()
        ))
    }

    fn _finalize_proof(ms: &BigNumber, init_proof: &PrimaryInitProof, c_h: &BigNumber,
                       encoded_attributes: &HashMap<String, Vec<String>>, revealed_attrs: &Vec<String>)
                       -> Result<PrimaryProof, CommonError> {
        info!(target: "anoncreds_service", "Prover finalize proof -> start");

        let eq_proof = Prover::_finalize_eq_proof(ms, &init_proof.eq_proof, c_h, encoded_attributes, revealed_attrs)?;
        let mut ge_proofs: Vec<PrimaryPredicateGEProof> = Vec::new();

        for init_ge_proof in init_proof.ge_proofs.iter() {
            let ge_proof = Prover::_finalize_ge_proof(c_h, init_ge_proof, &eq_proof)?;
            ge_proofs.push(ge_proof);
        }

        info!(target: "anoncreds_service", "Prover finalize proof -> done");

        Ok(PrimaryProof::new(eq_proof, ge_proofs))
    }

    fn _gen_c_list_params(claim: &RefCell<NonRevocationClaim>) -> Result<NonRevocProofXList, CommonError> {
        let claim = claim.borrow();
        let rho = GroupOrderElement::new()?;
        let r = GroupOrderElement::new()?;
        let r_prime = GroupOrderElement::new()?;
        let r_prime_prime = GroupOrderElement::new()?;
        let r_prime_prime_prime = GroupOrderElement::new()?;
        let o = GroupOrderElement::new()?;
        let o_prime = GroupOrderElement::new()?;
        let m = rho.mul_mod(&claim.c)?;
        let m_prime = r.mul_mod(&r_prime_prime)?;
        let t = o.mul_mod(&claim.c)?;
        let t_prime = o_prime.mul_mod(&r_prime_prime)?;
        let m2 = GroupOrderElement::from_bytes(&claim.m2.to_bytes()?)?;

        Ok(NonRevocProofXList::new(rho, r, r_prime, r_prime_prime, r_prime_prime_prime, o, o_prime,
                                   m, m_prime, t, t_prime, m2, claim.vr_prime_prime, claim.c))
    }

    fn _create_c_list_values(claim: &RefCell<NonRevocationClaim>, params: &NonRevocProofXList,
                             pkr: &RevocationPublicKey) -> Result<NonRevocProofCList, CommonError> {
        let claim = claim.borrow();
        let e = pkr.h
            .mul(&params.rho)?
            .add(
                &pkr.htilde.mul(&params.o)?
            )?;

        let d = pkr.g
            .mul(&params.r)?
            .add(
                &pkr.htilde.mul(&params.o_prime)?
            )?;

        let a = claim.sigma
            .add(
                &pkr.htilde.mul(&params.rho)?
            )?;

        let g = claim.g_i
            .add(
                &pkr.htilde.mul(&params.r)?
            )?;

        let w = claim.witness.omega
            .add(
                &pkr.h_cap.mul(&params.r_prime)?
            )?;

        let s = claim.witness.sigma_i
            .add(
                &pkr.h_cap.mul(&params.r_prime_prime)?
            )?;

        let u = claim.witness.u_i
            .add(
                &pkr.h_cap.mul(&params.r_prime_prime_prime)?
            )?;

        Ok(NonRevocProofCList::new(e, d, a, g, w, s, u))
    }

    fn _gen_tau_list_params() -> Result<NonRevocProofXList, CommonError> {
        Ok(NonRevocProofXList::new(GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?,
                                   GroupOrderElement::new()?, GroupOrderElement::new()?))
    }

    fn _finalize_non_revocation_proof(init_proof: &NonRevocInitProof, c_h: &BigNumber) -> Result<NonRevocProof, CommonError> {
        info!(target: "anoncreds_service", "Prover finalize non-revocation proof -> start");

        let ch_num_z = bignum_to_group_element(&c_h)?;
        let mut x_list: Vec<GroupOrderElement> = Vec::new();

        for (x, y) in init_proof.tau_list_params.as_list()?.iter().zip(init_proof.c_list_params.as_list()?.iter()) {
            x_list.push(x.add_mod(
                &ch_num_z.mul_mod(&y)?.mod_neg()?
            )?);
        }

        info!(target: "anoncreds_service", "Prover finalize non-revocation proof -> done");

        Ok(NonRevocProof::new(NonRevocProofXList::from_list(x_list), init_proof.c_list.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::anoncreds::verifier;
    use services::anoncreds::issuer;

    #[test]
    fn gen_primary_claim_init_data_works() {
        let pk = issuer::mocks::get_pk();
        let ms = BigNumber::from_dec("48366230316716542900569044107436065507876331091941474824005719405764413438920").unwrap();

        let res = Prover::_gen_primary_claim_init_data(&pk, &ms);
        assert!(res.is_ok());

        let claim_init_data = res.unwrap();

        assert_eq!(claim_init_data.v_prime.to_dec().unwrap(), "1921424195886158938744777125021406748763985122590553448255822306242766229793715475428833504725487921105078008192433858897449555181018215580757557939320974389877538474522876366787859030586130885280724299566241892352485632499791646228580480458657305087762181033556428779333220803819945703716249441372790689501824842594015722727389764537806761583087605402039968357991056253519683582539703803574767702877615632257021995763302779502949501243649740921598491994352181379637769188829653918416991301420900374928589100515793950374255826572066003334385555085983157359122061582085202490537551988700484875690854200826784921400257387622318582276996322436");
        assert_eq!(claim_init_data.u.to_dec().unwrap(), "76242448573590064405016258439737389305308751658939430245286640100438960019281437749200830095828154995656490316795623959413004501644803662299479412591058642431687903660665344655065168625525452586969727169375623723517902861969847048691526377607004762208719937819914640316377295513994692345889814194525691804485221810462520684486465466644645762808386096321825027491677390741996765477089812850102636281290306349225021109750689221122813209585062598487297616077690207210647793480450738894724087937015208576263139374972514675875069264408157796307069688316536519870595147545540606129541475897775356097530317320274539032783922");
    }

    #[test]
    fn init_primary_claim_works() {
        let claim_json = RefCell::new(mocks::get_gvt_claims_json());
        let v_prime = BigNumber::from_dec("21337277489659209697972694275961549241988800625063594810959897509238282352238626810206496164796042921922944861660722790127270481494898810301213699637204250648485409496039792926329367175253071514098050800946366413356551955763141949136004248502185266508852158851178744042138131595587172830689293368213380666221485155781604582222397593802865783047420570234359112294991344669207835283314629238445531337778860979843672592610159700225195191155581629856994556889434019851156913688584355226534153997989337803825600096764199505457938355614863559831818213663754528231270325956208966779676675180767488950507044412716354924086945804065215387295334083509").unwrap();

        let old_value = claim_json.borrow().signature.primary_claim.v.clone().unwrap();

        let res = Prover::_init_primary_claim(&claim_json, &v_prime);
        assert!(res.is_ok());

        assert_ne!(old_value, claim_json.borrow().signature.primary_claim.v);

        let new_v = BigNumber::from_dec("6477858587997811893327035319417510316563341854132851390093281262022504586945336581881563055213337677056181844572991952555932751996898440671581814053127951224635658321050035511444973918938951286397608407154945420576869136257515796028414378962335588462012678546940230947218473631620847322671867296043124087586400291121388864996880108619720604815227218240238018894734106036749434566128263766145147938204864471079326020636108875736950439614174893113941785014290729562585035442317715573694490415783867707489645644928275501455034338736759260129329435713263029873859553709178436828106858314991461880152652981178848566237411834715936997680351679484278048175488999620056712097674305032686536393318931401622256070852825807510445941751166073917118721482407482663237596774153152864341413225983416965337899803365905987145336353882936").unwrap();
        assert_eq!(new_v, claim_json.borrow().signature.primary_claim.v);
    }

    #[test]
    fn prepare_proof_claims_works() {
        let proof_req = mocks::get_proof_req_json();
        let mut schemas: HashMap<String, Schema> = HashMap::new();
        schemas.insert("1".to_string(), issuer::mocks::get_gvt_schema());
        schemas.insert("2".to_string(), issuer::mocks::get_xyz_schema());

        let mut claim_defs: HashMap<String, ClaimDefinition> = HashMap::new();
        claim_defs.insert("1".to_string(), mocks::get_gvt_claim_definition());
        claim_defs.insert("2".to_string(), mocks::get_xyz_claim_definition());

        let revoc_regs: HashMap<String, RevocationRegistry> = HashMap::new();

        let requested_claims = mocks::get_requested_claims();

        let mut claims: HashMap<String, ClaimJson> = HashMap::new();
        claims.insert("1".to_string(), mocks::get_gvt_claims_json());
        claims.insert("2".to_string(), mocks::get_xyz_claims_json());

        let res = Prover::_prepare_proof_claims(&proof_req, &schemas, &claim_defs, &revoc_regs, &requested_claims, claims);
        assert!(res.is_ok());

        let proof_claims = res.unwrap();

        assert_eq!(2, proof_claims.len());

        let gvt_proof_claim = proof_claims.get("1").unwrap();
        assert_eq!(1, gvt_proof_claim.revealed_attrs.len());
        assert_eq!(1, gvt_proof_claim.unrevealed_attrs.len());
        assert_eq!(2, gvt_proof_claim.predicates.len());

        assert_eq!(gvt_proof_claim.revealed_attrs[0], "name".to_string());
        assert_eq!(gvt_proof_claim.unrevealed_attrs[0], "sex".to_string());

        let xyz_proof_claim = proof_claims.get("2").unwrap();
        assert_eq!(1, xyz_proof_claim.revealed_attrs.len());
        assert_eq!(0, xyz_proof_claim.unrevealed_attrs.len());
        assert_eq!(0, xyz_proof_claim.predicates.len());

        assert_eq!(xyz_proof_claim.revealed_attrs[0], "status".to_string());
    }

    #[test]
    fn init_proof_works() {
        let pk = issuer::mocks::get_pk();
        let claim = mocks::get_gvt_primary_claim();
        let revealed_attrs = mocks::get_revealed_attrs();
        let m1_t = BigNumber::from_dec("21544287380986891419162473617242441136231665555467324140952028776483657408525689082249184862870856267009773225408151321864247533184196094757877079561221602250888815228824796823045594410522810417051146366939126434027952941761214129885206419097498982142646746254256892181011609282364766769899756219988071473111").unwrap();
        let m2_t = BigNumber::from_dec("20019436401620609773538287054563349105448394091395718060076065683409192012223520437097245209626164187921545268202389347437258706857508181049451308664304690853807529189730523256422813648391821847776735976798445049082387614903898637627680273723153113532585372668244465374990535833762731556501213399533698173874").unwrap();
        let predicates = vec![mocks::get_gvt_predicate()];
        let encoded_attributes = issuer::mocks::get_gvt_attributes();
        let schema = issuer::mocks::get_gvt_schema();
        let res = Prover::_init_proof(&pk, &schema, &claim, &encoded_attributes, &revealed_attrs, &predicates, &m1_t, Some(m2_t));

        assert!(res.is_ok());
    }

    #[test]
    fn finalize_proof_works() {
        let proof = mocks::get_primary_init_proof();
        let ms = BigNumber::from_dec("12017662702207397635206788416861773342711375658894915181302218291088885004642").unwrap();
        let c_h = BigNumber::from_dec("107686359310664445046126368677755391247164319345083587464043204013905993527834").unwrap();
        let encoded_attributes = issuer::mocks::get_gvt_attributes();
        let revealed_attributes = mocks::get_revealed_attrs();

        let res = Prover::_finalize_proof(&ms, &proof, &c_h, &encoded_attributes, &revealed_attributes);

        assert!(res.is_ok());
    }

    #[test]
    fn init_eq_proof_works() {
        let pk = issuer::mocks::get_pk();
        let claim = mocks::get_gvt_primary_claim();
        let revealed_attrs = mocks::get_revealed_attrs();
        let m1_tilde = BigNumber::from_dec("101699538176051593371744225919046760532786718077106502466570844730111441686747507159918166345843978280307167319698104055171476367527139548387778863611093261001762539719090094485796865232109859717006503205961984033284239500178635203251080574429593379622288524622977721677439771060806446693275003002447037756467").unwrap();
        let m2_tilde = BigNumber::from_dec("31230114293795576487127595372834830220228562310818079039836555160797619323909214967951444512173906589379330228717887451770324874651295781099491258571562527679146158488391908045190667642630077485518774594787164364584431134524117765512651773418307564918922308711232172267389727003411383955005915276810988726136").unwrap();
        let schema = issuer::mocks::get_gvt_schema();

        let res = Prover::_init_eq_proof(&pk, &schema, &claim, &revealed_attrs, &m1_tilde, Some(m2_tilde));
        assert!(res.is_ok());

        let eq_proof = res.unwrap();

        assert_eq!(eq_proof.a_prime.to_dec().unwrap(), "87057631969731126162889320560906357360267008247046682344994037071540708847648211770817155467322576564416024131016702461829141826154593193141015555408707962107434889154274101480021851047519249826871065068045489054940673687307364802393856912954529821530366129214823349578250933984191619715737300481000921545131737892947565265902387824838694421659738826630417546849137080518569690367670216680263229483688777919442405436226899082217495953507207561863892643215763362913098682050328209689762892828408774897957041802696642645714627207453405565027136962897066680484021579390417804092995897134437003639398170927787299154075285");
        assert_eq!(eq_proof.vprime.to_dec().unwrap(), "5979547362044420689643605161847007473090081436212966743842241286592937826625276385813360906453355392545643230503360670090004097274446022944279570878276259729306779668575697214067216866429507821180867566895648038856148919510059621853730813107074415548724255552174426281218098200918679203779943916397256259606901368304143824867249078714432422027782927278071444841086260224951432527743093933778851959693368146789991602066025734455616272412130589236198988320593653003193963066617573884531391745988882862687993383824150400809323307293852247592582410221809104069581125010219396759971113914000795860997210346078905489329838723780453966406654041083307266391458113165288688592430952227431062675696350809783088665646193119746626057641646852972527804891696692352131972390096122206815139645180412672265386643453131031235225649159719");
        assert_eq!(eq_proof.eprime.to_dec().unwrap(), "421208355533376344033560360084200567");
    }

    #[test]
    fn finalize_eq_proof_works() {
        let ms = BigNumber::from_dec("12017662702207397635206788416861773342711375658894915181302218291088885004642").unwrap();
        let c_h = BigNumber::from_dec("65052515950080385170056404271846666093263620691254624189854445495335700076548").unwrap();
        let init_proof = mocks::get_primary_equal_init_proof();
        let revealed_attrs = mocks::get_revealed_attrs();
        let attrs = issuer::mocks::get_gvt_attributes();

        let res = Prover::_finalize_eq_proof(&ms, &init_proof, &c_h, &attrs, &revealed_attrs);

        assert!(res.is_ok());
        let proof = res.unwrap();

        assert_eq!("46977509037563772921188771357228696971286986611479769037400887043024357260824466323972528739266623662424083138906804114233154076462225260", proof.e.to_dec().unwrap());
        assert_eq!("555894869457553465718054497220703310113847971154321206264039643437256150021765032391630230094549373082683761872900289443108844758698210311744008775755841424663713495335913737925610645231143512448736634848872651673398623671421680147672048516992617074237823416006998805743252732623168072887558380980816786967972208697482105496476584623670241498051382948079749991653743122008317688039944886441991890739570646377897115078595023503848923611116244104325820581549132685254973230215813377280331818752749674933449141701081762918502111898869410069368198046357103361141404701610657859033620340201121860748524404546417655599945090144921881183922296151990310095095955070183524924902826674801457725425394553828477598974723668103655265677518938090134981829839176785641671819341783587890027487090232485080219343288188381028474008022615299819430842220715432262971141278304167669686965655751310509796666256987764202199558192225907485643584", proof.v.to_dec().unwrap());
        assert_eq!("17884736668674953594474879343533841182802514514784532835710262264561805009458126297222977824304362311586622997817594769134550513911169868072027461607531075593532027490623438201429184516874637111394210856531406371117724109267267829196540990374669452129657796333114585130056514558678918989249474063851032294543", proof.m1.to_dec().unwrap());
        assert_eq!("33970939655505026872690051065527896936826240486176548712174703648151652129591217103741946892383483806205993341432925544541557374346350172352729633028700080895510117255197249531019938518779850139061087723518395934746900289855498383299025412840993553136695018502936439397825288787933388062548604655707739594437", proof.m2.to_dec().unwrap());
        assert_eq!("2976250595835739181594320238227653601426197318110939190760657852629456864395726135468275792741622452401456587655635268677703907105682407452071286027329441960908939293198715566259", proof.m.get("age").unwrap().to_dec().unwrap());
    }

    #[test]
    fn init_ge_proof_works() {
        let pk = issuer::mocks::get_pk();
        let eq_proof = mocks::get_primary_equal_init_proof();
        let predicate = mocks::get_gvt_predicate();
        let encoded_attributes = issuer::mocks::get_gvt_attributes();

        let res = Prover::_init_ge_proof(&pk, &eq_proof.mtilde, &encoded_attributes, &predicate);
        assert!(res.is_ok());

        let proof = res.unwrap();

        assert_eq!(proof.c_list.get(0).unwrap().to_dec().unwrap(), "66452646864713459129322124524496160239214129628844448912512675754382373114045232638792544050983258044571320479724542222159607548371946608278224646356448366445559828934782665270370014756906222313296871353700305312489013107502898521331193640487262241439496025903490697084701289251331970932030723857963667757918065298468726954493148633682914144253830507074421917845317843041768030700610129944001550144134234321487234247282527013708361275163765747931441214224397693734342806818103569845752619756970663088347173537279465064357347197203519585032404779938843725754592220777310937230037486845412937230858545348334751626327225");
        assert_eq!(proof.c_list.get(4).unwrap().to_dec().unwrap(), "12744073002342538466174266178920319716851536025528365678772164359094855375069597510967107907963978165383581958746728451817220119885059854369802587463275692110468863903085692788520163123046996971844187140303651001700638819763809506725152408953126623513326965559836659294476633000658736763344051801272123315367972537058814718428582311569246639308898362985600736985313610287370218545585443328912998714066030788971356972398823446394808259083145491780287377954911517455205043191986659486803525453280026699756592970920620102979774178487359570489964938005831483280782091403551604164735055022297589542910009750584030261291932");

        assert_eq!(proof.t.get("0").unwrap().to_dec().unwrap(), "66452646864713459129322124524496160239214129628844448912512675754382373114045232638792544050983258044571320479724542222159607548371946608278224646356448366445559828934782665270370014756906222313296871353700305312489013107502898521331193640487262241439496025903490697084701289251331970932030723857963667757918065298468726954493148633682914144253830507074421917845317843041768030700610129944001550144134234321487234247282527013708361275163765747931441214224397693734342806818103569845752619756970663088347173537279465064357347197203519585032404779938843725754592220777310937230037486845412937230858545348334751626327225");
        assert_eq!(proof.t.get("DELTA").unwrap().to_dec().unwrap(), "12744073002342538466174266178920319716851536025528365678772164359094855375069597510967107907963978165383581958746728451817220119885059854369802587463275692110468863903085692788520163123046996971844187140303651001700638819763809506725152408953126623513326965559836659294476633000658736763344051801272123315367972537058814718428582311569246639308898362985600736985313610287370218545585443328912998714066030788971356972398823446394808259083145491780287377954911517455205043191986659486803525453280026699756592970920620102979774178487359570489964938005831483280782091403551604164735055022297589542910009750584030261291932");

        assert_eq!(proof.u.get("0").unwrap().to_dec().unwrap(), "3");
        assert_eq!(proof.u.get("1").unwrap().to_dec().unwrap(), "1");
    }

    #[test]
    fn finalize_ge_proof_works() {
        let c_h = BigNumber::from_dec("107686359310664445046126368677755391247164319345083587464043204013905993527834").unwrap();
        let ge_proof = mocks::get_primary_ge_init_proof();
        let eq_proof = verifier::mocks::get_eq_proof();

        let res = Prover::_finalize_ge_proof(&c_h, &ge_proof, &eq_proof);

        assert!(res.is_ok());
        let proof = res.unwrap();

        assert_eq!("14530430712270780620115716831630456792731829285960002962064509786954277815652219734860240775632969505615425989813150680974232279981033881929825516835639704838509146807403579176456", proof.u.get("0").unwrap().to_dec().unwrap());
        assert_eq!("1415830066404575063558956955699897939417161777078791039926340455929989312103567388586750415279750275627689289774355989928259903201283164671369980334635402090593700202419576962251006803664979387881077329091553387025639738608978470326865096461988349436323051092921673039448207467310143157161249548690648317604663697956127142299857431279531067869166789113125108487447241380451860460435536386169606660126687136336515643267258245597749963499390882335368772524506108537160732974827392286571681871686360634706404457817326674394813236360450345475325164815205390904412548072443050097422540706146216417531228071209074620592598469883684966671309568705760392191743050877301212854432940753955279643358353605952631236345030655922045", proof.r.get("0").unwrap().to_dec().unwrap());
        assert_eq!("2909377521678119520977157959638852346549039931868195250658890196374980817755318676413066648981533034386605143040798380729872705956567376032225961933326117009011908374020093877002895162468521578763395678346621437225972600951965633549602979234732083149655058280123465723210167346545435946648092301500495871307611941306714133444462666462818882418100633983906555894992078138873969482714430788917034883079579778040749973092160959984323579215740942468398437958324399647532773947797685551797171537348210954088256282790659454179075257593928991997283548069103317735700818358235857780570873678690413979416837309542554490385517111819905278234351454124245103700468051202549165577210724696681231918320110736784038063606140146272860", proof.r.get("DELTA").unwrap().to_dec().unwrap());
        assert_eq!("44263308381149662900948673540609137605123483577985225626015193605421446490850432944403510911593807877995566074607735765400382861784877744789798777017960357051684400364048124004882741408393303775593487691064638002920853960645913535484864749193831701910596138125770720981871270085109534802728387292108961395671973015447681340852592012638839948998301809908713998541365956149792695654874324699264455657573099688614830144400409479952124271239106111005380360397720399778640177093636911827538708829123941248898780310301607124559838851222069991204870155414077086348071171421803569856093007812236846764361931252088960485440158830117131468627609450498244887243402854104282374544935516477360120294987311548247220633388905908551822949252630925854555366381978721601629564425954576926076828495554017163967076851067453147787769115012365426065129174495136", proof.alpha.to_dec().unwrap());
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn test_c_and_tau_list() {
        let issuer = Issuer::new();
        let prover = Prover::new();

        let (claim_definition, claim_definition_private) = issuer.generate_claim_definition(
            issuer::mocks::ISSUER_DID, issuer::mocks::get_gvt_schema(), None, true).unwrap();

        let (revocation_registry, revocation_registry_private) = issuer.issue_accumulator(
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap(),
            5, issuer::mocks::ISSUER_DID, 1).unwrap();

        let master_secret = prover.generate_master_secret().unwrap();

        let (claim_request, claim_init_data, revocation_claim_init_data) = prover.create_claim_request(
            claim_definition.clone().unwrap().data.public_key,
            claim_definition.clone().unwrap().data.public_key_revocation,
            master_secret, None, mocks::PROVER_DID).unwrap();

        let revocation_registry_ref_cell = Some(RefCell::new(revocation_registry));

        let claim_signature = issuer.create_claim(
            &claim_definition, &claim_definition_private.clone().unwrap(), &revocation_registry_ref_cell,
            &Some(revocation_registry_private.clone()), &claim_request,
            &issuer::mocks::get_gvt_attributes(), None).unwrap();

        let claim_json = ClaimJson::new(
            issuer::mocks::get_gvt_attributes(), claim_signature, 1,
            issuer::mocks::ISSUER_DID.to_string());

        let claim_json_ref_cell = RefCell::new(claim_json.clone().unwrap());

        let revocation_reg = revocation_registry_ref_cell.unwrap().clone();
        prover.process_claim(&claim_json_ref_cell, claim_init_data,
                             revocation_claim_init_data.clone(),
                             Some(claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap()),
                             &Some(revocation_reg.borrow().clone())).unwrap();

        let non_revocation_claim = claim_json_ref_cell.borrow().clone().unwrap().signature.non_revocation_claim.unwrap();

        let c_list_params = Prover::_gen_c_list_params(&non_revocation_claim).unwrap();
        let proof_c_list = Prover::_create_c_list_values(
            &non_revocation_claim, &c_list_params,
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap()).unwrap();
        let proof_tau_list = Issuer::_create_tau_list_values(
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap(),
            &revocation_reg.borrow().accumulator, &c_list_params, &proof_c_list).unwrap();
        let proof_tau_list_calc = Issuer::_create_tau_list_expected_values(
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap(),
            &revocation_reg.borrow().accumulator, &revocation_reg.borrow().acc_pk,
            &proof_c_list).unwrap();
        assert_eq!(proof_tau_list.as_slice().unwrap(), proof_tau_list_calc.as_slice().unwrap());
    }
}

#[cfg(test)]
mod find_claims_tests {
    use super::*;

    #[test]
    fn find_claims_empty_works() {
        let requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();

        assert_eq!(0, attributes.len());
        assert_eq!(0, predicates.len());
    }

    #[test]
    fn find_claims_works_for_revealed_attrs_only_with_same_schema() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(1), None));

        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, attributes.len());

        let claims_for_attribute = attributes.get("1").unwrap();
        assert_eq!(1, claims_for_attribute.len());

        let claim = claims_for_attribute.get(0).unwrap();
        assert!(claim.attrs.contains_key("name"));
        assert_eq!(claim.attrs.get("name").unwrap(), "Alex");
    }

    #[test]
    fn find_claims_works_for_revealed_attrs_only_with_other_schema() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(3), None));

        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, attributes.len());

        let claims_for_attribute = attributes.get("1").unwrap();
        assert_eq!(0, claims_for_attribute.len());
    }

    #[test]
    fn find_claims_works_for_predicate_satisfy() {
        let requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        let mut requested_predicates: HashMap<String, Predicate> = HashMap::new();
        requested_predicates.insert("1".to_string(), Predicate::new("age".to_string(), PredicateType::GE, 18, None, None));

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, predicates.len());

        let claims_for_predicates = predicates.get("1").unwrap();
        assert_eq!(1, claims_for_predicates.len());

        let claim = claims_for_predicates.get(0).unwrap();
        assert!(claim.attrs.contains_key("age"));
        assert!(claim.attrs.get("age").unwrap().parse::<i32>().unwrap() >= 18);
    }

    #[test]
    fn find_claims_works_for_does_not_satisfy_predicate() {
        let requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        let mut requested_predicates: HashMap<String, Predicate> = HashMap::new();
        requested_predicates.insert("1".to_string(), Predicate::new("age".to_string(), PredicateType::GE, 38, None, None));

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, predicates.len());

        let claims_for_predicate = predicates.get("1").unwrap();
        assert_eq!(0, claims_for_predicate.len());
    }

    #[test]
    fn find_claims_works_for_multiply_revealed_attrs() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(1), None));
        requested_attrs.insert("2".to_string(), AttributeInfo::new("status".to_string(), Some(2), None));

        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(2, attributes.len());

        let claims_for_attribute_name = attributes.get("1").unwrap();
        assert_eq!(1, claims_for_attribute_name.len());

        let claim = claims_for_attribute_name.get(0).unwrap();
        assert!(claim.attrs.contains_key("name"));
        assert_eq!(claim.attrs.get("name").unwrap(), "Alex");

        let claims_for_attribute_status = attributes.get("2").unwrap();
        assert_eq!(1, claims_for_attribute_status.len());

        let claim = claims_for_attribute_status.get(0).unwrap();
        assert!(claim.attrs.contains_key("status"));
        assert_eq!(claim.attrs.get("status").unwrap(), "partial");
    }

    #[test]
    fn find_claims_works_for_multiply_satisfy_predicates() {
        let requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        let mut requested_predicates: HashMap<String, Predicate> = HashMap::new();
        requested_predicates.insert("1".to_string(), Predicate::new("age".to_string(), PredicateType::GE, 18, None, None));
        requested_predicates.insert("2".to_string(), Predicate::new("period".to_string(), PredicateType::GE, 8, None, None));

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(2, predicates.len());

        let claims_for_predicates_age = predicates.get("1").unwrap();
        assert_eq!(1, claims_for_predicates_age.len());

        let claim = claims_for_predicates_age.get(0).unwrap();
        assert!(claim.attrs.contains_key("age"));
        assert!(claim.attrs.get("age").unwrap().parse::<i32>().unwrap() >= 18);

        let claims_for_predicates_period = predicates.get("2").unwrap();
        assert_eq!(1, claims_for_predicates_period.len());

        let claim = claims_for_predicates_period.get(0).unwrap();
        assert!(claim.attrs.contains_key("period"));
        assert!(claim.attrs.get("period").unwrap().parse::<i32>().unwrap() >= 8);
    }

    #[test]
    fn find_claims_works_for_multiply_attrs_and_satisfy_predicates() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(1), None));
        requested_attrs.insert("2".to_string(), AttributeInfo::new("status".to_string(), Some(2), None));

        let mut requested_predicates: HashMap<String, Predicate> = HashMap::new();
        requested_predicates.insert("1".to_string(), Predicate::new("age".to_string(), PredicateType::GE, 18, None, None));
        requested_predicates.insert("2".to_string(), Predicate::new("period".to_string(), PredicateType::GE, 8, None, None));

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(2, attributes.len());
        assert_eq!(2, predicates.len());

        let claims_for_attribute_name = attributes.get("1").unwrap();
        assert_eq!(1, claims_for_attribute_name.len());

        let claims_for_attribute_status = attributes.get("2").unwrap();
        assert_eq!(1, claims_for_attribute_status.len());

        let claims_for_predicates_age = predicates.get("1").unwrap();
        assert_eq!(1, claims_for_predicates_age.len());

        let claims_for_predicates_period = predicates.get("2").unwrap();
        assert_eq!(1, claims_for_predicates_period.len());
    }

    #[test]
    fn find_claims_works_for_several_matches_for_attribute() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(1), None));

        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info(),
            mocks::get_abc_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, attributes.len());

        let claims_for_attribute_name = attributes.get("1").unwrap();
        assert_eq!(2, claims_for_attribute_name.len());

        assert_eq!("1", claims_for_attribute_name.get(0).unwrap().claim_uuid);
        assert_eq!("3", claims_for_attribute_name.get(1).unwrap().claim_uuid);
    }

    #[test]
    fn find_claims_works_for_no_matches_for_attribute() {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("test".to_string(), Some(1), None));

        let requested_predicates: HashMap<String, Predicate> = HashMap::new();

        let claims = vec![
            mocks::get_gvt_claim_info(),
            mocks::get_xyz_claim_info()
        ];

        let prover = Prover::new();
        let res = prover.find_claims(requested_attrs, requested_predicates, claims);

        assert!(res.is_ok());
        let (attributes, predicates) = res.unwrap();
        assert_eq!(1, attributes.len());

        let claims_for_attribute_name = attributes.get("1").unwrap();
        assert_eq!(0, claims_for_attribute_name.len());
    }
}

pub mod mocks {
    use super::*;
    use services::anoncreds::issuer;
    use services::anoncreds::verifier;
    use services::anoncreds::types::{ClaimDefinitionData, Witness};
    use std::iter::FromIterator;
    use services::anoncreds::types::SignatureTypes;

    pub const PROVER_DID: &'static str = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

    pub fn get_non_revocation_proof_c_list() -> NonRevocProofCList {
        NonRevocProofCList::new(PointG1::new().unwrap(), PointG1::new().unwrap(),
                                PointG1::new().unwrap(), PointG1::new().unwrap(),
                                PointG2::new().unwrap(), PointG2::new().unwrap(),
                                PointG2::new().unwrap()
        )
    }

    pub fn get_non_revocation_proof_x_list() -> NonRevocProofXList {
        NonRevocProofXList::new(GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap(),
                                GroupOrderElement::new().unwrap())
    }

    pub fn get_gvt_predicate() -> Predicate {
        Predicate::new("age".to_string(), PredicateType::GE, 18, None, None)
    }

    pub fn get_xyz_predicate() -> Predicate {
        Predicate::new("period".to_string(), PredicateType::GE, 8, None, None)
    }

    pub fn get_gvt_primary_claim() -> PrimaryClaim {
        PrimaryClaim::new(BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap(),
                          BigNumber::from_dec("9718041686050466417394454846401911338135485472714675418729730425836367006101286571902254065185334609278478268966285580036221254487921329959035516004179696181846182303481304972520273119065229082628152074260549403953056671718537655331440869269274745137172330211653292094784431599793709932507153005886317395811504324510211401461248180054115028194976434036098410711049411182121148080258018668634613727512389415141820208171799071602314334918435751431063443005717167277426824339725300642890836588704754116628420091486522215319582218755888011754179925774397148116144684399342679279867598851549078956970579995906560499116598").unwrap(),
                          BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930098340478263817667896272954429430903").unwrap(),
                          BigNumber::from_dec("6477858587997811893327035319417510316563341854132851390093281262022504586945336581881563055213337677056181844572991952555932751996898440671581814053127951224635658321050035511444952581661461627187910434460669459027627147456890732433603419064826350179660439920130024451053677588698924377810206573252996817104905392311087651297242904369072119405731178447311689527558852965235336515327317399731791386249101329130190016387606690470441587455323714369899646882695142389754346148949502193028268930628086102907423247334472635671986918166524901017034444368593822038576239079939991296769079454011618207560042821478623371046256253086080003123462245464426891261800415264830177943676315694882710793222167202116798132497210943950614123537502319388887451156451273696457920098972385375390906181570700610413812857561840771758041019799427").unwrap()
        )
    }

    pub fn get_xyz_primary_claim() -> PrimaryClaim {
        PrimaryClaim::new(BigNumber::from_dec("15286000759172100591377181600470463901016563303508229099256868461439682297960").unwrap(),
                          BigNumber::from_dec("43408781019273294664105361779296865998719682917162544589998989929119545158736110398354782373487097567916720068393146407442522759465524978086454753905759545793463313344124355771811443434314961068264817560048863706416774950086764986003208711210634999865569049808488287390632316256564719056299637763267375333211821087200077890030359272146222631266721181554111124044208681571037538573069584354422205830667741943035073249429293717545002649455447823576929844586944437312395399980004204881381972730440043243134325220149938181771288726598116075075695030469920172383286087838334125452986626866574002045592988278504479246651359").unwrap(),
                          BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930308170826250847785686506076097675457").unwrap(),
                          BigNumber::from_dec("7317425522031871122929735014725915974219077916357946619324882999809902490147269232962296028836689309258771018375595524160662659624613729571392305833691669152259335217665129469797257019760976768390480752706278700726198757382847155041914663476330765482302082453258348762833072019199096655569755579732675778194731082929384728999646144810214262081001001610168832422312672453860834052510627627346824551328447573097827830742130142542088428980177134613143352951210154765966683768380267930430247816156756639251619256437708986533397482230542350135712118866336262892461386520892248250679440828723728022246922847534535121527862173935365408767109564029775935631584235878269228461929312723471684006178472632005435878448583443911005865851065020755776312530886070184936068216896674345747596811821466782799561319045722635649122612452222").unwrap()
        )
    }

    pub fn get_mtilde() -> HashMap<String, BigNumber> {
        let mut mtilde = HashMap::new();
        mtilde.insert("height".to_string(), BigNumber::from_dec("3373978431761662936864523680216977257584610980616339878140476966372383023266465253136551434714889555651032143048543421334122669369824546771790431199967902091704924294162747998714").unwrap());
        mtilde.insert("age".to_string(), BigNumber::from_dec("2976250595835739181594320238227653601426197318110939190760657852629456864395726135468275792741622450579986141053384483916124587493975756840689906672199964644984465423799113422915").unwrap());
        mtilde.insert("sex".to_string(), BigNumber::from_dec("1038496187132038951426769629254464579084684144036750642303206209710591608223417014007881207499688569061414518819199568509614376078846399946097722727271077857527181666924731796053").unwrap());
        mtilde
    }

    pub fn get_revealed_attrs() -> Vec<String> {
        let mut revealed_attrs: Vec<String> = Vec::new();
        revealed_attrs.push("name".to_string());
        revealed_attrs
    }

    pub fn get_unrevealed_attrs() -> Vec<String> {
        let mut unrevealed_attrs: Vec<String> = Vec::new();
        unrevealed_attrs.push("height".to_string());
        unrevealed_attrs.push("age".to_string());
        unrevealed_attrs.push("sex".to_string());
        unrevealed_attrs
    }

    pub fn get_primary_equal_init_proof() -> PrimaryEqualInitProof {
        let a_prime = BigNumber::from_dec("73257086769794587064099943967436413456606137933106600328493517494750494246990095654268081436982110418236942052043392353047210521286732387459211325220702233796797988644175700180272575648844736779152872382353777034795665067764357414889894540956741789789825768184852497440167487735512484852870071737572382353032530574683059753247452767913883743959993537969276743507336201600689240177338100796416706021606300904878397845520702439468069188914120053211111411367694831308267216395648656387187450864371933001748318901589141996368935626664855141812654806676458999719330682612787660793512632367212943940189704480718972567395396").unwrap();
        let t = BigNumber::from_dec("44674566012490574873221338726897300898913972309497258940219569980165585727901128041268469063382008728753943624549705899352321456091543114868302412585283526922484825880307252509503073791126004302810210154078010540383153531873388989179579827245098862102426681204339454264314340301557268884832227229252811218295369187558339702047951827768806306420746905540597187171789203160885305546843423145986246941522359210926851598959302132486285824149905366986262860649723244924769182483122471613582108897710332837686070090582706144278719293684893116662729424191599602937927245245078018737281020133694291784582308345229012480867237").unwrap();
        let e_tilde = BigNumber::from_dec("46977509037563772921188733388482759183920721890892331081711466073993908595985901848331080423617265862263799921846536660789175730995922544").unwrap();
        let e_prime = BigNumber::from_dec("583662989559444524697883298067925567").unwrap();
        let v_tilde = BigNumber::from_dec("555894869457553465718054081820422849162991995390494517944838822333270882977281784453553971006695084899118389412528359884323318041943325476169840344330169758975824153331145070636467566443788129175385227046128007984813738241967046976902336929862121881184186366859109984322069665187530843637401779413855818609802312254476734798474431968023612266562224855762384405063415925635256507132513009629860708092064413558502942291653812837032047514674344515747059720035918093216163460638260675950398390880830578559681142235013420891911126992440292399590994566860624336493535424361894744432273285682724123770355673224752107007429152867080154899799528690990463990548404671629807627523244386129350481398153531931938679507753616503159308561903414993607849227745071552258935672048341133052145284351204037153852982932148831702417091773975188604439616639047752092784493713122927003649804603056886698534968937477985617245235844137536420875188").unwrap();
        let v_prime = BigNumber::from_dec("6385614367009544498316319864543758599368125535237154281129593935195304840005981562825197155593411953165678474906281926931734345545746305450155060321085033621943087275107403410421778410927175029299691621870014311758603481338163542127748609425153803125698411340444632405699004049116623822070114354834294417100495058580661465651621088982873513323615197209830002327017414747343279393904208898726365331869009344688921360397873074029215826510233949892379862093346250740392060647414939231278435894873270850369894735486668772618984555075698111243885998180015446535353880393300721921216798608648100651591884384998694753149400256499979477096295284464637015155612555162482909528968752278735282245702719302108328105954407143650479954196184276137753771191346680837180603858473130837072734570076818412628985088803641214956190551904227").unwrap();
        let mtilde = mocks::get_mtilde();

        let m1_tilde = BigNumber::from_dec("17884736668674953594474879343533841182802514514784532835710262264561805009458126297222977824304362311586622997817594769134550513911169868072027461607531074811752832872590561469149850932518336232675337827949722723740491540895259903956542158590123078908328645673377676179125379936830018221094043943562296958727").unwrap();
        let m2_tilde = BigNumber::from_dec("33970939655505026872690051065527896936826240486176548712174703648151652129591217103741946892383483806205993341432925544541557374346350172352729633028700077053528659741067902223562294772771229606274461374185549251388524318740149589263256424345429891975622057372801133454251096604596597737126641279540347411289").unwrap();
        let m2 = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();

        PrimaryEqualInitProof::new(
            a_prime, t, e_tilde, e_prime, v_tilde, v_prime, mtilde,
            m1_tilde, m2_tilde, m2
        )
    }

    pub fn get_c_list() -> Vec<BigNumber> {
        let mut c_list: Vec<BigNumber> = Vec::new();
        c_list.push(BigNumber::from_dec("40419298688137869960380469261905532334637639358156591584198474730159922131845236332832025717302613443181736582484815352622543977612852994735900017491040605701377167257840237093127235154905233147231624795995550192527737607707481813233736307936765338317096333960487846640715651848248086837945953304627391859983207411514951469156988685936443758957189790705690990639460733132695525553505807698837031674923144499907591301228015553240722485660599743846214527228665753677346129919027033129697444096042970703607475089467398949054480185324997053077334850238886591657619835566943199882335077289734306701560214493298329372650208").unwrap());
        c_list.push(BigNumber::from_dec("47324660473671124619766812292419966979218618321195442620378932643647808062884161914306007419982240044457291065692968166148732382413212489017818981907451810722427822947434701298426390923083851509190004176754308805544221591456757905034099563880547910682773230595375415855727922588298088826548392572988130537249508717978384646013947582546019729481146325021203427278860772516903057439582612008766763139310189576482839673644190743850755863703998143105224320265752122772813607076484126428361088197863213824404833756768819688779202461859342789097743829182212846809717194485567647846915198890325457736010590303357798473896700").unwrap());
        c_list.push(BigNumber::from_dec("66450517869982062342267997954977032094273479808003128223349391866956221490486227999714708210796649990670474598595144373853545114810461129311488376523373030855652459048816291000188287472254577785187966494209478499264992271438571724964296278469527432908172064052750006541558566871906132838361892473377520708599782848821918665128705358243638618866198451401258608314504494676177177947997456537352832881339718141901132664969277082920274734598386059889447857289735878564021235996969965313779742103257439235693097049742098377325618673992118875810433536654414222034985875962188702260416140781008765351079345681492041353915517").unwrap());
        c_list.push(BigNumber::from_dec("78070105827196661040600041337907173457854153272544487321115604386049561730740327194221314976259005306609156189248394958383576900423218823055146785779218825861357426069962919084354758074120740816717011931695486881373830741590805899909505141118332615581712873355033382526097135102214961582694467049685680521168599662570089045106588071095868679795860083477878392645086886419842393734377034091691861772354369870695105905981921915221671803577058964332747681671537519176296905411380141019477128072347200017918410813327520323098847715450370454307294123150568469231654825506721027060142669757561165103933103053528023034511606").unwrap());
        c_list.push(BigNumber::from_dec("83200684536414956340494235687534491849084621311799273540992839950256544160417513543839780900524522144337818273323604172338904806642960330906344496013294511314421085013454657603118717753084155308020373268668810396333088299295804908264158817923391623116540755548965302906724851186886232431450985279429884730164260492598022651383336322153593491103199117187195782444754665111992163534318072330538584638714508386890137616826706777205862989966213285981526090164444190640439286605077153051456582398200856066916720632647408699812551248250054268483664698756596786352565981324521663234607300070180614929105425712839420242514321").unwrap());
        c_list
    }

    pub fn get_tau_list() -> Vec<BigNumber> {
        let mut tau_list: Vec<BigNumber> = Vec::new();
        tau_list.push(BigNumber::from_dec("15140192132563983584011198891415484817238186596993071283607396936354194583335316868900705320271111009411714831320691337831872126628439138871262533224307544703281477371807698525452223425670200750605763418449125326560417154215882193420051788620324946208921285413124444012185102142014009066082073507405990774347752529726721364286432450040059237148949753473594808640751722631907871436041823113427561411327410265647850452755588149194739107401612541934957588751200713263042014153310254117194222238408605703075357183065968515077548856751608663405886764709143763920973999261289863795465373404979606051217224017793032766958811").unwrap());
        tau_list.push(BigNumber::from_dec("22009325014877947630026527174200929317631472626208750791313439728894802205941501133457483305053287492055711395025700211096925855401324104745196675112371703883854338747182592204009840178348481147164357644090276358774264356146958774854024112737375489364695008508208970224155188285475467990251456534404860303212995739991780462885489625391318647267043983051823985749109827583921702054401295234951443763803867227290052184122075487663670525999631601499287795787258527407755075616126319202755499894030817914291589449384977544252255991849316879972035322419088010097341651222610917507166699253633464412656939604939686927779235").unwrap());
        tau_list.push(BigNumber::from_dec("15627964533527004998432038389165000103816136005375029988964505427070988976134211606408535227344041158417145070028255238801455392103113521695579086689578896155932875705490340075005561484163012535940306402641682521571945553659305990483808164193225425501204573377669678891599593106986761315653866565476157194483433336149271900598697489496190572244872015009221591483425535935884303531919258635347941316161540221899064295767010090897562893601204666639265613355995553950307149582738593763092807462903005018385092974255197604160149549388615872030971412913398039602109611976167048531483220445501083739737215277412870810099396").unwrap());
        tau_list.push(BigNumber::from_dec("69750460164463503832019239074179380223142417821933331668103242458939803887386159332871378045711353326082354712806990538579597154273250741009953395178245637905378411876747452614509288221818672025545306689963691675579404059572899417172145497523547047512889912370926674344888289753106210072610765364142940872887546059041780799075090797522396305865608421376284813869031711915938763531973096410258282809600437536302255350228578137679993463517124512267300176775839875875909783384538534171446077525775056641425609563775679897591880695823828105351526687827332736948255168213703139146311683168485769607106041873644234793657396").unwrap());
        tau_list.push(BigNumber::from_dec("34132763173107445610560830841313898488394524485228364539925353006770404496634510086661879191043246497239583692381010279276417009418352322742486751048568992101518984018378013150772900354967187656947771069077786822194631197139777633372530138823901112650920148029338833974489530448873513107614207475925912746846289211981300599307572467810317763139839748754562514339971268176553099225860038153231205184249842168570757272245458275526022597007402749355980752036595066753740086758919247309876789184990621533422299096077633094437542715030347647138342894730223750339935127139185670656368946989949841411629192230558551287266526").unwrap());
        tau_list.push(BigNumber::from_dec("76565683231858220413634970348355655533884193896594121193316140338326831295635725256889489793041881623418796770530744437643757750818290670869856629404442102804539779790470943920985982547126806372689451469829385561786853754658793672376584770590680698494872039421566522136994135799785364832139155336348898806149875050003083388070895449937350438703463774379388035785060136940608144835006837349223795491316522482304986804930841801932706957303647124712691616546214050336437883359026928636182057382080150720957312738870036121843132512663050961368923639527157611326078923388898194496216008348568701317636330495889266691635504").unwrap());
        tau_list
    }

    pub fn get_primary_ge_init_proof() -> PrimaryPredicateGEInitProof {
        let c_list: Vec<BigNumber> = get_c_list();
        let tau_list: Vec<BigNumber> = get_tau_list();

        let mut u: HashMap<String, BigNumber> = HashMap::new();
        u.insert("0".to_string(), BigNumber::from_dec("3").unwrap());
        u.insert("1".to_string(), BigNumber::from_dec("1").unwrap());
        u.insert("2".to_string(), BigNumber::from_dec("0").unwrap());
        u.insert("3".to_string(), BigNumber::from_dec("0").unwrap());

        let mut u_tilde = HashMap::new();
        u_tilde.insert("3".to_string(), BigNumber::from_dec("16150358755672241012460695129321325864817061205875004033795225851087833314854821728249641937105666018799012422371351449632923847984317420011432438475930370578146646594276080296620").unwrap());
        u_tilde.insert("1".to_string(), BigNumber::from_dec("919407332653360714123789350916436306282818598634846737462180851932618353714800404545973780338648396108988603165611273851585136854059054058096491382931469477309021233049221498113").unwrap());
        u_tilde.insert("2".to_string(), BigNumber::from_dec("12947014011443544528806555912324837244278059715103522101625396652490441127598860132430239390604274414152958526164107676952222456505578632937449151556057867144023854768899064453215").unwrap());
        u_tilde.insert("0".to_string(), BigNumber::from_dec("14530430712270780620115716831630456792731829285960002962064509786954277815652219734860240775632969505292366911881157345835853173947767708188332558800388942446379534765685598592954").unwrap());

        let mut r = HashMap::new();
        r.insert("3".to_string(), BigNumber::from_dec("24132544754399620065431039572698185029324955788479147508951988368652141824169968012401631405197526596910936236200501256582143713616923547154109572725575025831049700191992467054494004142728014553921553626557686986621281917316088996263926122140046634865717430166998367117286676599143409419427119266152736056710053609203711125989405212726237071472139024673721365397939677743276201109255641130117429575054170206689862492630448098516389565571101329687068784027116494371890703259752175194377877183611963716122547113191413743333828140272547100543539245187448059851898592306246455570727209949211247659088241502448651714103679374008105070016373294139").unwrap());
        r.insert("1".to_string(), BigNumber::from_dec("35594085114524945986639006224801730200805040269697932198069819550362676659001079845522469651677729918531683925947020457364678961154507874999789223287407843566694331672092132006386937192959717680231086106031364492343223860848813656183321276259834157693100328152560173336039125986710038567259388561327714033873384412441701350106617571828450963146214502461758094005490582378541947089847874178371413274096027707156703414573239039996352851800251963501114749923080129276591522903634133702734684169390809940285496300503809706037270335260091643596848671473612632965738250455900304403753944679890823052654248119197790585118329079277482895324313751745").unwrap());
        r.insert("2".to_string(), BigNumber::from_dec("12416745370495785706540664461803499515274608347250522372751993828760489306351885826979329832840050558190176950831767527159950310255159121407314662120565985630054402779252658020076760721381778346175310011216646031116221826523234356681794951060518746570363532356465500405602755795374789457390143843942758354075220594989212432418989437209512300563151542879411125346015814671481005582531474362744461151940296407107019178307871514140216555328464170666072131235143570187183316375551189197788487022794256230528166132115181407432283810165812226326503815433275045997075793535640301266413926518752768461289738628490190972639107320352430895111692883956").unwrap());
        r.insert("0".to_string(), BigNumber::from_dec("13147719687690861784642987903564117321119171978071399340721977775125245434410955945160797651733662914525457223263144997853255627605012387807755261713043301599172048697597969623088108659945671056128663376565520770635189017427518191119838455865079521045511096967890062994509991531319435529014076721106316221553877061138037619390089975320215668573127020159567603520558367466598464066051208531845265756516199237280615346212300039960390316563876371279899260670556125326105845359198862856997934813787872135942081650066851138525063820953011103923516149849171718879990909755711311066630273647571139362231496658023435123993551625990965120905367877028").unwrap());
        r.insert("DELTA".to_string(), BigNumber::from_dec("27017140706603837321930128683239640314000768158256873249678565317492691240380026575901913931941056190702376634224147699776972092380298850972547700066333918991951816592945434946683483826563040675037562054977204619980251439268131171446694007072677802224789195666130332806561436046366163420230684036395638111654271698281134816476714689333767613969261806762069371304995020522349204504739989730038026877050861981423166431273260095284622132391212425440148029904651623110816052419900003918839190100781461896988942446779821380489281562762932476888984542881369286357081355126723729214222892496254014829234244943392135453620530526273515539280130914262").unwrap());

        let mut r_tilde = HashMap::new();
        r_tilde.insert("3".to_string(), BigNumber::from_dec("1581310419623066984941512700585957369097463841185001482669660807480368207297113764053705737662920865913917179154960493364991851661497939487215481046202935838727534817426357413752818118478480001061422592").unwrap());
        r_tilde.insert("1".to_string(), BigNumber::from_dec("12698175784092390914196064326251972665080818640176357824753635500206769181493592026455460352953871545194375704442227937145765550620924766094755145832764559452913248804386143791786806665433772526875435831").unwrap());
        r_tilde.insert("2".to_string(), BigNumber::from_dec("17862530894611881146644634463381143206639453937332223200502790860790433041682100237129826201980749547269161308100519670647739748120710266271206949459654024958050006488529187007087901262025343947304658469").unwrap());
        r_tilde.insert("0".to_string(), BigNumber::from_dec("2998707557005793821174408437474970579753005270493800573947732417828426843052636578438933523490696647169032669416867456683467729604860634400510897331774306232996333435200605615727332230536004853848724693").unwrap());
        r_tilde.insert("DELTA".to_string(), BigNumber::from_dec("19088233876358835207419091970632588113690065223461360271820393633022806844306658668558786053764082234008649301641061865256819721316329021619475938398765638382289927962244874956969520735922406546981704352").unwrap());

        let alpha_tilde = BigNumber::from_dec("44263308381149662900948673540609137605123483577985225626015193605421446490850432944403510911593807877995566074607735765405553971901390456606499786829482599516431010417531712251971394967321246775153919925111546818075969608334965840293178801177046634728971628794958354733739862829268202974391880631744795540398548558220556991011193251909350421018299683294728391990188211711336282937525988363919530945046525731631119770997772548393939963391123532107813552269482929793072647468150911792469305880140318793207179607757703958258825655827605820657411086482548357455342445528631707138831116535366105159771271994970748831148128639376843296223110470512276276476446567585975474806154081654470617634795717498851405124307682847795651436514926925739847629355175444715922870618554631909406889698383588133721911769288573078161344190971202698069599055089014").unwrap();
        let predicate = get_gvt_predicate();

        let mut t = HashMap::new();
        t.insert("3".to_string(), BigNumber::from_dec("78070105827196661040600041337907173457854153272544487321115604386049561730740327194221314976259005306609156189248394958383576900423218823055146785779218825861357426069962919084354758074120740816717011931695486881373830741590805899909505141118332615581712873355033382526097135102214961582694467049685680521168599662570089045106588071095868679795860083477878392645086886419842393734377034091691861772354369870695105905981921915221671803577058964332747681671537519176296905411380141019477128072347200017918410813327520323098847715450370454307294123150568469231654825506721027060142669757561165103933103053528023034511606").unwrap());
        t.insert("1".to_string(), BigNumber::from_dec("47324660473671124619766812292419966979218618321195442620378932643647808062884161914306007419982240044457291065692968166148732382413212489017818981907451810722427822947434701298426390923083851509190004176754308805544221591456757905034099563880547910682773230595375415855727922588298088826548392572988130537249508717978384646013947582546019729481146325021203427278860772516903057439582612008766763139310189576482839673644190743850755863703998143105224320265752122772813607076484126428361088197863213824404833756768819688779202461859342789097743829182212846809717194485567647846915198890325457736010590303357798473896700").unwrap());
        t.insert("2".to_string(), BigNumber::from_dec("66450517869982062342267997954977032094273479808003128223349391866956221490486227999714708210796649990670474598595144373853545114810461129311488376523373030855652459048816291000188287472254577785187966494209478499264992271438571724964296278469527432908172064052750006541558566871906132838361892473377520708599782848821918665128705358243638618866198451401258608314504494676177177947997456537352832881339718141901132664969277082920274734598386059889447857289735878564021235996969965313779742103257439235693097049742098377325618673992118875810433536654414222034985875962188702260416140781008765351079345681492041353915517").unwrap());
        t.insert("0".to_string(), BigNumber::from_dec("40419298688137869960380469261905532334637639358156591584198474730159922131845236332832025717302613443181736582484815352622543977612852994735900017491040605701377167257840237093127235154905233147231624795995550192527737607707481813233736307936765338317096333960487846640715651848248086837945953304627391859983207411514951469156988685936443758957189790705690990639460733132695525553505807698837031674923144499907591301228015553240722485660599743846214527228665753677346129919027033129697444096042970703607475089467398949054480185324997053077334850238886591657619835566943199882335077289734306701560214493298329372650208").unwrap());
        t.insert("DELTA".to_string(), BigNumber::from_dec("83200684536414956340494235687534491849084621311799273540992839950256544160417513543839780900524522144337818273323604172338904806642960330906344496013294511314421085013454657603118717753084155308020373268668810396333088299295804908264158817923391623116540755548965302906724851186886232431450985279429884730164260492598022651383336322153593491103199117187195782444754665111992163534318072330538584638714508386890137616826706777205862989966213285981526090164444190640439286605077153051456582398200856066916720632647408699812551248250054268483664698756596786352565981324521663234607300070180614929105425712839420242514321").unwrap());

        PrimaryPredicateGEInitProof::new(
            c_list, tau_list, u, u_tilde, r, r_tilde, alpha_tilde, predicate, t
        )
    }

    pub fn get_primary_init_proof() -> PrimaryInitProof {
        PrimaryInitProof::new(get_primary_equal_init_proof(), vec![get_primary_ge_init_proof()])
    }

    pub fn get_gvt_claims_object() -> ClaimSignature {
        ClaimSignature::new(get_gvt_primary_claim(), None)
    }

    pub fn get_xyz_claims_object() -> ClaimSignature {
        ClaimSignature::new(get_xyz_primary_claim(), None)
    }

    pub fn get_public_key_revocation() -> RevocationPublicKey {
        RevocationPublicKey::new(PointG1::new().unwrap(), PointG2::new().unwrap(),
                                 PointG1::new().unwrap(), PointG1::new().unwrap(),
                                 PointG1::new().unwrap(), PointG1::new().unwrap(),
                                 PointG1::new().unwrap(), PointG2::new().unwrap(),
                                 PointG2::new().unwrap(), PointG1::new().unwrap(),
                                 PointG2::new().unwrap())
    }

    pub fn get_accumulator() -> Accumulator {
        let mut v: HashSet<i32> = HashSet::new();
        v.insert(1);
        Accumulator::new(PointG2::new().unwrap(), v, 5, 2)
    }

    pub fn get_tails() -> HashMap<i32, PointG1> {
        let mut res: HashMap<i32, PointG1> = HashMap::new();
        res.insert(1, PointG1::new().unwrap());
        res
    }

    pub fn get_witness() -> Witness {
        Witness::new(
            PointG2::new().unwrap(), PointG2::new().unwrap(), PointG1::new().unwrap(),
            PointG2::new().unwrap(), HashSet::from_iter(vec![1].iter().cloned()
            )
        )
    }

    pub fn get_gvt_non_revocation_claim() -> NonRevocationClaim {
        NonRevocationClaim::new(
            PointG1::new().unwrap(), GroupOrderElement::new().unwrap(),
            GroupOrderElement::new().unwrap(), get_witness(),
            PointG1::new().unwrap(), 1, GroupOrderElement::new().unwrap()
        )
    }

    pub fn get_gvt_claim_info() -> ClaimInfo {
        let attrs = issuer::mocks::get_gvt_row_attributes();
        ClaimInfo::new("1".to_string(), attrs, 1, issuer::mocks::ISSUER_DID.to_string())
    }

    pub fn get_xyz_claim_info() -> ClaimInfo {
        let attrs = issuer::mocks::get_xyz_row_attributes();
        ClaimInfo::new("2".to_string(), attrs, 2, issuer::mocks::ISSUER_DID.to_string())
    }

    pub fn get_abc_claim_info() -> ClaimInfo {
        let attrs = issuer::mocks::get_gvt_row_attributes();
        ClaimInfo::new("3".to_string(), attrs, 1, issuer::mocks::ISSUER_DID.to_string())
    }

    pub fn get_proof_req_json() -> ProofRequestJson {
        let mut requested_attrs: HashMap<String, AttributeInfo> = HashMap::new();
        requested_attrs.insert("1".to_string(), AttributeInfo::new("name".to_string(), Some(1), None));
        requested_attrs.insert("2".to_string(), AttributeInfo::new("status".to_string(), Some(2), None));
        requested_attrs.insert("3".to_string(), AttributeInfo::new("sex".to_string(), Some(1), None));

        let mut requested_predicates: HashMap<String, Predicate> = HashMap::new();
        requested_predicates.insert("1".to_string(), Predicate::new("age".to_string(), PredicateType::GE, 18, None, None));
        requested_predicates.insert("2".to_string(), Predicate::new("height".to_string(), PredicateType::GE, 180, None, None));

        let nonce = BigNumber::from_dec("123432421212").unwrap();

        ProofRequestJson {
            nonce: nonce,
            name: "name".to_string(),
            version: "version".to_string(),
            requested_attrs: requested_attrs,
            requested_predicates: requested_predicates
        }
    }

    pub fn get_gvt_claim_definition() -> ClaimDefinition {
        let claim_def_data = ClaimDefinitionData::new(issuer::mocks::get_pk(), None);
        ClaimDefinition {
            schema_seq_no: 1,
            issuer_did: issuer::mocks::ISSUER_DID.to_string(),
            signature_type: SignatureTypes::CL,
            data: claim_def_data
        }
    }

    pub fn get_xyz_claim_definition() -> ClaimDefinition {
        let claim_def_data = ClaimDefinitionData::new(issuer::mocks::get_pk(), None);
        ClaimDefinition {
            schema_seq_no: 2,
            issuer_did: issuer::mocks::ISSUER_DID.to_string(),
            signature_type: SignatureTypes::CL,
            data: claim_def_data
        }
    }

    pub fn get_revocation_registry() -> RevocationRegistry {
        RevocationRegistry {
            issuer_did: issuer::mocks::ISSUER_DID.to_string(),
            schema_seq_no: 1,
            accumulator: mocks::get_accumulator(),
            acc_pk: verifier::mocks::get_accum_publick_key()
        }
    }

    pub fn get_requested_claims() -> RequestedClaimsJson {
        let self_attested_attributes: HashMap<String, String> = HashMap::new();
        let mut requested_attrs: HashMap<String, (String, bool)> = HashMap::new();
        requested_attrs.insert("1".to_string(), ("1".to_string(), true));
        requested_attrs.insert("2".to_string(), ("2".to_string(), true));
        requested_attrs.insert("3".to_string(), ("1".to_string(), false));


        let mut requested_predicates: HashMap<String, String> = HashMap::new();
        requested_predicates.insert("1".to_string(), "1".to_string());
        requested_predicates.insert("2".to_string(), "1".to_string());

        RequestedClaimsJson {
            self_attested_attributes: self_attested_attributes,
            requested_attrs: requested_attrs,
            requested_predicates: requested_predicates
        }
    }

    pub fn get_gvt_claims_json() -> ClaimJson {
        ClaimJson {
            claim: issuer::mocks::get_gvt_attributes(),
            schema_seq_no: 1,
            signature: mocks::get_gvt_claims_object(),
            issuer_did: "did".to_string()
        }
    }

    pub fn get_xyz_claims_json() -> ClaimJson {
        ClaimJson {
            claim: issuer::mocks::get_xyz_attributes(),
            schema_seq_no: 2,
            signature: mocks::get_xyz_claims_object(),
            issuer_did: "did".to_string()
        }
    }
}
