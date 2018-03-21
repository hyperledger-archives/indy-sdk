extern crate indy_crypto;

use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;
use services::anoncreds::constants::*;
use services::anoncreds::types::*;
use services::anoncreds::helpers::*;
use utils::crypto::bn::BigNumber;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use self::indy_crypto::pair::{GroupOrderElement, PointG1, PointG2, Pair};
use self::indy_crypto::cl::issuer::Issuer as CryptoIssuer;
use services::anoncreds::converters::*;

extern crate time;

pub struct Issuer {}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }

    pub fn generate_claim_definition(&self, issuer_did: &str, schema: Schema, signature_type: Option<&str>,
                                     create_non_revoc: bool) -> Result<(ClaimDefinition, ClaimDefinitionPrivate), AnoncredsError> {
        info!(target: "anoncreds_service", "Issuer generate claim definition for Schema {:?} -> start", &schema);

        let signature_type = match signature_type {
            Some("CL") => SignatureTypes::CL,
            None => SignatureTypes::CL,
            _ => return Err(AnoncredsError::CommonError(CommonError::InvalidStructure(format!("Invalid Signature Type"))))
        };

        let (mut cred_pk, cred_sk, _) = CryptoIssuer::new_credential_def(&Schema_to_CredentialSchema(&schema), &get_non_schema_elements(), create_non_revoc)?;
        let (pk, pkr) = CredentialPublicKey_to_PublicKey(&mut cred_pk)?;
        let (sk, skr) = CredentialPrivateKey_to_SecretKey(&cred_sk)?;

        let claim_definition_data = ClaimDefinitionData::new(pk, pkr);
        let claim_definition = ClaimDefinition::new(schema.seq_no, issuer_did.to_string(), SignatureTypes::CL, claim_definition_data);
        let claim_definition_private = ClaimDefinitionPrivate::new(sk, skr);

        info!(target: "anoncreds_service", "Issuer generate claim definition for Schema {:?} -> done", &schema);
        Ok((claim_definition, claim_definition_private))
    }

    /*fn _generate_keys(schema: &Schema) -> Result<(PublicKey, SecretKey), CommonError> {
        info!(target: "anoncreds_service", "Issuer generate primary keys for Schema {:?} -> start", &schema);
        let mut ctx = BigNumber::new_context()?;

        if schema.data.attr_names.len() == 0 {
            return Err(CommonError::InvalidStructure(format!("List of attribute names is required to setup claim definition")));
        }

        info!(target: "anoncreds_service", "Issuer generate_safe_prime");
        let p = BigNumber::generate_safe_prime(LARGE_PRIME)?;
        let q = BigNumber::generate_safe_prime(LARGE_PRIME)?;
        info!(target: "anoncreds_service", "Issuer generate_safe_prime -> done");

        let mut p_prime = p.sub(&BigNumber::from_u32(1)?)?;
        p_prime.div_word(2)?;

        let mut q_prime = q.sub(&BigNumber::from_u32(1)?)?;
        q_prime.div_word(2)?;

        let n = p.mul(&q, Some(&mut ctx))?;
        let s = random_qr(&n)?;
        let xz = Issuer::_gen_x(&p_prime, &q_prime)?;
        let mut r: HashMap<String, BigNumber> = HashMap::new();

        for attribute in &schema.data.attr_names {
            let random = Issuer::_gen_x(&p_prime, &q_prime)?;
            r.insert(attribute.to_string(), s.mod_exp(&random, &n, Some(&mut ctx))?);
        }

        let z = s.mod_exp(&xz, &n, Some(&mut ctx))?;

        let rms = s.mod_exp(&Issuer::_gen_x(&p_prime, &q_prime)?, &n, Some(&mut ctx))?;
        let rctxt = s.mod_exp(&Issuer::_gen_x(&p_prime, &q_prime)?, &n, Some(&mut ctx))?;

        info!(target: "anoncreds_service", "Issuer generate primary keys for Schema {:?} -> done", &schema);
        Ok((
            PublicKey::new(n, s, rms, r, rctxt, z),
            SecretKey::new(p_prime, q_prime)
        ))
    }

    fn _generate_revocation_keys() -> Result<(Option<RevocationPublicKey>, Option<RevocationSecretKey>), CommonError> {
        info!(target: "anoncreds_service", "Issuer generate revocation keys -> start");
        let h = PointG1::new()?;
        let h0 = PointG1::new()?;
        let h1 = PointG1::new()?;
        let h2 = PointG1::new()?;
        let htilde = PointG1::new()?;
        let g = PointG1::new()?;

        let u = PointG2::new()?;
        let hcap = PointG2::new()?;

        let x = GroupOrderElement::new()?;
        let sk = GroupOrderElement::new()?;
        let gdash = PointG2::new()?;

        let pk = g.mul(&sk)?;
        let y = hcap.mul(&x)?;

        info!(target: "anoncreds_service", "Issuer generate revocation keys -> done");
        Ok((
            Some(RevocationPublicKey::new(g, gdash, h, h0, h1, h2, htilde, hcap, u, pk, y)),
            Some(RevocationSecretKey::new(x, sk))
        ))
    }*/

    #[cfg(test)]
    fn _gen_x(p: &BigNumber, q: &BigNumber) -> Result<BigNumber, CommonError> {
        Ok(BigNumber::from_dec("21756443327382027172985704617047967597993694788495380290694324827806324727974811069286883097008098972826137846700650885182803802394920367284736320514617598740869006348763668941791139304299497512001555851506177534398138662287596439312757685115968057647052806345903116050638193978301573172649243964671896070438965753820826200974052042958554415386005813811429117062833340444950490735389201033755889815382997617514953672362380638953231325483081104074039069074312082459855104868061153181218462493120741835250281211598658590317583724763093211076383033803581749876979865965366178002285968278439178209181121479879436785731938")?)
    }

    #[cfg(not(test))]
    fn _gen_x(p: &BigNumber, q: &BigNumber) -> Result<BigNumber, CommonError> {
        let mut result = p
            .mul(&q, None)?
            .sub_word(3)?
            .rand_range()?;
        result.add_word(2)?;

        Ok(result)
    }

    pub fn issue_accumulator(&self, pk_r: &RevocationPublicKey, max_claim_num: i32, issuer_did: &str, schema_seq_no: i32)
                             -> Result<(RevocationRegistry, RevocationRegistryPrivate), AnoncredsError> {
        info!(target: "anoncreds_service",
              "Issuer create accumulator for issuer_did {} and schema_seq_no {} -> start",
              issuer_did, schema_seq_no);
        let gamma = GroupOrderElement::new()?;
        let mut g: HashMap<i32, PointG1> = HashMap::new();
        let mut g_dash: HashMap<i32, PointG2> = HashMap::new();

        let g_count = 2 * max_claim_num;

        for i in 0..g_count {
            if i != max_claim_num + 1 {
                let i_bytes = transform_u32_to_array_of_u8(i as u32);
                let mut pow = GroupOrderElement::from_bytes(&i_bytes)?;
                pow = gamma.pow_mod(&pow)?;
                g.insert(i, pk_r.g.mul(&pow)?);
                g_dash.insert(i, pk_r.g_dash.mul(&pow)?);
            }
        }

        let mut z = Pair::pair(&pk_r.g, &pk_r.g_dash)?;
        let mut pow = GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8((max_claim_num + 1) as u32))?;
        pow = gamma.pow_mod(&pow)?;
        z = z.pow(&pow)?;
        let acc = PointG2::new_inf()?;
        let v: HashSet<i32> = HashSet::new();

        let acc = Accumulator::new(acc, v, max_claim_num, 1);
        let acc_pk = AccumulatorPublicKey::new(z);
        let acc_sk = AccumulatorSecretKey::new(gamma);

        let revocation_registry = RevocationRegistry::new(acc, acc_pk, issuer_did.to_string(), schema_seq_no);
        let revocation_registry_private = RevocationRegistryPrivate::new(acc_sk, g, g_dash);

        info!(target: "anoncreds_service",
              "Issuer create accumulator for issuer_did {} and schema_seq_no {} -> done",
              issuer_did, schema_seq_no);
        Ok((revocation_registry, revocation_registry_private))
    }

    pub fn create_claim(&self, claim_definition: &ClaimDefinition,
                        claim_definition_private: &ClaimDefinitionPrivate,
                        revocation_registry: &Option<RefCell<RevocationRegistry>>,
                        revocation_registry_private: &Option<RevocationRegistryPrivate>,
                        claim_request: &ClaimRequest,
                        attributes: &HashMap<String, Vec<String>>,
                        user_revoc_index: Option<i32>) -> Result<ClaimSignature, AnoncredsError> {
        info!(target: "anoncreds_service", "Issuer create claim for schema {} -> start", claim_definition.schema_seq_no);

        let context_attribute = CryptoIssuer::_gen_credential_context(&claim_request.prover_did, user_revoc_index.map(|n| n as u32))?;
        let blinded_secrets = gen_BlindedCredentialSecrets(&claim_request.u, &claim_request.ur)?;
        let cred_values = gen_known_CredentialValues(attributes)?;
        let (primary_claim, _) = CryptoIssuer::_generate_primary_credential(
            &claim_request.prover_did,
            &blinded_secrets,
            None,
            None,
            None,
            &cred_values,
            &PublicKey_to_CredentialPublicKey(&claim_definition.data.public_key)?,
            &SecretKey_to_CredentialPrivateKey(&claim_definition_private.secret_key)?,
        )?;

        let mut non_revocation_claim: Option<RefCell<NonRevocationClaim>> = None;
        if let (Some(ref pk_r), Some(ref sk_r),
            &Some(ref revoc_reg), &Some(ref revoc_reg_priv), Some(ref ur)) = (claim_definition.data.public_key_revocation.clone(),
                                                                              claim_definition_private.secret_key_revocation.clone(),
                                                                              revocation_registry, revocation_registry_private,
                                                                              claim_request.ur) {
            let (claim, timestamp) = Issuer::_issue_non_revocation_claim(
                &revoc_reg,
                &pk_r,
                &sk_r,
                &revoc_reg_priv.tails,
                &revoc_reg_priv.tails_dash,
                &revoc_reg_priv.acc_sk,
                &new_bn_to_old_bn(&context_attribute)?,
                &ur,
                user_revoc_index
            )?;
            non_revocation_claim = Some(RefCell::new(claim));
        };

        info!(target: "anoncreds_service", "Issuer create claim for schema {} -> done", claim_definition.schema_seq_no);
        Ok(ClaimSignature {
            primary_claim: PrimaryCredentialSignature_to_PrimaryClaim(&primary_claim)?,
            non_revocation_claim: non_revocation_claim
        })
    }

    fn _generate_context_attribute(accumulator_id: i32, prover_did: &str) -> Result<BigNumber, CommonError> {
        let accumulator_id_encoded = Issuer::_encode_attribute(&accumulator_id.to_string(), ByteOrder::Little)?;
        let prover_did_encoded = Issuer::_encode_attribute(prover_did, ByteOrder::Little)?;
        let mut s = vec![
            bitwise_or_big_int(&accumulator_id_encoded, &prover_did_encoded)?.to_bytes()?
        ];
        let pow_2 = BigNumber::from_u32(2)?.exp(&BigNumber::from_u32(LARGE_MASTER_SECRET)?, None)?;
        let h = get_hash_as_int(&mut s)?
            .modulus(&pow_2, None)?;
        Ok(h)
    }

    fn _issue_primary_claim(public_key: &PublicKey, secret_key: &SecretKey, u: &BigNumber, context_attribute: &BigNumber,
                            attributes: &HashMap<String, Vec<String>>) -> Result<PrimaryClaim, CommonError> {
        info!(target: "anoncreds_service", "Issuer issue primary claim for attributes {:?} -> start", &attributes.keys());

        let v_prime_prime = Issuer::_generate_v_prime_prime()?;
        let e_start = BigNumber::from_u32(2)?.exp(&BigNumber::from_u32(LARGE_E_START)?, None)?;
        let e_end = BigNumber::from_u32(2)?
            .exp(&BigNumber::from_u32(LARGE_E_END_RANGE)?, None)?
            .add(&e_start)?;

        let e = BigNumber::generate_prime_in_range(&e_start, &e_end)?;

        let a = Issuer::_sign(public_key, secret_key, context_attribute, &attributes, &v_prime_prime, u, &e)?;

        info!(target: "anoncreds_service", "Issuer issue primary claim -> done");

        Ok(PrimaryClaim::new(context_attribute.clone()?, a, e, v_prime_prime))
    }

    fn _sign(public_key: &PublicKey, secret_key: &SecretKey, context_attribute: &BigNumber,
             attributes: &HashMap<String, Vec<String>>, v: &BigNumber, u: &BigNumber, e: &BigNumber) -> Result<BigNumber, CommonError> {
        info!(target: "anoncreds_service", "Issuer sign attributes {:?} -> start", &attributes.keys());

        let mut context = BigNumber::new_context()?;
        let mut rx = BigNumber::from_u32(1)?;

        for (key, value) in attributes {
            let pk_r = public_key.r.get(key)
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in pk.r", key)))?;
            let cur_val = value.get(1)
                .ok_or(CommonError::InvalidStructure(format!("Encoded value by key '{}' not found in attributes", key)))?;

            rx = rx.mul(
                &pk_r.mod_exp(&BigNumber::from_dec(cur_val)?, &public_key.n, Some(&mut context))?,
                Some(&mut context)
            )?;
        }

        rx = public_key.rctxt.mod_exp(&context_attribute, &public_key.n, Some(&mut context))?
            .mul(&rx, Some(&mut context))?;

        if u != &BigNumber::from_u32(0)? {
            rx = u.modulus(&public_key.n, Some(&mut context))?
                .mul(&rx, Some(&mut context))?;
        }

        let n = secret_key.p.mul(&secret_key.q, Some(&mut context))?;
        let mut e_inverse = e.modulus(&n, Some(&mut context))?;

        let mut a = public_key.s
            .mod_exp(&v, &public_key.n, Some(&mut context))?
            .mul(&rx, Some(&mut context))?;
        a = public_key.z.mod_div(&a, &public_key.n)?;

        e_inverse = e_inverse.inverse(&n, Some(&mut context))?;
        a = a.mod_exp(&e_inverse, &public_key.n, Some(&mut context))?;

        info!(target: "anoncreds_service", "Issuer sign attributes -> done");
        Ok(a)
    }

    fn _issue_non_revocation_claim(revocation_registry: &RefCell<RevocationRegistry>, pk_r: &RevocationPublicKey,
                                   sk_r: &RevocationSecretKey, g: &HashMap<i32, PointG1>,
                                   g_dash: &HashMap<i32, PointG2>, sk_accum: &AccumulatorSecretKey,
                                   context_attribute: &BigNumber,
                                   ur: &PointG1, seq_number: Option<i32>) ->
                                   Result<(NonRevocationClaim, i64), AnoncredsError> {
        info!(target: "anoncreds_service", "Issuer issue non-revocation claim -> start");
        let ref mut accumulator = revocation_registry.borrow_mut().accumulator;

        if accumulator.is_full() {
            return Err(AnoncredsError::AccumulatorIsFull(
                format!("issuer_did: {} schema_seq_no: {}", revocation_registry.borrow().issuer_did, revocation_registry.borrow().schema_seq_no))
            );
        }

        let i = match seq_number {
            Some(x) => x,
            _ => accumulator.current_i
        };

        accumulator.current_i += 1;

        let vr_prime_prime = GroupOrderElement::new()?;
        let c = GroupOrderElement::new()?;
        let m2 = GroupOrderElement::from_bytes(&context_attribute.to_bytes()?)?;

        let g_i = g.get(&i)
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in g", i)))?;

        let sigma =
            pk_r.h0.add(&pk_r.h1.mul(&m2)?)?
                .add(&ur)?
                .add(g_i)?
                .add(&pk_r.h2.mul(&vr_prime_prime)?)?
                .mul(&sk_r.x.add_mod(&c)?.inverse()?)?;

        let mut omega = PointG2::new_inf()?;

        for j in &accumulator.v {
            let index = accumulator.max_claim_num + 1 - j + i;
            omega = omega.add(g_dash.get(&index)
                .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?)?;
        }

        let sigma_i = pk_r.g_dash
            .mul(&sk_r.sk
                .add_mod(&sk_accum.gamma
                    .pow_mod(&GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8(i as u32))?)?)?
                .inverse()?)?;
        let u_i = pk_r.u
            .mul(&sk_accum.gamma
                .pow_mod(&GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8(i as u32))?)?)?;

        let index = accumulator.max_claim_num + 1 - i;
        accumulator.acc = accumulator.acc.add(g_dash.get(&index)
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?)?;
        accumulator.v.insert(i);

        let witness = Witness::new(sigma_i, u_i, g_i.clone(), omega, accumulator.v.clone());
        let timestamp = time::now_utc().to_timespec().sec;
        info!(target: "anoncreds_service", "Issuer issue non-revocation claim -> done");
        Ok(
            (
                NonRevocationClaim::new(sigma, c, vr_prime_prime, witness, g_i.clone(), i, m2),
                timestamp
            )
        )
    }

    fn _encode_attribute(attribute: &str, byte_order: ByteOrder) -> Result<BigNumber, CommonError> {
        let mut result = BigNumber::hash(attribute.as_bytes())?;

        let index = result.iter().position(|&value| value == 0);
        if let Some(position) = index {
            result.truncate(position);
        }
        if let ByteOrder::Little = byte_order {
            result.reverse();
        }
        Ok(BigNumber::from_bytes(&result)?)
    }

    fn _generate_v_prime_prime() -> Result<BigNumber, CommonError> {
        let a = BigNumber::rand(LARGE_VPRIME_PRIME)?;
        let b = BigNumber::from_u32(2)?
            .exp(&BigNumber::from_u32(LARGE_VPRIME_PRIME - 1)?, None)?;
        let v_prime_prime = bitwise_or_big_int(&a, &b)?;
        Ok(v_prime_prime)
    }

    pub fn revoke(&self, revocation_registry: &RefCell<RevocationRegistry>,
                  g_dash: &HashMap<i32, PointG2>, i: i32) -> Result<i64, AnoncredsError> {
        info!(target: "anoncreds_service", "Issuer revoke claim by index {} -> start", i);

        let ref mut accumulator = revocation_registry.borrow_mut().accumulator;

        if !accumulator.v.remove(&i) {
            return Err(AnoncredsError::InvalidUserRevocIndex(
                format!("User index:{} not found in Accumulator", i))
            );
        }

        let index: i32 = accumulator.max_claim_num + 1 - i;
        let element = g_dash.get(&index)
            .ok_or(CommonError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?;
        accumulator.acc = accumulator.acc.sub(element)?;
        let timestamp = time::now_utc().to_timespec().sec;

        info!(target: "anoncreds_service", "Issuer revoke claim by index {} -> done", i);
        Ok(timestamp)
    }

    pub fn _create_tau_list_values(pk_r: &RevocationPublicKey, accumulator: &Accumulator,
                                   params: &NonRevocProofXList, proof_c: &NonRevocProofCList) -> Result<NonRevocProofTauList, CommonError> {
        let t1 = pk_r.h.mul(&params.rho)?.add(&pk_r.htilde.mul(&params.o)?)?;
        let mut t2 = proof_c.e.mul(&params.c)?
            .add(&pk_r.h.mul(&params.m.mod_neg()?)?)?
            .add(&pk_r.htilde.mul(&params.t.mod_neg()?)?)?;
        if t2.is_inf()? {
            t2 = PointG1::new_inf()?;
        }
        let t3 = Pair::pair(&proof_c.a, &pk_r.h_cap)?.pow(&params.c)?
            .mul(&Pair::pair(&pk_r.htilde, &pk_r.h_cap)?.pow(&params.r)?)?
            .mul(&Pair::pair(&pk_r.htilde, &pk_r.y)?.pow(&params.rho)?
                .mul(&Pair::pair(&pk_r.htilde, &pk_r.h_cap)?.pow(&params.m)?)?
                .mul(&Pair::pair(&pk_r.h1, &pk_r.h_cap)?.pow(&params.m2)?)?
                .mul(&Pair::pair(&pk_r.h2, &pk_r.h_cap)?.pow(&params.s)?)?.inverse()?)?;
        let t4 = Pair::pair(&pk_r.htilde, &accumulator.acc)?
            .pow(&params.r)?
            .mul(&Pair::pair(&pk_r.g.neg()?, &pk_r.h_cap)?.pow(&params.r_prime)?)?;
        let t5 = pk_r.g.mul(&params.r)?.add(&pk_r.htilde.mul(&params.o_prime)?)?;
        let mut t6 = proof_c.d.mul(&params.r_prime_prime)?
            .add(&pk_r.g.mul(&params.m_prime.mod_neg()?)?)?
            .add(&pk_r.htilde.mul(&params.t_prime.mod_neg()?)?)?;
        if t6.is_inf()? {
            t6 = PointG1::new_inf()?;
        }
        let t7 = Pair::pair(&pk_r.pk.add(&proof_c.g)?, &pk_r.h_cap)?.pow(&params.r_prime_prime)?
            .mul(&Pair::pair(&pk_r.htilde, &pk_r.h_cap)?.pow(&params.m_prime.mod_neg()?)?)?
            .mul(&Pair::pair(&pk_r.htilde, &proof_c.s)?.pow(&params.r)?)?;
        let t8 = Pair::pair(&pk_r.htilde, &pk_r.u)?.pow(&params.r)?
            .mul(&Pair::pair(&pk_r.g.neg()?, &pk_r.h_cap)?.pow(&params.r_prime_prime_prime)?)?;

        Ok(NonRevocProofTauList::new(t1, t2, t3, t4, t5, t6, t7, t8))
    }

    pub fn _create_tau_list_expected_values(pk_r: &RevocationPublicKey, accumulator: &Accumulator,
                                            accum_pk: &AccumulatorPublicKey, proof_c: &NonRevocProofCList) -> Result<NonRevocProofTauList, CommonError> {
        let t1 = proof_c.e;
        let t2 = PointG1::new_inf()?;
        let t3 = Pair::pair(&pk_r.h0.add(&proof_c.g)?, &pk_r.h_cap)?
            .mul(&Pair::pair(&proof_c.a, &pk_r.y)?.inverse()?)?;
        let t4 = Pair::pair(&proof_c.g, &accumulator.acc)?
            .mul(&Pair::pair(&pk_r.g, &proof_c.w)?.mul(&accum_pk.z)?.inverse()?)?;
        let t5 = proof_c.d;
        let t6 = PointG1::new_inf()?;
        let t7 = Pair::pair(&pk_r.pk.add(&proof_c.g)?, &proof_c.s)?
            .mul(&Pair::pair(&pk_r.g, &pk_r.g_dash)?.inverse()?)?;
        let t8 = Pair::pair(&proof_c.g, &pk_r.u)?
            .mul(&Pair::pair(&pk_r.g, &proof_c.u)?.inverse()?)?;

        Ok(NonRevocProofTauList::new(t1, t2, t3, t4, t5, t6, t7, t8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "revocation_tests")]
    use services::anoncreds::prover;
    #[cfg(feature = "revocation_tests")]
    use services::anoncreds::prover::Prover;
    #[cfg(feature = "revocation_tests")]
    use services::anoncreds::types::ClaimJson;

    #[test]
    fn generate_keys_works() {  // TODO: Fix this test
        let issuer = Issuer::new();
        let (claim_definition, claim_definition_private) = issuer.generate_claim_definition(mocks::ISSUER_DID, mocks::get_gvt_schema(), None, false).unwrap();
        assert_eq!(claim_definition, mocks::get_claim_definition());
        assert_eq!(claim_definition_private, mocks::get_claim_definition_private());
    }

    #[test]
    fn generate_v_prime_prime_works() {
        let result = BigNumber::from_dec("6620937836014079781509458870800001917950459774302786434315639456568768602266735503527631640833663968617512880802104566048179854406925811731340920442625764155409951969854303612644121780700879432308016935250101960876405664503219252820761501606507817390189252221968804450207070282033815280889897882643560437257171838117793768660731379360330750300543760457608638753190279419951706206819943151918535286779337023708838891906829360439545064730288538139152367417882097349210427894031568623898916625312124319876670702064561291393993815290033742478045530118808274555627855247830659187691067893683525651333064738899779446324124393932782261375663033826174482213348732912255948009062641783238846143256448824091556005023241191311617076266099622843011796402959351074671886795391490945230966123230485475995208322766090290573654498779155").unwrap();
        assert_eq!(Issuer::_generate_v_prime_prime().unwrap(), result);
    }

    #[test]
    fn generate_claim_definition_works_without_revocation_part() {
        let issuer = Issuer::new();
        let schema = mocks::get_gvt_schema();
        let signature_type = None;
        let create_non_revoc = false;

        let result = issuer.generate_claim_definition(mocks::ISSUER_DID, schema, signature_type, create_non_revoc);
        assert!(result.is_ok());

        let (claim_definition, claim_definition_private) = result.unwrap();

        assert!(claim_definition.data.public_key_revocation.is_none());
        assert!(claim_definition_private.secret_key_revocation.is_none());
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn generate_claim_definition_works_with_revocation_part() {
        let issuer = Issuer::new();
        let schema = mocks::get_gvt_schema();
        let signature_type = None;
        let create_non_revoc = true;

        let result = issuer.generate_claim_definition(mocks::ISSUER_DID, schema, signature_type, create_non_revoc);
        assert!(result.is_ok());

        let (claim_definition, claim_definition_private) = result.unwrap();

        assert!(claim_definition.data.public_key_revocation.is_some());
        assert!(claim_definition_private.secret_key_revocation.is_some());
    }

    #[test]
    fn generate_claim_definition_does_not_works_with_empty_attributes() {
        let issuer = Issuer::new();
        let mut schema = mocks::get_gvt_schema();
        schema.data.attr_names = HashSet::new();

        let signature_type = None;
        let create_non_revoc = false;

        let result = issuer.generate_claim_definition(mocks::ISSUER_DID, schema, signature_type, create_non_revoc);
        assert!(result.is_err());
    }

    #[test]
    fn encode_attribute_works() {
        let test_str = "5435";
        let test_answer = "83761840706354868391674207739241454863743470852830526299004654280720761327142";
        assert_eq!(test_answer, Issuer::_encode_attribute(test_str, ByteOrder::Big).unwrap().to_dec().unwrap());
    }

    #[test]
    fn generate_context_attribute_works() {
        let accumulator_id = 110;
        let user_id = "111";
        let answer = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let result = Issuer::_generate_context_attribute(accumulator_id, user_id).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn sign_works() {
        let public_key = mocks::get_pk();
        let secret_key = mocks::get_secret_key();
        let context_attribute = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let attributes = mocks::get_gvt_attributes();
        let v = BigNumber::from_dec("5237513942984418438429595379849430501110274945835879531523435677101657022026899212054747703201026332785243221088006425007944260107143086435227014329174143861116260506019310628220538205630726081406862023584806749693647480787838708606386447727482772997839699379017499630402117304253212246286800412454159444495341428975660445641214047184934669036997173182682771745932646179140449435510447104436243207291913322964918630514148730337977117021619857409406144166574010735577540583316493841348453073326447018376163876048624924380855323953529434806898415857681702157369526801730845990252958130662749564283838280707026676243727830151176995470125042111348846500489265328810592848939081739036589553697928683006514398844827534478669492201064874941684905413964973517155382540340695991536826170371552446768460042588981089470261358687308").unwrap();
        let u = BigNumber::from_dec("72637991796589957272144423539998982864769854130438387485781642285237707120228376409769221961371420625002149758076600738245408098270501483395353213773728601101770725294535792756351646443825391806535296461087756781710547778467803194521965309091287301376623972321639262276779134586366620773325502044026364814032821517244814909708610356590687571152567177116075706850536899272749781370266769562695357044719529245223811232258752001942940813585440938291877640445002571323841625932424781535818087233087621479695522263178206089952437764196471098717335358765920438275944490561172307673744212256272352897964947435086824617146019").unwrap();
        let e = BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930214202955935602153431795703076242907").unwrap();
        let result = BigNumber::from_dec("18970881790876593286488783486386867538450674270137197011105008151201183300028283403854725282778638150217936721942434319741164063687946275930536223863520768657672755664180955901543160149915323325151339912941454195063854083578091043058101001054089316795088554097754632405106453701959655043761308676687984722831097067744306280339099944309055300662730322057853217855619342132319369757252485139011180518031078822262681093763592682724354563150664662385847044702450408149239372444565988153918412684418519832197112374827438788434448252992414094101094582772269873015514685057917124494501480003311040042093731740782916169155664").unwrap();
        assert_eq!(result, Issuer::_sign(&public_key, &secret_key, &context_attribute, &attributes, &v, &u, &e).unwrap());
    }

    #[test]
    #[cfg(feature = "revocation_tests")]
    fn test_init_non_revoc_claim() {
        let issuer = Issuer::new();
        let prover = Prover::new();

        let (claim_definition, claim_definition_private) = issuer.generate_claim_definition(
            mocks::ISSUER_DID, mocks::get_gvt_schema(), None, true).unwrap();

        let (revocation_registry, revocation_registry_private) = issuer.issue_accumulator(
            &claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap(),
            5, mocks::ISSUER_DID, 1).unwrap();

        let master_secret = prover.generate_master_secret().unwrap();

        let (claim_request, claim_init_data, revocation_claim_init_data) = prover.create_claim_request(
            claim_definition.clone().unwrap().data.public_key,
            claim_definition.clone().unwrap().data.public_key_revocation,
            master_secret, None,prover::mocks::PROVER_DID).unwrap();

        let revocation_registry_ref_cell = Some(RefCell::new(revocation_registry));

        let claim_signature = issuer.create_claim(
            &claim_definition, &claim_definition_private, &revocation_registry_ref_cell,
            &Some(revocation_registry_private), &claim_request,
            &mocks::get_gvt_attributes(), None).unwrap();

        let non_revocation_claim = claim_signature.clone().unwrap().non_revocation_claim.unwrap();
        let old_v = non_revocation_claim.borrow().vr_prime_prime;

        let claim_json = ClaimJson::new(
            mocks::get_gvt_attributes(), claim_signature, 1,
            mocks::ISSUER_DID.to_string());

        let claim_json_ref_cell = RefCell::new(claim_json.clone().unwrap());

        let revoc_reg = revocation_registry_ref_cell.unwrap().clone();
        prover.process_claim(&claim_json_ref_cell, claim_init_data,
                             revocation_claim_init_data.clone(),
                             Some(claim_definition.clone().unwrap().data.public_key_revocation.clone().unwrap()),
                             &Some(revoc_reg.borrow().clone())).unwrap();

        let non_revocation_claim = claim_json_ref_cell.borrow().clone().unwrap().signature.non_revocation_claim.unwrap();
        let new_v = non_revocation_claim.borrow().vr_prime_prime;

        let vr_prime = revocation_claim_init_data.unwrap().v_prime;
        assert_eq!(old_v.add_mod(&vr_prime).unwrap(), new_v);
    }
}

pub mod mocks {
    use super::*;
    use services::anoncreds::types::SchemaData;

    pub const ISSUER_DID: &'static str = "NcYxiDXkpYi6ov5FcYDi1e";

    pub fn get_claim_definition() -> ClaimDefinition {
        let mut r: HashMap<String, BigNumber> = HashMap::new();
        r.insert("sex".to_string(), BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap());
        r.insert("name".to_string(), BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap());
        r.insert("age".to_string(), BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap());
        r.insert("height".to_string(), BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap());
        let public_key = PublicKey::new(
            BigNumber::from_dec("89057765651800459030103911598694169835931320404459570102253965466045532669865684092518362135930940112502263498496335250135601124519172068317163741086983519494043168252186111551835366571584950296764626458785776311514968350600732183408950813066589742888246925358509482561838243805468775416479523402043160919428168650069477488093758569936116799246881809224343325540306266957664475026390533069487455816053169001876208052109360113102565642529699056163373190930839656498261278601357214695582219007449398650197048218304260447909283768896882743373383452996855450316360259637079070460616248922547314789644935074980711243164129").unwrap(),
            BigNumber::from_dec("64684820421150545443421261645532741305438158267230326415141505826951816460650437611148133267480407958360035501128469885271549378871140475869904030424615175830170939416512594291641188403335834762737251794282186335118831803135149622404791467775422384378569231649224208728902565541796896860352464500717052768431523703881746487372385032277847026560711719065512366600220045978358915680277126661923892187090579302197390903902744925313826817940566429968987709582805451008234648959429651259809188953915675063700676546393568304468609062443048457324721450190021552656280473128156273976008799243162970386898307404395608179975243").unwrap(),
            BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap(),
            BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap(),
            r,
            BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap(),
            BigNumber::from_dec("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap()
        );
        let claim_def_data = ClaimDefinitionData::new(public_key, None);
        ClaimDefinition::new(1, ISSUER_DID.to_string(), SignatureTypes::CL, claim_def_data)
    }

    pub fn get_claim_definition_private() -> ClaimDefinitionPrivate {
        let secret_key = SecretKey::new(BigNumber::from_dec("149212738775716179659508649034140914067267873385650452563221860367878267143635191771233591587868730221903476199105022913859057555905442876114559838735355652672950963033972314646471235775711934244481758977047119803475879470383993713606231800156950590334088086141997103196482505556481059579729337361392854778311").unwrap(), BigNumber::from_dec("149212738775716179659508649034140914067267873385650452563221860367878267143635191771233591587868730221903476199105022913859057555905442876114559838735355652672950963033972314646471235775711934244481758977047119803475879470383993713606231800156950590334088086141997103196482505556481059579729337361392854778311").unwrap());
        ClaimDefinitionPrivate::new(secret_key, None)
    }

    pub fn get_gvt_schema() -> Schema {
        let mut keys: HashSet<String> = HashSet::new();
        keys.insert("name".to_string());
        keys.insert("age".to_string());
        keys.insert("height".to_string());
        keys.insert("sex".to_string());

        let schema_data = SchemaData::new("gvt".to_string(), "1.0".to_string(), keys);
        Schema {
            seq_no: 1,
            data: schema_data
        }
    }

    pub fn get_xyz_schema() -> Schema {
        let mut keys: HashSet<String> = HashSet::new();
        keys.insert("status".to_string());
        keys.insert("period".to_string());

        let schema_data = SchemaData::new("xyz".to_string(), "1.0".to_string(), keys);
        Schema {
            seq_no: 2,
            data: schema_data
        }
    }

    pub fn get_gvt_attributes() -> HashMap<String, Vec<String>> {
        let mut attributes: HashMap<String, Vec<String>> = HashMap::new();
        attributes.insert("name".to_string(), vec!["Alex".to_string(), "1139481716457488690172217916278103335".to_string()]);
        attributes.insert("age".to_string(), vec!["28".to_string(), "28".to_string()]);
        attributes.insert("sex".to_string(), vec!["male".to_string(), "5944657099558967239210949258394887428692050081607692519917050011144233115103".to_string()]);
        attributes.insert("height".to_string(), vec!["175".to_string(), "175".to_string()]);
        attributes
    }

    pub fn get_xyz_attributes() -> HashMap<String, Vec<String>> {
        let mut attributes: HashMap<String, Vec<String>> = HashMap::new();
        attributes.insert("status".to_string(), vec!["partial".to_string(), "51792877103171595686471452153480627530895".to_string()]);
        attributes.insert("period".to_string(), vec!["8".to_string(), "8".to_string()]);
        attributes
    }

    pub fn get_gvt_encoded_revealed_attributes() -> HashMap<String, BigNumber> {
        let mut encoded_attributes: HashMap<String, BigNumber> = HashMap::new();
        encoded_attributes.insert("name".to_string(), BigNumber::from_dec("1139481716457488690172217916278103335").unwrap());
        encoded_attributes
    }

    pub fn get_gvt_encoded_attributes() -> HashMap<String, BigNumber> {
        let mut encoded_attributes: HashMap<String, BigNumber> = HashMap::new();
        encoded_attributes.insert("name".to_string(), BigNumber::from_dec("1139481716457488690172217916278103335").unwrap());
        encoded_attributes.insert("age".to_string(), BigNumber::from_dec("28").unwrap());
        encoded_attributes.insert("sex".to_string(), BigNumber::from_dec("5944657099558967239210949258394887428692050081607692519917050011144233115103").unwrap());
        encoded_attributes.insert("height".to_string(), BigNumber::from_dec("175").unwrap());
        encoded_attributes
    }

    pub fn get_xyz_encoded_attributes() -> HashMap<String, BigNumber> {
        let mut encoded_attributes: HashMap<String, BigNumber> = HashMap::new();
        encoded_attributes.insert("status".to_string(), BigNumber::from_dec("51792877103171595686471452153480627530895").unwrap());
        encoded_attributes.insert("period".to_string(), BigNumber::from_dec("8").unwrap());
        encoded_attributes
    }

    pub fn get_xyz_row_attributes() -> HashMap<String, String> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("status".to_string(), "partial".to_string());
        attributes.insert("period".to_string(), "8".to_string());
        attributes
    }

    pub fn get_gvt_row_attributes() -> HashMap<String, String> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("name".to_string(), "Alex".to_string());
        attributes.insert("age".to_string(), "28".to_string());
        attributes.insert("sex".to_string(), "male".to_string());
        attributes.insert("height".to_string(), "175".to_string());
        attributes
    }

    pub fn get_secret_key() -> SecretKey {
        SecretKey::new(
            BigNumber::from_dec("157329491389375793912190594961134932804032426403110797476730107804356484516061051345332763141806005838436304922612495876180233509449197495032194146432047460167589034147716097417880503952139805241591622353828629383332869425029086898452227895418829799945650973848983901459733426212735979668835984691928193677469").unwrap(),
            BigNumber::from_dec("151323892648373196579515752826519683836764873607632072057591837216698622729557534035138587276594156320800768525825023728398410073692081011811496168877166664537052088207068061172594879398773872352920912390983199416927388688319207946493810449203702100559271439586753256728900713990097168484829574000438573295723").unwrap()
        )
    }

    pub fn get_pk() -> PublicKey {
        let mut r = HashMap::new();
        r.insert("name".to_string(), BigNumber::from_dec("55636937636844819812189791288187243913404055721058334520072574568680438360936320682628189506248931475232504868784141809162526982794777886937554791279646171992316154768489491205932973020390955775825994246509354890417980543491344959419958264200222321573290332068573840656874584148318471805081070819330139498643368112616125508016850665039138240007045133711819182960399913468566074586611076818097815310939823561848962949647054263397457358507697316036204724311688330058092618087260011626918624130336633163118234963001890740389604366796070789463043007475519162863457847133916866147682877703700016314519649272629853810342756").unwrap());
        r.insert("height".to_string(), BigNumber::from_dec("32014206266070285395118493698246684536543402308857326229844369749153998025988120078148833919040926762489849787174726278317154939222455553684674979640533728771798727404529140716275948809394914126446467274094766630776034154814466245563241594664595503357965283703581353868787640425189228669159837529621065262578472511140258233443082035493432067002995028424708181638248338655901732889892559561796172833245307347288440850886016760883963087954594369665160758244185860669353304463245326602784567519981372129418674907732019485821481470791951576038671383506105840172336020165255666872489673679749492975692222529386986002548508").unwrap());
        r.insert("age".to_string(), BigNumber::from_dec("5573886601587513393941805393558438475134278869721908377896820376573868172897985632537697650826768061917733566546691785934393119648542993289296693181509209448802827620254572500988956963540401872482092959068516484681223765164669694589952326903719257213107559712016680752042520470482095682948519795635218252370953948099226141669796718651544648226881826585169101432801215379161624527044414118535373924688074790569833168081423701512430033511620744395776217769497965549575153091462845485986562792539143519413414753164756782101386489471333391388468474082175228293592033872018644198196278046021752128670441648674265160079365").unwrap());
        r.insert("sex".to_string(), BigNumber::from_dec("44319112097252841415305877008967513656231862316131581238409828513703699212059952418622049664178569730633939544882861264006945675755509881864438312327074402062963599178195087536260752294006450133601248863198870283839961116512248865885787100775903023034879852152846002669257161013317472827548494571935048240800817870893700771269978535707078640961353407573194897812343272563394036737677668293122931520603798620428922052839619195929427039933665104815440476791376703125056734891504425929510493567119107731184250744646520780647416583157402277832961026300695141515177928171182043898138863324570665593349095177082259229019129").unwrap());

        let n = BigNumber::from_dec("95230844261716231334966278654105782744493078250034916428724307571481648650972254096365233503303500776910009532385733941342231244809050180342216701303297309484964627111488667613567243812137828734726055835536190375874228378361894062875040911721595668269426387378524841651770329520854646198182993599992246846197622806018586940960824812499707703407200235006250330376435395757240807360245145895448238973940748414130249165698642798758094515234629492123379833360060582377815656998861873479266942101526163937107816424422201955494796734174781894506437514751553369884508767256335322189421050651814494097369702888544056010606733").unwrap();
        let s = BigNumber::from_dec("83608735581956052060766602122241456047092927591272898317077507857903324472083195301035502442829713523495655160192410742120440247481077060649728889735943333622709039987090137325037494001551239812739256925595650405403616377574225590614582056226657979932825031688262428848508620618206304014287232713708048427099425348438343473342088258502098208531627321778163620061043269821806176268690486341352405206188888371253713940995260309747672937693391957731544958179245054768704977202091642139481745073141174316305851938990898215928942632876267309335084279137046749673230694376359278715909536580114502953378593787412958122696491").unwrap();
        let rms = BigNumber::from_dec("12002410972675035848706631786298987049295298281772467607461994087192649160666347028767622091944497528304565759377490497287538655369597530498218287879384450121974605678051982553150980093839175365101087722528582689341030912237571526676430070213849160857477430406424356131111577547636360346507596843363617776545054084329725294982409132506989181200852351104199115448152798956456818387289142907618956667090125913885442746763678284193811934837479547315881192351556311788630337391374089308234091189363160599574268958752271955343795665269131980077642259235693653829664040302092446308732796745472579352704501330580826351662240").unwrap();
        let rpa = BigNumber::from_dec("12002410972675035848706631786298987049295298281772467607461994087192649160666347028767622091944497528304565759377490497287538655369597530498218287879384450121974605678051982553150980093839175365101087722528582689341030912237571526676430070213849160857477430406424356131111577547636360346507596843363617776545054084329725294982409132506989181200852351104199115448152798956456818387289142907618956667090125913885442746763678284193811934837479547315881192351556311788630337391374089308234091189363160599574268958752271955343795665269131980077642259235693653829664040302092446308732796745472579352704501330580826351662240").unwrap();
        let rctxt = BigNumber::from_dec("77129119521935975385795386930301402827628026853991528755303486255023263353142617098662225360498227999564663438861313570702364984107826653399214544314002820732458443871729599318191904265844432709910182014204478532265518566229953111318413830009256162339443077098917698777223763712267731802804425167444165048596271025553618253855465562660530445682078873631967934956107222619891473818051441942768338388425312823594456990243766677728754477201176089151138798586336262283249409402074987943625960454785501038059209634637204497573094989557296328178873844804605590768348774565136642366470996059740224170274762372312531963184654").unwrap();
        let z = BigNumber::from_dec("55164544925922114758373643773121488212903100773688663772257168750760838562077540114734459902014369305346806516101767509487128278169584105585138623374643674838487232408713159693511105298301789373764578281065365292802332455328842835614608027129883137292324033168485729810074426971615144489078436563295402449746541981155232849178606822309310700682675942602404109375598809372735287212196379089816519481644996930522775604565458855945697714216633192192613598668941671920105596720544264146532180330974698466182799108850159851058132630467033919618658033816306014912309279430724013987717126519405488323062369100827358874261055").unwrap();

        PublicKey::new(n, s, rms, rpa, r, rctxt, z)
    }
}
