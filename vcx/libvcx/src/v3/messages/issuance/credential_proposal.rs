use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::issuance::CredentialPreviewData;
use v3::messages::mime_type::MimeType;
use error::VcxResult;
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CredentialProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub credential_proposal: CredentialPreviewData,
    pub schema_id: String,
    pub cred_def_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "~thread")]
    pub thread: Option<Thread>
}

impl CredentialProposal {
    pub fn create() -> Self {
        CredentialProposal::default()
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
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

    pub fn set_thread_id(mut self, id: &str) -> Self {
        self.thread = Some(Thread::new().set_thid(id.to_string()));
        self
    }
}

a2a_message!(CredentialProposal);

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::issuance::credential_offer::tests::{thread, thread_id, _value};

    fn _attachment() -> ::serde_json::Value {
        json!({"credential offer": {}})
    }

    fn _comment() -> String {
        String::from("comment")
    }

    fn _schema_id() -> String { String::from("schema:id") }

    fn _cred_def_id() -> String { String::from("cred_def_id:id") }

    fn _credential_preview_data() -> CredentialPreviewData {
        let (name, value) = _value();

        CredentialPreviewData::new()
            .add_value(name, value, MimeType::Plain).unwrap()
    }

    pub fn _credential_proposal() -> CredentialProposal {
        CredentialProposal {
            id: MessageId::id(),
            comment: Some(_comment()),
            credential_proposal: _credential_preview_data(),
            schema_id: _schema_id(),
            thread: Some(thread()),
            cred_def_id: _cred_def_id(),
        }
    }

    #[test]
    fn test_credential_proposal_build_works() {
        let (name, value) = _value();

        let credential_proposal: CredentialProposal = CredentialProposal::create()
            .set_comment(_comment())
            .set_thread_id(&thread_id())
            .set_cred_def_id(_cred_def_id())
            .set_schema_id(_schema_id())
            .add_credential_preview_data(name, value, MimeType::Plain).unwrap();

        assert_eq!(_credential_proposal(), credential_proposal);
    }
}