use services::crypto::anoncreds::constants::{LARGE_MASTER_SECRET};
use services::crypto::wrappers::bn::bn_impl::BigNumber;
use errors::crypto::CryptoError;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }
    pub fn generate_master_secret(&self) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        bn = try!(bn.rand(LARGE_MASTER_SECRET as i32));
        Ok(bn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {

    }
}