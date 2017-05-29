extern crate serde;
extern crate amcl;

use self::amcl::big::BIG;
use self::amcl::rom::{
    CURVE_GX,
    CURVE_GY,
    CURVE_ORDER,
    CURVE_PXA,
    CURVE_PYA,
    CURVE_PXB,
    CURVE_PYB
};
use self::amcl::ecp::ECP;
use self::amcl::ecp2::ECP2;
use self::amcl::fp12::FP12;
use self::amcl::fp2::FP2;
use self::amcl::pair::ate;
use self::amcl::rand::RAND;

use errors::crypto::CryptoError;
use services::anoncreds::helpers::BytesView;

extern crate rand;

use self::rand::os::{OsRng};
use self::rand::Rng;
use self::serde::ser::{Serialize, Serializer, Error as SError};
use self::serde::de::{Deserialize, Deserializer, Visitor, Error as DError};
use std::fmt;

pub const GROUP_ORDER: BIG = BIG::new_ints(&CURVE_ORDER);

fn random_mod_order() -> Result<BIG, CryptoError> {
    let mut seed = vec![0; 32];
    let mut os_rng = OsRng::new().unwrap();
    os_rng.fill_bytes(&mut seed.as_mut_slice());
    let mut rng = RAND::new();
    rng.clean();
    rng.seed(32, &seed);
    Ok(BIG::randomnum(&GROUP_ORDER, &mut rng))
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG1 {
    point: ECP
}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        let point_x = BIG::new_ints(&CURVE_GX);
        let point_y = BIG::new_ints(&CURVE_GY);
        let mut gen_g1 = ECP::new_bigs(&point_x, &point_y);
        let point = gen_g1.mul(&mut random_mod_order()?);

        Ok(PointG1 {
            point: point
        })
    }

    pub fn new_inf() -> Result<PointG1, CryptoError> {
        let mut r = ECP::new();
        r.inf();
        Ok(PointG1 {
            point: r
        })
    }

    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        let mut bn = e.bn;
        Ok(PointG1 {
            point: r.mul(&mut bn)
        })
    }

    pub fn add(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        let mut point = q.point;
        r.add(&mut point);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn sub(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        let mut point = q.point;
        r.sub(&mut point);
        Ok(PointG1 {
            point: r
        })
    }

    pub fn neg(&self) -> Result<PointG1, CryptoError> {
        let mut r = self.point;
        r.neg();
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
        let mut r = self.point;
        let mut vec = vec![0;32];
        r.tobytes(&mut vec);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        Ok(
            PointG1 {
                point: ECP::frombytes(b)
            }
        )
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
        let point_xa = BIG::new_ints(&CURVE_PXA);
        let point_xb = BIG::new_ints(&CURVE_PXB);
        let point_ya = BIG::new_ints(&CURVE_PYA);
        let point_yb = BIG::new_ints(&CURVE_PYB);

        let point_x = FP2::new_bigs(&point_xa, &point_xb);
        let point_y = FP2::new_bigs(&point_ya, &point_yb);

        let mut gen_g2 = ECP2::new_fp2s(&point_x, &point_y);
        let point = gen_g2.mul(&random_mod_order()?);

        Ok(PointG2 {
            point: point
        })
    }

    pub fn new_inf() -> Result<PointG2, CryptoError> {
        let mut point = ECP2::new();
        point.inf();

        Ok(PointG2 {
            point: point
        })
    }

    pub fn add(&self, q: &PointG2) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        let mut point = q.point;
        r.add(&mut point);

        Ok(PointG2 {
            point: r
        })
    }

    pub fn sub(&self, q: &PointG2) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        let mut point = q.point;
        r.sub(&mut point);

        Ok(PointG2 {
            point: r
        })
    }

    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG2, CryptoError> {
        let mut r = self.point;
        let mut bn = e.bn;
        Ok(PointG2 {
            point: r.mul(&mut bn)
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
        let mut point = self.point;
        let mut vec = vec![0; 32];
        point.tobytes(&mut vec);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG2, CryptoError> {
        Ok(
            PointG2 {
                point: ECP2::frombytes(b)
            }
        )
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
            bn: base.powmod(&mut pow, &GROUP_ORDER)
        })
    }

    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        let mut sum = self.bn;
        sum.add(&r.bn);
        sum.rmod(&GROUP_ORDER);
        Ok(GroupOrderElement {
            bn: sum
        })
    }

    pub fn sub_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        //need to use modneg if sub is negative
        let mut diff = self.bn;
        diff.sub(&r.bn);
        let mut zero = BIG::new();
        zero.zero();

        if diff < zero {
            return Ok(GroupOrderElement {
                bn: BIG::modneg(&mut diff, &GROUP_ORDER)
            })
        }

        Ok(GroupOrderElement {
            bn: diff
        })

    }

    pub fn mul_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        let mut base = self.bn;
        let mut r = r.bn;
        Ok(GroupOrderElement {
            bn: BIG::modmul(&mut base, &mut r, &GROUP_ORDER)
        })
    }

    pub fn inverse(&self) -> Result<GroupOrderElement, CryptoError> {
        let mut bn = self.bn;
        bn.invmodp(&GROUP_ORDER);

        Ok(GroupOrderElement {
            bn: bn
        })
    }

    pub fn mod_neg(&self) -> Result<GroupOrderElement, CryptoError> {
        let mut r = self.bn;
        r = BIG::modneg(&mut r, &GROUP_ORDER);
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
        let mut bn = self.bn;
        let mut vec: [u8; 32] = [0; 32];
        bn.tobytes(&mut vec);
        Ok(vec.to_vec())
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        Ok(
            GroupOrderElement {
                bn: BIG::frombytes(b)
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
        let mut p_new = *p;
        let mut q_new = *q;

        Ok(Pair {
            pair: ate(&mut q_new.point, &mut p_new.point)
        })
    }

    pub fn mul(&self, b: &Pair) -> Result<Pair, CryptoError> {
        let mut base = self.pair;
        let mut b = b.pair;
        base.mul(&mut b);
        Ok(Pair {
            pair: base
        })
    }

    pub fn pow(&self, b: &GroupOrderElement) -> Result<Pair, CryptoError> {
        let mut base = self.pair;
        let mut b = b.bn;
        base.pow(&mut b);
        Ok(Pair {
            pair: base
        })
    }

    pub fn inverse(&self) -> Result<Pair, CryptoError> {
        let mut r = self.pair;
        r.inverse();
        Ok(Pair {
            pair: r
        })
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        Ok(FP12::to_hex(&self.pair))
    }

    pub fn from_string(str: &str) -> Result<Pair, CryptoError> {
        Ok(Pair {
            pair: FP12::from_hex(str.to_string())
        })
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
        let point_g1 = PointG1 {
            point: PointG1::from_string("1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0").unwrap().point
        };
        let point_g2 = PointG2 {
            point: PointG2::from_string("0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0").unwrap().point
        };
        let pair = TestPairStructure {
            field: Pair::pair(&point_g1, &point_g2).unwrap()
        };
        let str = r#"{"field":"70CC65D2371808 FD4F75244E5B72 40359FC95F7204 FA308613F34F1D 551BB55FA 7CB294CCE69B4A AE3C7228995A41 F27CD79430990 DE04BB58428296 5303A03BD 7FFA8A31C72E82 A8E1AA2E51D4B2 87B33F735B7ADF 19EADA0B95227E 392C800DC 74571A20806B80 ABDB72819D0A70 D4B1EDF5A54E6F FAF8EA4B2EFC2D 3CE6F2507 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002"}"#;
        let serialized = serde_json::to_string(&pair).unwrap();

        assert_eq!(str, serialized);
    }

    #[test]
    fn deserialize_works_for_pair() {
        let point_g1 = PointG1 {
            point: PointG1::from_string("1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0").unwrap().point
        };
        let point_g2 = PointG2 {
            point: PointG2::from_string("0 53104BD1A92BE9 4CBF937B44DAA 1D191B0496A14B 276529199F4D1B 4A996C2 3B2712E2EC37FF CF7C4390E8071C EF8C973AD5EDAA 547DD84375861 169CBAC9 5224321CF032B7 B9D2063515A045 9833D500F6EEBE DB9D00AED36ED2 7916166 22D7513761F614 4CD0E53D855FC3 950F3C38908717 A0261AC49D33A0 1B221531 A96F211585EDB F2942F28DB526F 2FF74229029FCD F4EABE779E75E4 3C3FED4 0 0 0 0 0").unwrap().point
        };
        let pair = TestPairStructure {
            field: Pair::pair(&point_g1, &point_g2).unwrap()
        };
        let str = r#"{"field":"70CC65D2371808 FD4F75244E5B72 40359FC95F7204 FA308613F34F1D 551BB55FA 7CB294CCE69B4A AE3C7228995A41 F27CD79430990 DE04BB58428296 5303A03BD 7FFA8A31C72E82 A8E1AA2E51D4B2 87B33F735B7ADF 19EADA0B95227E 392C800DC 74571A20806B80 ABDB72819D0A70 D4B1EDF5A54E6F FAF8EA4B2EFC2D 3CE6F2507 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 1EF2EE10541C8A 7C8B52D128C803 9D4A4954550B73 922CD02BD9DA10 AF8000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002 5543B5BF0C1826 69623262363316 E78DA0826F5875 2CEAD78790F397 948000002"}"#;
        let deserialized: TestPairStructure = serde_json::from_str(&str).unwrap();

        assert_eq!(pair, deserialized);
    }

    #[test] //TODO: remove it
    fn stack_smashing_detected() {
        let point = PointG2::new().unwrap();
        println!("pstr: {}", point.to_string().unwrap());
    }
}