use v3::messages::connection::agent_info::AgentInfo;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::ack::Ack;

use messages::update_connection::send_delete_connection_message;

use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidExchangeStateSM {
    pub state: DidExchangeState // TODO FIX public
}

/// Transitions of Connection state
/// Null -> Invited
/// Invited -> Requested, Null
/// Requested -> Responded, Null
/// Responded -> Complete, Invited
/// Complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeState {
    Null(NullState),
    Invited(InvitedState),
    Requested(RequestedState),
    Responded(RespondedState),
    Complete(CompleteState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullState {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedState {
    pub invitation: Invitation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    pub invitation: Invitation,
    pub remote_info: RemoteConnectionInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub invitation: Invitation,
    pub remote_info: RemoteConnectionInfo,
    pub prev_agent_info: Option<AgentInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
    pub invitation: Invitation,
    pub remote_info: RemoteConnectionInfo
}

impl From<(NullState, Invitation)> for InvitedState {
    fn from((state, invitation): (NullState, Invitation)) -> InvitedState {
        trace!("DidExchangeStateSM: transit state from NullState to InvitedState");
        InvitedState { invitation }
    }
}

impl From<(InvitedState, ProblemReport)> for NullState {
    fn from((state, error): (InvitedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from InvitedState to NullState");
        NullState {}
    }
}

impl From<(InvitedState, Request)> for RequestedState {
    fn from((state, request): (InvitedState, Request)) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RequestedState { invitation: state.invitation, remote_info: RemoteConnectionInfo::from(request) }
    }
}

impl From<InvitedState> for RequestedState {
    fn from(state: InvitedState) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RequestedState { invitation: state.invitation.clone(), remote_info: RemoteConnectionInfo::from(state.invitation) }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RequestedState to NullState");
        NullState {}
    }
}

impl From<(RequestedState, AgentInfo)> for RespondedState {
    fn from((state, agent_info): (RequestedState, AgentInfo)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        RespondedState { invitation: state.invitation, remote_info: state.remote_info, prev_agent_info: Some(agent_info) }
    }
}

impl From<(RequestedState, Response)> for RespondedState {
    fn from((state, response): (RequestedState, Response)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        let mut remote_info = RemoteConnectionInfo::from(response);
        remote_info.set_label(state.remote_info.label);
        RespondedState { invitation: state.invitation, remote_info, prev_agent_info: None }
    }
}

impl From<(RespondedState, ProblemReport)> for NullState {
    fn from((state, error): (RespondedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RespondedState to NullState");
        NullState {}
    }
}

impl From<(RespondedState, Ack)> for CompleteState {
    fn from((state, ack): (RespondedState, Ack)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        CompleteState { invitation: state.invitation, remote_info: state.remote_info }
    }
}

impl From<RespondedState> for CompleteState {
    fn from(state: RespondedState) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        CompleteState { invitation: state.invitation, remote_info: state.remote_info }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Messages {
    Error(ProblemReport),
    InvitationSent(Invitation),
    InvitationReceived(Invitation),
    ExchangeRequestSent(Request),
    ExchangeRequestReceived(Request),
    ExchangeResponseSent(Response, AgentInfo),
    ExchangeResponseReceived(Response),
    AckSent(Ack),
    AckReceived(Ack),
}

impl DidExchangeStateSM {
    pub fn new() -> Self {
        DidExchangeStateSM {
            state: DidExchangeState::Null(NullState {})
        }
    }

    pub fn state(&self) -> u32 {
        match self.state {
            DidExchangeState::Null(_) => 1,
            DidExchangeState::Invited(_) => 2,
            DidExchangeState::Requested(_) => 3,
            DidExchangeState::Responded(_) => 5, // for backward compatibility
            DidExchangeState::Complete(_) => 4,
        }
    }

    pub fn step(self, message: Messages) -> VcxResult<DidExchangeStateSM> {
        trace!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeStateSM { state } = self;
        let state = match state {
            DidExchangeState::Null(state) => {
                match message {
                    Messages::InvitationSent(invitation) => DidExchangeState::Invited((state, invitation).into()),
                    Messages::InvitationReceived(invitation) => DidExchangeState::Invited((state, invitation).into()),
                    _ => DidExchangeState::Null(state)
                }
            }
            DidExchangeState::Invited(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Null((state, error).into()),
                    Messages::ExchangeRequestSent(request) => DidExchangeState::Requested(state.into()),
                    Messages::ExchangeRequestReceived(request) => DidExchangeState::Requested((state, request).into()),
                    _ => DidExchangeState::Invited(state)
                }
            }
            DidExchangeState::Requested(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Null((state, error).into()),
                    Messages::ExchangeResponseSent(response, agent_info) => DidExchangeState::Responded((state, agent_info).into()),
                    Messages::ExchangeResponseReceived(response) => DidExchangeState::Responded((state, response).into()),
                    _ => DidExchangeState::Requested(state)
                }
            }
            DidExchangeState::Responded(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Null((state, error).into()),
                    Messages::AckSent(ack) => DidExchangeState::Complete(state.into()),
                    Messages::AckReceived(ack) => {
                        if let Some(ref info) = state.prev_agent_info {
                            send_delete_connection_message(&info.pw_did, &info.pw_vk, &info.agent_did, &info.agent_vk)?;
                        }

                        DidExchangeState::Complete((state, ack).into())
                    },
                    _ => DidExchangeState::Responded(state)
                }
            }
            DidExchangeState::Complete(state) => DidExchangeState::Complete(state)
        };
        Ok(DidExchangeStateSM { state })
    }

    pub fn remote_connection_info(&self, actor: &Actor) -> Option<RemoteConnectionInfo> {
        match self.state {
            DidExchangeState::Null(_) => None,
            DidExchangeState::Invited(ref state) => {
                match actor {
                    Actor::Inviter => None,
                    Actor::Invitee => Some(RemoteConnectionInfo::from(state.invitation.clone()))
                }
            }
            DidExchangeState::Requested(ref state) => Some(state.remote_info.clone()),
            DidExchangeState::Responded(ref state) => Some(state.remote_info.clone()),
            DidExchangeState::Complete(ref state) => Some(state.remote_info.clone()),
        }
    }

    pub fn get_invitation(&self) -> Option<&Invitation> {
        match self.state {
            DidExchangeState::Null(_) => None,
            DidExchangeState::Invited(ref state) => Some(&state.invitation),
            DidExchangeState::Requested(ref state) => Some(&state.invitation),
            DidExchangeState::Responded(ref state) => Some(&state.invitation),
            DidExchangeState::Complete(ref state) => Some(&state.invitation),
        }
    }

    pub fn prev_agent_info(&self) -> Option<&AgentInfo>{
        match self.state {
            DidExchangeState::Responded(ref state) => state.prev_agent_info.as_ref(),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    Inviter,
    Invitee
}
