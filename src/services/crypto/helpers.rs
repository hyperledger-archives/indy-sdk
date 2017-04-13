extern crate rand;
extern crate milagro_crypto;

use self::milagro_crypto::big::wrappers::MODBYTES;
use self::milagro_crypto::ff::FF;
use self::milagro_crypto::hash::wrappers::hash256;
use std::cmp::max;
//use services::crypto::anoncreds::constants::{
//    BIG_SIZE,
//    BN_MASK,
//    PRIMES,
//    NUM_PRIMES,
//    LARGE_PRIME
//};
use services::crypto::anoncreds::types::{ByteOrder};
use services::crypto::wrappers::bn::BigNumber;
use errors::crypto::CryptoError;

//pub fn generate_random_seed() -> [u8; 32] {
//    let mut seed: [u8; 32] = [0; 32];
//    let mut rng = OsRng::new().unwrap();
//    rng.fill_bytes(&mut seed);
//    seed
//}

//pub fn generate_big_random(size: usize) -> FF {
//    let seed = generate_random_seed();
//    let mut rng = Random::new(seed);
//
//    let b_size: usize = size / 8 + 1; // number of bytes for mod value
//    let big_size: usize = (b_size + MODBYTES - 1) / MODBYTES; // number of BIGs for mod value
//
//    // init mod bytes with 0 and set 1 in proper place
//    let mut bytes = vec![0; big_size * MODBYTES];
//    bytes[big_size * MODBYTES - b_size] = (1 as u8).wrapping_shl((size - (b_size - 1) * 8) as u32);
//
//    let bv = FF::from_bytes(&bytes[0..], big_size * MODBYTES, BIG_SIZE);
//    let r = FF::randomnum(&bv, &mut rng);
//    r
//}

//pub fn generate_prime(size: usize) -> FF {
//    let seed = generate_random_seed();
//    let mut rng = Random::new(seed);
//    let mut iteration = 0;
//    let mut is_prime = false;
//
//    let mut prime = generate_big_random(size);
//    let mut prime_bytes = prime.to_bytes();
//    let length = prime_bytes.len();
//    let last_byte = prime_bytes[length - 1];
//    prime_bytes[length - 1] = last_byte | 3;
//    prime = FF::from_bytes(&prime_bytes, length, BIG_SIZE);
//    while !is_prime {
//        prime.inc(4);
//        iteration += 1;
//        is_prime = FF::is_prime(&prime, &mut rng);
//    }
//    debug!("Iteration: {}\nFound prime: {}", iteration, prime);
//
//    prime
//}

//pub fn generate_prime_2p_plus_1(size: usize) -> FF {
//    let seed = generate_random_seed();
//    let mut rng = Random::new(seed);
//    let (mut is_prime, mut iteration) = (false, 0);
//    let mut prime = FF::new(BIG_SIZE);
//
//    while !is_prime {
//        iteration += 1;
//        prime = generate_prime(size);
//        let mut prime_for_check = FF::mul(&prime, &FF::from_hex("2", BIG_SIZE));
//        prime_for_check.inc(1);
//        is_prime = FF::is_prime(&prime_for_check, &mut rng);
//        debug!("Iteration: {}\nFound prime: {}\nis_prime: {}\n", iteration, prime, is_prime);
//    }
//    prime
//}

pub fn random_qr(n: &BigNumber) -> Result<BigNumber, CryptoError> {
    let mut random = try!(n.rand_range());
    random = try!(random.sqr(None));
    random = try!(random.modulus(&n, None));
    Ok(random)
}

fn bitwise_or_big_int(a: &BigNumber, b: &BigNumber) -> Result<BigNumber, CryptoError> {
    let significant_bits = max(try!(a.num_bits()), try!(b.num_bits()));
    let mut result = try!(BigNumber::new());
    for i in 0..significant_bits {
        if try!(a.is_bit_set(i)) || try!(b.is_bit_set(i)) {
            try!(result.set_bit(i));
        }
    }
    Ok(result)
}

//pub fn random_in_range(start: &FF, end: &FF) -> FF {
//    let sub = end - start;
//    let size = significant_bits(&sub);
//    let mut random_number = generate_big_random(size);
//
//    while (&random_number + start) > *end {
//        random_number = generate_big_random(size);
//    }
//
//    random_number = &random_number + start;
//    debug!("start: {}\nend: {}\nsub: {}\nrandom: {}", start, end, sub, random_number);
//    random_number
//}

