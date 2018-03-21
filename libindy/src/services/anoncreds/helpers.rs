extern crate rand;
extern crate openssl;
extern crate indy_crypto;

use errors::common::CommonError;

use services::anoncreds::constants::{LARGE_MVECT, LINK_SECRET_NAME, POLICY_ADDRESS_NAME};
use utils::crypto::bn::BigNumber;
use std::hash::Hash;
use std::cmp::max;
use std::collections::HashMap;
use self::indy_crypto::pair::{GroupOrderElement, PointG1, Pair};
use self::indy_crypto::cl::NonCredentialSchemaElements;
use self::rand::Rng;


#[cfg(not(test))]
pub fn random_qr(n: &BigNumber) -> Result<BigNumber, CommonError> {
    let random = n
        .rand_range()?
        .sqr(None)?
        .modulus(&n, None)?;
    Ok(random)
}

#[cfg(test)]
pub fn random_qr(n: &BigNumber) -> Result<BigNumber, CommonError> {
    Ok(BigNumber::from_dec("64684820421150545443421261645532741305438158267230326415141505826951816460650437611148133267480407958360035501128469885271549378871140475869904030424615175830170939416512594291641188403335834762737251794282186335118831803135149622404791467775422384378569231649224208728902565541796896860352464500717052768431523703881746487372385032277847026560711719065512366600220045978358915680277126661923892187090579302197390903902744925313826817940566429968987709582805451008234648959429651259809188953915675063700676546393568304468609062443048457324721450190021552656280473128156273976008799243162970386898307404395608179975243")?)
}

pub fn bitwise_or_big_int(a: &BigNumber, b: &BigNumber) -> Result<BigNumber, CommonError> {
    let significant_bits = max(a.num_bits()?, b.num_bits()?);
    let mut result = BigNumber::new()?;
    for i in 0..significant_bits {
        if a.is_bit_set(i)? || b.is_bit_set(i)? {
            result.set_bit(i)?;
        }
    }
    Ok(result)
}

//Byte order: Little
pub fn transform_u32_to_array_of_u8(x: u32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for i in (0..4).rev() {
        result.push((x >> i * 8) as u8);
    }
    result
}

pub fn get_hash_as_int(nums: &mut Vec<Vec<u8>>) -> Result<BigNumber, CommonError> {
    nums.sort();

    let mut hashed_array: Vec<u8> = BigNumber::hash_array(&nums)?;
    hashed_array.reverse();

    BigNumber::from_bytes(&hashed_array[..])
}

pub fn get_mtilde(unrevealed_attrs: &Vec<String>)
                  -> Result<HashMap<String, BigNumber>, CommonError> {
    let mut mtilde: HashMap<String, BigNumber> = HashMap::new();

    for attr in unrevealed_attrs.iter() {
        mtilde.insert(attr.clone(), BigNumber::rand(LARGE_MVECT)?);
    }
    Ok(mtilde)
}

fn largest_square_less_than(delta: usize) -> usize {
    (delta as f64).sqrt().floor() as usize
}

pub fn four_squares(delta: i32) -> Result<HashMap<String, BigNumber>, CommonError> {
    if delta < 0 {
        return Err(CommonError::InvalidStructure(format!("Cannot get the four squares for delta {} ", delta)));
    }

    let d = delta as usize;
    let mut roots: [usize; 4] = [largest_square_less_than(d), 0, 0, 0];

    'outer: for i in (1 .. roots[0] + 1).rev() {
        roots[0] = i;
        if d == roots[0].pow(2) {
            roots[1] = 0;
            roots[2] = 0;
            roots[3] = 0;
            break 'outer;
        }
        roots[1] = largest_square_less_than(d - roots[0].pow(2));
        for j in (1 .. roots[1] + 1).rev() {
            roots[1] = j;
            if d == roots[0].pow(2) + roots[1].pow(2) {
                roots[2] = 0;
                roots[3] = 0;
                break 'outer;
            }
            roots[2] = largest_square_less_than(d - roots[0].pow(2) - roots[1].pow(2));
            for k in (1 .. roots[2] + 1).rev() {
                roots[2] = k;
                if d == roots[0].pow(2) + roots[1].pow(2) + roots[2].pow(2) {
                    roots[3] = 0;
                    break 'outer;
                }
                roots[3] = largest_square_less_than(d - roots[0].pow(2) - roots[1].pow(2) - roots[2].pow(2));
                if d == roots[0].pow(2) + roots[1].pow(2) + roots[2].pow(2) + roots[3].pow(2) {
                    break 'outer;
                }
            }
        }
    }

    let mut res: HashMap<String, BigNumber> = HashMap::new();
    res.insert("0".to_string(), BigNumber::from_dec(&roots[0].to_string()[..])?);
    res.insert("1".to_string(), BigNumber::from_dec(&roots[1].to_string()[..])?);
    res.insert("2".to_string(), BigNumber::from_dec(&roots[2].to_string()[..])?);
    res.insert("3".to_string(), BigNumber::from_dec(&roots[3].to_string()[..])?);

    Ok(res)
}


