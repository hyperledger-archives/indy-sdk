extern crate openssl;
use self::openssl::sha::sha256;
use self::openssl::bn::BigNum;
use utils::error::BIG_NUMBER_ERROR;
use error::prelude::*;

pub fn encode(s: &str ) -> VcxResult<String> {
    match s.parse::<u32>() {
        Ok(_) => Ok(s.to_string()),
        Err(_) => {
            let hash = sha256(s.as_bytes());
            let bignum = match BigNum::from_slice(&hash) {
                Ok(b) => b,
                Err(err) => {
                    warn!("{}", BIG_NUMBER_ERROR.message);
                    return Err(VcxError::from_msg(VcxErrorKind::EncodeError, format!("Cannot encode string: {}", err)))
                }
            };
            match bignum.to_dec_str() {
                Ok(s) => Ok(s.to_string()),
                Err(err) => {
                    warn!("{}", BIG_NUMBER_ERROR.message);
                    return Err(VcxError::from_msg(VcxErrorKind::EncodeError, format!("Cannot encode string: {}", err)))
                }
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use std::str;
    #[test]
    fn test_encoding(){
        let big_integer_as_string = "32770349619296211525721019403974704547883091481854305319049714074652726739013";
        let big_number_from_dec_str = BigNum::from_dec_str(big_integer_as_string).unwrap();
        let cat = b"Cat";
        let cat_string = match str::from_utf8(cat){
            Ok(v) => v,
            Err(e) => panic!("Invliad UTF 8 {}", e),
        };
        let hash = sha256(cat);
        let bignum_hash = BigNum::from_slice(&hash).unwrap();
        assert_eq!(BigNum::from_slice(cat).unwrap(),BigNum::from_u32(4415860).unwrap());
        // this value was derived from an outside tool
        // https://www.mobilefish.com/services/big_number/big_number.php
        // with the bignum_hash value as the input.
        let hex_str = "48735C4FAE42D1501164976AFEC76730B9E5FE467F680BDD8DAFF4BB77674045";
        let bignum_from_hex = BigNum::from_hex_str(hex_str).unwrap();
        assert_eq!(bignum_from_hex, bignum_hash);
        assert_eq!(bignum_from_hex, big_number_from_dec_str);
        let encoded_str = encode(&cat_string).unwrap();
        assert_eq!(encoded_str, big_integer_as_string);

        let number_as_string = "123";
        assert_eq!(number_as_string, encode(number_as_string).unwrap());
    }



}