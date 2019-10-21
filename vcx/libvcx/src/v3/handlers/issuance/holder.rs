use v3::handlers::issuance::states::{HolderState, InitialState};
use v3::handlers::issuance::messages::CredentialIssuanceMessage;

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
                    todo!("Accept or reject offer");
                    let offer_accepted = true;
                    if offer_accepted {
                        todo!("Send credential request");
                        HolderState::RequestSent(state.into())
                    } else {
                        todo!("Send Problem report");
                        HolderState::FinishedState(state.into())
                    }
                },
                _ => {
                    warn!("Credential Issuance can only start on holder side with Credential Offer");
                    HolderState::Initial(state_data)
                }
            },
            HolderState::RequestSent(state_data) => match cim {
                CredentialIssuanceMessage::Credential(_credential) => {
                    todo!("Accept and send ack or problem report");
                    let ok = true;
                    if ok {
                        todo!("send ack");
                    } else {
                        todo!("send problem report");
                    }
                    HolderState::FinishedState(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_report) => {
                    todo!("Finalize state");
                    Holder::FinishedState(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Credential and Problem Report");
                    HolderState::RequestSent(state_data)
                }
            },
            HolderState::FinishedState(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                HolderState::Finished(state_data)
            }
        };
        HolderSM::step(state)
    }
}