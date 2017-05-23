extern crate milagro_crypto;
extern crate serde;

use self::milagro_crypto::big::wrappers::{
    CURVE_Gx,
    CURVE_Gy,
    CURVE_Order,
    CURVE_Pxa,
    CURVE_Pya,
    CURVE_Pxb,
    CURVE_Pyb,
    BIG
};
use self::milagro_crypto::ecp::wrappers::ECP;
use self::milagro_crypto::ecp2::wrappers::ECP2;
use self::milagro_crypto::fp12::wrappers::FP12;

use errors::crypto::CryptoError;
use services::anoncreds::helpers::BytesView;

use self::milagro_crypto::randapi::Random;

extern crate rand;

use self::rand::os::{OsRng};
use self::rand::Rng;
use self::serde::ser::{Serialize, Serializer, Error as SError};
use self::serde::de::{Deserialize, Deserializer, Visitor, Error as DError};
use std::fmt;

fn random_mod_order() -> Result<BIG, CryptoError> {
    let mut seed = vec![0; 32];
    let mut os_rng = OsRng::new().unwrap();
    os_rng.fill_bytes(&mut seed.as_mut_slice());
    let mut rng = Random::new(&seed);
    Ok(BIG::randomnum(&unsafe { CURVE_Order }.clone(), &mut rng))
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG1 {
    point: ECP
}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        let gen_g1: ECP = ECP::new_bigs(&unsafe { CURVE_Gx }.clone(), &unsafe { CURVE_Gy }.clone());
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

    pub fn from_string(str: &str) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl BytesView for PointG1 {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl Serialize for PointG1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("PointG1", &self.to_string().map_err(SError::custom)?)
    }
}

impl<'a> Deserialize<'a> for PointG1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct PointG1Visitor;

        impl<'a> Visitor<'a> for PointG1Visitor {
            type Value = PointG1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected PointG1")
            }

            fn visit_str<E>(self, value: &str) -> Result<PointG1, E>
                where E: DError
            {
                Ok(PointG1::from_string(value).map_err(DError::custom)?)
            }
        }

        deserializer.deserialize_str(PointG1Visitor)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG2 {
    point: ECP2
}

impl PointG2 {
    pub fn new() -> Result<PointG2, CryptoError> {
        unimplemented!();
    }

    pub fn new_inf() -> Result<PointG2, CryptoError> {
        unimplemented!();
    }

    pub fn add(&self, q: &PointG2) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        ECP2::add(&mut r, &q.point);
        Ok(PointG2 {
            point: r
        })
    }

    pub fn sub(&self, q: &PointG2) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        ECP2::sub(&mut r, &q.point);
        Ok(PointG2 {
            point: r
        })
    }

    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        ECP2::mul(&mut r, &e.bn);
        Ok(PointG2 {
            point: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn from_string(str: &str) -> Result<PointG2, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl Serialize for PointG2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("PointG2", &self.to_string().map_err(SError::custom)?)
    }
}

