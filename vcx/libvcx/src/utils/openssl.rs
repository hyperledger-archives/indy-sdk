use openssl::sha::sha256;
use openssl::bn::BigNum;
use error::prelude::*;

pub fn encode(s: &str) -> VcxResult<String> {
    match s.parse::<u32>() {
        Ok(val) => Ok(val.to_string()),
        Err(_) => {
            let hash = sha256(s.as_bytes());
            let bignum = BigNum::from_slice(&hash)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::EncodeError, format!("Cannot encode string: {}", err)))?;

            let encoded = bignum.to_dec_str()
                .map_err(|err| VcxError::from_msg(VcxErrorKind::EncodeError, format!("Cannot encode string: {}", err)))?
                .to_string();

            Ok(encoded)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encoding() {
        // number
        {
            let value = "1234";
            let expected_value = value;

            let encoded_value = encode(value).unwrap();
            assert_eq!(expected_value, encoded_value);
        }

        // number with leading zero
        {
            let value = "01234";
            let expected_value = "1234";

            let encoded_value = encode(value).unwrap();
            assert_eq!(expected_value, encoded_value);
        }

        // string
        {
            let value = "Cat";
            let expected_value = "32770349619296211525721019403974704547883091481854305319049714074652726739013";

            let encoded_value = encode(value).unwrap();
            assert_eq!(expected_value, encoded_value);
        }
    }
}