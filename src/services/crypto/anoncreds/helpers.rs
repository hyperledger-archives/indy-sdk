extern crate rand;
extern crate milagro_crypto;
extern crate openssl;

use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::LARGE_MVECT;
use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::GroupOrderElement;
use std::hash::Hash;
use std::cmp::max;
use std::collections::{HashMap, HashSet};


pub fn random_qr(n: &BigNumber) -> Result<BigNumber, CryptoError> {
    let random = n
        .rand_range()?
        .sqr(None)?
        .modulus(&n, None)?;
    Ok(random)
}

pub fn bitwise_or_big_int(a: &BigNumber, b: &BigNumber) -> Result<BigNumber, CryptoError> {
    let significant_bits = max(a.num_bits()?, b.num_bits()?);
    let mut result = BigNumber::new()?;
    for i in 0..significant_bits {
        if a.is_bit_set(i)? || b.is_bit_set(i)? {
            result.set_bit(i)?;
        }
    }
    Ok(result)
}

pub fn transform_u32_to_array_of_u8(x:u32) -> Vec<u8> {
    let mut result:  Vec<u8> = vec![0; 28];
    for i in (0..4).rev() {
        let shift = i * 8;
        let b = (x >> shift) as u8;
        result.push(b);
    }
    result
}

pub fn get_hash_as_int(nums: &mut Vec<Vec<u8>>) -> Result<BigNumber, CryptoError> {
    nums.sort();

    let mut hashed_array: Vec<u8> = BigNumber::hash_array(&nums)?;
    hashed_array.reverse();

    BigNumber::from_bytes(&hashed_array[..])
}

pub fn split_revealed_attrs(encoded_attrs: &HashMap<String, BigNumber>, revealed_ttrs: &HashSet<String>)
                            -> Result<(HashMap<String, BigNumber>, HashMap<String, BigNumber>), CryptoError> {
    let mut ar: HashMap<String, BigNumber> = HashMap::new();
    let mut aur: HashMap<String, BigNumber> = HashMap::new();

    for (attr, value) in encoded_attrs.iter() {
        if revealed_ttrs.contains(attr) {
            ar.insert(attr.clone(), value.clone()?);
        } else {
            aur.insert(attr.clone(), value.clone()?);
        }
    }
    Ok((ar, aur))
}

pub fn get_mtilde(unrevealed_attrs: &HashMap<String, BigNumber>)
                  -> Result<HashMap<String, BigNumber>, CryptoError> {
    let mut mtilde: HashMap<String, BigNumber> = HashMap::new();

    for (attr, _) in unrevealed_attrs.iter() {
        mtilde.insert(attr.clone(), BigNumber::rand(LARGE_MVECT)?);
    }
    Ok(mtilde)
}

fn largest_square_less_than(delta: i64) -> i64 {
    (delta as f64).sqrt().floor() as i64
}

pub fn four_squares(delta: i64) -> Result<HashMap<String, BigNumber>, CryptoError> {
    let u1 = largest_square_less_than(delta);
    let u2 = largest_square_less_than(delta - u1.pow(2));
    let u3 = largest_square_less_than(delta - u1.pow(2) - u2.pow(2));
    let u4 = largest_square_less_than(delta - u1.pow(2) - u2.pow(2) - u3.pow(2));

    if u1.pow(2) + u2.pow(2) + u3.pow(2) + u4.pow(2) == delta {
        let mut res: HashMap<String, BigNumber> = HashMap::new();
        res.insert("0".to_string(), BigNumber::from_dec(&u1.to_string()[..])?);
        res.insert("1".to_string(), BigNumber::from_dec(&u2.to_string()[..])?);
        res.insert("2".to_string(), BigNumber::from_dec(&u3.to_string()[..])?);
        res.insert("3".to_string(), BigNumber::from_dec(&u4.to_string()[..])?);

        Ok(res)
    } else {
        Err(CryptoError::InvalidStructure(format!("Cannot get the four squares for delta {} ", delta)))
    }
}

pub trait BytesView {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError>;
}


pub trait AppendByteArray {
    fn append_vec<T: BytesView>(&mut self, other: &Vec<T>) -> Result<(), CryptoError>;
}

impl AppendByteArray for Vec<Vec<u8>> {
    fn append_vec<T: BytesView>(&mut self, other: &Vec<T>) -> Result<(), CryptoError> {
        for el in other.iter() {
            self.push(el.to_bytes()?);
        }
        Ok(())
    }
}

pub trait AppendBigNumArray {
    fn append_vec(&mut self, other: &Vec<BigNumber>) -> Result<(), CryptoError>;
}

impl AppendBigNumArray for Vec<BigNumber> {
    fn append_vec(&mut self, other: &Vec<BigNumber>) -> Result<(), CryptoError> {
        for el in other.iter() {
            self.push(el.clone()?);
        }
        Ok(())
    }
}

pub fn clone_bignum_map<K: Clone + Eq + Hash>(other: &HashMap<K, BigNumber>)
                                              -> Result<HashMap<K, BigNumber>, CryptoError> {
    let mut res: HashMap<K, BigNumber> = HashMap::new();
    for (k, v) in other {
        res.insert(k.clone(), v.clone()?);
    }
    Ok(res)
}

pub fn group_element_to_bignum(el: &GroupOrderElement) -> Result<BigNumber, CryptoError> {
    Ok(BigNumber::from_bytes(&el.to_bytes()?)?)
}

pub fn bignum_to_group_element(num: &BigNumber) -> Result<GroupOrderElement, CryptoError> {
    Ok(GroupOrderElement::from_bytes(&num.to_bytes()?)?)
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
        let res = four_squares(10 as i64);

        assert!(res.is_ok());
        let res_data = res.unwrap();

        assert_eq!("3".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("1".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("0".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("0".to_string(), res_data.get("3").unwrap().to_dec().unwrap());
    }

    #[test]
    fn split_revealed_attrs_works() {
        let mut encoded_attrs: HashMap<String, BigNumber> = HashMap::new();
        encoded_attrs.insert("name".to_string(), BigNumber::from_dec("1").unwrap());
        encoded_attrs.insert("age".to_string(), BigNumber::from_dec("1").unwrap());
        encoded_attrs.insert("sex".to_string(), BigNumber::from_dec("1").unwrap());

        let revealed_attrs = ::services::crypto::anoncreds::prover::mocks::get_revealed_attrs();

        let res = split_revealed_attrs(&encoded_attrs, &revealed_attrs);

        assert!(res.is_ok());

        let (revealed, unrevealed) = res.unwrap();

        assert_eq!(1, revealed.len());
        assert_eq!(2, unrevealed.len());
        assert!(revealed.contains_key("name"));
        assert!(unrevealed.contains_key("sex"));
        assert!(unrevealed.contains_key("age"));
    }

    #[test]
    fn transform_u32_to_array_of_u8_works() {
        let int = 1958376517;
        let answer = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 186, 116, 69];
        assert_eq!(transform_u32_to_array_of_u8(int), answer)
    }
}