impl<'a> Deserialize<'a> for PointG2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct PointG2Visitor;

        impl<'a> Visitor<'a> for PointG2Visitor {
            type Value = PointG2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected PointG2")
            }

            fn visit_str<E>(self, value: &str) -> Result<PointG2, E>
                where E: DError
            {
                Ok(PointG2::from_string(value).map_err(DError::custom)?)
            }
        }

        deserializer.deserialize_str(PointG2Visitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GroupOrderElement {
    bn: BIG
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
            bn: BIG::powmod(&mut base, &mut pow, &unsafe { CURVE_Order }.clone())
        })
    }

    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        let mut sum = BIG::add(&self.bn, &r.bn);
        BIG::rmod(&mut sum, &unsafe { CURVE_Order }.clone());
        Ok(GroupOrderElement {
            bn: sum
        })
    }

    pub fn sub_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        //need to use modneg if sub is negative
        let mut sub = BIG::sub(&self.bn, &r.bn);
        if sub < BIG::default() {
            let mut r: BIG = BIG::default();
            BIG::modneg(&mut r, &mut sub, &unsafe { CURVE_Order }.clone());
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
            bn: BIG::modmul(&self.bn, &r.bn, &unsafe { CURVE_Order }.clone())
        })
    }

    pub fn inverse(&self) -> Result<GroupOrderElement, CryptoError> {
        Ok(GroupOrderElement {
            bn: BIG::invmodp(&self.bn, &unsafe { CURVE_Order }.clone())
        })
    }

    pub fn mod_neg(&self) -> Result<GroupOrderElement, CryptoError> {
        let mut r: BIG = BIG::default();
        let mut bn = self.bn;
        BIG::modneg(&mut r, &mut bn, &unsafe { CURVE_Order }.clone());
        Ok(GroupOrderElement {
            bn: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        Ok(BIG::to_hex(&self.bn))
    }

    pub fn from_string(str: &str) -> Result<GroupOrderElement, CryptoError> {
        Ok(GroupOrderElement {
            bn: BIG::from_hex(str.to_string())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        let mut vec: [u8; 32] = [0; 32];
        BIG::toBytes(&mut vec, &self.bn);
        Ok(vec.to_vec())
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        Ok(
            GroupOrderElement {
                bn: BIG::fromBytes(b)
            }
        )
    }
}

impl BytesView for GroupOrderElement {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl Serialize for GroupOrderElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("GroupOrderElement", &self.to_string().map_err(SError::custom)?)
    }
}

impl<'a> Deserialize<'a> for GroupOrderElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct GroupOrderElementVisitor;

        impl<'a> Visitor<'a> for GroupOrderElementVisitor {
            type Value = GroupOrderElement;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected GroupOrderElement")
            }

            fn visit_str<E>(self, value: &str) -> Result<GroupOrderElement, E>
                where E: DError
            {
                Ok(GroupOrderElement::from_string(value).map_err(DError::custom)?)
            }
        }

        deserializer.deserialize_str(GroupOrderElementVisitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pair {
    pair: FP12
}

impl Pair {
    pub fn pair(p: &PointG1, q: &PointG2) -> Result<Pair, CryptoError> {
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

    pub fn from_string(str: &str) -> Result<Pair, CryptoError> {
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

impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("Pair", &self.to_string().map_err(SError::custom)?)
    }
}

impl<'a> Deserialize<'a> for Pair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct PairVisitor;

        impl<'a> Visitor<'a> for PairVisitor {
            type Value = Pair;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected Pair")
            }

            fn visit_str<E>(self, value: &str) -> Result<Pair, E>
                where E: DError
            {
                Ok(Pair::from_string(value).map_err(DError::custom)?)
            }
        }

        deserializer.deserialize_str(PairVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::logger::LoggerUtils;

    extern crate serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestGroupOrderElementStructure {
        field: GroupOrderElement
    }

    #[derive(Serialize, Deserialize)]
    struct TestPointG1Structure {
        field: PointG1
    }

    #[derive(Serialize, Deserialize)]
    struct TestPointG2Structure {
        field: PointG2
    }

    #[derive(Serialize, Deserialize)]
    struct TestPairStructure {
        field: Pair
    }

    #[test]
    fn serialize_works_for_group_order_element() {
        let structure = TestGroupOrderElementStructure {
            field: GroupOrderElement::from_string("C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA").unwrap()
        };

        let serialized = serde_json::to_string(&structure).unwrap();
        assert_eq!("{\"field\":\"C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA\"}", serialized);
    }

    #[test]
    fn deserialize_works_for_group_order_element() {

    }

    #[test]
    fn serialize_works_for_point_g1() {

    }

    #[test]
    fn deserialize_works_for_point_g1() {

    }

    #[test]
    fn serialize_works_for_point_g2() {

    }

    #[test]
    fn deserialize_works_for_point_g2() {

    }
    #[test]
    fn serialize_works_for_pair() {

    }

    #[test]
    fn deserialize_works_for_pair() {

    }
}