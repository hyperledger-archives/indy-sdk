use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_PRIME
};
use services::crypto::anoncreds::types::{
    PublicKey,
    SecretKey,
    Schema
};
use services::crypto::helpers::{
    random_qr,
    random_in_range
};
use services::crypto::wrappers::bn::BigNumber;

use std::collections::HashMap;

pub struct Issuer {

}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }
    pub fn generate_keys(&self, schema: &Schema) -> Result<((PublicKey, SecretKey)), CryptoError> {
        (Issuer::_generate_keys(&schema));
        unimplemented!();
    }

    fn _generate_keys(schema: &Schema) -> Result<(PublicKey, SecretKey), CryptoError> {
        let bn = try!(BigNumber::new());
        let p = try!(bn.safe_prime(LARGE_PRIME));
        let q = try!(bn.safe_prime(LARGE_PRIME));

        let mut p_prime = try!(p.sub(&try!(BigNumber::from_u32(1))));
        try!(p_prime.div_word(2));

        let mut q_prime = try!(q.sub(&try!(BigNumber::from_u32(1))));
        try!(q_prime.div_word(2));

        let n = try!(p.mul(&q, None));
        let s = try!(random_qr(&n));
        let xz = try!(Issuer::_gen_x(&p_prime, &q_prime));
        let mut r: HashMap<String, BigNumber> = HashMap::new();

        for attribute in &schema.attribute_names {
            let random = try!(Issuer::_gen_x(&p_prime, &q_prime));
            r.insert(attribute.to_string(), try!(s.mod_exp(&random, &n, None)));
        }

        let z = try!(s.mod_exp(&xz, &n, None));

        let rms = try!(s.mod_exp(&try!(Issuer::_gen_x(&p_prime, &q_prime)), &n, None));
        let rctxt = try!(s.mod_exp(&try!(Issuer::_gen_x(&p_prime, &q_prime)), &n, None));
        Ok((
            PublicKey {
                n: n,
                rms: rms,
                rctxt: rctxt,
                r: r,
                s: s,
                z: z
            },
            SecretKey {
                p: p_prime,
                q: q_prime
            }
        ))
    }

    fn _generate_revocation_keys() {

    }

    pub fn issuer_primary_claim(&self) {

    }

    fn _gen_x(p: &BigNumber, q: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut value = try!(p.mul(&q, None));
        try!(value.sub_word(3));

        let mut result = try!(value.rand_range());
        try!(result.add_word(2));
        Ok(result)
    }
}