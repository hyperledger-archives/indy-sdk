use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::attachment::{
    Attachments,
    Attachment,
    Json,
    AttachmentEncoding
};
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Credential {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename = "credentials~attach")]
    pub credentials_attach: Attachments,
    pub thread: Thread
}

impl Credential {
    pub fn create() -> Self {
        Credential {
            msg_type: MessageType::build(A2AMessageKinds::Credential),
            id: MessageId::new(),
            comment: String::new(),
            credentials_attach: Attachments::new(),
            thread: Thread::new()
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_credential(mut self, credential: String) -> VcxResult<Credential> {
        let json: Json = Json::new(::serde_json::Value::String(credential), AttachmentEncoding::Base64)?;
        self.credentials_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = thread;
        self
    }
}
