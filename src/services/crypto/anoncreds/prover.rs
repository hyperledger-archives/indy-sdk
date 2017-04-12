use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_MASTER_SECRET,
    LARGE_VPRIME
};
use services::crypto::anoncreds::types::{
    PublicKey,
    ClaimInitData
};
use services::crypto::wrappers::bn::BigNumber;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }
    pub fn generate_master_secret(&self) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        bn = try!(bn.rand(LARGE_MASTER_SECRET));
        Ok(bn)
    }

    fn gen_claim_init_data(public_key: &PublicKey, ms: &BigNumber) -> Result<ClaimInitData, CryptoError> {
        let bn = try!(BigNumber::new());
        let v_prime = try!(bn.rand(LARGE_VPRIME));

        let result_mul_one = try!(public_key.s.mod_exp(&v_prime, &public_key.n, None));

        let result_mul_two = try!(public_key.rms.mod_exp(&ms, &public_key.n, None));

        let mut u = try!(result_mul_one.mul(&result_mul_two, None));
        u = try!(u.modulus(&public_key.n, None));

        Ok(ClaimInitData {
            u: u,
            v_prime: v_prime
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {

    }
}