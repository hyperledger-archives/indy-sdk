use v3::messages::MessageId;
use v3::messages::attachment::Attachment;

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename="requests~attach")]
    pub requests_attach: Attachment
}