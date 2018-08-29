extern crate base64;

use errors::common::CommonError;

pub fn encode(doc: &[u8]) -> String {
    base64::encode_config(doc, base64::URL_SAFE)
}

pub fn decode(doc: &str) -> Result<Vec<u8>, CommonError> {
    base64::decode(doc)
        .map_err(|err| CommonError::InvalidStructure(format!("{}", err)))
}

pub fn decode_to_string(doc: &str) -> String {
    let bytes = base64::decode(doc);
    match bytes {
        Ok(v) => match String::from_utf8(v) {
            Ok(decoded_str) => decoded_str,
            Err(e) => e.to_string()
        },
        Err(err) => err.to_string()
    }
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

    fn decode_to_string_works() {
        let result = decode_to_string("dGVzdF9zdHJpbmc=");
        assert_eq!(result, "test_string".to_string());
    }
}