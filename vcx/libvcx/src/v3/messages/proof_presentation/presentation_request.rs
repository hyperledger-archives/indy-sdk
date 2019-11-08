use v3::messages::a2a::{MessageId, A2AMessage};
use v3::messages::attachment::{
    Attachments,
    Attachment,
    Json,
    AttachmentEncoding
};
use error::prelude::*;
use std::convert::TryInto;

pub use messages::proofs::proof_request::{ProofRequestMessage, ProofRequestData, ProofRequestVersion};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationRequest {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "request_presentations~attach")]
    pub request_presentations_attach: Attachments
}

impl PresentationRequest {
    pub fn create() -> Self {
        PresentationRequest {
            id: MessageId::new(),
            comment: None,
            request_presentations_attach: Attachments::new(),
        }
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
        let json: Json = Json::new(json!(request_presentations), AttachmentEncoding::Base64)?;
        self.request_presentations_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::PresentationRequest(self.clone()) // TODO: THINK how to avoid clone
    }

    pub fn to_json(&self) -> VcxResult<String> {
        serde_json::to_string(self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize PresentationRequest: {}", err)))
    }
}

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