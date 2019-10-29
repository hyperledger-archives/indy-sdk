use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::attachment::{
    Attachment,
    Json,
    ENCODING_BASE64
};
use error::prelude::*;

pub use messages::proofs::proof_request::{ProofRequestData, ProofRequestVersion};

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

    pub fn set_request_presentations_attach(mut self, request_presentations: String) -> VcxResult<PresentationRequest> {
        let json: Json = Json::new(::serde_json::from_str(&request_presentations)
                                       .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Invalid Presentations Request: {:?}", err)))?,
                                   ENCODING_BASE64
        )?;
        self.request_presentations_attach = Attachment::JSON(json);
        Ok(self)
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::PresentationRequest(self.clone()) // TODO: THINK how to avoid clone
    }
}

pub type PresentationRequestData = ProofRequestData;
