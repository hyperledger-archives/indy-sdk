extern crate rand;
extern crate milagro_crypto;

use self::milagro_crypto::hash::wrappers::hash256;
use std::cmp::max;

use services::crypto::wrappers::bn::BigNumber;
use errors::crypto::CryptoError;

pub fn random_qr(n: &BigNumber) -> Result<BigNumber, CryptoError> {
    let mut random = try!(n.rand_range());
    random = try!(random.sqr(None));
    random = try!(random.modulus(&n, None));
    Ok(random)
}

pub fn bitwise_or_big_int(a: &BigNumber, b: &BigNumber) -> Result<BigNumber, CryptoError> {
    let significant_bits = max(try!(a.num_bits()), try!(b.num_bits()));
    let mut result = try!(BigNumber::new());
    for i in 0..significant_bits {
        if try!(a.is_bit_set(i)) || try!(b.is_bit_set(i)) {
            try!(result.set_bit(i));
        }
    }
    Ok(result)
}

pub fn get_hash_as_int(nums: &mut Vec<BigNumber>) -> Result<BigNumber, CryptoError> {
    let mut sha256: hash256 = hash256::new();

    nums.sort();

    for num in nums.iter() {
        let array_bytes: Vec<u8> = try!(num.to_bytes());

        let index = array_bytes.iter().position(|&value| value != 0).unwrap_or(array_bytes.len());

        for byte in array_bytes[index..].iter() {
            sha256.process(*byte);
        }
    }

    let mut hashed_array: Vec<u8> =
        sha256.hash().iter()
            .map(|v| *v as u8)
            .collect();

    hashed_array.reverse();

    BigNumber::from_bytes(&hashed_array[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_or_big_int_works () {
        let a = BigNumber::from_dec("778378032744961463933002553964902776831187587689736807008034459507677878432383414623740074");
        let b = BigNumber::from_dec("1018517988167243043134222844204689080525734196832968125318070224677190649881668353091698688");
        let result = BigNumber::from_dec("1796896020912204507067225398169591857356921784522704932326104684184868528314051767715438762");
        assert_eq!(result.unwrap(), bitwise_or_big_int(&a.unwrap(), &b.unwrap()).unwrap());
    }

    #[test]
    fn get_hash_as_in_works() {
        let mut nums = vec![
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa94d3fdf6abfbff").unwrap(),
            BigNumber::from_hex("ff9d2eedfee9cffd9ef6dbffedff3fcbef4caecb9bffe79bfa9168615ccbc546").unwrap()
        ];
        let res = get_hash_as_int(&mut nums);

        assert!(res.is_ok());
        assert_eq!("9E2A0653691B96A9B55B3D1133F9FEE2F2C37B848DBADF2F70DFFFE9E47C5A5D", res.unwrap().to_hex().unwrap());
    }
}