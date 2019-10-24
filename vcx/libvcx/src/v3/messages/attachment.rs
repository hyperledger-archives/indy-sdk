use utils::error::{self, Error};
use serde_json;
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
    pub fn new(json: serde_json::Value, encoding: &str) -> Result<Json, Error> {
        let data: AttachmentData = match encoding {
            "base64" => {
                AttachmentData::Base64(
                    base64::encode(
                        &serde_json::to_string(&json).map_err(|_| error::INVALID_JSON)?
                    )
                )
            }
            _ => {
                return Err(error::UNKNOWN_ATTACHMENT_ENCODING)
            }
        };
        Ok(Json {
            id: MessageId::new(),
            data
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AttachmentData {
    #[serde(rename = "base64")]
    Base64(String)
}

impl AttachmentData {
    pub fn get_bytes(&self) -> Result<Vec<u8>, error::Error> {
        match self {
            AttachmentData::Base64(s) => {
                base64::decode(s).map_err(|_| error::INVALID_ATTACHMENT_ENCODING)
            }
        }
    }
}