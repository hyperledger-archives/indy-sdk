use v3::messages::MessageId;
use v3::messages::issuance::CredentialPreviewData;
use error::{VcxError, VcxResult, VcxErrorKind};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CredentialProposal {
    #[serde(rename="@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_proposal: CredentialPreviewData,
    pub schema_id: String,
    pub cred_def_id: String
}

impl CredentialProposal {
    pub fn create() -> Self {
        CredentialProposal {
            id: MessageId::new(),
            comment: String::new(),
            credential_proposal: CredentialPreviewData::new(),
            schema_id: String::new(),
            cred_def_id: String::new()
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_schema_id(mut self, schema_id: String) -> Self {
        self.schema_id = schema_id;
        self
    }

    pub fn set_cred_def_id(mut self, cred_def_id: String) -> Self {
        self.cred_def_id = cred_def_id;
        self
    }

    pub fn add_credential_preview_data(mut self, name: &str, value: &str, mime_type: &str) -> VcxResult<CredentialProposal> {
        self.credential_proposal = self.credential_proposal.add_value(name, value, mime_type)?;
        Ok(self)
    }

}