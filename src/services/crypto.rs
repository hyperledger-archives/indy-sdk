extern crate rand;
extern crate milagro_crypto;
use self::milagro_crypto::randapi::Random;
use self::milagro_crypto::big::wrappers::MODBYTES;
use self::milagro_crypto::ff::FF;
use self::rand::os::OsRng;
use self::rand::Rng;

const BIG_SIZE: usize = 32;

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
    println!("Iteration: {}\nFound prime: {}", iteration, prime);

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
        println!("Iteration: {}\nFound prime: {}\nis_prime: {}\n", iteration, prime, is_prime);
    }
    prime
}