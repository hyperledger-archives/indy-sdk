use std::str::from_utf8;
use serde_json;

use error::{VcxResult, VcxError, VcxErrorKind};
use v3::messages::a2a::MessageId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachments(pub Vec<Attachment>);

impl Attachments {
    pub fn new() -> Attachments {
        Attachments(Vec::new())
    }

    pub fn get(&self) -> Option<&Attachment> {
        self.0.get(0)
    }

    pub fn add(&mut self, attachment: Attachment) {
        self.0.push(attachment);
    }

    pub fn add_json_attachment(&mut self, json: serde_json::Value, encoding: AttachmentEncoding) -> VcxResult<()> {
        let json: Json = Json::new(json, encoding)?;
        self.add(Attachment::JSON(json));
        Ok(())
    }

    pub fn content(&self) -> VcxResult<String> {
        match self.get() {
            Some(Attachment::JSON(ref attach)) => attach.get_data(),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "Unsupported Attachment type"))
        }
    }
}

#[serde(tag = "mime-type")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Attachment {
    #[serde(rename = "application/json")]
    JSON(Json),
    Blank
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Json {
    #[serde(rename = "@id")]
    id: MessageId,
    data: AttachmentData
}

impl Json {
    pub fn new(json: serde_json::Value, encoding: AttachmentEncoding) -> VcxResult<Json> {
        let data: AttachmentData = match encoding {
            AttachmentEncoding::Base64 => {
                AttachmentData::Base64(
                    base64::encode(&
                        match json {
                            ::serde_json::Value::Object(obj) => {
                                serde_json::to_string(&obj)
                                    .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Attachment Json".to_string()))?
                            }
                            ::serde_json::Value::String(str) => str,
                            val => return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Unsupported Json value: {:?}", val)))
                        }
                    )
                )
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AttachmentEncoding {
    Base64
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _json() -> serde_json::Value {
        json!({"field": "value"})
    }

    #[test]
    fn test_create_json_attachment_works() {
        let json_attachment: Json = Json::new(_json(), AttachmentEncoding::Base64).unwrap();
        assert_eq!(vec![123, 34, 102, 105, 101, 108, 100, 34, 58, 34, 118, 97, 108, 117, 101, 34, 125], json_attachment.data.get_bytes().unwrap());
        assert_eq!(_json().to_string(), json_attachment.get_data().unwrap());
    }

    #[test]
    fn test_attachments_works() {
        {
            let mut attachments = Attachments::new();
            assert_eq!(0, attachments.0.len());

            let json: Json = Json::new(_json(), AttachmentEncoding::Base64).unwrap();
            attachments.add(Attachment::JSON(json));
            assert_eq!(1, attachments.0.len());

            assert_eq!(_json().to_string(), attachments.content().unwrap());
        }

        {
            let mut attachments = Attachments::new();
            attachments.add_json_attachment(_json(), AttachmentEncoding::Base64).unwrap();
            assert_eq!(_json().to_string(), attachments.content().unwrap());
        }
    }
}