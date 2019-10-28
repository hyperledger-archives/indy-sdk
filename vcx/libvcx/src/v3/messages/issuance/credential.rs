use v3::messages::MessageId;
use v3::messages::attachment::{
    Attachment,
    Json,
    ENCODING_BASE64
};
use error::{VcxError, VcxResult, VcxErrorKind};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Credential {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename="credentials~attach")]
    pub credentials_attach: Attachment
}

impl Credential {
    pub fn create() -> Self {
        Credential {
            id: MessageId::new(),
            comment: String::new(),
            credentials_attach: Attachment::Blank
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_credential(mut self, credential: String) -> VcxResult<Credential> {
        let json: Json = Json::new(
            serde_json::from_str(&credential)
                .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?,
            ENCODING_BASE64
        )?;
        self.credentials_attach = Attachment::JSON(json);
        Ok(self)
    }
}
