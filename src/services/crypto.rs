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

    let mut prime = generate_big_random(size);
    let mut prime_bytes = prime.to_bytes();
    let length = prime_bytes.len();
    let last_byte = prime_bytes[length - 1];
    prime_bytes[length - 1] = last_byte | 3;
    prime = FF::from_bytes(&prime_bytes, length, BIG_SIZE);
    while !FF::is_prime(&prime, &mut rng) {
        prime = FF::add(&prime, &FF::from_hex("4", BIG_SIZE));
    }

    prime
}