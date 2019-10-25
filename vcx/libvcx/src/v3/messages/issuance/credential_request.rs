use v3::messages::MessageId;
use v3::messages::attachment::{Attachment, Json, ENCODING_BASE64};
use error::{VcxError, VcxResult, VcxErrorKind};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialRequest {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename="requests~attach")]
    pub requests_attach: Attachment
}

impl CredentialRequest {
    pub fn create() -> Self {
        CredentialRequest {
            id: MessageId::new(),
            comment: String::new(),
            requests_attach: Attachment::Blank
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_requests_attach(mut self, credential_request: String) -> VcxResult<CredentialRequest> {
        let json: Json = Json::new(
            serde_json::from_str(&credential_request)
                .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Request Json".to_string()))?,
            ENCODING_BASE64
        )?;
        self.requests_attach = Attachment::JSON(json);
        Ok(self)
    }
}