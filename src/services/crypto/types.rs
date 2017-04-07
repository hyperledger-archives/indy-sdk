extern crate milagro_crypto;
use self::milagro_crypto::ff::FF;
use std::collections::HashMap;

pub enum ByteOrder {
    Big,
    Little
}

pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32
}

pub struct PrimaryEqualProof {
    pub revealed_attr_names: Vec<String>,
    pub a_prime: FF,
    pub e: FF,
    pub v: FF,
    pub m: HashMap<String, FF>,
    pub m1: FF,
    pub m2: FF
}

pub struct PrimaryPredicateGEProof {
    pub u: HashMap<String, FF>,
    pub r: HashMap<String, FF>,
    pub mj: FF,
    pub alpha: FF,
    pub t: HashMap<String, FF>,
    pub predicate: Predicate
}

pub struct PublicKey {
    pub n: FF,
    pub s: FF,
    pub rms: FF,
    pub r: HashMap<String, FF>,
    pub rctxt: FF,
    pub z: FF
}