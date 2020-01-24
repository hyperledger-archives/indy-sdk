use v3::messages::a2a::message_type::MessageType;
use v3::messages::a2a::message_family::MessageFamilies;
use v3::messages::mime_type::MimeType;
use error::VcxResult;

pub mod credential;
pub mod credential_offer;
pub mod credential_proposal;
pub mod credential_request;
pub mod credential_ack;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialPreviewData {
    #[serde(rename = "@type")]
    pub _type: MessageType,
    pub attributes: Vec<CredentialValue>,
}

impl CredentialPreviewData {
    pub fn new() -> Self {
        CredentialPreviewData::default()
    }

    pub fn add_value(mut self, name: &str, value: &str, mime_type: MimeType) -> VcxResult<CredentialPreviewData> {
        let data_value = match mime_type {
            MimeType::Plain => {
                CredentialValue {
                    name: name.to_string(),
                    value: value.to_string(),
                    _type: None,
                }
            }
        };
        self.attributes.push(data_value);
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CredentialValue {
    pub name: String,
    pub value: String,
    #[serde(rename = "mime-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<MimeType>,
}

impl Default for CredentialPreviewData {
    fn default() -> CredentialPreviewData {
        CredentialPreviewData {
            _type: MessageType::build(MessageFamilies::CredentialIssuance, "credential-preview"),
            attributes: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use v3::messages::ack;
    use v3::messages::error;
    use v3::messages::issuance::credential_offer::tests::_credential_offer;

    pub fn _ack() -> ack::Ack {
        ack::tests::_ack().set_thread_id(&_credential_offer().id.0)
    }

    pub fn _problem_report() -> error::ProblemReport {
        error::tests::_problem_report().set_thread_id(&_credential_offer().id.0)
    }
}