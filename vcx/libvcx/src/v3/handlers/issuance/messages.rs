use v3::messages::MessageId;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;

#[serde(tag = "@type")]
#[derive(Debug, Serialize, Deserialize)]
pub enum CredentialIssuanceMessage {
    #[serde(rename = "init")]
    CredentialInit(CredentialInit),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/propose-credential")]
    CredentialProposal(CredentialProposal),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/offer-credential")]
    CredentialOffer(CredentialOffer),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/request-credential")]
    CredentialRequest(CredentialRequest),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/issue-credential/1.0/issue-credential")]
    Credential(Credential),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/notification/1.0/ack")]
    Ack(Ack),
    #[serde(rename = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/notification/1.0/problem-report")]
    ProblemReport(ProblemReport)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialInit {

}