use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_MASTER_SECRET,
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
    bitwise_or_big_int,
    get_hash_as_int
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
        let p = try!(bn.generate_safe_prime(LARGE_PRIME));
        let q = try!(bn.generate_safe_prime(LARGE_PRIME));

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

    fn _issuer_primary_claim() {

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

    fn _generate_context_attribute(accumulator_id: &String, user_id: &String) -> Result<BigNumber, CryptoError> {
        let accumulator_id_encoded = try!(Issuer::_encode_attribute(&accumulator_id, ByteOrder::Little));
        let user_id_encoded = try!(Issuer::_encode_attribute(&user_id, ByteOrder::Little));
        let mut s = vec![try!(bitwise_or_big_int(
            &try!(BigNumber::from_dec(&accumulator_id_encoded)),
            &try!(BigNumber::from_dec(&user_id_encoded))
        ))];
        let mut h = try!(get_hash_as_int(&mut s));
        let mut pow_2 = try!(BigNumber::from_u32(2));
        pow_2 = try!(pow_2.exp(&try!(BigNumber::from_u32(LARGE_MASTER_SECRET as u32)), None));
        h = try!(h.modulus(&pow_2, None));
        Ok(h)
    }

    fn _sign(public_key: &PublicKey, secret_key: &SecretKey, context_attribute: &BigNumber,
             attributes: &HashMap<String, String>, v: &BigNumber, u: &BigNumber, e: &BigNumber) {
        unimplemented!()
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
    fn encode_attribute_works_short_hash() {
        let test_str = "Alexer5435";
        let test_answer = "62794";
        assert_eq!(test_answer, Issuer::_encode_attribute(test_str, ByteOrder::Big).unwrap());
    }

    #[test]
    fn encode_attribute_works_long_hash() {
        let test_str = "Alexer";
        let test_answer = "93838255634171043313693932530283701522875554780708470423762684802192372035729";
        assert_eq!(test_answer, Issuer::_encode_attribute(test_str, ByteOrder::Big).unwrap());
    }

    #[test]
    fn encode_attributes_works() {
        assert_eq!(mocks::get_encoded_attributes(), Issuer::_encode_attributes(&mocks::get_attributes()).unwrap());
    }

    #[test]
    fn generate_context_attribute_works() {
        let accumulator_id = "110".to_string();
        let user_id = "111".to_string();
        let answer = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let result = Issuer::_generate_context_attribute(&accumulator_id, &user_id).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn sign_works() {
        //let public_key =
        //let secret_key =
        let context_attribute = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let attributes = mocks::get_encoded_attributes();
        let v = BigNumber::from_dec("5237513942984418438429595379849430501110274945835879531523435677101657022026899212054747703201026332785243221088006425007944260107143086435227014329174143861116260506019310628220538205630726081406862023584806749693647480787838708606386447727482772997839699379017499630402117304253212246286800412454159444495341428975660445641214047184934669036997173182682771745932646179140449435510447104436243207291913322964918630514148730337977117021619857409406144166574010735577540583316493841348453073326447018376163876048624924380855323953529434806898415857681702157369526801730845990252958130662749564283838280707026676243727830151176995470125042111348846500489265328810592848939081739036589553697928683006514398844827534478669492201064874941684905413964973517155382540340695991536826170371552446768460042588981089470261358687308").unwrap();
        let u = BigNumber::from_dec("72637991796589957272144423539998982864769854130438387485781642285237707120228376409769221961371420625002149758076600738245408098270501483395353213773728601101770725294535792756351646443825391806535296461087756781710547778467803194521965309091287301376623972321639262276779134586366620773325502044026364814032821517244814909708610356590687571152567177116075706850536899272749781370266769562695357044719529245223811232258752001942940813585440938291877640445002571323841625932424781535818087233087621479695522263178206089952437764196471098717335358765920438275944490561172307673744212256272352897964947435086824617146019").unwrap();
        let e = BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930214202955935602153431795703076242907").unwrap();
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