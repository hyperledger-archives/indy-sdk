use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;
use v3::messages::A2AMessage;


#[derive(Debug, Clone)]
pub enum CredentialIssuanceMessage {
    CredentialInit(u32),
    CredentialSend(),
    CredentialProposal(CredentialProposal, u32),
    CredentialOffer(CredentialOffer, u32),
    CredentialRequestSend(u32),
    CredentialRequest(CredentialRequest, u32),
    Credential(Credential, u32),
    Ack(Ack),
    ProblemReport(ProblemReport),
    Unknown
}

impl From<(&A2AMessage, u32)> for CredentialIssuanceMessage {
    fn from((msg, handle): (&A2AMessage, u32)) -> Self {
        match msg {
            A2AMessage::CredentialProposal(proposal) => {
                CredentialIssuanceMessage::CredentialProposal(proposal.clone(), handle)
            },
            A2AMessage::CredentialOffer(offer) => {
                CredentialIssuanceMessage::CredentialOffer(offer.clone(), handle)
            },
            A2AMessage::CredentialRequest(request) => {
                CredentialIssuanceMessage::CredentialRequest(request.clone(), handle)
            },
            A2AMessage::Credential(credential) => {
                CredentialIssuanceMessage::Credential(credential.clone(), handle)
            },
            A2AMessage::Ack(ack) => {
                CredentialIssuanceMessage::Ack(ack.clone())
            },
            A2AMessage::CommonProblemReport(report) => {
                CredentialIssuanceMessage::ProblemReport(report.clone())
            },
            _ => {
                CredentialIssuanceMessage::Unknown
            }
        }
    }
}