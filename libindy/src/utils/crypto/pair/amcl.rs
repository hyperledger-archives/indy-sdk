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
    CURVE_PYB,
    MODBYTES
};
use self::amcl::ecp::ECP;
use self::amcl::ecp2::ECP2;
use self::amcl::fp12::FP12;
use self::amcl::fp2::FP2;
use self::amcl::pair::{ate, g1mul, g2mul, gtpow, fexp};
use self::amcl::rand::RAND;

use errors::common::CommonError;
use services::anoncreds::helpers::BytesView;

extern crate rand;

use self::rand::os::{OsRng};
use self::rand::Rng;
use self::serde::ser::{Serialize, Serializer, Error as SError};
use self::serde::de::{Deserialize, Deserializer, Visitor, Error as DError};
use std::fmt;

fn random_mod_order() -> Result<BIG, CommonError> {
    let mut seed = vec![0; MODBYTES];
    let mut os_rng = OsRng::new().unwrap();
    os_rng.fill_bytes(&mut seed.as_mut_slice());
    let mut rng = RAND::new();
    rng.clean();
    rng.seed(MODBYTES, &seed);
    Ok(BIG::randomnum(&BIG::new_ints(&CURVE_ORDER), &mut rng))
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointG1 {
    point: ECP
}

impl PointG1 {

    /// Creates new random PointG1
    pub fn new() -> Result<PointG1, CommonError> {
        // generate random point from the group G1
        let point_x = BIG::new_ints(&CURVE_GX);
        let point_y = BIG::new_ints(&CURVE_GY);
        let mut gen_g1 = ECP::new_bigs(&point_x, &point_y);

        let point = g1mul(&mut gen_g1, &mut random_mod_order()?);

        Ok(PointG1 {
            point: point
        })
    }

    /// Creates new infinity PointG1
    pub fn new_inf() -> Result<PointG1, CommonError> {
        let mut r = ECP::new();
        r.inf();
        Ok(PointG1 {
            point: r
        })
    }

    /// Checks infinity
    pub fn is_inf(&self) -> Result<bool, CommonError> {
        let mut r = self.point;
        Ok(r.is_infinity())
    }

    /// PointG1 ^ GroupOrderElement
    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG1, CommonError> {
        let mut r = self.point;
        let mut bn = e.bn;
        Ok(PointG1 {
            point: g1mul(&mut r, &mut bn)
        })
    }

    /// PointG1 * PointG1
    pub fn add(&self, q: &PointG1) -> Result<PointG1, CommonError> {
        let mut r = self.point;
        let mut point = q.point;
        r.add(&mut point);
        Ok(PointG1 {
            point: r
        })
    }

    /// PointG1 / PointG1
    pub fn sub(&self, q: &PointG1) -> Result<PointG1, CommonError> {
        let mut r = self.point;
        let mut point = q.point;
        r.sub(&mut point);
        Ok(PointG1 {
            point: r
        })
    }

    /// 1 / PointG1
    pub fn neg(&self) -> Result<PointG1, CommonError> {
        let mut r = self.point;
        r.neg();
        Ok(PointG1 {
            point: r
        })
    }

    pub fn to_string(&self) -> Result<String, CommonError> {
        Ok(self.point.to_hex())
    }

    pub fn from_string(str: &str) -> Result<PointG1, CommonError> {
        Ok(PointG1 {
            point: ECP::from_hex(str.to_string())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        let mut r = self.point;
        let mut vec = vec![0; MODBYTES*4];
        r.tobytes(&mut vec);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CommonError> {
        Ok(
            PointG1 {
                point: ECP::frombytes(b)
            }
        )
    }
}

impl BytesView for PointG1 {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
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
    /// Creates new random PointG2
    pub fn new() -> Result<PointG2, CommonError> {
        let point_xa = BIG::new_ints(&CURVE_PXA);
        let point_xb = BIG::new_ints(&CURVE_PXB);
        let point_ya = BIG::new_ints(&CURVE_PYA);
        let point_yb = BIG::new_ints(&CURVE_PYB);

        let point_x = FP2::new_bigs(&point_xa, &point_xb);
        let point_y = FP2::new_bigs(&point_ya, &point_yb);

        let mut gen_g2 = ECP2::new_fp2s(&point_x, &point_y);

        let point = g2mul(&mut gen_g2, &mut random_mod_order()?);

        Ok(PointG2 {
            point: point
        })
    }

    /// Creates new infinity PointG2
    pub fn new_inf() -> Result<PointG2, CommonError> {
        let mut point = ECP2::new();
        point.inf();

        Ok(PointG2 {
            point: point
        })
    }

    /// PointG2 * PointG2
    pub fn add(&self, q: &PointG2) -> Result<PointG2, CommonError> {
        let mut r = self.point;
        let mut point = q.point;
        r.add(&mut point);

        Ok(PointG2 {
            point: r
        })
    }

    /// PointG2 / PointG2
    pub fn sub(&self, q: &PointG2) -> Result<PointG2, CommonError> {
        let mut r = self.point;
        let mut point = q.point;
        r.sub(&mut point);

        Ok(PointG2 {
            point: r
        })
    }

    /// PointG2 ^ GroupOrderElement
    pub fn mul(&self, e: &GroupOrderElement) -> Result<PointG2, CommonError> {
        let mut r = self.point;
        let mut bn = e.bn;
        Ok(PointG2 {
            point: g2mul(&mut r, &mut bn)
        })
    }

    pub fn to_string(&self) -> Result<String, CommonError> {
        Ok(self.point.to_hex())
    }

    pub fn from_string(str: &str) -> Result<PointG2, CommonError> {
        Ok(PointG2 {
            point: ECP2::from_hex(str.to_string())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        let mut point = self.point;
        let mut vec = vec![0; MODBYTES*4];
        point.tobytes(&mut vec);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG2, CommonError> {
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
    pub fn new() -> Result<GroupOrderElement, CommonError> {
        // returns random element in 0, ..., GroupOrder-1
        Ok(GroupOrderElement {
            bn: random_mod_order()?
        })
    }

    /// (GroupOrderElement ^ GroupOrderElement) mod GroupOrder
    pub fn pow_mod(&self, e: &GroupOrderElement) -> Result<GroupOrderElement, CommonError> {
        let mut base = self.bn;
        let mut pow = e.bn;
        Ok(GroupOrderElement {
            bn: base.powmod(&mut pow, &BIG::new_ints(&CURVE_ORDER))
        })
    }

    /// (GroupOrderElement + GroupOrderElement) mod GroupOrder
    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CommonError> {
        let mut sum = self.bn;
        sum.add(&r.bn);
        sum.rmod(&BIG::new_ints(&CURVE_ORDER));
        Ok(GroupOrderElement {
            bn: sum
        })
    }

    /// (GroupOrderElement - GroupOrderElement) mod GroupOrder
    pub fn sub_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CommonError> {
        //need to use modneg if sub is negative
        let mut diff = self.bn;
        diff.sub(&r.bn);
        let mut zero = BIG::new();
        zero.zero();

        if diff < zero {
            return Ok(GroupOrderElement {
                bn: BIG::modneg(&mut diff, &BIG::new_ints(&CURVE_ORDER))
            })
        }

        Ok(GroupOrderElement {
            bn: diff
        })

    }

    /// (GroupOrderElement * GroupOrderElement) mod GroupOrder
    pub fn mul_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CommonError> {
        let mut base = self.bn;
        let mut r = r.bn;
        Ok(GroupOrderElement {
            bn: BIG::modmul(&mut base, &mut r, &BIG::new_ints(&CURVE_ORDER))
        })
    }

    /// 1 / GroupOrderElement
    pub fn inverse(&self) -> Result<GroupOrderElement, CommonError> {
        let mut bn = self.bn;
        bn.invmodp(&BIG::new_ints(&CURVE_ORDER));

        Ok(GroupOrderElement {
            bn: bn
        })
    }

    /// - GroupOrderElement mod GroupOrder
    pub fn mod_neg(&self) -> Result<GroupOrderElement, CommonError> {
        let mut r = self.bn;
        r = BIG::modneg(&mut r, &BIG::new_ints(&CURVE_ORDER));
        Ok(GroupOrderElement {
            bn: r
        })
    }

    pub fn to_string(&self) -> Result<String, CommonError> {
        Ok(self.bn.to_hex())
    }

    pub fn from_string(str: &str) -> Result<GroupOrderElement, CommonError> {
        Ok(GroupOrderElement {
            bn: BIG::from_hex(str.to_string())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        let mut bn = self.bn;
        let mut vec: [u8; MODBYTES] = [0; MODBYTES];
        bn.tobytes(&mut vec);
        Ok(vec.to_vec())
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CommonError> {
        let mut vec = b.to_vec();
        let len = vec.len();
        if len < MODBYTES {
            let diff = MODBYTES - len;
            let mut result = vec![0; diff];
            result.append(&mut vec);
            return Ok(
                GroupOrderElement {
                    bn: BIG::frombytes(&result)
                }
            )
        }
        Ok(
            GroupOrderElement {
                bn: BIG::frombytes(b)
            }
        )
    }
}

impl BytesView for GroupOrderElement {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
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
    /// e(PointG1, PointG2)
    pub fn pair(p: &PointG1, q: &PointG2) -> Result<Pair, CommonError> {
        let mut p_new = *p;
        let mut q_new = *q;
        let mut result = fexp(&ate(&mut q_new.point, &mut p_new.point));
        result.reduce();

        Ok(Pair {
            pair: result
        })
    }

    /// e() * e()
    pub fn mul(&self, b: &Pair) -> Result<Pair, CommonError> {
        let mut base = self.pair;
        let mut b = b.pair;
        base.mul(&mut b);
        base.reduce();
        Ok(Pair {
            pair: base
        })
    }

    /// e() ^ GroupOrderElement
    pub fn pow(&self, b: &GroupOrderElement) -> Result<Pair, CommonError> {
        let mut base = self.pair;
        let mut b = b.bn;

        Ok(Pair {
            pair: gtpow(&mut base, &mut b)
        })
    }

    /// 1 / e()
    pub fn inverse(&self) -> Result<Pair, CommonError> {
        let mut r = self.pair;
        r.conj();
        Ok(Pair {
            pair: r
        })
    }

    pub fn to_string(&self) -> Result<String, CommonError> {
        Ok(self.pair.to_hex())
    }

    pub fn from_string(str: &str) -> Result<Pair, CommonError> {
        Ok(Pair {
            pair: FP12::from_hex(str.to_string())
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        let mut r = self.pair;
        let mut vec = vec![0; MODBYTES*16];
        r.tobytes(&mut vec);
        Ok(vec)
    }

    pub fn from_bytes(b: &[u8]) -> Result<Pair, CommonError> {
        unimplemented!();
    }
}

impl BytesView for Pair {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
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

#[cfg(feature = "revocation_tests")]
#[cfg(test)]
mod tests {
    use super::*;

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
    fn from_bytes_to_bytes_works_for_group_order_element() {
        let vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 221, 243, 243, 0, 77, 170, 65, 179, 245, 119, 182, 251, 185, 78, 98, 172, 55, 2, 233, 88, 37, 108, 222, 13, 114, 25, 200, 43, 193, 165, 197];
        let bytes = GroupOrderElement::from_bytes(&vec).unwrap();
        let result = bytes.to_bytes().unwrap();
        assert_eq!(vec, result);
    }

    #[test]
    fn serialize_deserialize_works_for_group_order_element() {
        let structure = TestGroupOrderElementStructure {
            field: GroupOrderElement::from_string("C4D05C20EC7BAC 2FBB155341552D 6AA4C1EA344257 E84BFFBF1408B3 194D3FBA").unwrap()
        };
        let deserialized: TestGroupOrderElementStructure = serde_json::from_str(&serde_json::to_string(&structure).unwrap()).unwrap();

        assert_eq!(structure, deserialized);
    }

    #[test]
    fn serialize_deserialize_works_for_point_g1() {
        let structure = TestPointG1Structure {
            field: PointG1::from_string("true 2EECD9DE38C76F D006CA023A62FC F896DFE9BDD67E 4EDC83F2F88686 39D6CFB6AE4C5F AD41D5E8FFF5BB 46A9EE94E8EC 91210CA1BEE64A D0528EBCE5A3F 644EA24C7C9248 8433949C4FDFFB 32F422BC05E873 E8E64BD56D37 2466FE12D325 8094CE251D2839 5A656663FA5F96 157FD83C6B2426 2FEC3584898982 617BEAC39065DB A9FEFC2AE937BB 6393541E1FFC").unwrap()
        };

        let deserialized: TestPointG1Structure = serde_json::from_str(&serde_json::to_string(&structure).unwrap()).unwrap();

        assert_eq!(structure, deserialized);
    }

    #[test]
    fn deserialize_works_for_point_g2() {
        let structure = TestPointG2Structure {
            field: PointG2::from_string("true D11B6D8D20603F 81E85DD0E5F940 58637C2CEFE82 C9AA5515E23FF9 769669C8986A4F B5A4325CEA7B0 66E949BD16A3 77D948C5888160 F81A47F4DD46FE 93485FD0E0275E 328A14F82C2D82 4B9FDD2474E6F5 7A2C5A48F032E6 3249E376F483 6DCCFE77B6B0C1 84C2FD6C046348 808EC9FEB85E2D 773FDC47D8873A D2898B104A8722 4642BDD3AF2B33 A8C52C0D6BF 50D29D832D0799 7163F19F154FE3 5D049BB7C3D8C9 F8AB1B6E4A17F 882A2BC9B08B18 BB13529EA3B95F 3671B9C0F8A3 8094CE251D2839 5A656663FA5F96 157FD83C6B2426 2FEC3584898982 617BEAC39065DB A9FEFC2AE937BB 6393541E1FFC 0 0 0 0 0 0 0").unwrap()
        };
        let deserialized: TestPointG2Structure = serde_json::from_str(&serde_json::to_string(&structure).unwrap()).unwrap();

        assert_eq!(structure, deserialized);
    }

    #[test]
    fn serialize_deserialize_works_for_pair() {
        let point_g1 = PointG1 {
            point: PointG1::from_string("false 2EECD9DE38C76F D006CA023A62FC F896DFE9BDD67E 4EDC83F2F88686 39D6CFB6AE4C5F AD41D5E8FFF5BB 46A9EE94E8EC 91210CA1BEE64A D0528EBCE5A3F 644EA24C7C9248 8433949C4FDFFB 32F422BC05E873 E8E64BD56D37 2466FE12D325 8094CE251D2839 5A656663FA5F96 157FD83C6B2426 2FEC3584898982 617BEAC39065DB A9FEFC2AE937BB 6393541E1FFC").unwrap().point
        };
        let point_g2 = PointG2 {
            point: PointG2::from_string("false D11B6D8D20603F 81E85DD0E5F940 58637C2CEFE82 C9AA5515E23FF9 769669C8986A4F B5A4325CEA7B0 66E949BD16A3 77D948C5888160 F81A47F4DD46FE 93485FD0E0275E 328A14F82C2D82 4B9FDD2474E6F5 7A2C5A48F032E6 3249E376F483 6DCCFE77B6B0C1 84C2FD6C046348 808EC9FEB85E2D 773FDC47D8873A D2898B104A8722 4642BDD3AF2B33 A8C52C0D6BF 50D29D832D0799 7163F19F154FE3 5D049BB7C3D8C9 F8AB1B6E4A17F 882A2BC9B08B18 BB13529EA3B95F 3671B9C0F8A3 8094CE251D2839 5A656663FA5F96 157FD83C6B2426 2FEC3584898982 617BEAC39065DB A9FEFC2AE937BB 6393541E1FFC 0 0 0 0 0 0 0").unwrap().point
        };
        let pair = TestPairStructure {
            field: Pair::pair(&point_g1, &point_g2).unwrap()
        };
        let deserialized: TestPairStructure = serde_json::from_str(&serde_json::to_string(&pair).unwrap()).unwrap();

        assert_eq!(pair, deserialized);
    }

    #[test]
    fn pairing_definition_bilinearity() {
        let a = GroupOrderElement::new().unwrap();
        let b = GroupOrderElement::new().unwrap();
        let p = PointG1::new().unwrap();
        let q = PointG2::new().unwrap();
        let left = Pair::pair(&p.mul(&a).unwrap(), &q.mul(&b).unwrap()).unwrap();
        let right = Pair::pair(&p, &q).unwrap().pow(&a.mul_mod(&b).unwrap()).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn point_g1_infinity_test() {
        let p = PointG1::new_inf().unwrap();
        let q = PointG1::new().unwrap();
        let result = p.add(&q).unwrap();
        assert_eq!(q, result);
    }

    #[test]
    fn point_g1_infinity_test2() {
        let p = PointG1::new().unwrap();
        let inf = p.sub(&p).unwrap();
        let q = PointG1::new().unwrap();
        let result = inf.add(&q).unwrap();
        assert_eq!(q, result);
    }

    #[test]
    fn point_g2_infinity_test() {
        let p = PointG2::new_inf().unwrap();
        let q = PointG2::new().unwrap();
        let result = p.add(&q).unwrap();
        assert_eq!(q, result);
    }

    #[test]
    fn inverse_for_pairing() {
        let p1 = PointG1::new().unwrap();
        let q1 = PointG2::new().unwrap();
        let p2 = PointG1::new().unwrap();
        let q2 = PointG2::new().unwrap();
        let pair1 = Pair::pair(&p1, &q1).unwrap();
        let pair2 = Pair::pair(&p2, &q2).unwrap();
        let pair_result = pair1.mul(&pair2).unwrap();
        let pair3 = pair_result.mul(&pair1.inverse().unwrap()).unwrap();
        assert_eq!(pair2, pair3);
    }
}