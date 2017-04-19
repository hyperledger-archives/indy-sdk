use services::crypto::wrappers::bn::BigNumber;
use std::collections::HashMap;
use std::rc::Rc;
use errors::crypto::CryptoError;
use services::crypto::anoncreds::helpers::CopyFrom;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

pub struct ClaimRequest {
    pub user_id: String,
    pub u: BigNumber,
    //    ur: BigNumber
}

pub struct ClaimInitData {
    pub u: BigNumber,
    pub v_prime: BigNumber
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

#[derive(Debug)]
pub struct PrimaryClaim {
    pub attributes: Rc<Vec<Rc<Attribute>>>,
    pub encoded_attributes: HashMap<String, BigNumber>,
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
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
    pub ts: Option<String>,
    pub pubseq_no: Option<String>
}

pub struct Proof {
    pub primary_proof: Rc<PrimaryProof>
    //non_revocation_proof
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

pub struct PrimaryProof {
    pub eq_proof: Rc<PrimaryEqualProof>,
    pub ge_proofs: Vec<Rc<PrimaryPredicateGEProof>>
}

pub struct PrimaryInitProof {
    pub eq_proof: Rc<PrimaryEqualInitProof>,
    pub ge_proofs: Vec<Rc<PrimaryPrecicateGEInitProof>>
}

pub struct InitProof {
    pub primary_init_proof: Rc<PrimaryInitProof>,
    //nonRevocInitProof
}

pub struct Claims {
    pub primary_claim: Rc<PrimaryClaim>,
    //nonRevocClai,
}

pub struct ProofClaims {
    pub claims: Rc<Claims>,
    pub revealed_attrs: Vec<String>,
    pub predicates: Vec<Rc<Predicate>>
}


//impl block
impl PrimaryEqualInitProof {
    pub fn as_c_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![try!(self.a_prime.clone())])
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![try!(self.t.clone())])
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
        let mut c_list: Vec<BigNumber> = try!(self.eq_proof.as_c_list());
        for ge_proof in self.ge_proofs.iter() {
            try!(c_list.clone_from_vector(
                try!(ge_proof.as_c_list())
            ));
        }
        Ok(c_list)
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        let mut tau_list: Vec<BigNumber> = try!(self.eq_proof.as_tau_list());
        for ge_proof in self.ge_proofs.iter() {
            try!(tau_list.clone_from_vector(
                try!(ge_proof.as_tau_list())
            ));
        }
        Ok(tau_list)
    }
}