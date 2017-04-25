use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::{GroupOrderElement, PointG1, Pair};
use std::collections::{HashMap, HashSet};
use errors::crypto::CryptoError;
use services::crypto::anoncreds::helpers::CopyFrom;
use utils::json::{JsonEncodable, JsonDecodable};

pub enum ByteOrder {
    Big,
    Little
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SchemaKey {
    pub name: String,
    pub version: String,
    pub issue_id: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub attribute_names: HashSet<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKey {
    pub n: BigNumber,
    pub s: BigNumber,
    pub rms: BigNumber,
    pub r: HashMap<String, BigNumber>,
    pub rctxt: BigNumber,
    pub z: BigNumber
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationPublicKey {
    pub g: PointG1,
    pub h: PointG1,
    pub h0: PointG1,
    pub h1: PointG1,
    pub h2: PointG1,
    pub htilde: PointG1,
    pub u: PointG1,
    pub pk: PointG1,
    pub y: PointG1,
    pub x: GroupOrderElement
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationSecretKey {
    pub x: GroupOrderElement,
    pub sk: GroupOrderElement
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretKey {
    pub p: BigNumber,
    pub q: BigNumber
}

pub struct AccumulatorPublicKey {
    pub z: Pair
}

pub struct AccumulatorSecretKey {
    pub gamma: GroupOrderElement
}

pub struct Accumulator {
    pub accumulator_id: i32,
    pub acc: PointG1,
    pub v: HashSet<i32>,
    pub max_claim_num: i32,
    pub current_i: i32
}

impl Accumulator {
    pub fn is_full(&self) -> bool {
        self.current_i > self.max_claim_num
    }
}

pub struct Witness {
    pub sigma_i: PointG1,
    pub u_i: PointG1,
    pub g_i: PointG1,
    pub omega: PointG1,
    pub v: HashSet<i32>
}

pub struct ClaimRequest {
    pub user_id: String,
    pub u: BigNumber,
    pub ur: PointG1
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub encode: bool
}

pub struct ClaimInitData {
    pub u: BigNumber,
    pub v_prime: BigNumber
}

pub struct Claims {
    pub primary_claim: PrimaryClaim,
    //nonRevocClai,
}

#[derive(Debug)]
pub struct PrimaryClaim {
    pub encoded_attributes: HashMap<String, BigNumber>,
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
    pub v_prime: BigNumber
}

pub struct NonRevocationClaim {
    pub accumulator_id: i32,
    pub sigma: PointG1,
    pub c: GroupOrderElement,
    pub vr_prime_prime: GroupOrderElement,
    pub witness: Witness,
    pub g_i: PointG1,
    pub i: i32,
    pub m2: GroupOrderElement
}

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

pub struct NonRevocProofTauList {
    pub t1: Pair,
    pub t2: Pair,
    pub t3: Pair,
    pub t4: Pair,
    pub t5: Pair,
    pub t6: Pair,
    pub t7: Pair,
    pub t8: Pair
}

#[derive(Clone)]
pub struct NonRevocProofCList {
    pub e: PointG1,
    pub d: PointG1,
    pub a: PointG1,
    pub g: PointG1,
    pub w: PointG1,
    pub s: PointG1,
    pub u: PointG1
}

pub struct ProofInput {
    pub revealed_attrs: HashSet<String>,
    pub predicates: Vec<Predicate>,
    pub ts: Option<String>,
    pub pubseq_no: Option<String>
}

pub struct ProofClaims {
    pub claims: Claims,
    pub revealed_attrs: HashSet<String>,
    pub predicates: Vec<Predicate>
}

pub struct FullProof {
    pub c_hash: BigNumber,
    pub schema_keys: Vec<SchemaKey>,
    pub proofs: Vec<Proof>,
    pub c_list: Vec<BigNumber>
}

pub struct Proof {
    pub primary_proof: PrimaryProof
    //non_revocation_proof
}

pub struct InitProof {
    pub primary_init_proof: PrimaryInitProof,
    //nonRevocInitProof
}

pub struct PrimaryInitProof {
    pub eq_proof: PrimaryEqualInitProof,
    pub ge_proofs: Vec<PrimaryPrecicateGEInitProof>
}

pub struct PrimaryProof {
    pub eq_proof: PrimaryEqualProof,
    pub ge_proofs: Vec<PrimaryPredicateGEProof>
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
    pub unrevealed_attrs: HashSet<String>,
    pub revealed_attrs: HashSet<String>,
    pub encoded_attributes: HashMap<String, BigNumber>,
    pub m2: BigNumber
}

pub struct PrimaryPrecicateGEInitProof {
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

pub struct PrimaryEqualProof {
    pub revealed_attr_names: HashSet<String>,
    pub a_prime: BigNumber,
    pub e: BigNumber,
    pub v: BigNumber,
    pub m: HashMap<String, BigNumber>,
    pub m1: BigNumber,
    pub m2: BigNumber
}

pub struct PrimaryPredicateGEProof {
    pub u: HashMap<String, BigNumber>,
    pub r: HashMap<String, BigNumber>,
    pub mj: BigNumber,
    pub alpha: BigNumber,
    pub t: HashMap<String, BigNumber>,
    pub predicate: Predicate
}


//impl block
impl PrimaryClaim {
    pub fn update_vprime(&mut self, v_prime: &BigNumber) -> Result<(), CryptoError> {
        self.v_prime = self.v_prime.add(&v_prime)?;
        Ok(())
    }
}

impl Claims {
    pub fn prepare_primary_claim(&mut self, v_prime: &BigNumber) -> Result<(), CryptoError> {
        self.primary_claim.update_vprime(v_prime)?;
        Ok(())
    }
}

impl PrimaryEqualInitProof {
    pub fn as_c_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.a_prime.clone()?])
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.t.clone()?])
    }
}

impl PrimaryPrecicateGEInitProof {
    pub fn as_c_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.c_list)
    }

    pub fn as_tau_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.tau_list)
    }
}

impl PrimaryInitProof {
    pub fn as_c_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        let mut c_list: Vec<BigNumber> = self.eq_proof.as_c_list()?;
        for ge_proof in self.ge_proofs.iter() {
            c_list.clone_from_vector(ge_proof.as_c_list()?)?;
        }
        Ok(c_list)
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        let mut tau_list: Vec<BigNumber> = self.eq_proof.as_tau_list()?;
        for ge_proof in self.ge_proofs.iter() {
            tau_list.clone_from_vector(ge_proof.as_tau_list()?)?;
        }
        Ok(tau_list)
    }
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

impl JsonEncodable for PublicKey {}

impl<'a> JsonDecodable<'a> for PublicKey {}

impl JsonEncodable for SecretKey {}

impl<'a> JsonDecodable<'a> for SecretKey {}

impl JsonEncodable for RevocationPublicKey {}

impl<'a> JsonDecodable<'a> for RevocationPublicKey {}

impl JsonEncodable for RevocationSecretKey {}

impl<'a> JsonDecodable<'a> for RevocationSecretKey {}
