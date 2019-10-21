use v3::messages::attachment::Attachment;
use v3::messages::MessageId;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;

#[serde(tag = "@type")]
#[derive(Debug, Serialize, Deserialize)]
pub enum CredentialIssuanceMessage {
    #[serde(rename = "init")]
    CredentialInit(CredentialInit),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/propose-credential")]
    CredentialProposal(CredentialProposal),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/offer-credential")]
    CredentialOffer(CredentialOffer),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/request-credential")]
    CredentialRequest(CredentialRequest),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/issue-credential")]
    Credential(Credential),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/notification/1.0/ack")]
    Ack(Ack),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/notification/1.0/problem-report")]
    ProblemReport(ProblemReport)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialProposal {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_proposal: CredentialPreviewData,
    pub schema_id: String,
    pub cred_def_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialOffer {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_preview: CredentialPreviewData,
    #[serde(rename="offers~attach")]
    pub offers_attach: Attachment
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRequest {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename="requests~attach")]
    pub requests_attach: Attachment
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename="credentials~attach")]
    pub credentials_attach: Attachment
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialPreviewData {
    #[serde(rename="@type")]
    pub _type: String,
    pub attributes: Vec<CredentialValue>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mime-type")]
pub enum CredentialValue {
    #[serde(rename="text/plain")]
    String(CredentialValueData)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialValueData {
    pub name: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialInit {

}