use v3::messages::MessageId;
use v3::messages::attachment::{
    Attachment,
    Json,
    ENCODING_BASE64
};
use utils::error::{self, Error};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn set_credential(mut self, credential: String) -> Result<Credential, Error> {
        let json: Json = Json::new(serde_json::from_str(&credential).map_err(|_| error::INVALID_JSON)?, ENCODING_BASE64)?;
        self.credentials_attach = Attachment::JSON(json);
        Ok(self)
    }
}
