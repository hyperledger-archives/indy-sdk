use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{Attachments, AttachmentEncoding};
use error::VcxResult;
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialRequest {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread
}

impl CredentialRequest {
    pub fn create() -> Self {
        CredentialRequest {
            id: MessageId::new(),
            comment: None,
            requests_attach: Attachments::new(),
            thread: Thread::new(),
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_requests_attach(mut self, credential_request: String) -> VcxResult<CredentialRequest> {
        self.requests_attach.add_json_attachment(::serde_json::Value::String(credential_request), AttachmentEncoding::Base64)?;
        Ok(self)
    }

    pub fn set_thread_id(mut self, id: String) -> Self {
        self.thread.thid = Some(id);
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::CredentialRequest(self.clone()) // TODO: THINK how to avoid clone
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::issuance::credential_offer::tests::{thread, thread_id};

    fn _attachment() -> ::serde_json::Value {
        json!({
            "prover_did":"VsKV7grR1BUE29mG2Fm2kX",
            "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG1"
        })
    }

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn _credential_request() -> CredentialRequest {
        let mut attachment = Attachments::new();
        attachment.add_json_attachment(_attachment(), AttachmentEncoding::Base64).unwrap();

        CredentialRequest {
            id: MessageId::id(),
            comment: Some(_comment()),
            requests_attach: attachment,
            thread: thread(),
        }
    }

    #[test]
    fn test_credential_request_build_works() {
        let credential_request: CredentialRequest = CredentialRequest::create()
            .set_comment(_comment())
            .set_thread_id(thread_id())
            .set_requests_attach(_attachment().to_string()).unwrap();

        assert_eq!(_credential_request(), credential_request);
    }
}