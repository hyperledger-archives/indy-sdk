extern crate milagro_crypto;
extern crate serde;

use self::milagro_crypto::big::wrappers::{CURVE_Gx, CURVE_Gy, CURVE_Order, BIG};
use self::milagro_crypto::ecp::wrappers::ECP;
use self::milagro_crypto::fp12::wrappers::FP12;

use errors::crypto::CryptoError;
use services::crypto::anoncreds::helpers::BytesView;

use self::milagro_crypto::randapi::Random;

extern crate rand;

use self::rand::os::{OsRng};
use self::rand::Rng;
use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Deserialize, Deserializer};

fn random_mod_order() -> Result<BIG, CryptoError> {
    let mut seed: [u8; 32] = [0; 32];
    let mut os_rng = OsRng::new().unwrap();
    os_rng.fill_bytes(&mut seed);
    let mut rng = Random::new(seed);
    Ok(BIG::randomnum(unsafe { &CURVE_Order }, &mut rng))
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG1 {
    point: ECP
}

pub struct PointG2 {}

#[derive(Debug, Copy, Clone)]
pub struct GroupOrderElement {
    bn: BIG
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pair {
    pair: FP12
}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        let gen_g1: ECP = ECP::new_bigs(unsafe { &CURVE_Gx }, unsafe { &CURVE_Gy });
        let mut point = gen_g1;
        ECP::mul(&mut point, &random_mod_order()?);
        Ok(PointG1 {
            point: point
        })
    }

    pub fn new_inf() -> Result<PointG1, CryptoError> {
        let mut r = ECP::default();
        ECP::inf(&mut r);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        ECP::mul(&mut r, &e.bn);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn add(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        ECP::add(&mut r, &q.point);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn sub(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        ECP::sub(&mut r, &q.point);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn neg(&self) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        ECP::neg(&mut r);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl GroupOrderElement {
    pub fn new() -> Result<GroupOrderElement, CryptoError> {
        // returns random element in 0, ..., GroupOrder-1
        Ok(GroupOrderElement {
            bn: random_mod_order()?
        })
    }

    pub fn pow_mod(&self, e: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        let mut base = self.bn;
        let mut pow = e.bn;
        Ok(GroupOrderElement {
            bn: BIG::powmod(&mut base, &mut pow, unsafe { &CURVE_Order })
        })
    }

    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        let mut sum = BIG::add(&self.bn, &r.bn);
        BIG::rmod(&mut sum, unsafe { &CURVE_Order });
        Ok(GroupOrderElement {
            bn: sum
        })
    }

    pub fn sub_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        //need to use modneg if sub is negative
        let mut sub = BIG::sub(&self.bn, &r.bn);
        if sub < BIG::default() {
            let mut r: BIG = BIG::default();
            BIG::modneg(&mut r, &mut sub, unsafe { &CURVE_Order });
            Ok(GroupOrderElement {
                bn: r
            })
        } else {
            Ok(GroupOrderElement {
                bn: sub
            })
        }
    }

    pub fn mul_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        Ok(GroupOrderElement {
            bn: BIG::modmul(&self.bn, &r.bn, unsafe { &CURVE_Order })
        })
    }

    pub fn inverse(&self) -> Result<GroupOrderElement, CryptoError> {
        Ok(GroupOrderElement {
            bn: BIG::invmodp(&self.bn, unsafe { &CURVE_Order })
        })
    }

    pub fn mod_neg(&self) -> Result<GroupOrderElement, CryptoError> {
        let mut r: BIG = BIG::default();
        let mut bn = self.bn;
        BIG::modneg(&mut r, &mut bn, unsafe { &CURVE_Order });
        Ok(GroupOrderElement {
            bn: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        let mut vec: Vec<u8> = Vec::new();
        BIG::toBytes(&mut vec, &self.bn);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        Ok(
            GroupOrderElement {
                bn: BIG::fromBytes(b)
            }
        )
    }
}

impl Pair {
    pub fn pair(p: &PointG1, q: &PointG1) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn mul(&self, b: &Pair) -> Result<Pair, CryptoError> {
        let mut pair = self.pair;
        FP12::mul(&mut pair, &b.pair);
        Ok(Pair {
            pair: pair
        })
    }

    pub fn pow(&self, b: &GroupOrderElement) -> Result<Pair, CryptoError> {
        let mut r = FP12::default();
        FP12::pow(&mut r, &self.pair, &b.bn);
        Ok(Pair {
            pair: r
        })
    }

    pub fn inverse(&self) -> Result<Pair, CryptoError> {
        let mut r = FP12::default();
        FP12::inv(&mut r, &self.pair);
        Ok(Pair {
            pair: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<Pair, CryptoError> {
        unimplemented!();
    }
}

impl BytesView for Pair {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for PointG1 {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for GroupOrderElement {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl Serialize for GroupOrderElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for GroupOrderElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}

impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for Pair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}

impl Serialize for PointG1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        unimplemented!();
    }
}

impl<'a> Deserialize<'a> for PointG1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        unimplemented!();
    }
}