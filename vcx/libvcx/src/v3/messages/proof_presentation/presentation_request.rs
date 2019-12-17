use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{Attachments, AttachmentEncoding};

use error::prelude::*;
use std::convert::TryInto;

pub use messages::proofs::proof_request::{ProofRequestMessage, ProofRequestData, ProofRequestVersion};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PresentationRequest {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "request_presentations~attach")]
    pub request_presentations_attach: Attachments,
}

impl PresentationRequest {
    pub fn create() -> Self {
        PresentationRequest::default()
    }

    pub fn set_id(mut self, id: String) -> Self {
        self.id = MessageId(id);
        self
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_request_presentations_attach(mut self, request_presentations: &PresentationRequestData) -> VcxResult<PresentationRequest> {
        self.request_presentations_attach.add_json_attachment(json!(request_presentations), AttachmentEncoding::Base64)?;
        Ok(self)
    }

    pub fn to_json(&self) -> VcxResult<String> {
        serde_json::to_string(self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize PresentationRequest: {}", err)))
    }
}

a2a_message!(PresentationRequest);

impl TryInto<PresentationRequest> for ProofRequestMessage {
    type Error = VcxError;

    fn try_into(self) -> Result<PresentationRequest, Self::Error> {
        let presentation_request = PresentationRequest::create()
            .set_id(self.thread_id.unwrap_or_default())
            .set_request_presentations_attach(&self.proof_request_data)?;

        Ok(presentation_request)
    }
}

impl TryInto<ProofRequestMessage> for PresentationRequest {
    type Error = VcxError;

    fn try_into(self) -> Result<ProofRequestMessage, Self::Error> {
        let proof_request: ProofRequestMessage = ProofRequestMessage::create()
            .set_proof_request_data(
                ::serde_json::from_str(&self.request_presentations_attach.content()?
                ).map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, err))?
            )?
            .type_version("1.0")?
            .proof_data_version("0.1")?
            .set_thread_id(self.id.0.clone())?
            .clone();

        Ok(proof_request)
    }
}

pub type PresentationRequestData = ProofRequestData;

#[cfg(test)]
pub mod tests {
    use super::*;
    use messages::thread::Thread;

    pub fn _presentation_request_data() -> PresentationRequestData {
        PresentationRequestData::default()
            .set_requested_attributes(json!([{"name": "name"}]).to_string()).unwrap()
    }

    fn _attachment() -> ::serde_json::Value {
        json!(_presentation_request_data())
    }

    fn _comment() -> String {
        String::from("comment")
    }

    pub fn thread_id() -> String {
        _presentation_request().id.0
    }

    pub fn thread() -> Thread {
        Thread::new().set_thid(_presentation_request().id.0)
    }

    pub fn _presentation_request() -> PresentationRequest {
        let mut attachment = Attachments::new();
        attachment.add_json_attachment(_attachment(), AttachmentEncoding::Base64).unwrap();

        PresentationRequest {
            id: MessageId::id(),
            comment: Some(_comment()),
            request_presentations_attach: attachment,
        }
    }

    #[test]
    fn test_presentation_request_build_works() {
        let presentation_request: PresentationRequest = PresentationRequest::default()
            .set_comment(_comment())
            .set_request_presentations_attach(&_presentation_request_data()).unwrap();

        assert_eq!(_presentation_request(), presentation_request);
    }
}