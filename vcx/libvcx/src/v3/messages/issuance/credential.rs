use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{
    Attachments,
    Attachment,
    Json,
    AttachmentEncoding
};
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;
use issuer_credential::CredentialMessage;
use messages::payload::PayloadKinds;
use std::convert::TryInto;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Credential {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename = "credentials~attach")]
    pub credentials_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread
}

impl Credential {
    pub fn create() -> Self {
        Credential {
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

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Credential(self.clone()) // TODO: THINK how to avoid clone
    }
}

impl TryInto<Credential> for CredentialMessage {
    type Error = VcxError;

    fn try_into(self) -> Result<Credential, Self::Error> {
        Credential::create()
            .set_thread(Thread::new().set_thid(self.claim_offer_id))
            .set_credential(self.libindy_cred)
    }
}

impl TryInto<CredentialMessage> for Credential {
    type Error = VcxError;

    fn try_into(self) -> Result<CredentialMessage, Self::Error> {
        let indy_credential_json = self.credentials_attach.content()?;

        let indy_credential: ::serde_json::Value = ::serde_json::from_str(&indy_credential_json)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize Indy Credential: {:?}", err)))?;

        Ok(CredentialMessage {
            msg_type: PayloadKinds::Cred.name().to_string(),
            libindy_cred: self.credentials_attach.content()?,
            claim_offer_id: self.thread.thid.clone().unwrap_or_default(),
            cred_revoc_id: None,
            revoc_reg_delta_json: None,
            version: String::from("0.1"),
            from_did: String::new(),
            cred_def_id: indy_credential["cred_def_id"].as_str().map(String::from).unwrap_or_default(),
            rev_reg_def_json: String::new(),
        })
    }
}