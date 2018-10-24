extern crate serde_json;

use ::utils::error;
use serde_json::{Value};

pub fn get_value_to_string(key: &str, map: &Value) -> Result<String, u32> {
    Ok(
        map.get(key)
            .ok_or(error::INVALID_JSON.code_num)?
            .as_str()
            .ok_or(error::INVALID_JSON.code_num)?
            .to_string()
    )
}

