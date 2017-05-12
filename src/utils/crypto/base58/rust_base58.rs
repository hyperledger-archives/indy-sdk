extern crate rust_base58;

use errors::crypto::CryptoError;

use self::rust_base58::{ToBase58, FromBase58};

pub struct Base58 {}

impl Base58 {
    pub fn encode(doc: &[u8]) -> String {
        doc.to_base58()
    }

    pub fn decode(doc: &str) -> Result<Vec<u8>, CryptoError> {
        doc.from_base58()
            .map_err(|err| CryptoError::InvalidStructure(format!("{}", err)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let result = Base58::encode(&[1, 2, 3]);
        assert_eq!("Ldp", &result, "Got unexpected data");
    }

    #[test]
    fn decode_works() {
        let result = Base58::decode("Ldp");

        assert!(result.is_ok(), "Got error");
        assert_eq!(&[1, 2, 3], &result.unwrap()[..], "Get unexpected data");
    }
}