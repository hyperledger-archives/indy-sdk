extern crate serde_json;

use self::serde_json::Value;
use errors::crypto::CryptoError;

pub fn serialize_signature(string: &str) -> Result<String, CryptoError> {
    let v: Value = serde_json::from_str(string)?;
    Ok(_serialize(v))
}

fn _serialize(v: Value) -> String {
    match v {
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value,
        Value::Array(array) => {
            let mut result = "".to_string();
            let length = array.len();
            for (index, element) in array.iter().enumerate() {
                result += &_serialize(element.clone());
                if index < length - 1 {
                    result += ",";
                }
            }
            result
        },
        Value::Object(map) => {
            let mut result = "".to_string();
            let length = map.len();
            for (index, key) in map.keys().enumerate() {
                result = result + key + ":" + &_serialize(map[key].clone());
                if index < length - 1 {
                    result += "|";
                }
            }
            result
        },
        _ => "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_serialize_works() {
        let data = "{
                        \"name\": \"John Doe\",
                        \"age\": 43,
                        \"operation\": {
                            \"hash\": \"cool hash\",
                            \"dest\": 54
                        },
                        \"phones\": [
                          \"1234567\",
                          \"2345678\",
                          {\"rust\": 5, \"age\": 1},
                          3
                        ]
                    }";
        let result = "age:43|name:John Doe|operation:dest:54|hash:cool hash|phones:1234567,2345678,age:1|rust:5,3";

        assert_eq!(serialize_signature(data).unwrap(), result)
    }
}