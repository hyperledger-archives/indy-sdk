extern crate milagro_crypto;
use self::milagro_crypto::ff::FF;
use services::crypto::helpers::{random_qr};
use services::crypto::constants::{BIG_SIZE};

pub struct Issuer {

}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }
    pub fn issuer_generate_keys(&self) {
        //    let p_prime = generate_prime_2p_plus_1(LARGE_PRIME);
        //    let q_prime = generate_prime_2p_plus_1(LARGE_PRIME);
        let p_prime = FF::from_hex("d1a2a65b9b574dd3e8416aa93f6d570adc2b5fc26925f78216225de6c882ebf431c5fec9d5fab19237092699f3e1b31c94912926b5e7dd03983328465dffa76a6a227d6518632ac99ebf103e84f8e492e8e2ec37395f2f50b38753f3f3a529f80944cf84c2cc5534dae121bb1c65f62705882d279d18ff9d76a7f8d2546a3407", BIG_SIZE);
        let q_prime = FF::from_hex("c15bb30a08c65b35f17f52c28c86f89f67e786cd87c57792c6dbddd5b9fb83cc38d56bed6b7f67f36e7f1f5df80b93d47be95ca3e11d79038cb23b8ce9809f3ecb79be259e5b65fb4d9317743f724a2c20673300baeb1bdaa532f3a2fe9c65f70e3834b3a51db5b6a0ed590ef52b86b4fd4db72ea9c439b2825003d33a49068b", BIG_SIZE);
        let mut p = &p_prime * &FF::from_hex("2", BIG_SIZE);
        p.inc(1);
        let mut q = &q_prime * &FF::from_hex("2", BIG_SIZE);
        q.inc(1);
        let n = &p * &q;
        let s = random_qr(&n);
    }
}