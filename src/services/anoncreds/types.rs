use utils::crypto::bn::BigNumber;
use utils::crypto::pair::{GroupOrderElement, PointG1, PointG2, Pair};
use errors::common::CommonError;
use services::anoncreds::helpers::{AppendByteArray, clone_bignum_map};
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use utils::json::{JsonEncodable, JsonDecodable};

pub enum ByteOrder {
    Big,
    Little
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PredicateType {
    GE
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Accumulator {
    pub acc: PointG2,
    pub v: HashSet<i32>,
    pub max_claim_num: i32,
    pub current_i: i32
}

impl Accumulator {
    pub fn new(acc: PointG2, v: HashSet<i32>, max_claim_num: i32,
               current_i: i32) -> Accumulator {
        Accumulator {
            acc: acc,
            v: v,
            max_claim_num: max_claim_num,
            current_i: current_i
        }
    }

    pub fn is_full(&self) -> bool {
        self.current_i > self.max_claim_num
    }
}

impl JsonEncodable for Accumulator {}

impl<'a> JsonDecodable<'a> for Accumulator {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccumulatorPublicKey {
    pub z: Pair
}

impl AccumulatorPublicKey {
    pub fn new(z: Pair) -> AccumulatorPublicKey {
        AccumulatorPublicKey {
            z: z
        }
    }
}

impl JsonEncodable for AccumulatorPublicKey {}

impl<'a> JsonDecodable<'a> for AccumulatorPublicKey {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccumulatorSecretKey {
    pub gamma: GroupOrderElement
}

impl AccumulatorSecretKey {
    pub fn new(gamma: GroupOrderElement) -> AccumulatorSecretKey {
        AccumulatorSecretKey {
            gamma: gamma
        }
    }
}

impl JsonEncodable for AccumulatorSecretKey {}

impl<'a> JsonDecodable<'a> for AccumulatorSecretKey {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatedProof {
    pub c_hash: BigNumber,
    pub c_list: Vec<Vec<u8>>
}

impl AggregatedProof {
    pub fn new(c_hash: BigNumber, c_list: Vec<Vec<u8>>) -> AggregatedProof {
        AggregatedProof {
            c_hash: c_hash,
            c_list: c_list
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub schema_seq_no: i32,
    pub name: String
}

impl AttributeInfo {
    pub fn new(schema_seq_no: i32, name: String) -> AttributeInfo {
        AttributeInfo {
            schema_seq_no: schema_seq_no,
            name: name
        }
    }
}

impl JsonEncodable for AttributeInfo {}

impl<'a> JsonDecodable<'a> for AttributeInfo {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl ClaimOffer {
    pub fn new(issuer_did: String, claim_def_seq_no: i32) -> ClaimOffer {
        ClaimOffer {
            issuer_did: issuer_did,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimOfferFilter{
    pub issuer_did: Option<String>,
    pub claim_def_seq_no: Option<i32>

}

impl<'a> JsonDecodable<'a> for ClaimOfferFilter {}

impl JsonEncodable for ClaimOffer {}

impl<'a> JsonDecodable<'a> for ClaimOffer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimRequestJson {
    pub claim_request: ClaimRequest,
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}

impl ClaimRequestJson {
    pub fn new(claim_request: ClaimRequest, issuer_did: String, claim_def_seq_no: i32) -> ClaimRequestJson {
        ClaimRequestJson {
            claim_request: claim_request,
            issuer_did: issuer_did,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

impl JsonEncodable for ClaimRequestJson {}

impl<'a> JsonDecodable<'a> for ClaimRequestJson {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClaimInfo {
    pub claim_uuid: String,
    pub attrs: HashMap<String, String>,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32
}

impl ClaimInfo {
    pub fn new(claim_uuid: String, attrs: HashMap<String, String>, claim_def_seq_no: i32,
               revoc_reg_seq_no: Option<i32>, schema_seq_no: i32) -> ClaimInfo {
        ClaimInfo {
            claim_uuid: claim_uuid,
            attrs: attrs,
            claim_def_seq_no: claim_def_seq_no,
            revoc_reg_seq_no: revoc_reg_seq_no,
            schema_seq_no: schema_seq_no
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimInfoFilter{
    pub issuer_did: Option<String>,
    pub claim_def_seq_no: Option<i32>,
    pub schema_seq_no: Option<i32>

}

impl<'a> JsonDecodable<'a> for ClaimInfoFilter {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub prover_did: String,
    pub u: BigNumber,
    pub ur: Option<PointG1>
}

impl ClaimRequest {
    pub fn new(prover_did: String, u: BigNumber, ur: Option<PointG1>) -> ClaimRequest {
        ClaimRequest {
            prover_did: prover_did,
            u: u,
            ur: ur
        }
    }
}

impl JsonEncodable for ClaimRequest {}

impl<'a> JsonDecodable<'a> for ClaimRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimProof {
    pub proof: Proof,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: Option<i32>
}

impl ClaimProof {
    pub fn new(proof: Proof, claim_def_seq_no: i32, revoc_reg_seq_no: Option<i32>) -> ClaimProof {
        ClaimProof {
            proof: proof,
            claim_def_seq_no: claim_def_seq_no,
            revoc_reg_seq_no: revoc_reg_seq_no
        }
    }
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinition {
    pub public_key: PublicKey,
    pub public_key_revocation: Option<RevocationPublicKey>,
    pub schema_seq_no: i32,
    pub signature_type: String
}

impl ClaimDefinition {
    pub fn new(public_key: PublicKey, public_key_revocation: Option<RevocationPublicKey>,
               schema_seq_no: i32, signature_type: String) -> ClaimDefinition {
        ClaimDefinition {
            public_key: public_key,
            public_key_revocation: public_key_revocation,
            schema_seq_no: schema_seq_no,
            signature_type: signature_type
        }
    }

    pub fn clone(&self) -> Result<ClaimDefinition, CommonError> {
        Ok(ClaimDefinition {
            public_key: self.public_key.clone()?,
            public_key_revocation: self.public_key_revocation.clone(),
            schema_seq_no: self.schema_seq_no.clone(),
            signature_type: self.signature_type.clone(),
        })
    }
}

impl JsonEncodable for ClaimDefinition {}

impl<'a> JsonDecodable<'a> for ClaimDefinition {}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinitionPrivate {
    pub secret_key: SecretKey,
    pub secret_key_revocation: Option<RevocationSecretKey>
}

impl ClaimDefinitionPrivate {
    pub fn new(secret_key: SecretKey, secret_key_revocation: Option<RevocationSecretKey>) -> ClaimDefinitionPrivate {
        ClaimDefinitionPrivate {
            secret_key: secret_key,
            secret_key_revocation: secret_key_revocation
        }
    }
}

impl JsonEncodable for ClaimDefinitionPrivate {}

impl<'a> JsonDecodable<'a> for ClaimDefinitionPrivate {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimSignature {
    pub primary_claim: PrimaryClaim,
    pub non_revocation_claim: Option<RefCell<NonRevocationClaim>>
}

impl ClaimSignature {
    pub fn new(primary_claim: PrimaryClaim,
               non_revocation_claim: Option<RefCell<NonRevocationClaim>>) -> ClaimSignature {
        ClaimSignature {
            primary_claim: primary_claim,
            non_revocation_claim: non_revocation_claim
        }
    }
}

impl JsonEncodable for ClaimSignature {}

impl<'a> JsonDecodable<'a> for ClaimSignature {}

#[derive(Deserialize, Serialize)]
pub struct ClaimInitData {
    pub u: BigNumber,
    pub v_prime: BigNumber
}

impl ClaimInitData {
    pub fn new(u: BigNumber, v_prime: BigNumber) -> ClaimInitData {
        ClaimInitData {
            u: u,
            v_prime: v_prime
        }
    }
}

impl JsonEncodable for ClaimInitData {}

impl<'a> JsonDecodable<'a> for ClaimInitData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimJson {
    pub claim: HashMap<String, Vec<String>>,
    pub claim_def_seq_no: i32,
    pub revoc_reg_seq_no: Option<i32>,
    pub schema_seq_no: i32,
    pub signature: ClaimSignature
}

impl ClaimJson {
    pub fn new(claim: HashMap<String, Vec<String>>, claim_def_seq_no: i32, revoc_reg_seq_no: Option<i32>,
               signature: ClaimSignature, schema_seq_no: i32) -> ClaimJson {
        ClaimJson {
            claim: claim,
            claim_def_seq_no: claim_def_seq_no,
            revoc_reg_seq_no: revoc_reg_seq_no,
            schema_seq_no: schema_seq_no,
            signature: signature
        }
    }
}

impl JsonEncodable for ClaimJson {}

impl<'a> JsonDecodable<'a> for ClaimJson {}

pub struct InitProof {
    pub primary_init_proof: PrimaryInitProof,
    pub non_revoc_init_proof: Option<NonRevocInitProof>
}

impl InitProof {
    pub fn new(primary_init_proof: PrimaryInitProof, non_revoc_init_proof: Option<NonRevocInitProof>) -> InitProof {
        InitProof {
            primary_init_proof: primary_init_proof,
            non_revoc_init_proof: non_revoc_init_proof
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonRevocationClaim {
    pub sigma: PointG1,
    pub c: GroupOrderElement,
    pub vr_prime_prime: GroupOrderElement,
    pub witness: Witness,
    pub g_i: PointG1,
    pub i: i32,
    pub m2: GroupOrderElement
}

impl NonRevocationClaim {
    pub fn new(sigma: PointG1, c: GroupOrderElement, vr_prime_prime: GroupOrderElement,
               witness: Witness, g_i: PointG1, i: i32, m2: GroupOrderElement) -> NonRevocationClaim {
        NonRevocationClaim {
            sigma: sigma,
            c: c,
            vr_prime_prime: vr_prime_prime,
            witness: witness,
            g_i: g_i,
            i: i,
            m2: m2
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonRevocProofXList {
    pub rho: GroupOrderElement,
    pub r: GroupOrderElement,
    pub r_prime: GroupOrderElement,
    pub r_prime_prime: GroupOrderElement,
    pub r_prime_prime_prime: GroupOrderElement,
    pub o: GroupOrderElement,
    pub o_prime: GroupOrderElement,
    pub m: GroupOrderElement,
    pub m_prime: GroupOrderElement,
    pub t: GroupOrderElement,
    pub t_prime: GroupOrderElement,
    pub m2: GroupOrderElement,
    pub s: GroupOrderElement,
    pub c: GroupOrderElement
}

impl NonRevocProofXList {
    pub fn new(rho: GroupOrderElement, r: GroupOrderElement, r_prime: GroupOrderElement,
               r_prime_prime: GroupOrderElement, r_prime_prime_prime: GroupOrderElement,
               o: GroupOrderElement, o_prime: GroupOrderElement, m: GroupOrderElement,
               m_prime: GroupOrderElement, t: GroupOrderElement, t_prime: GroupOrderElement,
               m2: GroupOrderElement, s: GroupOrderElement,
               c: GroupOrderElement) -> NonRevocProofXList {
        NonRevocProofXList {
            rho: rho,
            r: r,
            r_prime: r_prime,
            r_prime_prime: r_prime_prime,
            r_prime_prime_prime: r_prime_prime_prime,
            o: o,
            o_prime: o_prime,
            m: m,
            m_prime: m_prime,
            t: t,
            t_prime: t_prime,
            m2: m2,
            s: s,
            c: c
        }
    }

    pub fn as_list(&self) -> Result<Vec<GroupOrderElement>, CommonError> {
        Ok(vec![self.rho, self.o, self.c, self.o_prime, self.m, self.m_prime, self.t, self.t_prime,
                self.m2, self.s, self.r, self.r_prime, self.r_prime_prime, self.r_prime_prime_prime])
    }

    pub fn from_list(seq: Vec<GroupOrderElement>) -> NonRevocProofXList {
        NonRevocProofXList::new(seq[0], seq[1], seq[2], seq[3], seq[4], seq[5], seq[6], seq[7],
                                seq[8], seq[9], seq[10], seq[11], seq[12], seq[13])
    }
}

impl JsonEncodable for NonRevocProofXList {}

impl<'a> JsonDecodable<'a> for NonRevocProofXList {}

pub struct NonRevocProofTauList {
    pub t1: PointG1,
    pub t2: PointG1,
    pub t3: Pair,
    pub t4: Pair,
    pub t5: PointG1,
    pub t6: PointG1,
    pub t7: Pair,
    pub t8: Pair
}

impl NonRevocProofTauList {
    pub fn new(t1: PointG1, t2: PointG1, t3: Pair, t4: Pair, t5: PointG1, t6: PointG1, t7: Pair,
               t8: Pair) -> NonRevocProofTauList {
        NonRevocProofTauList {
            t1: t1,
            t2: t2,
            t3: t3,
            t4: t4,
            t5: t5,
            t6: t6,
            t7: t7,
            t8: t8
        }
    }

    pub fn as_slice(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        Ok(vec![self.t1.to_bytes()?, self.t2.to_bytes()?, self.t3.to_bytes()?, self.t4.to_bytes()?,
                self.t5.to_bytes()?, self.t6.to_bytes()?, self.t7.to_bytes()?, self.t8.to_bytes()?])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonRevocProofCList {
    pub e: PointG1,
    pub d: PointG1,
    pub a: PointG1,
    pub g: PointG1,
    pub w: PointG2,
    pub s: PointG2,
    pub u: PointG2
}

impl NonRevocProofCList {
    pub fn new(e: PointG1, d: PointG1, a: PointG1, g: PointG1, w: PointG2, s: PointG2,
               u: PointG2) -> NonRevocProofCList {
        NonRevocProofCList {
            e: e,
            d: d,
            a: a,
            g: g,
            w: w,
            s: s,
            u: u
        }
    }

    pub fn as_list(&self) -> Result<Vec<Vec<u8>>, CryptoError> {
        Ok(vec![self.e.to_bytes()?, self.d.to_bytes()?, self.a.to_bytes()?, self.g.to_bytes()?,
                self.w.to_bytes()?, self.s.to_bytes()?, self.u.to_bytes()?])
    }
}

impl JsonEncodable for NonRevocProofCList {}

impl<'a> JsonDecodable<'a> for NonRevocProofCList {}

pub struct NonRevocInitProof {
    pub c_list_params: NonRevocProofXList,
    pub tau_list_params: NonRevocProofXList,
    pub c_list: NonRevocProofCList,
    pub tau_list: NonRevocProofTauList
}

impl NonRevocInitProof {
    pub fn new(c_list_params: NonRevocProofXList, tau_list_params: NonRevocProofXList,
               c_list: NonRevocProofCList, tau_list: NonRevocProofTauList) -> NonRevocInitProof {
        NonRevocInitProof {
            c_list_params: c_list_params,
            tau_list_params: tau_list_params,
            c_list: c_list,
            tau_list: tau_list
        }
    }

    pub fn as_c_list(&self) -> Result<Vec<Vec<u8>>, CryptoError> {
        let vec = self.c_list.as_list()?;
        Ok(vec)
    }

    pub fn as_tau_list(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        let vec = self.tau_list.as_slice()?;
        Ok(vec)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NonRevocProof {
    pub x_list: NonRevocProofXList,
    pub c_list: NonRevocProofCList
}

impl NonRevocProof {
    pub fn new(x_list: NonRevocProofXList, c_list: NonRevocProofCList) -> NonRevocProof {
        NonRevocProof {
            x_list: x_list,
            c_list: c_list
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    pub n: BigNumber,
    pub s: BigNumber,
    pub rms: BigNumber,
    pub r: HashMap<String, BigNumber>,
    pub rctxt: BigNumber,
    pub z: BigNumber
}

impl PublicKey {
    pub fn new(n: BigNumber, s: BigNumber, rms: BigNumber, r: HashMap<String, BigNumber>,
               rctxt: BigNumber, z: BigNumber) -> PublicKey {
        PublicKey {
            n: n,
            s: s,
            rms: rms,
            r: r,
            rctxt: rctxt,
            z: z
        }
    }

    pub fn clone(&self) -> Result<PublicKey, CommonError> {
        Ok(PublicKey {
            s: self.s.clone()?,
            n: self.n.clone()?,
            rms: self.rms.clone()?,
            r: clone_bignum_map(&self.r)?,
            rctxt: self.rctxt.clone()?,
            z: self.z.clone()?
        })
    }
}

impl JsonEncodable for PublicKey {}

impl<'a> JsonDecodable<'a> for PublicKey {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: PredicateType,
    pub value: i32
}

impl Predicate {
    pub fn new(attr_name: String, p_type: PredicateType, value: i32) -> Predicate {
        Predicate {
            attr_name: attr_name,
            p_type: p_type,
            value: value
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryClaim {
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
    pub v_prime: BigNumber
}

impl PrimaryClaim {
    pub fn new(m2: BigNumber, a: BigNumber, e: BigNumber, v_prime: BigNumber) -> PrimaryClaim {
        PrimaryClaim {
            m2: m2,
            a: a,
            e: e,
            v_prime: v_prime
        }
    }
}

pub struct ProofClaims {
    pub claim_json: ClaimJson,
    pub schema: Schema,
    pub claim_definition: ClaimDefinition,
    pub revocation_registry: Option<RevocationRegistry>,
    pub revealed_attrs: Vec<String>,
    pub unrevealed_attrs: Vec<String>,
    pub predicates: Vec<Predicate>
}

impl ProofClaims {
    pub fn new(claim_json: ClaimJson, schema: Schema, claim_definition: ClaimDefinition,
               revocation_registry: Option<RevocationRegistry>, predicates: Vec<Predicate>,
               revealed_attrs: Vec<String>, unrevealed_attrs: Vec<String>) -> ProofClaims {
        ProofClaims {
            claim_json: claim_json,
            schema: schema,
            claim_definition: claim_definition,
            revocation_registry: revocation_registry,
            revealed_attrs: revealed_attrs,
            predicates: predicates,
            unrevealed_attrs: unrevealed_attrs
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub primary_proof: PrimaryProof,
    pub non_revoc_proof: Option<NonRevocProof>
}

impl Proof {
    pub fn new(primary_proof: PrimaryProof, non_revoc_proof: Option<NonRevocProof>) -> Proof {
        Proof {
            primary_proof: primary_proof,
            non_revoc_proof: non_revoc_proof
        }
    }
}

pub struct PrimaryInitProof {
    pub eq_proof: PrimaryEqualInitProof,
    pub ge_proofs: Vec<PrimaryPredicateGEInitProof>
}

impl PrimaryInitProof {
    pub fn new(eq_proof: PrimaryEqualInitProof,
               ge_proofs: Vec<PrimaryPredicateGEInitProof>) -> PrimaryInitProof {
        PrimaryInitProof {
            eq_proof: eq_proof,
            ge_proofs: ge_proofs
        }
    }

    pub fn as_c_list(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        let mut c_list: Vec<Vec<u8>> = self.eq_proof.as_list()?;
        for ge_proof in self.ge_proofs.iter() {
            c_list.append_vec(ge_proof.as_list()?)?;
        }
        Ok(c_list)
    }

    pub fn as_tau_list(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        let mut tau_list: Vec<Vec<u8>> = self.eq_proof.as_tau_list()?;
        for ge_proof in self.ge_proofs.iter() {
            tau_list.append_vec(ge_proof.as_tau_list()?)?;
        }
        Ok(tau_list)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryProof {
    pub eq_proof: PrimaryEqualProof,
    pub ge_proofs: Vec<PrimaryPredicateGEProof>
}

impl PrimaryProof {
    pub fn new(eq_proof: PrimaryEqualProof, ge_proofs: Vec<PrimaryPredicateGEProof>) -> PrimaryProof {
        PrimaryProof {
            eq_proof: eq_proof,
            ge_proofs: ge_proofs
        }
    }
}

pub struct PrimaryEqualInitProof {
    pub a_prime: BigNumber,
    pub t: BigNumber,
    pub etilde: BigNumber,
    pub eprime: BigNumber,
    pub vtilde: BigNumber,
    pub vprime: BigNumber,
    pub mtilde: HashMap<String, BigNumber>,
    pub m1_tilde: BigNumber,
    pub m2_tilde: BigNumber,
    pub m2: BigNumber
}

impl PrimaryEqualInitProof {
    pub fn new(a_prime: BigNumber, t: BigNumber, etilde: BigNumber, eprime: BigNumber,
               vtilde: BigNumber, vprime: BigNumber, mtilde: HashMap<String, BigNumber>,
               m1_tilde: BigNumber, m2_tilde: BigNumber, m2: BigNumber) -> PrimaryEqualInitProof {
        PrimaryEqualInitProof {
            a_prime: a_prime,
            t: t,
            etilde: etilde,
            eprime: eprime,
            vtilde: vtilde,
            vprime: vprime,
            mtilde: mtilde,
            m1_tilde: m1_tilde,
            m2_tilde: m2_tilde,
            m2: m2
        }
    }

    pub fn as_list(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        Ok(vec![self.a_prime.to_bytes()?])
    }

    pub fn as_tau_list(&self) -> Result<Vec<Vec<u8>>, CommonError> {
        Ok(vec![self.t.to_bytes()?])
    }
}

pub struct PrimaryPredicateGEInitProof {
    pub c_list: Vec<BigNumber>,
    pub tau_list: Vec<BigNumber>,
    pub u: HashMap<String, BigNumber>,
    pub u_tilde: HashMap<String, BigNumber>,
    pub r: HashMap<String, BigNumber>,
    pub r_tilde: HashMap<String, BigNumber>,
    pub alpha_tilde: BigNumber,
    pub predicate: Predicate,
    pub t: HashMap<String, BigNumber>
}

impl PrimaryPredicateGEInitProof {
    pub fn new(c_list: Vec<BigNumber>, tau_list: Vec<BigNumber>, u: HashMap<String, BigNumber>,
               u_tilde: HashMap<String, BigNumber>, r: HashMap<String, BigNumber>,
               r_tilde: HashMap<String, BigNumber>, alpha_tilde: BigNumber, predicate: Predicate,
               t: HashMap<String, BigNumber>) -> PrimaryPredicateGEInitProof {
        PrimaryPredicateGEInitProof {
            c_list: c_list,
            tau_list: tau_list,
            u: u,
            u_tilde: u_tilde,
            r: r,
            r_tilde: r_tilde,
            alpha_tilde: alpha_tilde,
            predicate: predicate,
            t: t
        }
    }

    pub fn as_list(&self) -> Result<&Vec<BigNumber>, CommonError> {
        Ok(&self.c_list)
    }

    pub fn as_tau_list(&self) -> Result<&Vec<BigNumber>, CommonError> {
        Ok(&self.tau_list)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryEqualProof {
    pub revealed_attrs: HashMap<String, String>,
    pub a_prime: BigNumber,
    pub e: BigNumber,
    pub v: BigNumber,
    pub m: HashMap<String, BigNumber>,
    pub m1: BigNumber,
    pub m2: BigNumber
}

impl PrimaryEqualProof {
    pub fn new(revealed_attrs: HashMap<String, String>, a_prime: BigNumber, e: BigNumber,
               v: BigNumber, m: HashMap<String, BigNumber>, m1: BigNumber,
               m2: BigNumber) -> PrimaryEqualProof {
        PrimaryEqualProof {
            revealed_attrs: revealed_attrs,
            a_prime: a_prime,
            e: e,
            v: v,
            m: m,
            m1: m1,
            m2: m2
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryPredicateGEProof {
    pub u: HashMap<String, BigNumber>,
    pub r: HashMap<String, BigNumber>,
    pub mj: BigNumber,
    pub alpha: BigNumber,
    pub t: HashMap<String, BigNumber>,
    pub predicate: Predicate
}

impl PrimaryPredicateGEProof {
    pub fn new(u: HashMap<String, BigNumber>, r: HashMap<String, BigNumber>, mj: BigNumber,
               alpha: BigNumber, t: HashMap<String, BigNumber>,
               predicate: Predicate) -> PrimaryPredicateGEProof {
        PrimaryPredicateGEProof {
            u: u,
            r: r,
            mj: mj,
            alpha: alpha,
            t: t,
            predicate: predicate
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofClaimsJson {
    pub attrs: HashMap<String, Vec<ClaimInfo>>,
    pub predicates: HashMap<String, Vec<ClaimInfo>>
}

impl ProofClaimsJson {
    pub fn new(attrs: HashMap<String, Vec<ClaimInfo>>, predicates: HashMap<String, Vec<ClaimInfo>>) -> ProofClaimsJson {
        ProofClaimsJson {
            attrs: attrs,
            predicates: predicates
        }
    }
}

impl JsonEncodable for ProofClaimsJson {}

impl<'a> JsonDecodable<'a> for ProofClaimsJson {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequestJson {
    pub nonce: BigNumber,
    pub requested_attrs: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, Predicate>
}

impl ProofRequestJson {
    pub fn new(nonce: BigNumber, requested_attr: HashMap<String, AttributeInfo>,
               requested_predicate: HashMap<String, Predicate>) -> ProofRequestJson {
        ProofRequestJson {
            nonce: nonce,
            requested_attrs: requested_attr,
            requested_predicates: requested_predicate
        }
    }
}

impl JsonEncodable for ProofRequestJson {}

impl<'a> JsonDecodable<'a> for ProofRequestJson {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofJson {
    pub proofs: HashMap<String, ClaimProof>,
    pub aggregated_proof: AggregatedProof,
    pub requested_proof: RequestedProofJson
}

impl ProofJson {
    pub fn new(proofs: HashMap<String, ClaimProof>, aggregated_proof: AggregatedProof,
               requested_proof: RequestedProofJson) -> ProofJson {
        ProofJson {
            proofs: proofs,
            aggregated_proof: aggregated_proof,
            requested_proof: requested_proof
        }
    }
}

impl JsonEncodable for ProofJson {}

impl<'a> JsonDecodable<'a> for ProofJson {}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistry {
    pub claim_def_seq_no: i32,
    pub accumulator: Accumulator,
    pub acc_pk: AccumulatorPublicKey
}

impl RevocationRegistry {
    pub fn new(accumulator: Accumulator, acc_pk: AccumulatorPublicKey,
               claim_def_seq_no: i32) -> RevocationRegistry {
        RevocationRegistry {
            accumulator: accumulator,
            acc_pk: acc_pk,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

impl JsonEncodable for RevocationRegistry {}

impl<'a> JsonDecodable<'a> for RevocationRegistry {}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RevocationRegistryPrivate {
    pub acc_sk: AccumulatorSecretKey,
    pub tails: HashMap<i32, PointG1>,
    pub tails_dash: HashMap<i32, PointG2>
}

impl RevocationRegistryPrivate {
    pub fn new(acc_sk: AccumulatorSecretKey, tails: HashMap<i32, PointG1>,
               tails_dash: HashMap<i32, PointG2>) -> RevocationRegistryPrivate {
        RevocationRegistryPrivate {
            acc_sk: acc_sk,
            tails: tails,
            tails_dash: tails_dash
        }
    }
}

impl JsonEncodable for RevocationRegistryPrivate {}

impl<'a> JsonDecodable<'a> for RevocationRegistryPrivate {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RevocationPublicKey {
    pub g: PointG1,
    pub g_dash: PointG2,
    pub h: PointG1,
    pub h0: PointG1,
    pub h1: PointG1,
    pub h2: PointG1,
    pub htilde: PointG1,
    pub h_cap: PointG2,
    pub u: PointG2,
    pub pk: PointG1,
    pub y: PointG2,
    pub x: GroupOrderElement
}

impl RevocationPublicKey {
    pub fn new(g: PointG1, g_dash: PointG2, h: PointG1, h0: PointG1, h1: PointG1, h2: PointG1, htilde: PointG1,
               h_cap: PointG2, u: PointG2, pk: PointG1, y: PointG2, x: GroupOrderElement) -> RevocationPublicKey {
        RevocationPublicKey {
            g: g,
            g_dash: g_dash,
            h: h,
            h0: h0,
            h1: h1,
            h2: h2,
            htilde: htilde,
            h_cap: h_cap,
            u: u,
            pk: pk,
            y: y,
            x: x
        }
    }
}

impl JsonEncodable for RevocationPublicKey {}

impl<'a> JsonDecodable<'a> for RevocationPublicKey {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RevocationSecretKey {
    pub x: GroupOrderElement,
    pub sk: GroupOrderElement
}

impl RevocationSecretKey {
    pub fn new(x: GroupOrderElement, sk: GroupOrderElement) -> RevocationSecretKey {
        RevocationSecretKey {
            x: x,
            sk: sk
        }
    }
}

impl JsonEncodable for RevocationSecretKey {}

impl<'a> JsonDecodable<'a> for RevocationSecretKey {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RevocationClaimInitData {
    pub u: PointG1,
    pub v_prime: GroupOrderElement
}

impl RevocationClaimInitData {
    pub fn new(u: PointG1, v_prime: GroupOrderElement) -> RevocationClaimInitData {
        RevocationClaimInitData {
            u: u,
            v_prime: v_prime
        }
    }
}

impl JsonEncodable for RevocationClaimInitData {}

impl<'a> JsonDecodable<'a> for RevocationClaimInitData {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedClaimsJson {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

impl RequestedClaimsJson {
    pub fn new(self_attested_attributes: HashMap<String, String>, requested_attrs: HashMap<String, (String, bool)>,
               requested_predicates: HashMap<String, String>) -> RequestedClaimsJson {
        RequestedClaimsJson {
            self_attested_attributes: self_attested_attributes,
            requested_attrs: requested_attrs,
            requested_predicates: requested_predicates
        }
    }
}

impl JsonEncodable for RequestedClaimsJson {}

impl<'a> JsonDecodable<'a> for RequestedClaimsJson {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedProofJson {
    pub revealed_attrs: HashMap<String, (String, String, String)>,
    pub unrevealed_attrs: HashMap<String, String>,
    pub self_attested_attrs: HashMap<String, String>,
    pub predicates: HashMap<String, String>
}

impl RequestedProofJson {
    pub fn new(revealed_attrs: HashMap<String, (String, String, String)>,
               unrevealed_attrs: HashMap<String, String>,
               self_attested_attrs: HashMap<String, String>,
               predicates: HashMap<String, String>) -> RequestedProofJson {
        RequestedProofJson {
            revealed_attrs: revealed_attrs,
            unrevealed_attrs: unrevealed_attrs,
            self_attested_attrs: self_attested_attrs,
            predicates: predicates
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub keys: HashSet<String>,
    pub seq_no: i32
}

impl Schema {
    pub fn new(name: String, version: String, keys: HashSet<String>, seq_no: i32) -> Schema {
        Schema {
            name: name,
            version: version,
            keys: keys,
            seq_no: seq_no
        }
    }
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SecretKey {
    pub p: BigNumber,
    pub q: BigNumber
}

impl SecretKey {
    pub fn new(p: BigNumber, q: BigNumber) -> SecretKey {
        SecretKey {
            p: p,
            q: q
        }
    }
}

impl JsonEncodable for SecretKey {}

impl<'a> JsonDecodable<'a> for SecretKey {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Witness {
    pub sigma_i: PointG2,
    pub u_i: PointG2,
    pub g_i: PointG1,
    pub omega: PointG2,
    pub v: HashSet<i32>
}

impl Witness {
    pub fn new(sigma_i: PointG2, u_i: PointG2, g_i: PointG1, omega: PointG2,
               v: HashSet<i32>) -> Witness {
        Witness {
            sigma_i: sigma_i,
            u_i: u_i,
            g_i: g_i,
            omega: omega,
            v: v
        }
    }
}
