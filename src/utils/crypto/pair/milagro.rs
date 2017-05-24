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
use self::milagro_crypto::fp2::wrappers::FP2;

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
        let mut gen_g1: ECP = ECP::new_bigs(&unsafe { CURVE_Gx }.clone(), &unsafe { CURVE_Gy }.clone());

        ECP::mul(&mut gen_g1, &random_mod_order()?);
        Ok(PointG1 {
            point: gen_g1
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
        Ok(ECP::to_hex(&self.point))
    }

    pub fn from_string(str: &str) -> Result<PointG1, CryptoError> {
        Ok(PointG1 {
            point: ECP::from_hex(str.to_string())
        })
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
        let mut point_x = FP2::default();
        let mut point_y = FP2::default();
        let mut point_z = FP2::default();
        FP2::from_BIGs(&mut point_x, &unsafe { CURVE_Pxa }.clone(), &unsafe { CURVE_Pxb }.clone());
        FP2::from_BIGs(&mut point_y, &unsafe { CURVE_Pya }.clone(), &unsafe { CURVE_Pyb }.clone());
        FP2::from_BIGs(&mut point_z, &BIG::from_hex("1".to_string()), &BIG::from_hex("0".to_string()));
        let mut gen_g2: ECP2 = ECP2::new_fp2s(point_x, point_y, point_z);

        ECP2::mul(&mut gen_g2, &random_mod_order()?);
        Ok(PointG2 {
            point: gen_g2
        })
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
        Ok(ECP2::to_hex(&self.point))
    }

    pub fn from_string(str: &str) -> Result<PointG2, CryptoError> {
        Ok(PointG2 {
            point: ECP2::from_hex(str.to_string())
        })
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestGroupOrderElementStructure {
        field: GroupOrderElement
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestPointG1Structure {
        field: PointG1
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestPointG2Structure {
        field: PointG2
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestPairStructure {
        field: Pair
    }

    #[test]
    fn serialize_works_for_group_order_element() {
        let structure = TestGroupOrderElementStructure {
            field: GroupOrderElement::from_string("C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA").unwrap()
        };
        let str = r#"{"field":"C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA"}"#;

        let serialized = serde_json::to_string(&structure).unwrap();
        assert_eq!(str, serialized);
    }

    #[test]
    fn deserialize_works_for_group_order_element() {
        let structure = TestGroupOrderElementStructure {
            field: GroupOrderElement::from_string("C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA").unwrap()
        };
        let str = r#"{"field":"C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA"}"#;
        let deserialized: TestGroupOrderElementStructure = serde_json::from_str(&str).unwrap();

        assert_eq!(structure, deserialized);
    }

    #[test]
    fn serialize_works_for_point_g1() {
        let structure = TestPointG1Structure {
            field: PointG1::from_string("1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0").unwrap()
        };
        let str = r#"{"field":"1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0"}"#;

        let serialized = serde_json::to_string(&structure).unwrap();
        assert_eq!(str, serialized);
    }

    #[test]
    fn deserialize_works_for_point_g1() {
        let structure = TestPointG1Structure {
            field: PointG1::from_string("1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0").unwrap()
        };

        let str = r#"{"field":"1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0"}"#;
        let deserialized: TestPointG1Structure = serde_json::from_str(&str).unwrap();

        assert_eq!(structure, deserialized);
    }

    #[test]
    fn serialize_works_for_point_g2() {
        let structure = TestPointG2Structure {
            field: PointG2::from_string("0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0").unwrap()
        };

        let str = r#"{"field":"0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0"}"#;
        let serialized = serde_json::to_string(&structure).unwrap();

        assert_eq!(str, serialized);
    }

    #[test]
    fn deserialize_works_for_point_g2() {
        let structure = TestPointG2Structure {
            field: PointG2::from_string("0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0").unwrap()
        };
        let str = r#"{"field":"0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0"}"#;
        let deserialized: TestPointG2Structure = serde_json::from_str(&str).unwrap();

        assert_eq!(structure, deserialized);
    }
    #[test]
    fn serialize_works_for_pair() {

    }

    #[test]
    fn deserialize_works_for_pair() {

    }
}