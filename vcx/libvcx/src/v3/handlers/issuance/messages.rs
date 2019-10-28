use v3::messages::MessageId;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;

use std::collections::HashMap;

#[derive(Debug)]
pub enum CredentialIssuanceMessage {
    CredentialInit(String, String, u32),
    CredentialProposal(CredentialProposal, u32),
    CredentialOffer(CredentialOffer, u32),
    CredentialRequest(CredentialRequest, u32, u32),
    Credential(Credential, u32),
    Ack(Ack),
    ProblemReport(ProblemReport)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialInit {
    pub cred_def_id: String,
    pub credential_json: String,
    pub connection_handle: u32
}