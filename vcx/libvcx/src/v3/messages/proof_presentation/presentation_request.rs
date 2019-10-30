use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::attachment::{
    Attachment,
    Json,
    ENCODING_BASE64
};
use error::prelude::*;

pub use messages::proofs::proof_request::{ProofRequestMessage, ProofRequestData, ProofRequestVersion};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationRequest {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename = "request_presentations~attach")]
    pub request_presentations_attach: Attachment
}

impl PresentationRequest {
    pub fn create() -> Self {
        PresentationRequest {
            msg_type: MessageType::build(A2AMessageKinds::PresentationRequest),
            id: MessageId::new(),
            comment: String::new(),
            request_presentations_attach: Attachment::Blank,
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_request_presentations_attach(mut self, request_presentations: &PresentationRequestData) -> VcxResult<PresentationRequest> {
        let json: Json = Json::new(json!(request_presentations), ENCODING_BASE64)?;
        self.request_presentations_attach = Attachment::JSON(json);
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

impl From<ProofRequestMessage> for PresentationRequest {
    fn from(proof_request: ProofRequestMessage) -> Self {
        let mut presentation_request = PresentationRequest::create();
        presentation_request = presentation_request.set_request_presentations_attach(&proof_request.proof_request_data).unwrap();
        presentation_request
    }
}

pub type PresentationRequestData = ProofRequestData;
