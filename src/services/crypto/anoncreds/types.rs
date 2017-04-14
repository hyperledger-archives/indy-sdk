use services::crypto::wrappers::bn::BigNumber;
use std::collections::HashMap;
use std::rc::Rc;

pub enum ByteOrder {
    Big,
    Little
}

pub struct SchemaKey {
    pub name: String,
    pub version: String,
    pub issue_id: String
}

pub struct Schema {
    pub name: String,
    pub version: String,
    pub attribute_names: Vec<String>
}

#[derive(Debug)]
pub struct PublicKey {
    pub n: BigNumber,
    pub s: BigNumber,
    pub rms: BigNumber,
    pub r: HashMap<String, BigNumber>,
    pub rctxt: BigNumber,
    pub z: BigNumber
}

#[derive(Debug)]
pub struct SecretKey {
    pub p: BigNumber,
    pub q: BigNumber
}

pub struct ClaimInitData {
    pub u: BigNumber,
    pub v_prime: BigNumber
}

pub struct FullProof {
    pub c_hash: BigNumber,
    pub schema_keys: Vec<Rc<SchemaKey>>,
    pub proofs: Vec<Rc<Proof>>,
    pub c_list: Vec<BigNumber>
}

pub struct ProofInput {
    pub revealed_attrs: Vec<String>,
    pub predicates: Vec<Rc<Predicate>>,
    pub ts: String,
    pub pubseq_no: String
}

pub struct Proof {
    pub primary_proof: Option<Rc<PrimaryProof>>
    //non_revocation_proof
}

pub struct PrimaryProof {
    pub eq_proof: Rc<PrimaryEqualProof>,
    pub ge_proofs: Vec<Rc<PrimaryPredicateGEProof>>
}

pub struct PrimaryEqualProof {
    pub revealed_attr_names: Vec<String>,
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
    pub t: Rc<HashMap<String, BigNumber>>,
    pub predicate: Rc<Predicate>
}

#[derive(Clone, Debug)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32
}

pub struct PrimaryClaim {
    pub encoded_attrs: HashMap<String, BigNumber>,
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
    pub v: BigNumber
}

pub struct PrimaryEqualInitProof {
    pub c1: Rc<PrimaryClaim>,
    pub a_prime: BigNumber,
    pub t: BigNumber,
    pub etilde: BigNumber,
    pub eprime: BigNumber,
    pub vtilde: BigNumber,
    pub vprime: BigNumber,
    pub mtilde: HashMap<String, BigNumber>,
    pub m1_tilde: BigNumber,
    pub m2_tilde: BigNumber,
    pub unrevealed_attrs: Vec<String>,
    pub revealed_attrs: Vec<String>
}

pub struct PrimaryPrecicateGEInitProof {
    pub c_list: Vec<BigNumber>,
    pub tau_list: Vec<BigNumber>,
    pub u: HashMap<String, BigNumber>,
    pub u_tilde: HashMap<String, BigNumber>,
    pub r: HashMap<String, BigNumber>,
    pub r_tilde: HashMap<String, BigNumber>,
    pub alpha_tilde: BigNumber,
    pub predicate: Rc<Predicate>,
    pub t: Rc<HashMap<String, BigNumber>>
}

pub struct PrimaryInitProof {
    pub eq_proof: Rc<PrimaryEqualInitProof>,
    pub ge_proofs: Vec<Rc<PrimaryPrecicateGEInitProof>>
}

pub struct ClaimInitDataType {
    pub u: BigNumber,
    pub v_prime: BigNumber
}