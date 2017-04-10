extern crate milagro_crypto;
use self::milagro_crypto::ff::FF;
use services::crypto::constants::{LARGE_MASTER_SECRET};
use services::crypto::helpers::{generate_big_random};

pub fn generate_master_secret() -> FF {
    generate_big_random(LARGE_MASTER_SECRET)
}