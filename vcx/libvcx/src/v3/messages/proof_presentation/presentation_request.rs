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

impl Into<VcxResult<PresentationRequest>> for ProofRequestMessage {
    fn into(self) -> VcxResult<PresentationRequest> {
        let mut presentation_request = PresentationRequest::create();
        presentation_request = presentation_request.set_request_presentations_attach(&self.proof_request_data)?;
        Ok(presentation_request)
    }
}

impl Into<VcxResult<ProofRequestMessage>> for PresentationRequest {
    fn into(self) -> VcxResult<ProofRequestMessage> {
        let proof_request = ProofRequestMessage::create()
            .set_proof_request_data(
                ::serde_json::from_str(&self.request_presentations_attach.content()?
                ).map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, err))?
            )?
            .type_version("1.0")?
            .proof_data_version("0.1")?
            .clone();
        Ok(proof_request)
    }
}

pub type PresentationRequestData = ProofRequestData;
