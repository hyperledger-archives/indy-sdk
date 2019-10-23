use v3::messages::MessageId;
use v3::messages::issuance::CredentialPreviewData;
use v3::messages::attachment::Attachment;
use utils::error::{self, Error};

#[derive(Debug, Serialize, Deserialize)]
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
            credential_preview: (),
            offers_attach: Attachment::Blank
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_offers_attach(mut self, credential_offer: String) -> Result<CredentialOffer, Error> {

    }
}