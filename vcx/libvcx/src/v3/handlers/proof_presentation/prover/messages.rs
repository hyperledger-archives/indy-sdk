use v3::messages::proof_presentation::presentation_request::PresentationRequestData;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;
use v3::messages::a2a::A2AMessage;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ProverMessages {
    PresentationRequestReceived(PresentationRequestData),
    PreparePresentation((String, String)),
    SendPresentation(u32),
    PresentationAckReceived(Ack),
    PresentationRejectReceived(ProblemReport),
    Unknown
}

impl From<A2AMessage> for ProverMessages {
    fn from(msg: A2AMessage) -> Self {
        match msg {
            A2AMessage::Ack(ack) => {
                ProverMessages::PresentationAckReceived(ack)
            }
            A2AMessage::CommonProblemReport(report) => {
                ProverMessages::PresentationRejectReceived(report)
            }
            _ => {
                ProverMessages::Unknown
            }
        }
    }
}