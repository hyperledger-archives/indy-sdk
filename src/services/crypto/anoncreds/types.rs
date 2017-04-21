use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::{GroupOrderElement, PointG1, Pair};
use errors::crypto::CryptoError;
use services::crypto::anoncreds::helpers::AppendBigNumArray;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub attribute_names: HashSet<String>
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

pub struct RevocationSecretKey {
    pub x: GroupOrderElement,
    pub sk: GroupOrderElement
}

#[derive(Debug)]
pub struct SecretKey {
    pub p: BigNumber,
    pub q: BigNumber
}

pub struct ClaimRequest {
    pub user_id: String,
    pub u: BigNumber,
    pub ur: Option<PointG1>
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
    pub non_revocation_claim: Option<RefCell<NonRevocationClaim>>
}

#[derive(Debug)]
pub struct PrimaryClaim {
    pub encoded_attributes: HashMap<String, BigNumber>,
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
    pub v_prime: BigNumber
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
    pub c_list: Vec<Vec<u8>>
}

pub struct Proof {
    pub primary_proof: PrimaryProof,
    pub non_revoc_proof: Option<NonRevocProof>
}

pub struct InitProof {
    pub primary_init_proof: PrimaryInitProof,
    pub non_revoc_init_proof: Option<NonRevocInitProof>
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

pub struct RevocationClaimInitData {
    pub u: PointG1,
    pub v_prime: GroupOrderElement
}

pub struct Witness {
    pub sigmai: PointG1,
    pub ui: PointG1,
    pub gi: PointG1,
    pub omega: PointG1,
    pub v: HashSet<i32>
}

pub struct NonRevocationClaim {
    pub ia: String,
    pub sigma: PointG1,
    pub c: GroupOrderElement,
    pub v: GroupOrderElement,
    pub gi: PointG1,
    pub witness: Witness,
    pub i: i32,
    pub m2: BigNumber
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

#[derive(Clone)]
pub struct Accumulator {
    pub l: i32,
    pub v: HashSet<i32>,
    pub acc: PointG1,
    pub current_i: i32,
    pub ia: String
}

pub struct NonRevocInitProof {
    pub c_list_params: NonRevocProofXList,
    pub tau_list_params: NonRevocProofXList,
    pub c_list: NonRevocProofCList,
    pub tau_list: NonRevocProofTauList
}

pub struct NonRevocProof {
    pub x_list: NonRevocProofXList,
    pub c_list: NonRevocProofCList
}

pub struct AccumulatorPublicKey {
    pub z: PointG1,
    pub seq_id: i32
}


//impl block
impl PrimaryClaim {
    pub fn update_vprime(&mut self, v_prime: &BigNumber) -> Result<(), CryptoError> {
        self.v_prime = self.v_prime.add(v_prime)?;
        Ok(())
    }
}

impl NonRevocationClaim {
    pub fn update_v(&mut self, v_prime: &GroupOrderElement) -> Result<(), CryptoError> {
        self.v = self.v.add_mod(v_prime)?;
        Ok(())
    }
}

impl Claims {
    pub fn init_primary_claim(&mut self, v_prime: &BigNumber) -> Result<(), CryptoError> {
        self.primary_claim.update_vprime(v_prime)?;
        Ok(())
    }

    pub fn init_non_revocation_claim(&mut self, v_prime: &GroupOrderElement) -> Result<(), CryptoError> {
        if let Some(ref mut non_revocation_claim) = self.non_revocation_claim {
            non_revocation_claim.borrow_mut().update_v(v_prime)?;
        }
        Ok(())
    }
}

impl PrimaryEqualInitProof {
    pub fn as_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.a_prime.clone()?])
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.t.clone()?])
    }
}

impl PrimaryPrecicateGEInitProof {
    pub fn as_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.c_list)
    }

    pub fn as_tau_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.tau_list)
    }
}

impl PrimaryInitProof {
    pub fn as_c_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        let mut c_list: Vec<BigNumber> = self.eq_proof.as_list()?;
        for ge_proof in self.ge_proofs.iter() {
            c_list.append_vec(ge_proof.as_list()?)?;
        }
        Ok(c_list)
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        let mut tau_list: Vec<BigNumber> = self.eq_proof.as_tau_list()?;
        for ge_proof in self.ge_proofs.iter() {
            tau_list.append_vec(ge_proof.as_tau_list()?)?;
        }
        Ok(tau_list)
    }
}

impl NonRevocProofCList {
    pub fn as_list(&self) -> Result<Vec<PointG1>, CryptoError> {
        Ok(vec![self.e, self.d, self.a, self.g, self.w, self.s, self.u])
    }
}

impl NonRevocProofTauList {
    pub fn as_list(&self) -> Result<Vec<Pair>, CryptoError> {
        Ok(vec![self.t1, self.t2, self.t3, self.t4, self.t5, self.t6, self.t7, self.t8])
    }
}

impl NonRevocInitProof {
    pub fn as_c_list(&self) -> Result<Vec<PointG1>, CryptoError> {
        let vec = self.c_list.as_list()?;
        Ok(vec)
    }

    pub fn as_tau_list(&self) -> Result<Vec<Pair>, CryptoError> {
        let vec = self.tau_list.as_list()?;
        Ok(vec)
    }
}

impl NonRevocProofXList {
    pub fn as_list(&self) -> Result<Vec<GroupOrderElement>, CryptoError> {
        Ok(vec![self.rho, self.o, self.c, self.o_prime, self.m, self.m_prime, self.t, self.t_prime,
                self.m2, self.s, self.r, self.r_prime, self.r_prime_prime, self.r_prime_prime_prime])
    }

    pub fn from_list(seq: Vec<GroupOrderElement>) -> NonRevocProofXList {
        NonRevocProofXList {
            rho: seq[0],
            o: seq[1],
            c: seq[2],
            o_prime: seq[3],
            m: seq[4],
            m_prime: seq[5],
            t: seq[6],
            t_prime: seq[7],
            m2: seq[8],
            s: seq[9],
            r: seq[10],
            r_prime: seq[11],
            r_prime_prime: seq[12],
            r_prime_prime_prime: seq[13]
        }
    }
}