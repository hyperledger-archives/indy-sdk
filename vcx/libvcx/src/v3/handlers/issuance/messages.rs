use v3::messages::error::ProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;
use v3::messages::issuance::credential_ack::CredentialAck;
use v3::messages::a2a::A2AMessage;


#[derive(Debug, Clone)]
pub enum CredentialIssuanceMessage {
    CredentialInit(u32),
    CredentialSend(),
    CredentialProposal(CredentialProposal),
    CredentialOffer(CredentialOffer),
    CredentialRequestSend(u32),
    CredentialRequest(CredentialRequest),
    Credential(Credential),
    CredentialAck(CredentialAck),
    ProblemReport(ProblemReport),
    Unknown
}

impl From<A2AMessage> for CredentialIssuanceMessage {
    fn from(msg: A2AMessage) -> Self {
        match msg {
            A2AMessage::CredentialProposal(proposal) => {
                CredentialIssuanceMessage::CredentialProposal(proposal)
            },
            A2AMessage::CredentialOffer(offer) => {
                CredentialIssuanceMessage::CredentialOffer(offer)
            },
            A2AMessage::CredentialRequest(request) => {
                CredentialIssuanceMessage::CredentialRequest(request)
            },
            A2AMessage::Credential(credential) => {
                CredentialIssuanceMessage::Credential(credential)
            },
            A2AMessage::Ack(ack) | A2AMessage::CredentialAck(ack) => {
                CredentialIssuanceMessage::CredentialAck(ack)
            },
            A2AMessage::CommonProblemReport(report) => {
                CredentialIssuanceMessage::ProblemReport(report)
            },
            _ => {
                CredentialIssuanceMessage::Unknown
            }
        }
    }
}