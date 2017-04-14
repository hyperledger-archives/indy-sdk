use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_PRIME,
    LARGE_VPRIME_PRIME
};
use services::crypto::anoncreds::types::{
    Attribute,
    ByteOrder,
    PublicKey,
    Schema,
    SecretKey
};
use services::crypto::helpers::{
    random_qr,
    bitwise_or_big_int
};
use services::crypto::wrappers::bn::BigNumber;

use std::collections::HashMap;

pub struct Issuer {

}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }
    pub fn generate_keys(&self, schema: &Schema) -> Result<((PublicKey, SecretKey)), CryptoError> {
        //(Issuer::_generate_keys(&schema));
        unimplemented!();
    }

    pub fn create_claim() {

    }

    fn _generate_keys(schema: &Schema) -> Result<(PublicKey, SecretKey), CryptoError> {
        let bn = try!(BigNumber::new());
        let p = try!(bn.safe_prime(LARGE_PRIME));
        let q = try!(bn.safe_prime(LARGE_PRIME));

        let mut p_prime = try!(p.sub(&try!(BigNumber::from_u32(1))));
        try!(p_prime.div_word(2));

        let mut q_prime = try!(q.sub(&try!(BigNumber::from_u32(1))));
        try!(q_prime.div_word(2));

        let n = try!(p.mul(&q, None));
        let s = try!(random_qr(&n));
        let xz = try!(Issuer::_gen_x(&p_prime, &q_prime));
        let mut r: HashMap<String, BigNumber> = HashMap::new();

        for attribute in &schema.attribute_names {
            let random = try!(Issuer::_gen_x(&p_prime, &q_prime));
            r.insert(attribute.to_string(), try!(s.mod_exp(&random, &n, None)));
        }

        let z = try!(s.mod_exp(&xz, &n, None));

        let rms = try!(s.mod_exp(&try!(Issuer::_gen_x(&p_prime, &q_prime)), &n, None));
        let rctxt = try!(s.mod_exp(&try!(Issuer::_gen_x(&p_prime, &q_prime)), &n, None));
        Ok((
            PublicKey {
                n: n,
                rms: rms,
                rctxt: rctxt,
                r: r,
                s: s,
                z: z
            },
            SecretKey {
                p: p_prime,
                q: q_prime
            }
        ))
    }

    fn _generate_revocation_keys() {

    }

    fn _issuer_primary_claim(&self) {

    }

    fn _encode_attribute(attribute: &str, byte_order: ByteOrder) -> Result<String, CryptoError> {
        let mut result = try!(BigNumber::hash(&attribute.as_bytes()));
        let index = result.iter().position(|&value| value == 0);
        if let Some(position) = index {
            result.truncate(position);
        }
        if let ByteOrder::Little = byte_order {
            result.reverse();
        }
        let encoded_attribute = try!(BigNumber::from_bytes(&result));
        Ok(try!(encoded_attribute.to_dec()).to_string())
    }

    fn _encode_attributes(attributes: &Vec<Attribute>) -> Result<HashMap<String, String>, CryptoError> {
        let mut encoded_attributes: HashMap<String, String> = HashMap::new();
        for i in attributes {
            if i.encode {
                encoded_attributes.insert(i.name.clone(), try!(Issuer::_encode_attribute(&i.value, ByteOrder::Big)));
            }
                else {
                    encoded_attributes.insert(i.name.clone(), i.value.clone());
                }
        }
        Ok(encoded_attributes)
    }

//    fn issue_primary_claim(attributes: &Vec<AttributeType>, u: &BigNum, accumulator_id: &str, user_id: &str) {
//        let mut ctx = BigNumContext::new().unwrap();
//        let vprimeprime = AnoncredsService::generate_vprimeprime();
//        let (mut e_start, mut e_end) = (BigNum::new().unwrap(), BigNum::new().unwrap());
//        e_start.exp(&BigNum::from_u32(2).unwrap(), &BigNum::from_u32(LARGE_E_START as u32).unwrap(), &mut ctx);
//        e_end.exp(&BigNum::from_u32(2).unwrap(), &BigNum::from_u32(LARGE_E_END_RANGE as u32).unwrap(), &mut ctx);
//        e_end = &e_start + &e_end;
//        let e = AnoncredsService::generate_prime_in_range(&e_start, &e_end).unwrap();
//        let encoded_attributes = AnoncredsService::encode_attributes(attributes);
//        let m2 = AnoncredsService::generate_context(accumulator_id, user_id);
//    }

    fn _gen_x(p: &BigNumber, q: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut value = try!(p.mul(&q, None));
        try!(value.sub_word(3));

        let mut result = try!(value.rand_range());
        try!(result.add_word(2));
        Ok(result)
    }

    fn _generate_v_prime_prime() -> Result<BigNumber, CryptoError> {
        let bn = try!(BigNumber::new());
        let a = try!(bn.rand(LARGE_VPRIME_PRIME));
        let mut b = try!(BigNumber::from_u32(2));
        b = try!(b.exp(&try!(BigNumber::from_u32((LARGE_VPRIME_PRIME - 1) as u32)), None));
        let v_prime_prime = try!(bitwise_or_big_int(&a, &b));
        Ok(v_prime_prime)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_attribute_works() {
        let test_str_one = "Alexer5435";
        let test_str_two = "Alexer";
        let test_answer_one = "62794";
        let test_answer_two = "93838255634171043313693932530283701522875554780708470423762684802192372035729";
        assert_eq!(test_answer_one, Issuer::_encode_attribute(test_str_one, ByteOrder::Big).unwrap());
        assert_eq!(test_answer_two, Issuer::_encode_attribute(test_str_two, ByteOrder::Big).unwrap());
    }

    #[test]
    fn encode_attributes_works() {
        assert_eq!(mocks::get_encoded_attributes(), Issuer::_encode_attributes(&mocks::get_attributes()).unwrap());
    }
}

mod mocks {
    use super::*;

    pub fn get_attributes() -> Vec<Attribute> {
        let attributes: Vec<Attribute> = vec![
            Attribute {
                name: "name".to_string(),
                value: "Alex".to_string(),
                encode: true
            },
            Attribute {
                name: "age".to_string(),
                value: "28".to_string(),
                encode: false
            },
            Attribute {
                name: "sex".to_string(),
                value: "male".to_string(),
                encode: true
            },
            Attribute {
                name: "height".to_string(),
                value: "175".to_string(),
                encode: false
            }
        ];
        attributes
    }

    pub fn get_encoded_attributes() -> HashMap<String, String> {
        let mut encoded_attributes: HashMap<String, String> = HashMap::new();
        encoded_attributes.insert("name".to_string(),
                                  "1139481716457488690172217916278103335".to_string());
        encoded_attributes.insert("age".to_string(), "28".to_string());
        encoded_attributes.insert(
            "sex".to_string(),
            "5944657099558967239210949258394887428692050081607692519917050011144233115103".to_string());
        encoded_attributes.insert("height".to_string(), "175".to_string());
        encoded_attributes
    }
}