use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::issuance::{CredentialPreviewData, CredentialValue};
use v3::messages::attachment::{Attachments, Attachment, Json, AttachmentEncoding};
use v3::messages::mime_type::MimeType;
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialOffer {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_preview: CredentialPreviewData,
    #[serde(rename = "offers~attach")]
    pub offers_attach: Attachments,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>
}

impl CredentialOffer {
    pub fn create() -> Self {
        CredentialOffer {
            id: MessageId::new(),
            comment: String::new(),
            credential_preview: CredentialPreviewData::new(),
            offers_attach: Attachments::new(),
            thread: None,
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_offers_attach(mut self, credential_offer: &str) -> VcxResult<CredentialOffer> {
        let json: Json = Json::new(::serde_json::Value::String(credential_offer.to_string()), AttachmentEncoding::Base64)?;
        self.offers_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn add_credential_preview_data(mut self, name: &str, value: &str, mime_type: MimeType) -> VcxResult<CredentialOffer> {
        self.credential_preview = self.credential_preview.add_value(name, value, mime_type)?;
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = Some(thread);
        self
    }
}