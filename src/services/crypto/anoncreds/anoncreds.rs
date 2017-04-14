//TODO: remove this code after all functions will be rewritten with Milagro library.
const LARGE_MASTER_SECRET: i32 = 256;
const LARGE_PRIME: i32 = 1024;
const LARGE_VPRIME: i32 = 2128;
const LARGE_E_START: i32 = 596;
const LARGE_E_END_RANGE: i32 = 119;
extern crate openssl;
extern crate int_traits;
extern crate milagro_crypto;
use self::int_traits::IntTraits;
use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
use services::crypto;

pub struct AnoncredsService {
    dummy: String
}

impl AnoncredsService {

    fn count_rounds_for_prime_check(prime: &BigNum) -> i32 {
        let prime_len = prime.to_dec_str().unwrap().len();
        prime_len.log2() as i32
    }
    fn generate_prime_in_range(start: &BigNum, end: &BigNum) -> Result<BigNum, &'static str>{
        let (mut iteration, max_iterations, mut prime_is_found, mut prime, mut ctx) = (
            0,
            100000,
            false,
            BigNum::new().unwrap(),
            BigNumContext::new().unwrap()
        );

        while (iteration < max_iterations) && !prime_is_found {
            prime = AnoncredsService::random_in_range(&start, &end);
            let checks = AnoncredsService::count_rounds_for_prime_check(&prime);

            if prime.is_prime(checks, &mut ctx).unwrap() {
                prime_is_found = true;
                println!("Found prime in {} iteration", iteration);
            }
            iteration += 1;
        }

        if !prime_is_found {
            println!("Cannot find prime in {} iterations", max_iterations);
            Err("Prime is not found")
        } else {
            Ok(prime)
        }
    }
    fn create_claim_request() -> ClaimRequest {
        let public_key = AnoncredsService::generate_public_key();
        let master_secret = AnoncredsService::generate_master_secret();
        let u = AnoncredsService::gen_u(public_key, master_secret);
        let claim_request = ClaimRequest {u: u};
        claim_request
    }
    fn generate_context(accumulator_id: &str, user_id: &str) {
        let accumulator_id_encoded = AnoncredsService::encode_attribute(accumulator_id, ByteOrder::Little);
        let user_id_encoded = AnoncredsService::encode_attribute(user_id, ByteOrder::Little);
        let a_e = BigNum::from_dec_str(&accumulator_id_encoded).unwrap();
        let u_e = BigNum::from_dec_str(&user_id_encoded).unwrap();
        let s = AnoncredsService::bitwise_or_big_int(&a_e, &u_e);
        let mut result = hash(MessageDigest::sha256(), s.to_hex_str().unwrap().as_bytes()).unwrap();
        let encoded_attribute = AnoncredsService::transform_byte_array_to_big_integer(&result);
        println!("attr{:?}", encoded_attribute);
    }
}

#[derive(Debug)]
struct PublicKey {
    n: BigNum,
    s: BigNum,
    rms: BigNum
}

#[derive(Debug)]
struct ClaimRequest {
    u: BigNum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_in_range_works() {
        let (mut start, mut end) = (BigNum::new().unwrap(), BigNum::new().unwrap());

        start.rand(LARGE_PRIME, MSB_MAYBE_ZERO, false).unwrap();
        end.rand(LARGE_PRIME, MSB_MAYBE_ZERO, false).unwrap();

        while end < start {
            end.rand(LARGE_PRIME, MSB_MAYBE_ZERO, false).unwrap();
        }

        let random_in_range = AnoncredsService::random_in_range(&start, &end);
        assert!((random_in_range > start) && (random_in_range < end));
    }
}