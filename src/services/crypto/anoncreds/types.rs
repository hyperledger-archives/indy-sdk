use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::{GroupOrderElement, PointG1, Pair};
use errors::crypto::CryptoError;
use services::crypto::anoncreds::helpers::AppendBigNumArray;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use utils::json::{JsonEncodable, JsonDecodable};

pub enum ByteOrder {
    Big,
    Little
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ClaimDefinition {
    pub public_key: PublicKey,
    pub public_key_revocation: RevocationPublicKey,
    pub schema_seq_no: i32,
    pub signature_type: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ClaimDefinitionPrivate {
    pub secret_key: SecretKey,
    pub secret_key_revocation: RevocationSecretKey
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RevocationRegistry {
    pub claim_def_seq_no: i32,
    pub acc: Accumulator,
    pub acc_pk: AccumulatorPublicKey,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RevocationRegistryPrivate {
    pub acc_sk: AccumulatorSecretKey,
    pub tails: HashMap<i32, PointG1>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaKey {
    pub name: String,
    pub version: String,
    pub issue_id: String
}

impl SchemaKey {
    pub fn new(name: String, version: String, issuer_id: String) -> SchemaKey {
        SchemaKey {
            name: name,
            version: version,
            issue_id: issuer_id
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub attribute_names: HashSet<String>,
    pub seq_no: i32
}

impl Schema {
    pub fn new(name: String, version: String, attributes_names: HashSet<String>, seq_no: i32) -> Schema {
        Schema {
            name: name,
            version: version,
            attribute_names: attributes_names,
            seq_no: seq_no
        }
    }
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

impl RevocationPublicKey {
    pub fn new(g: PointG1, h: PointG1, h0: PointG1, h1: PointG1, h2: PointG1, htilde: PointG1,
               u: PointG1, pk: PointG1, y: PointG1, x: GroupOrderElement) -> RevocationPublicKey {
        RevocationPublicKey {
            g: g,
            h: h,
            h0: h0,
            h1: h1,
            h2: h2,
            htilde: htilde,
            u: u,
            pk: pk,
            y: y,
            x: x
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Accumulator {
    pub acc: PointG1,
    pub v: HashSet<i32>,
    pub max_claim_num: i32,
    pub current_i: i32
}

impl Accumulator {
    pub fn new(acc: PointG1, v: HashSet<i32>, max_claim_num: i32,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Witness {
    pub sigma_i: PointG1,
    pub u_i: PointG1,
    pub g_i: PointG1,
    pub omega: PointG1,
    pub v: HashSet<i32>
}

impl Witness {
    pub fn new(sigma_i: PointG1, u_i: PointG1, g_i: PointG1, omega: PointG1,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub user_id: i32,
    pub u: BigNumber,
    pub ur: Option<PointG1>
}

impl ClaimRequest {
    pub fn new(user_id: i32, u: BigNumber, ur: Option<PointG1>) -> ClaimRequest {
        ClaimRequest {
            user_id: user_id,
            u: u,
            ur: ur
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32
}

impl Predicate {
    pub fn new(attr_name: String, p_type: String, value: i32) -> Predicate {
        Predicate {
            attr_name: attr_name,
            p_type: p_type,
            value: value
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub encode: bool
}

impl Attribute {
    pub fn new(name: String, value: String, encode: bool) -> Attribute {
        Attribute {
            name: name,
            value: value,
            encode: encode
        }
    }
}

pub struct ClaimInitData {
    pub u: BigNumber,
    pub v_prime: BigNumber
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub primary_claim: PrimaryClaim,
    pub non_revocation_claim: Option<RefCell<NonRevocationClaim>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryClaim {
    pub encoded_attributes: HashMap<String, BigNumber>,
    pub m2: BigNumber,
    pub a: BigNumber,
    pub e: BigNumber,
    pub v_prime: BigNumber
}

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn as_list(&self) -> Result<Vec<GroupOrderElement>, CryptoError> {
        Ok(vec![self.rho, self.o, self.c, self.o_prime, self.m, self.m_prime, self.t, self.t_prime,
                self.m2, self.s, self.r, self.r_prime, self.r_prime_prime, self.r_prime_prime_prime])
    }

    pub fn from_list(seq: Vec<GroupOrderElement>) -> NonRevocProofXList {
        NonRevocProofXList::new(seq[0], seq[1], seq[2], seq[3], seq[4], seq[5], seq[6], seq[7],
                                seq[8], seq[9], seq[10], seq[11], seq[12], seq[13])
    }
}

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
    pub fn as_slice(&self) -> Result<Vec<Vec<u8>>, CryptoError> {
        Ok(vec![self.t1.to_bytes()?, self.t2.to_bytes()?, self.t3.to_bytes()?, self.t4.to_bytes()?,
                self.t5.to_bytes()?, self.t6.to_bytes()?, self.t7.to_bytes()?, self.t8.to_bytes()?])
    }
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

impl NonRevocProofCList {
    pub fn as_list(&self) -> Result<Vec<PointG1>, CryptoError> {
        Ok(vec![self.e, self.d, self.a, self.g, self.w, self.s, self.u])
    }
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

impl PrimaryEqualInitProof {
    pub fn as_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.a_prime.clone()?])
    }

    pub fn as_tau_list(&self) -> Result<Vec<BigNumber>, CryptoError> {
        Ok(vec![self.t.clone()?])
    }
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

impl PrimaryPrecicateGEInitProof {
    pub fn as_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.c_list)
    }

    pub fn as_tau_list(&self) -> Result<&Vec<BigNumber>, CryptoError> {
        Ok(&self.tau_list)
    }
}

pub struct RevocationClaimInitData {
    pub u: PointG1,
    pub v_prime: GroupOrderElement
}

pub struct NonRevocInitProof {
    pub c_list_params: NonRevocProofXList,
    pub tau_list_params: NonRevocProofXList,
    pub c_list: NonRevocProofCList,
    pub tau_list: NonRevocProofTauList
}

impl NonRevocInitProof {
    pub fn as_c_list(&self) -> Result<Vec<PointG1>, CryptoError> {
        let vec = self.c_list.as_list()?;
        Ok(vec)
    }

    pub fn as_tau_list(&self) -> Result<Vec<Vec<u8>>, CryptoError> {
        let vec = self.tau_list.as_slice()?;
        Ok(vec)
    }
}

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

impl ClaimDefinition {
    pub fn new(public_key: PublicKey, public_key_revocation: RevocationPublicKey,
               schema_seq_no: i32, signature_type: String) -> ClaimDefinition {
        ClaimDefinition {
            public_key: public_key,
            public_key_revocation: public_key_revocation,
            schema_seq_no: schema_seq_no,
            signature_type: signature_type
        }
    }
}

impl ClaimDefinitionPrivate {
    pub fn new(secret_key: SecretKey, secret_key_revocation: RevocationSecretKey) -> ClaimDefinitionPrivate {
        ClaimDefinitionPrivate {
            secret_key: secret_key,
            secret_key_revocation: secret_key_revocation
        }
    }
}

impl RevocationRegistry {
    pub fn new(acc: Accumulator, acc_pk: AccumulatorPublicKey, claim_def_seq_no: i32) -> RevocationRegistry {
        RevocationRegistry {
            acc: acc,
            acc_pk: acc_pk,
            claim_def_seq_no: claim_def_seq_no
        }
    }
}

impl RevocationRegistryPrivate {
    pub fn new(acc_sk: AccumulatorSecretKey, tails: HashMap<i32, PointG1>) -> RevocationRegistryPrivate {
        RevocationRegistryPrivate {
            acc_sk: acc_sk,
            tails: tails
        }
    }
}

impl JsonEncodable for ClaimDefinition {}

impl<'a> JsonDecodable<'a> for ClaimDefinition {}

impl JsonEncodable for ClaimDefinitionPrivate {}

impl<'a> JsonDecodable<'a> for ClaimDefinitionPrivate {}


impl JsonEncodable for RevocationRegistry {}

impl<'a> JsonDecodable<'a> for RevocationRegistry {}

impl JsonEncodable for RevocationRegistryPrivate {}

impl<'a> JsonDecodable<'a> for RevocationRegistryPrivate {}

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

impl JsonEncodable for Accumulator {}

impl<'a> JsonDecodable<'a> for Accumulator {}

impl JsonEncodable for AccumulatorPublicKey {}

impl<'a> JsonDecodable<'a> for AccumulatorPublicKey {}

impl JsonEncodable for AccumulatorSecretKey {}

impl<'a> JsonDecodable<'a> for AccumulatorSecretKey {}

impl JsonEncodable for ClaimRequest {}

impl<'a> JsonDecodable<'a> for ClaimRequest {}