pub trait BytesView {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError>;
}

impl BytesView for PointG1 {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for GroupOrderElement {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        Ok(self.to_bytes()?)
    }
}

impl BytesView for Pair {
    fn to_bytes(&self) -> Result<Vec<u8>, CommonError> {
        Ok(self.to_bytes()?)
    }
}

pub trait AppendByteArray {
    fn append_vec<T: BytesView>(&mut self, other: &Vec<T>) -> Result<(), CommonError>;
}

impl AppendByteArray for Vec<Vec<u8>> {
    fn append_vec<T: BytesView>(&mut self, other: &Vec<T>) -> Result<(), CommonError> {
        for el in other.iter() {
            self.push(el.to_bytes()?);
        }
        Ok(())
    }
}

pub fn clone_bignum_map<K: Clone + Eq + Hash>(other: &HashMap<K, BigNumber>)
                                              -> Result<HashMap<K, BigNumber>, CommonError> {
    let mut res: HashMap<K, BigNumber> = HashMap::new();
    for (k, v) in other {
        res.insert(k.clone(), v.clone()?);
    }
    Ok(res)
}

pub fn group_element_to_bignum(el: &GroupOrderElement) -> Result<BigNumber, CommonError> {
    Ok(BigNumber::from_bytes(&el.to_bytes()?)?)
}

pub fn bignum_to_group_element(num: &BigNumber) -> Result<GroupOrderElement, CommonError> {
    Ok(GroupOrderElement::from_bytes(&num.to_bytes()?)?)
}

pub fn get_composite_id(issuer_did: &str, schema_seq_no: i32) -> String {
    issuer_did.to_string() + ":" + &schema_seq_no.to_string()
}

pub fn get_non_schema_elements() -> NonCredentialSchemaElements {
    let mut set = ::std::collections::BTreeSet::new();
    set.insert(String::from(LINK_SECRET_NAME));
    set.insert(String::from(POLICY_ADDRESS_NAME));
    NonCredentialSchemaElements {
        attrs: set
    }
}

pub fn get_random_string() -> String {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range(b'A', b'Z') as char;
    let number: u32 = rng.gen_range(0, 9999999999);
    format!("{}{:10}", letter, number)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_or_big_int_works() {
        let a = BigNumber::from_dec("778378032744961463933002553964902776831187587689736807008034459507677878432383414623740074");
        let b = BigNumber::from_dec("1018517988167243043134222844204689080525734196832968125318070224677190649881668353091698688");
        let result = BigNumber::from_dec("1796896020912204507067225398169591857356921784522704932326104684184868528314051767715438762");
        assert_eq!(result.unwrap(), bitwise_or_big_int(&a.unwrap(), &b.unwrap()).unwrap());
    }

    #[test]
    fn get_hash_as_int_works() {
        let mut nums = vec![
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa94d3fdf6abfbff").unwrap().to_bytes().unwrap(),
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa9168615ccbc546").unwrap().to_bytes().unwrap()
        ];
        let res = get_hash_as_int(&mut nums);

        assert!(res.is_ok());
        assert_eq!("9E2A0653691B96A9B55B3D1133F9FEE2F2C37B848DBADF2F70DFFFE9E47C5A5D", res.unwrap().to_hex().unwrap());
    }

    #[test]
    fn four_squares_works() {
        let res = four_squares(107 as i32);
        let res_data = res.unwrap();

        assert_eq!("9".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("5".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("1".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("0".to_string(), res_data.get("3").unwrap().to_dec().unwrap());

        let res = four_squares(112 as i32);
        let res_data = res.unwrap();

        assert_eq!("10".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("2".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("2".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("2".to_string(), res_data.get("3").unwrap().to_dec().unwrap());


        let res = four_squares(253 as i32);
        let res_data = res.unwrap();

        assert_eq!("14".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("7".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("2".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("2".to_string(), res_data.get("3").unwrap().to_dec().unwrap());

        let res = four_squares(1506099439 as i32);
        let res_data = res.unwrap();

        assert_eq!("38807".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("337".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("50".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("11".to_string(), res_data.get("3").unwrap().to_dec().unwrap());
    }

    #[test]
    fn transform_u32_to_array_of_u8_works() {
        let int = 0x74BA7445;
        let answer = vec![0x74, 0xBA, 0x74, 0x45];
        assert_eq!(transform_u32_to_array_of_u8(int), answer)
    }
}