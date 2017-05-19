extern crate serde_json;

use self::serde_json::Value;

use errors::crypto::CryptoError;
use utils::crypto::hash::Hash;
use utils::crypto::base58::Base58;

pub fn serialize_signature(v: Value) -> Result<String, CryptoError> {
    match v {
        Value::Bool(value) => Ok(value.to_string()),
        Value::Number(value) => Ok(value.to_string()),
        Value::String(value) => Ok(value),
        Value::Array(array) => {
            let mut result = "".to_string();
            let length = array.len();
            for (index, element) in array.iter().enumerate() {
                result += &serialize_signature(element.clone())?;
                if index < length - 1 {
                    result += ",";
                }
            }
            Ok(result)
        },
        Value::Object(map) => {
            let mut result = "".to_string();
            let length = map.len();
            for (index, key) in map.keys().enumerate() {
                let mut value = map[key].clone();
                if key == "raw" {
                    let mut ctx = Hash::new_context()?;
                    ctx.update(&value.as_str().ok_or(CryptoError::BackendError("Cannot update hash context".to_string()))?.as_bytes())?;
                    let vector = Base58::encode(&ctx.finish2()?.to_vec());
                    value = Value::String(vector);
                }
                result = result + key + ":" + &serialize_signature(value)?;
                if index < length - 1 {
                    result += "|";
                }
            }
            Ok(result)
        },
        _ => Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_serialize_works() {
        let data = r#"{
                        "name": "John Doe",
                        "age": 43,
                        "operation": {
                            "hash": "cool hash",
                            "dest": 54
                        },
                        "phones": [
                          "1234567",
                          "2345678",
                          {"rust": 5, "age": 1},
                          3
                        ]
                    }"#;
        let msg: Value = serde_json::from_str(data).unwrap();

        let result = "age:43|name:John Doe|operation:dest:54|hash:cool hash|phones:1234567,2345678,age:1|rust:5,3";

        assert_eq!(serialize_signature(msg).unwrap(), result)
    }

    #[test]
    fn signature_serialize_works_with_raw() {
        let data = r#"{
                        "name": "John Doe",
                        "age": 43,
                        "operation": {
                            "hash": "cool hash",
                            "dest": 54,
                            "raw": "string for hash"
                        },
                        "phones": [
                          "1234567",
                          "2345678",
                          {"rust": 5, "age": 1},
                          3
                        ]
                    }"#;
        let msg: Value = serde_json::from_str(data).unwrap();

        let result = "age:43|name:John Doe|operation:dest:54|hash:cool hash|raw:31L9oDUaMfZu3t3eCWuoG6PbWigVMqHqrWuS8Ly9oH4t|phones:1234567,2345678,age:1|rust:5,3";

        assert_eq!(serialize_signature(msg).unwrap(), result)
    }
}