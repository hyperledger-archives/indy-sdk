extern crate base64;

use errors::common::CommonError;
use errors::route::RouteError;

pub fn encode(doc: &[u8]) -> String {
    base64::encode_config(doc, base64::URL_SAFE)
}

pub fn decode(doc: &str) -> Result<Vec<u8>, CommonError> {
    base64::decode_config(doc, base64::URL_SAFE)
        .map_err(|err| CommonError::InvalidStructure(format!("{}", err)))
}

pub fn decode_to_string(doc: &str) -> Result<String, RouteError> {
    let bytes = base64::decode(doc)
        .map_err(|err| RouteError::DecodeError(format!("{}", err)));
    String::from_utf8(bytes?)
        .map_err(|err| RouteError::DecodeError(format!("{}", err)))
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

    #[test]
    fn decode_to_string_works() {
        let result = decode_to_string("dGVzdF9zdHJpbmc=");
        assert_eq!(result.unwrap(), "test_string".to_string());
    }
}