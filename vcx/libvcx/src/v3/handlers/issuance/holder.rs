use v3::handlers::issuance::states::{HolderState, InitialState};
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::connection::send_message;
use v3::messages::A2AMessage;

pub struct HolderSM {
    state: HolderState
}

impl HolderSM {
    pub fn new() -> Self {
        HolderSM {
            state: HolderState::Initial(InitialState {})
        }
    }

    pub fn step(state: HolderState) -> Self {
        HolderSM{state}
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> Self {
        let HolderSM {state} = self;
        let state = match state {
            HolderState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialOffer(offer) => {
                    panic!("Accept or reject offer");
                    let offer_accepted = true;
                    let (msg, state) = if offer_accepted {
                        // TODO: change for A2A cred request
                        let msg = A2AMessage::Generic("cred_request".to_string());
                        (msg, HolderState::RequestSent(state_data.into()))
                    } else {
                        // TODO: change for A2A common problem report
                        let msg = A2AMessage::Generic("cred_request".to_string());
                        (msg, HolderState::Finished(state_data.into()))
                    };
                    send_message(state_data.connection_handle, msg);
                    state
                },
                _ => {
                    warn!("Credential Issuance can only start on holder side with Credential Offer");
                    HolderState::Initial(state_data)
                }
            },
            HolderState::RequestSent(state_data) => match cim {
                CredentialIssuanceMessage::Credential(_credential) => {
                    panic!("Accept and send ack or problem report");
                    let ok = true;
                    if ok {
                        panic!("send ack");
                    } else {
                        panic!("send problem report");
                    }
                    HolderState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_report) => {
                    panic!("Finalize state");
                    HolderState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Credential and Problem Report");
                    HolderState::RequestSent(state_data)
                }
            },
            HolderState::Finished(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                HolderState::Finished(state_data)
            }
        };
        HolderSM::step(state)
    }
}