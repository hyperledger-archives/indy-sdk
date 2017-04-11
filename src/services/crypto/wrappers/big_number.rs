extern crate openssl;
use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
use self::openssl::hash::{hash, MessageDigest};
use services::crypto::constants::{LARGE_PRIME};


pub struct BigNumber {
    bn: BigNum
}

impl BigNumber {
    pub fn new() -> BigNumber {
        BigNumber {
            bn: BigNum::new().unwrap()
        }
    }

    pub fn safe_prime(&mut self)  {
        self.bn.generate_prime(LARGE_PRIME as i32, true, None, None).unwrap()
    }

    pub fn from_dec(dec: &str) -> BigNumber {
        BigNumber {
            bn: BigNum::from_dec_str(dec).unwrap()
        }
    }

    pub fn from_hex(hex: &str) -> BigNumber {
        BigNumber {
            bn: BigNum::from_hex_str(hex).unwrap()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> BigNumber {
        BigNumber {
            bn: BigNum::from_slice(bytes).unwrap()
        }
    }

    pub fn to_dec(&self) -> String {
        self.bn.to_dec_str().unwrap().to_string()
    }

    pub fn to_hex(&self) -> String {
        self.bn.to_hex_str().unwrap().to_string()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bn.to_vec()
    }

    pub fn add(&mut self, a: &BigNumber, b: &BigNumber) {
        self.bn.checked_add(&a.bn, &b.bn).unwrap()
    }

    pub fn sub(&mut self, a: &BigNumber, b: &BigNumber) {
        self.bn.checked_sub(&a.bn, &b.bn).unwrap()
    }

    pub fn mul(&mut self, a: &BigNumber, b: &BigNumber) {
        let mut ctx = BigNumContext::new().unwrap();
        self.bn.checked_mul(&a.bn, &b.bn, &mut ctx).unwrap()
    }

    pub fn add_word(&mut self, w:u32) {
        self.add_word(w);
    }

    pub fn sub_word(&mut self, w:u32) {
        self.sub_word(w);
    }

    pub fn mul_word(&mut self, w:u32) {
        self.mul_word(w);
    }

    pub fn div_word(&mut self, w:u32) {
        self.div_word(w);
    }

    pub fn mod_exp(&mut self, a: &BigNumber, b: &BigNumber, c: &BigNumber) {
        let mut ctx = BigNumContext::new().unwrap();
        self.bn.mod_exp(&a.bn, &b.bn, &c.bn, &mut ctx).unwrap()
    }

    fn modulus(&mut self, a: &BigNumber, b: &BigNumber) {
        let mut ctx = BigNumContext::new().unwrap();
        self.bn.nnmod(&a.bn, &b.bn, &mut ctx).unwrap()
    }
}