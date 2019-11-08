use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::issuance::CredentialPreviewData;
use v3::messages::mime_type::MimeType;
use error::VcxResult;
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    pub credential_proposal: CredentialPreviewData,
    pub schema_id: String,
    pub cred_def_id: String,
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>
}

impl CredentialProposal {
    pub fn create() -> Self {
        CredentialProposal {
            id: MessageId::new(),
            comment: String::new(),
            credential_proposal: CredentialPreviewData::new(),
            schema_id: String::new(),
            cred_def_id: String::new(),
            thread: None,
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

    pub fn add_credential_preview_data(mut self, name: &str, value: &str, mime_type: MimeType) -> VcxResult<CredentialProposal> {
        self.credential_proposal = self.credential_proposal.add_value(name, value, mime_type)?;
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = Some(thread);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::CredentialProposal(self.clone()) // TODO: THINK how to avoid clone
    }
}