const LARGE_MASTER_SECRET: i32 = 256;
const LARGE_PRIME: i32 = 1024;
const LARGE_VPRIME: i32 = 2128;

extern crate openssl;
extern crate int_traits;
use self::int_traits::IntTraits;
use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
pub struct AnoncredsService {
    dummy: String
}



impl AnoncredsService {
    pub fn new() -> AnoncredsService {
        trace!(target: "AnoncredsService", "new");
        AnoncredsService { dummy: "anoncreds_dummy".to_string() }
    }
    fn generate_master_secret() -> BigNum {
        let mut ms = BigNum::new().unwrap();
        ms.rand(LARGE_MASTER_SECRET, MSB_MAYBE_ZERO, false);
        ms
    }
    fn generate_public_key() -> PublicKey {
        //let (p_prime, q_prime) = (AnoncredsService::generate_prime(), AnoncredsService::generate_prime());
        let p_prime = BigNum::from_dec_str("147210949676505370022291901638323344651935110597590993130806944871698104433042968489453214046274983960765508724336420649095413993801340223096499490318385863961435462627523137938481776395548210420546733337321351064531462114552738775282293300556323029911674068388889455348206728016707243243859948314986927502343").unwrap();
        let q_prime = BigNum::from_dec_str("135780746061008989066681842882411968289578365330121870655195830818464118363874946689390282395824911410416094765498522070170715656164604448597511036312331994824492100665472180363433381994083327828179950784236529457340933711810709515143629906739084420423785456874473704622664344722021987863690561674302204741259").unwrap();
        println!("p: {}\nq: {}", p_prime, q_prime);

        let (mut p, mut q, mut ctx, mut n, mut s, mut rms) = (
            BigNum::new().unwrap(),
            BigNum::new().unwrap(),
            BigNumContext::new().unwrap(),
            BigNum::new().unwrap(),
            BigNum::new().unwrap(),
            BigNum::new().unwrap()
        );

        p.checked_mul(&p_prime, &BigNum::from_u32(2).unwrap(), &mut ctx);
        p.add_word(1);
        q.checked_mul(&q_prime, &BigNum::from_u32(2).unwrap(), &mut ctx);
        q.add_word(1);
        println!("p: {}\nq: {}", p, q);

        let mut n = BigNum::new().unwrap();
        n.checked_mul(&p, &q, &mut ctx);
        println!("n: {}", n);

        s = AnoncredsService::random_qr(&n);
        println!("s: {}", s);

        rms.mod_exp(&s, &AnoncredsService::gen_x(&p_prime, &q_prime), &n, &mut ctx);
        println!("rms: {}", rms);

        PublicKey {n: n, s: s, rms: rms}
    }
    fn gen_x(p: &BigNum, q: &BigNum) -> BigNum {
        let mut ctx = BigNumContext::new().unwrap();
        let mut value = BigNum::new().unwrap();
        let mut result = BigNum::new().unwrap();

        value.checked_mul(&p, &q, &mut ctx);
        value.sub_word(3);

        result = AnoncredsService::random_in_range(&value);
        result.add_word(2);
        result
    }
    fn gen_u(public_key: PublicKey, ms: BigNum) -> BigNum {
        let mut ctx = BigNumContext::new().unwrap();
        let mut vprime = BigNum::new().unwrap();
        vprime.rand(LARGE_VPRIME, MSB_MAYBE_ZERO, false);

        let mut result_mul_one = BigNum::new().unwrap();
        result_mul_one.mod_exp(&public_key.s, &vprime, &public_key.n, &mut ctx);

        let mut result_mul_two = BigNum::new().unwrap();
        result_mul_two.mod_exp(&public_key.rms, &ms, &public_key.n, &mut ctx);

        let mut u = &result_mul_one * &result_mul_two;
        u = &u % &public_key.n;
        u
    }
    fn random_in_range(n: &BigNum) -> BigNum {
        let mut random_number = BigNum::new().unwrap();
        random_number.rand(n.num_bits(), MSB_MAYBE_ZERO, false).unwrap();

        while random_number > *n {
            random_number.rand(n.num_bits(), MSB_MAYBE_ZERO, false).unwrap();
        }

        random_number
    }
    fn random_qr(n: &BigNum) -> BigNum {
        let (mut ctx, mut random_qr) = (BigNumContext::new().unwrap(), BigNum::new().unwrap());
        random_qr.sqr(&AnoncredsService::random_in_range(&n), &mut ctx);
        random_qr
    }
    fn count_rounds_for_prime_check(prime: &BigNum) -> i32 {
        let prime_len = prime.to_dec_str().unwrap().len();
        prime_len.log2() as i32
    }
    fn generate_prime() -> BigNum {
        let mut ctx = BigNumContext::new().unwrap();
        let mut prime = BigNum::new().unwrap();
        let (mut is_prime, mut iteration) = (false, 0);

        while !is_prime {
            iteration += 1;
            prime.generate_prime(LARGE_PRIME, false, None, None);
            let checks = AnoncredsService::count_rounds_for_prime_check(&prime);
            let mut prime_for_check = BigNum::new().unwrap();
            prime_for_check.checked_mul(&prime, &BigNum::from_u32(2).unwrap(), &mut ctx).unwrap();
            prime_for_check.add_word(1);
            is_prime = prime_for_check.is_prime(checks, &mut ctx).unwrap();
            println!("Iteration: {}\nFound prime: {}\nis_prime: {}\n", iteration, prime, is_prime);
        }

        println!("Generated prime: {}", prime);
        prime
    }
    fn create_claim_request() -> ClaimRequest {
        let public_key = AnoncredsService::generate_public_key();
        let master_secret = AnoncredsService::generate_master_secret();
        let u = AnoncredsService::gen_u(public_key, master_secret);
        let claim_request = ClaimRequest {u: u};
        claim_request
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
    fn service_creation_is_possible() {
        let anoncreds_service = AnoncredsService::new();
        assert_eq!("anoncreds_dummy", anoncreds_service.dummy, "Dummy field is filled by constructor");
    }
    #[test]
    fn master_secret_generator_works() {
        let ms: BigNum = AnoncredsService::generate_master_secret();
        assert_eq!(LARGE_MASTER_SECRET/8, ms.num_bytes());
    }
//    #[test]
//    fn generate_prime_works() {
//        let prime = AnoncredsService::generate_prime();
//        let mut ctx = BigNumContext::new().unwrap();
//        let checks = AnoncredsService::count_rounds_for_prime_check(&prime);
//        let is_prime = prime.is_prime(checks, &mut ctx).unwrap();
//        assert_eq!(is_prime, true);
//    }
    #[test]
    fn create_claim_request_works() {
        let claim_request = AnoncredsService::create_claim_request();
        println!("{:?}", claim_request)
    }
}