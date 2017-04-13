use errors::crypto::CryptoError;

extern crate openssl;

use self::openssl::bn::{BigNum, BigNumRef, BigNumContext, MSB_MAYBE_ZERO};
use self::openssl::error::ErrorStack;
use self::openssl::hash::{hash, MessageDigest};
use std::cmp::Ord;
use std::cmp::Ordering;

use std::error::Error;

pub struct BigNumberContext {
    openssl_bn_context: BigNumContext
}

#[derive(Debug)]
pub struct BigNumber {
    openssl_bn: BigNum
}

impl BigNumber {
    pub fn new_context() -> Result<BigNumberContext, CryptoError> {
        let ctx = try!(BigNumContext::new());
        Ok(BigNumberContext {
            openssl_bn_context: ctx
        })
    }

    pub fn new() -> Result<BigNumber, CryptoError> {
        let bn = try!(BigNum::new());
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn safe_prime(&self, size: usize) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        try!(BigNumRef::generate_prime(&mut bn.openssl_bn, size as i32, true, None, None));
        Ok(bn)
    }

    pub fn rand(&self, size: usize) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        try!(BigNumRef::rand(&mut bn.openssl_bn, size as i32, MSB_MAYBE_ZERO, false));
        Ok(bn)
    }

    pub fn rand_range(&self) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        try!(BigNumRef::rand_range(&self.openssl_bn, &mut bn.openssl_bn));
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

    pub fn mul(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::checked_mul(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::checked_mul(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
        Ok(bn)
    }

    pub fn div(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::checked_div(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::checked_div(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
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

    pub fn mod_exp(&self, a: &BigNumber, b: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::mod_exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &b.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::mod_exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &b.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
        Ok(bn)
    }

    pub fn modulus(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::nnmod(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::nnmod(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
        Ok(bn)
    }

    pub fn exp(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
        Ok(bn)
    }

    pub fn inverse(&self, n: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        match ctx {
            Some(context) => try!(BigNumRef::mod_inverse(&mut bn.openssl_bn, &self.openssl_bn, &n.openssl_bn, &mut context.openssl_bn_context)),
            None => {
                let mut ctx = try!(BigNumber::new_context());
                try!(BigNumRef::mod_inverse(&mut bn.openssl_bn, &self.openssl_bn, &n.openssl_bn, &mut ctx.openssl_bn_context));
            }
        }
        Ok(bn)
    }

    pub fn mod_div(&self, b: &BigNumber, p: &BigNumber) -> Result<BigNumber, CryptoError> {
        //(a*  (1/b mod p) mod p)

        let mut context = try!(BigNumber::new_context());

        let mut res = try!(
            b
                .inverse(p, Some(&mut context))?
                .mul(&self, Some(&mut context))?
                .modulus(&p, Some(&mut context))
        );
        Ok(res)
    }

    pub fn compare(&self, other: &BigNumber) -> bool {
        self.openssl_bn == other.openssl_bn
    }

    pub fn clone(&self) -> Result<BigNumber, CryptoError> {
        let bytes = try!(self.to_bytes());
        let mut bn = try!(BigNumber::from_bytes(bytes.as_slice()));
        Ok(bn)
    }
}

impl Ord for BigNumber {
    fn cmp(&self, other: &BigNumber) -> Ordering {
        self.openssl_bn.ucmp(&other.openssl_bn)
    }
}

impl Eq for BigNumber {}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &BigNumber) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other: &BigNumber) -> bool {
        self.openssl_bn == other.openssl_bn
    }
}

impl From<ErrorStack> for CryptoError {
    fn from(err: ErrorStack) -> CryptoError {
        CryptoError::CryptoBackendError(err.description().to_string())
    }
}
