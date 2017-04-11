#[cfg(feature = "bn_openssl")]
pub mod bn_impl {
    extern crate openssl;

    use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
    use self::openssl::hash::{hash, MessageDigest};
    use self::openssl::error::ErrorStack;
    use services::crypto::anoncreds::constants::{LARGE_PRIME};
    use errors::crypto::CryptoError;
    use std::error::Error;

    pub struct BigNumber {
        openssl_bn: BigNum
    }

    impl BigNumber {
        pub fn new() -> Result<BigNumber, CryptoError> {
            let bn = try!(BigNum::new());
            Ok(BigNumber {
                openssl_bn: bn
            })
        }

        pub fn safe_prime(&self, size: i32) -> Result<BigNumber, CryptoError> {
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::generate_prime(&mut bn.openssl_bn, size, true, None, None));
            Ok(bn)
        }

        pub fn rand(&self, size: i32) -> Result<BigNumber, CryptoError> {
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::rand(&mut bn.openssl_bn, size, MSB_MAYBE_ZERO, false));
            Ok(bn)
        }

        pub fn from_u32(n: u32) -> Result<BigNumber, CryptoError> {
            let bn = try!(BigNum::from_u32(n));
            Ok(BigNumber {
                openssl_bn: bn
            })
        }

        pub fn from_dec(dec: &str) -> Result<BigNumber, CryptoError> {
            let bn = try!(BigNum::from_dec_str(dec));
            Ok(BigNumber {
                openssl_bn: bn
            })
        }

        pub fn from_hex(hex: &str) -> Result<BigNumber, CryptoError> {
            let bn = try!(BigNum::from_hex_str(hex));
            Ok(BigNumber {
                openssl_bn: bn
            })
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<BigNumber, CryptoError> {
            let bn = try!(BigNum::from_slice(bytes));
            Ok(BigNumber {
                openssl_bn: bn
            })
        }

        pub fn to_dec(&self) -> Result<String, CryptoError> {
            let result = try!(self.openssl_bn.to_dec_str());
            Ok(result.to_string())
        }

        pub fn to_hex(&self) -> Result<String, CryptoError> {
            let result = try!(self.openssl_bn.to_hex_str());
            Ok(result.to_string())
        }

        pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
            Ok(self.openssl_bn.to_vec())
        }

        pub fn add(&self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::checked_add(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn));
            Ok(bn)
        }

        pub fn sub(&self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::checked_sub(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn));
            Ok(bn)
        }

        pub fn mul(&mut self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut ctx = BigNumContext::new().unwrap();
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::checked_mul(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx));
            Ok(bn)
        }

        pub fn div(&mut self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut ctx = BigNumContext::new().unwrap();
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::checked_div(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx));
            Ok(bn)
        }

        pub fn add_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
            try!(BigNumRef::add_word(&mut self.openssl_bn, w));
            Ok(self)
        }

        pub fn sub_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
            try!(BigNumRef::sub_word(&mut self.openssl_bn, w));
            Ok(self)
        }

        pub fn mul_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
            try!(BigNumRef::mul_word(&mut self.openssl_bn, w));
            Ok(self)
        }

        pub fn div_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
            try!(BigNumRef::div_word(&mut self.openssl_bn, w));
            Ok(self)
        }

        pub fn mod_exp(&self, a: &BigNumber, b: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut ctx = BigNumContext::new().unwrap();
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::mod_exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &b.openssl_bn, &mut ctx));
            Ok(bn)
        }

        fn modulus(&self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
            let mut ctx = BigNumContext::new().unwrap();
            let mut bn = try!(BigNumber::new());
            try!(BigNumRef::nnmod(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx));
            Ok(bn)
        }
    }

    impl From<ErrorStack> for CryptoError {
        fn from(err: ErrorStack) -> CryptoError {
            CryptoError::CryptoBackendError(err.description().to_string())
        }
    }
}