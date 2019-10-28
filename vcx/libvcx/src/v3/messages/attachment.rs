use std::str::from_utf8;
use serde_json;

use error::{VcxResult, VcxError, VcxErrorKind};
use v3::messages::MessageId;

#[serde(tag = "mime-type")]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Attachment {
    #[serde(rename = "application/json")]
    JSON(Json),
    Blank
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Json {
    #[serde(rename = "@id")]
    id: MessageId,
    data: AttachmentData
}

pub static ENCODING_BASE64: &str = "base64";

impl Json {
    pub fn new(json: serde_json::Value, encoding: &str) -> VcxResult<Json> {
        let data: AttachmentData = match encoding {
            "base64" => {
                AttachmentData::Base64(
                    base64::encode(
                        &serde_json::to_string(&json)
                            .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Attachment Json".to_string()))?
                    )
                )
            }
            _ => {
                return Err(VcxError::from_msg(VcxErrorKind::IOError, "Unknown encoding"))
            }
        };
        Ok(Json {
            id: MessageId::new(),
            data
        })
    }

    pub fn get_data(&self) -> VcxResult<String> {
        let data = self.data.get_bytes()?;
        from_utf8(data.as_slice())
            .map(|s| s.to_string())
            .map_err(|_| VcxError::from_msg(VcxErrorKind::IOError, "Wrong bytes in attachment".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AttachmentData {
    #[serde(rename = "base64")]
    Base64(String)
}

impl AttachmentData {
    pub fn get_bytes(&self) -> VcxResult<Vec<u8>> {
        match self {
            AttachmentData::Base64(s) => {
                base64::decode(s).map_err(|_| VcxError::from_msg(VcxErrorKind::IOError, "Wrong bytes in attachment"))
            }
        }
    }
}