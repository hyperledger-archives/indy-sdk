use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState};

pub struct IssuerSM {
    state: IssuerState,
}

impl IssuerSM {
    pub fn new() -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState{})
        }
    }

    pub fn step(state: IssuerState) -> Self {
        IssuerSM{
            state
        }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> Self {
        let IssuerSM { state } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialInit(_init) => {
                    todo!("Send CredentialOffer message");
                    IssuerState::OfferSent(state_data.into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            },
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(_request) => {
                    todo!("Send IssueCredential message");
                    IssuerState::CredentialSent(state_data.into())
                }
                CredentialIssuanceMessage::CredentialProposal(_proposal) => {
                    todo!("Send problem report with reason: Do not support negotiation");
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    todo!("Finalize the issuance");
                    IssuerState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferSent(state_data)
                }
            }
            IssuerState::CredentialSent(state_data) => match cim {
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    todo!("Report the problem with issuance, close internaction");
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::Ack(_ack) => {
                    todo!("Report successful issuance, close interaction");
                    IssuerState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Ack and Problem Report");
                    IssuerState::CredentialSent(state_data)
                }
            }
            IssuerState::Finished(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                IssuerState::Finished(state_data)
            }
        };
        IssuerSM::step(state)
    }
}