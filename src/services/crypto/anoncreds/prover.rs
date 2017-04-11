extern crate milagro_crypto;
use self::milagro_crypto::ff::FF;
use services::crypto::anoncreds::constants::{LARGE_MASTER_SECRET};
use services::crypto::helpers::{generate_big_random};

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }
    pub fn generate_master_secret(&self) -> FF {
        generate_big_random(LARGE_MASTER_SECRET)
    }
}