pub fn encode_attribute(attribute: &str, byte_order: ByteOrder) -> FF {
    let array_bytes = attribute.as_bytes();
    let mut sha256: hash256 = hash256::new();

    for byte in array_bytes[..].iter() {
        sha256.process(*byte);
    }

    let mut hashed_array: Vec<u8> =
        sha256.hash().iter()
            .map(|v| *v as u8)
            .collect();

    let index = hashed_array.iter().position(|&value| value == 0);
    if let Some(position) = index {
        hashed_array.truncate(position);
    }

    if let ByteOrder::Little = byte_order {
        hashed_array.reverse();
    }

    if hashed_array.len() < 32 {
        for i in 0..(32 - hashed_array.len()) {
            hashed_array.insert(0, 0);
        }
    }
    FF::from_bytes(&hashed_array, MODBYTES, 32)
}

fn significant_bytes(n: &FF) -> Vec<u8> {
    let mut bytes = n.to_bytes();
    let length = bytes.len();
    let index = bytes.iter().position(|&value| value != 0);
    if let Some(index) = index {
        bytes.reverse();
        bytes.truncate(length - index);
        bytes.reverse();
    }
    bytes
}

fn significant_bits(n: &FF) -> usize {
    let bytes = significant_bytes(n);
    let mut result = (bytes.len() - 1) * 8;
    result += format!("{:b}", bytes[0]).len();
    result
}

//fn generate_probable_prime(size: usize) {
//    let mut random_number = generate_big_random(size);
//    let mut mods: Vec<FF> = Vec::new();
//    for i in 1..NUM_PRIMES {
//        debug!("{}", i);
//        let bytes = random_number.to_bytes();
//        let mut new_random = FF::from_bytes(&bytes, size, BIG_SIZE);
//        let prime = FF::from_hex(&format!("{:x}", PRIMES[i])[..], BIG_SIZE);
//        FF::modulus(&mut new_random, &prime);
//        mods.push(new_random);
//    }
//    //TODO loop for mods check
//}

pub fn get_hash_as_int(nums: &mut Vec<BigNumber>) -> Result<BigNumber, CryptoError> {
    let mut sha256: hash256 = hash256::new();

    nums.sort();

    for num in nums.iter() {
        let array_bytes: Vec<u8> = try!(num.to_bytes());

        let index = array_bytes.iter().position(|&value| value != 0).unwrap_or(array_bytes.len());

        for byte in array_bytes[index..].iter() {
            sha256.process(*byte);
        }
    }

    let mut hashed_array: Vec<u8> =
        sha256.hash().iter()
            .map(|v| *v as u8)
            .collect();

    hashed_array.reverse();

    BigNumber::from_bytes(&hashed_array[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_or_big_int_works () {
        let a = BigNumber::from_dec("778378032744961463933002553964902776831187587689736807008034459507677878432383414623740074");
        let b = BigNumber::from_dec("1018517988167243043134222844204689080525734196832968125318070224677190649881668353091698688");
        let result = BigNumber::from_dec("1796896020912204507067225398169591857356921784522704932326104684184868528314051767715438762");
        assert_eq!(result.unwrap(), bitwise_or_big_int(&a.unwrap(), &b.unwrap()).unwrap());
    }

//    #[test]
//    fn random_in_range_works() {
//        ::env_logger::init().unwrap();
//
//        let start = generate_big_random(100);
//        let mut end = generate_big_random(120);
//
//        while end < start {
//            end = generate_big_random(30);
//        }
//
//        let random = random_in_range(&start, &end);
//        assert!((random > start) && (random < end));
//    }

//    #[test]
//    fn encode_attribute_works() {
//        let test_str_one = "Alexer5435";
//        let test_str_two = "Alexer";
//        let test_answer_one = "f54a";
//        let test_answer_two = "cf76920dae32802476cc5e8d2518fd21c16b5f83e713a684db1aeb7089c67091";
//        assert_eq!(FF::from_hex(test_answer_one, BIG_SIZE), encode_attribute(test_str_one, ByteOrder::Big));
//        assert_eq!(FF::from_hex(test_answer_two, BIG_SIZE), encode_attribute(test_str_two, ByteOrder::Big));
//    }

    #[test]
    fn get_hash_as_in_works() {
        let mut nums = vec![
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa94d3fdf6abfbff").unwrap(),
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa9168615ccbc546").unwrap()
        ];
        let res = get_hash_as_int(&mut nums);

        assert!(res.is_ok());
        assert_eq!("9E2A0653691B96A9B55B3D1133F9FEE2F2C37B848DBADF2F70DFFFE9E47C5A5D", res.unwrap().to_hex().unwrap());
    }
}