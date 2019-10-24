use v3::messages::MessageId;
use v3::messages::issuance::CredentialPreviewData;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialProposal {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_proposal: CredentialPreviewData,
    pub schema_id: String,
    pub cred_def_id: String
}