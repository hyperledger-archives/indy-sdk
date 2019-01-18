extern crate rmp_serde;

use std::error::Error;


#[derive(Serialize, Deserialize, Debug)]
pub struct ComboBox {
    pub msg: String,
    pub sender: String,
    pub nonce: String
}

impl ComboBox {
    pub fn to_msg_pack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::encode::to_vec_named(self)
    }

    pub fn from_msg_pack(bytes: &[u8]) -> Result<ComboBox, rmp_serde::decode::Error> {
        rmp_serde::decode::from_slice(bytes)
    }

    pub fn to_base64(&self) -> Result<String, serde_json::Error> {
        let combo_box_str = serde_json::to_string(self)?;
        Ok(base64::encode(combo_box_str.as_bytes()))
    }

    pub fn from_base64(byte_array: Vec<u8>) -> Result<ComboBox, Box<Error>> {
        let combo_box_encoded_str = String::from_utf8(byte_array).map_err(|err|Box::new(err))?;
        let combo_box_decoded = base64::decode(&combo_box_encoded_str).map_err(|err|Box::new(err))?;
        let parsed_msg : ComboBox = serde_json::from_slice(combo_box_decoded.as_slice()).map_err(|err|Box::new(err))?;
        Ok(parsed_msg)
    }
}