use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState};
use v3::handlers::connection::send_message;

pub struct IssuerSM {
    state: IssuerState,
}

impl IssuerSM {
    pub fn new(connection_handle: u32) -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState{
                connection_handle
            })
        }
    }

    pub fn step(state: IssuerState) -> Self {
        IssuerSM {
            state
        }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> Self {
        let IssuerSM { state } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialInit(_init) => {
                    panic!("Send CredentialOffer message");
                    IssuerState::OfferSent(state_data.into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            },
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(_request) => {
                    panic!("Send IssueCredential message");
                    IssuerState::CredentialSent(state_data.into())
                }
                CredentialIssuanceMessage::CredentialProposal(_proposal) => {
                    panic!("Send problem report with reason: Do not support negotiation");
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    panic!("Finalize the issuance");
                    IssuerState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferSent(state_data)
                }
            }
            IssuerState::CredentialSent(state_data) => match cim {
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    panic!("Report the problem with issuance, close internaction");
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::Ack(_ack) => {
                    panic!("Report successful issuance, close interaction");
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