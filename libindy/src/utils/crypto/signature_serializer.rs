extern crate serde_json;
extern crate hex;

use self::hex::ToHex;
use self::serde_json::Value;

use errors::common::CommonError;
use utils::crypto::hash::Hash;

pub fn serialize_signature(v: Value) -> Result<String, CommonError> {
    match v {
        Value::Bool(value) => Ok(if value { "True".to_string() } else { "False".to_string() }),
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
        }
        Value::Object(map) => {
            let mut result = "".to_string();
            let length = map.len();
            for (index, key) in map.keys().enumerate() {
                let mut value = map[key].clone();
                if key == "raw" || key == "hash" || key == "enc" {
                    let mut ctx = Hash::new_context()?;
                    ctx.update(&value.as_str().ok_or(CommonError::InvalidState("Cannot update hash context".to_string()))?.as_bytes())?;
                    value = Value::String(ctx.finish2()?.as_ref().to_hex());
                }
                result = result + key + ":" + &serialize_signature(value)?;
                if index < length - 1 {
                    result += "|";
                }
            }
            Ok(result)
        }
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

        let result = "age:43|name:John Doe|operation:dest:54|hash:46aa0c92129b33ee72ee1478d2ae62fa6e756869dedc6c858af3214a6fcf1904|phones:1234567,2345678,age:1|rust:5,3";

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

        let result = "age:43|name:John Doe|operation:dest:54|hash:46aa0c92129b33ee72ee1478d2ae62fa6e756869dedc6c858af3214a6fcf1904|raw:1dcd0759ce38f57049344a6b3c5fc18144fca1724713090c2ceeffa788c02711|phones:1234567,2345678,age:1|rust:5,3";

        assert_eq!(serialize_signature(msg).unwrap(), result)
    }

    #[test]
    #[ignore] /* FIXME implement ignoring signature field as in python code */
    fn signature_serialize_works_with_null() {
        let data = r#"{"signature": null}"#;
        let v: serde_json::Value = serde_json::from_str(data).unwrap();
        let serialized = serialize_signature(v).unwrap();
        println!("{:?}", serialized);
        assert_eq!(serialized, "");
    }
}