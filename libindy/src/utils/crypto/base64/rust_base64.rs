extern crate base64;

use errors::prelude::*;
use failure::ResultExt;

pub fn encode(doc: &[u8]) -> String {
    base64::encode(doc)
}

pub fn decode(doc: &str) -> Result<Vec<u8>, IndyError> {
    base64::decode(doc)
        .context("Invalid base64 sequence")
        .context(IndyErrorKind::InvalidStructure)
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let result = encode(&[1, 2, 3]);
        assert_eq!("AQID", &result);
    }

    #[test]
    fn decode_works() {
        let result = decode("AQID");

        assert!(result.is_ok(), "Got error");
        assert_eq!(&[1, 2, 3], &result.unwrap()[..]);
    }
}