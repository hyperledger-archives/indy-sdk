extern crate base64;

use errors::common::CommonError;

pub fn encode(doc: &[u8]) -> String {
    base64::encode(doc)
}

pub fn decode(doc: &str) -> Result<Vec<u8>, CommonError> {
    base64::decode(doc)
        .map_err(|err| CommonError::InvalidStructure(format!("{}", err)))
}

pub fn encode_urlsafe(doc: &[u8]) -> String {
    base64::encode_config(doc, base64::URL_SAFE)
}

pub fn decode_urlsafe(doc: &str) -> Result<Vec<u8>, CommonError> {
    base64::decode_config(doc, base64::URL_SAFE)
        .map_err(|err| CommonError::InvalidStructure(format!("{}", err)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let result = encode(&[1, 2, 3]);
        assert_eq!("Ldp", &result, "Got unexpected data");
    }

    #[test]
    fn decode_works() {
        let result = decode("Ldp");

        assert!(result.is_ok(), "Got error");
        assert_eq!(&[1, 2, 3], &result.unwrap()[..], "Get unexpected data");
    }

    #[test]
    fn encode_urlsafe_works() {
        let result = encode_urlsafe(&[1, 2, 3]);
        assert_eq!("AQID", &result);
    }

    #[test]
    fn decode_urlsafe_works() {
        let result = decode_urlsafe("AQID");

        assert!(result.is_ok(), "Got error");
        assert_eq!(&[1, 2, 3], &result.unwrap()[..]);
    }
}