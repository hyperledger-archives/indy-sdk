extern crate rand;
extern crate milagro_crypto;
use self::milagro_crypto::randapi::Random;
use self::milagro_crypto::big::wrappers::MODBYTES;
use self::milagro_crypto::ff::FF;
use self::milagro_crypto::hash::wrappers::hash256;
use self::rand::os::OsRng;
use self::rand::Rng;
use services::crypto::constants::{
    BIG_SIZE,
    BN_MASK,
    PRIMES,
    NUM_PRIMES,
    LARGE_PRIME
};
use services::crypto::types::{ByteOrder};

pub fn generate_random_seed() -> [u8; 32] {
    let mut seed: [u8; 32] = [0; 32];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut seed);
    seed
}

pub fn generate_big_random(size: usize) -> FF {
    let seed = generate_random_seed();
    let mut rng = Random::new(seed);

    let b_size: usize = size / 8 + 1; // number of bytes for mod value
    let big_size: usize = (b_size + MODBYTES - 1) / MODBYTES; // number of BIGs for mod value

    // init mod bytes with 0 and set 1 in proper place
    let mut bytes = vec![0; big_size * MODBYTES];
    bytes[big_size * MODBYTES - b_size] = (1 as u8).wrapping_shl((size - (b_size - 1) * 8) as u32);

    let bv = FF::from_bytes(&bytes[0..], big_size * MODBYTES, BIG_SIZE);
    let r = FF::randomnum(&bv, &mut rng);
    r
}

pub fn generate_prime(size: usize) -> FF {
    let seed = generate_random_seed();
    let mut rng = Random::new(seed);
    let mut iteration = 0;
    let mut is_prime = false;

    let mut prime = generate_big_random(size);
    let mut prime_bytes = prime.to_bytes();
    let length = prime_bytes.len();
    let last_byte = prime_bytes[length - 1];
    prime_bytes[length - 1] = last_byte | 3;
    prime = FF::from_bytes(&prime_bytes, length, BIG_SIZE);
    while !is_prime {
        prime.inc(4);
        iteration += 1;
        is_prime = FF::is_prime(&prime, &mut rng);
    }
    debug!("Iteration: {}\nFound prime: {}", iteration, prime);

    prime
}

pub fn generate_prime_2p_plus_1(size: usize) -> FF {
    let seed = generate_random_seed();
    let mut rng = Random::new(seed);
    let (mut is_prime, mut iteration) = (false, 0);
    let mut prime = FF::new(BIG_SIZE);

    while !is_prime {
        iteration += 1;
        prime = generate_prime(size);
        let mut prime_for_check = FF::mul(&prime, &FF::from_hex("2", BIG_SIZE));
        prime_for_check.inc(1);
        is_prime = FF::is_prime(&prime_for_check, &mut rng);
        debug!("Iteration: {}\nFound prime: {}\nis_prime: {}\n", iteration, prime, is_prime);
    }
    prime
}

pub fn random_qr(n: &FF){
    println!("n :{}", n);
    let mut random = random_in_range(&FF::from_hex("0", BIG_SIZE), n);
    println!("random :{}", random);
    random = FF::sqr(&random);
    println!("random sqr :{}", random);
//    let mut nn = n.clone();
//    nn.set_size(32);
//    random.set_size(32);
    let random1 = FF::modulus(&random, &n);
    println!("random1 :{}", random1);
//    let (mut ctx, mut random_qr) = (BigNumContext::new().unwrap(), BigNum::new().unwrap());
//    random_qr.sqr(&AnoncredsService::random_in_range(&BigNum::from_u32(0).unwrap(), &n), &mut ctx);
//    random_qr
}

pub fn random_in_range(start: &FF, end: &FF) -> FF {
    let sub = end - start;
    let size = significant_bits(&sub);
    let mut random_number = generate_big_random(size);

    while (&random_number + start) > *end {
        random_number = generate_big_random(size);
    }

    random_number = &random_number + start;
    debug!("start: {}\nend: {}\nsub: {}\nrandom: {}", start, end, sub, random_number);
    random_number
}

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

fn generate_probable_prime(size: usize) {
    let mut random_number = generate_big_random(size);
    let mut mods: Vec<FF> = Vec::new();
    for i in 1..NUM_PRIMES {
        debug!("{}", i);
        let bytes = random_number.to_bytes();
        let mut new_random = FF::from_bytes(&bytes, size, BIG_SIZE);
        let prime = FF::from_hex(&format!("{:x}", PRIMES[i])[..], BIG_SIZE);
        FF::modulus(&mut new_random, &prime);
        mods.push(new_random);
    }
    //TODO loop for mods check
}

pub fn get_hash_as_int(nums: &mut Vec<FF>) -> FF {
    let mut sha256: hash256 = hash256::new();

    nums.sort();

    for num in nums.iter() {
        let array_bytes: Vec<u8> = num.to_bytes();

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

    FF::from_bytes(&hashed_array[..], hashed_array.len(), 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_in_range_works() {
        ::env_logger::init().unwrap();

        let start = generate_big_random(100);
        let mut end = generate_big_random(120);

        while end < start {
            end = generate_big_random(30);
        }

        let random = random_in_range(&start, &end);
        assert!((random > start) && (random < end));
    }

    #[test]
    fn test_random_qr() {
//        let n = generate_big_random(10);
//        random_qr(&n);
    }

    #[test]
    fn encode_attribute_works() {
        let test_str_one = "Alexer5435";
        let test_str_two = "Alexer";
        let test_answer_one = "f54a";
        let test_answer_two = "cf76920dae32802476cc5e8d2518fd21c16b5f83e713a684db1aeb7089c67091";
        assert_eq!(FF::from_hex(test_answer_one, BIG_SIZE), encode_attribute(test_str_one, ByteOrder::Big));
        assert_eq!(FF::from_hex(test_answer_two, BIG_SIZE), encode_attribute(test_str_two, ByteOrder::Big));
    }

    #[test]
    fn get_hash_as_in_works(){
        let mut nums = vec![
            FF::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa94d3fdf6abfbff", 32),
            FF::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa9168615ccbc546", 32)
        ];
        let res =  get_hash_as_int(&mut nums);
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000000 9E2A0653691B96A9B55B3D1133F9FEE2F2C37B848DBADF2F70DFFFE9E47C5A5D", res.to_hex());
    }
}