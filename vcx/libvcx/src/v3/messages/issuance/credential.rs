use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{Attachments, AttachmentId};
use v3::messages::ack::PleaseAck;
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;
use issuer_credential::CredentialMessage;
use messages::payload::PayloadKinds;
use std::convert::TryInto;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Credential {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "credentials~attach")]
    pub credentials_attach: Attachments,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "~please_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub please_ack: Option<PleaseAck>,
}

impl Credential {
    pub fn create() -> Self {
        Credential::default()
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_credential(mut self, credential: String) -> VcxResult<Credential> {
        self.credentials_attach.add_base64_encoded_json_attachment(AttachmentId::Credential, ::serde_json::Value::String(credential))?;
        Ok(self)
    }
}

please_ack!(Credential);
threadlike!(Credential);
a2a_message!(Credential);

impl TryInto<Credential> for CredentialMessage {
    type Error = VcxError;

    fn try_into(self) -> Result<Credential, Self::Error> {
        Credential::create()
            .set_thread_id(&self.claim_offer_id)
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::issuance::credential_offer::tests::{thread, thread_id};

    fn _attachment() -> ::serde_json::Value {
        json!({
            "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
            "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG1",
            "values":{"name":{"raw":"Name","encoded":"1139481716457488690172217916278103335"}}
        })
    }

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn _credential() -> Credential {
        let mut attachment = Attachments::new();
        attachment.add_base64_encoded_json_attachment(AttachmentId::Credential, _attachment()).unwrap();

        Credential {
            id: MessageId::id(),
            comment: Some(_comment()),
            thread: thread(),
            credentials_attach: attachment,
            please_ack: None,
        }
    }

    #[test]
    fn test_credential_build_works() {
        let credential: Credential = Credential::create()
            .set_comment(_comment())
            .set_thread_id(&thread_id())
            .set_credential(_attachment().to_string()).unwrap();

        assert_eq!(_credential(), credential);
    }
}