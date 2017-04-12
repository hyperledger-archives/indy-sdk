//TODO: remove this code after all functions will be rewritten with Milagro library.
const LARGE_MASTER_SECRET: i32 = 256;
const LARGE_PRIME: i32 = 1024;
const LARGE_VPRIME: i32 = 2128;
const LARGE_E_START: i32 = 596;
const LARGE_E_END_RANGE: i32 = 119;
const LARGE_VPRIME_PRIME: i32 = 2724;
extern crate openssl;
extern crate int_traits;
extern crate milagro_crypto;
use self::int_traits::IntTraits;
use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
use self::openssl::hash::{hash, MessageDigest};
use std::cmp::max;
use services::crypto;

enum ByteOrder {
    Big,
    Little
}

pub struct AnoncredsService {
    dummy: String
}

impl AnoncredsService {
    fn generate_public_key() -> PublicKey {
        //let (p_prime, q_prime) = (AnoncredsService::generate_prime(), AnoncredsService::generate_prime());
        let p_prime = BigNum::from_dec_str("147210949676505370022291901638323344651935110597590993130806944871698104433042968489453214046274983960765508724336420649095413993801340223096499490318385863961435462627523137938481776395548210420546733337321351064531462114552738775282293300556323029911674068388889455348206728016707243243859948314986927502343").unwrap();
        let q_prime = BigNum::from_dec_str("135780746061008989066681842882411968289578365330121870655195830818464118363874946689390282395824911410416094765498522070170715656164604448597511036312331994824492100665472180363433381994083327828179950784236529457340933711810709515143629906739084420423785456874473704622664344722021987863690561674302204741259").unwrap();
        //println!("p: {}\nq: {}", p_prime, q_prime);

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
        //println!("p: {}\nq: {}", p, q);

        let mut n = BigNum::new().unwrap();
        n.checked_mul(&p, &q, &mut ctx);
        //println!("n: {}", n);

        s = AnoncredsService::random_qr(&n);
        //println!("s: {}", s);

        rms.mod_exp(&s, &AnoncredsService::gen_x(&p_prime, &q_prime), &n, &mut ctx);
        //println!("rms: {}", rms);

        PublicKey {n: n, s: s, rms: rms}
    }
    fn random_qr(n: &BigNum) -> BigNum {
        let (mut ctx, mut random_qr) = (BigNumContext::new().unwrap(), BigNum::new().unwrap());
        random_qr.sqr(&AnoncredsService::random_in_range(&BigNum::from_u32(0).unwrap(), &n), &mut ctx);
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

        //println!("Generated prime: {}", prime);
        prime
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
    fn issue_primary_claim(attributes: &Vec<AttributeType>, u: &BigNum, accumulator_id: &str, user_id: &str) {
        let mut ctx = BigNumContext::new().unwrap();
        let vprimeprime = AnoncredsService::generate_vprimeprime();
        let (mut e_start, mut e_end) = (BigNum::new().unwrap(), BigNum::new().unwrap());
        e_start.exp(&BigNum::from_u32(2).unwrap(), &BigNum::from_u32(LARGE_E_START as u32).unwrap(), &mut ctx);
        e_end.exp(&BigNum::from_u32(2).unwrap(), &BigNum::from_u32(LARGE_E_END_RANGE as u32).unwrap(), &mut ctx);
        e_end = &e_start + &e_end;
        let e = AnoncredsService::generate_prime_in_range(&e_start, &e_end).unwrap();
        let encoded_attributes = AnoncredsService::encode_attributes(attributes);
        let m2 = AnoncredsService::generate_context(accumulator_id, user_id);
    }
    fn sign(attributes: &Vec<AttributeType>, v: &BigNum, u: &BigNum, e: &BigNum) {

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
    fn generate_vprimeprime() -> BigNum {
        let mut ctx = BigNumContext::new().unwrap();
        let mut a = BigNum::new().unwrap();
        let mut b = BigNum::new().unwrap();
        a.rand(LARGE_VPRIME_PRIME, MSB_MAYBE_ZERO, false).unwrap();
        b.exp(&BigNum::from_u32(2).unwrap(), &BigNum::from_u32((LARGE_VPRIME_PRIME - 1) as u32).unwrap(), &mut ctx);
        AnoncredsService::bitwise_or_big_int(&a, &b)
    }
    fn transform_byte_array_to_big_integer(vec: &Vec<u8>) -> BigNum {
        let mut ctx = BigNumContext::new().unwrap();
        let mut result = BigNum::from_u32(0).unwrap();
        for i in (0..vec.len()).rev() {
            let mut pow256 = BigNum::new().unwrap();
            pow256.exp(&BigNum::from_u32(256).unwrap(), &BigNum::from_u32(i as u32).unwrap(), &mut ctx);
            pow256 = &pow256 * &BigNum::from_u32(vec[vec.len() - 1 - i] as u32).unwrap();
            result = &result + &pow256;
        }
        result
    }
    fn encode_attribute(attribute: &str, byte_order: ByteOrder) -> String {
        let mut result = hash(MessageDigest::sha256(), attribute.as_bytes()).unwrap();
        let index = result.iter().position(|&value| value == 0);
        if let Some(position) = index {
            result.truncate(position);
        }
        if let ByteOrder::Little = byte_order {
            result.reverse();
        }
        let encoded_attribute = AnoncredsService::transform_byte_array_to_big_integer(&result);
        encoded_attribute.to_dec_str().unwrap().to_string()
    }
    fn encode_attributes(attributes: &Vec<AttributeType>) -> Vec<AttributeType> {
        let mut encoded_attributes = Vec::new();
        for i in attributes {
            if i.encode {
                encoded_attributes.push(AttributeType {name: i.name.clone(), value: AnoncredsService::encode_attribute(&i.value, ByteOrder::Big), encode: i.encode.clone()});
            }
            else {
                encoded_attributes.push(AttributeType {name: i.name.clone(), value: i.value.clone(), encode: i.encode.clone()});
            }
        }
        encoded_attributes
    }
    fn bitwise_or_big_int(a: &BigNum, b: &BigNum) -> BigNum {
        let significant_bits = max(a.num_bits(), b.num_bits());
        let mut result = BigNum::new().unwrap();
        for i in 0..significant_bits {
            if a.is_bit_set(i) || b.is_bit_set(i) {
                result.set_bit(i);
            }
        }
        result
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

#[derive(Debug)]
struct AttributeType {
    name: String,
    value: String,
    encode: bool
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
//    #[test]
//    fn generate_prime_works() {
//        let prime = AnoncredsService::generate_prime();
//        let mut ctx = BigNumContext::new().unwrap();
//        let checks = AnoncredsService::count_rounds_for_prime_check(&prime);
//        let is_prime = prime.is_prime(checks, &mut ctx).unwrap();
//        assert_eq!(is_prime, true);
//    }
    #[test]
    fn encode_attribute_works() {
        let test_str_one = "Alexer5435";
        let test_str_two = "Alexer";
        let test_answer_one = "62794";
        let test_answer_two = "93838255634171043313693932530283701522875554780708470423762684802192372035729";
        assert_eq!(test_answer_one, AnoncredsService::encode_attribute(test_str_one, ByteOrder::Big));
        assert_eq!(test_answer_two, AnoncredsService::encode_attribute(test_str_two, ByteOrder::Big));
    }
    #[test]
    fn bitwise_or_big_int_works () {
        let a = BigNum::from_dec_str("778378032744961463933002553964902776831187587689736807008034459507677878432383414623740074").unwrap();
        let b = BigNum::from_dec_str("1018517988167243043134222844204689080525734196832968125318070224677190649881668353091698688").unwrap();
        let result = BigNum::from_dec_str("1796896020912204507067225398169591857356921784522704932326104684184868528314051767715438762").unwrap();
        assert_eq!(result, AnoncredsService::bitwise_or_big_int(&a, &b));
    }
    #[test]
    fn anoncreds_works() {
        let attributes = vec![
            AttributeType {name: "name".to_string(), value: "Alex".to_string(), encode: true},
            AttributeType {name: "age".to_string(), value: "28".to_string(), encode: false},
            AttributeType {name: "height".to_string(), value: "175".to_string(), encode: false},
            AttributeType {name: "sex".to_string(), value: "male".to_string(), encode: true}
        ];
        let (user_id, accumulator_id) = ("111", "110");
        let claim_request = AnoncredsService::create_claim_request();
        let claim = AnoncredsService::issue_primary_claim(&attributes, &claim_request.u, &accumulator_id, &user_id);
    }
//    #[test]
//    fn test_random() {
//        let prime = crypto::generate_prime(1024);
//        println!("prime is: {}", prime)
//    }
}