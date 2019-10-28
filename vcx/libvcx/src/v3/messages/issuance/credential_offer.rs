use v3::messages::MessageId;
use v3::messages::issuance::{CredentialPreviewData, CredentialValueData, CredentialValue};
use v3::messages::attachment::{Attachment, Json, ENCODING_BASE64};
use error::{VcxError, VcxResult, VcxErrorKind};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialOffer {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_preview: CredentialPreviewData,
    #[serde(rename="offers~attach")]
    pub offers_attach: Attachment
}

impl CredentialOffer {
    pub fn create() -> Self {
        CredentialOffer {
            id: MessageId::new(),
            comment: String::new(),
            credential_preview: CredentialPreviewData::new(),
            offers_attach: Attachment::Blank
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_offers_attach(mut self, credential_offer: String) -> VcxResult<CredentialOffer> {
        let json: Json = Json::new(
            serde_json::from_str(&credential_offer)
                .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Offer Json".to_string()))?,
            ENCODING_BASE64
        )?;
        self.offers_attach = Attachment::JSON(json);
        Ok(self)
    }

    pub fn add_credential_preview_data(mut self, name: &str, value: &str, mime_type: &str) -> VcxResult<CredentialOffer> {
        self.credential_preview = self.credential_preview.add_value(name, value, mime_type)?;
        Ok(self)
    }
}