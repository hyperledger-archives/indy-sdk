use v3::messages::{MessageType, A2AMessageKinds};
use error::{VcxError, VcxResult, VcxErrorKind};

pub mod credential;
pub mod credential_offer;
pub mod credential_proposal;
pub mod credential_request;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialPreviewData {
    #[serde(rename="@type")]
    pub _type: MessageType,
    pub attributes: Vec<CredentialValue>
}

impl CredentialPreviewData {
    pub fn new() -> Self {
        CredentialPreviewData {
            _type: MessageType::build(A2AMessageKinds::CredentialPreview),
            attributes: vec![]
        }
    }

    pub fn add_value(mut self, name: &str, value: &str, mime_type: &str) -> VcxResult<CredentialPreviewData> {
        let data_value = match mime_type {
            "text/plain" => {
                Ok(CredentialValue::String(
                    CredentialValueData {
                        name: name.to_string(),
                        value: value.to_string()
                    })
                )
            },
            _ => {
                Err(VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure, "Invalid mime type of value in credential preview"))
            }
        }?;
        self.attributes.push(data_value);
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mime-type")]
pub enum CredentialValue {
    #[serde(rename="text/plain")]
    String(CredentialValueData)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialValueData {
    pub name: String,
    pub value: String
}