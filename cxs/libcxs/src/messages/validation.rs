extern crate rust_base58;
extern crate openssl;

use self::openssl::bn::{ BigNum };
use self::rust_base58::{FromBase58};
use utils::error;
use url::Url;

pub fn validate_did(did: &str) -> Result<String, u32> {
    //    assert len(base58.b58decode(did)) == 16
    let check_did = String::from(did);
    match check_did.from_base58() {
        Ok(ref x) if x.len() == 16 => Ok(check_did),
        Ok(_) => Err(error::INVALID_DID.code_num),
        Err(x) => Err(error::NOT_BASE58.code_num)
    }
}

pub fn validate_verkey(verkey: &str) -> Result<String, u32> {
    //    assert len(base58.b58decode(ver_key)) == 32
    let check_verkey = String::from(verkey);
    match check_verkey.from_base58() {
        Ok(ref x) if x.len() == 32 => Ok(check_verkey),
        Ok(_) => Err(error::INVALID_VERKEY.code_num),
        Err(x) => Err(error::NOT_BASE58.code_num)
    }
}

pub fn validate_nonce(nonce: &str) -> Result<String, u32> {
    match BigNum::from_dec_str(nonce) {
        Ok(x) => {
            if x.num_bits() > 80 {
                return Err(error::INVALID_NONCE.code_num)
            }
            Ok(nonce.to_string())
        },
        Err(_) => Err(error::INVALID_NONCE.code_num),
    }
}

pub fn validate_key_delegate(delegate: &str) -> Result<String, u32> {
    //todo: find out what needs to be validated for key_delegate
    let check_delegate = String::from(delegate);
    Ok(check_delegate)
}

pub fn validate_url(url: &str)->Result<String, u32>{
    match Url::parse(url) {
        Ok(_) => Ok(url.to_string()),
        Err(x) => Err(error::INVALID_URL.code_num),
    }
}

pub fn validate_phone_number(p_num: &str)->Result<String, u32>{
    Ok(String::from(p_num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_is_b58_and_valid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => panic!("Should be valid did"),
            Ok(x) => assert_eq!(x, to_did.to_string())

        }
    }

    #[test]
    fn test_did_is_b58_but_invalid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoT";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num),
            Ok(x) => panic!("Should be invalid did"),

        }
    }

    #[test]
    fn test_validate_did_with_non_base58() {
        let to_did = "8*Fh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, error::NOT_BASE58.code_num),
            Ok(x) => panic!("Should be invalid did"),
        }
    }

    #[test]
    fn test_verkey_is_b58_and_valid_length() {
        let verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        match validate_verkey(&verkey) {
            Err(x) => panic!("Should be valid verkey"),
            Ok(x) => assert_eq!(x, verkey)

        }
    }

    #[test]
    fn test_verkey_is_b58_but_invalid_length() {
        let verkey = "8XFh8yBzrpJQmNyZzgoT";
        match validate_verkey(&verkey) {
            Err(x) => assert_eq!(x, error::INVALID_VERKEY.code_num),
            Ok(x) => panic!("Should be invalid verkey"),

        }
    }

    #[test]
    fn test_validate_verkey_with_non_base58() {
        let verkey = "*kVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        match validate_verkey(&verkey) {
            Err(x) => assert_eq!(x, error::NOT_BASE58.code_num),
            Ok(x) => panic!("Should be invalid verkey"),
        }
    }


}