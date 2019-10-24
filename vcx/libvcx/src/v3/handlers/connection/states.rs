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
    pub agent_info: AgentInfo,
    pub state: ActorDidExchangeState // TODO FIX public
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorDidExchangeState {
    Inviter(DidExchangeState),
    Invitee(DidExchangeState),
}

/// Transitions of Connection state
/// Null -> Invited
/// Invited -> Requested, Null
/// Requested -> Responded, Null
/// Responded -> Complete, Invited
/// Completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeState {
    Null(NullState),
    Invited(InvitedState),
    Requested(RequestedState),
    Responded(RespondedState),
    Completed(CompleteState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullState {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedState {
    pub invitation: Invitation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    pub remote_info: RemoteConnectionInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub remote_info: RemoteConnectionInfo,
    pub prev_agent_info: Option<AgentInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
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
        RequestedState { remote_info: RemoteConnectionInfo::from(request) }
    }
}

impl From<InvitedState> for RequestedState {
    fn from(state: InvitedState) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RequestedState { remote_info: RemoteConnectionInfo::from(state.invitation) }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RequestedState to NullState");
        NullState {}
    }
}

impl From<(RequestedState, Response, AgentInfo)> for RespondedState {
    fn from((state, response, agent_info): (RequestedState, Response, AgentInfo)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        RespondedState { remote_info: state.remote_info, prev_agent_info: Some(agent_info) }
    }
}

impl From<(RequestedState, Response)> for RespondedState {
    fn from((state, response): (RequestedState, Response)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        let mut remote_info = RemoteConnectionInfo::from(response);
        remote_info.set_label(state.remote_info.label);
        RespondedState { remote_info, prev_agent_info: None }
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
        CompleteState { remote_info: state.remote_info }
    }
}

impl From<RespondedState> for CompleteState {
    fn from(state: RespondedState) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        CompleteState { remote_info: state.remote_info }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Messages {
    InvitationSent(Invitation),
    InvitationReceived(Invitation),
    ExchangeRequestReceived(Request),
    ExchangeRequestSent(Request),
    ExchangeResponseReceived(Response),
    ExchangeResponseSent(Response, AgentInfo),
    AckReceived(Ack),
    AckSent(Ack),
    ProblemReport(ProblemReport),
}

impl DidExchangeStateSM {
    pub fn new(actor: Actor) -> Self {
        match actor {
            Actor::Inviter => {
                DidExchangeStateSM {
                    state: ActorDidExchangeState::Inviter(DidExchangeState::Null(NullState {})),
                    agent_info: AgentInfo::default()
                }
            }
            Actor::Invitee => {
                DidExchangeStateSM {
                    state: ActorDidExchangeState::Invitee(DidExchangeState::Null(NullState {})),
                    agent_info: AgentInfo::default()
                }
            }
        }
    }

    pub fn set_agent_info(&mut self, agent_info: AgentInfo) {
        self.agent_info = agent_info
    }

    pub fn agent_info(&self) -> &AgentInfo {
        &self.agent_info
    }

    pub fn state(&self) -> u32 {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Null(_)) => 1,
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)) => 2,
            ActorDidExchangeState::Inviter(DidExchangeState::Requested(_)) => 3,
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(_)) => 5, // for backward compatibility
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)) => 4,
            ActorDidExchangeState::Invitee(DidExchangeState::Null(_)) => 1,
            ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)) => 2,
            ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)) => 3,
            ActorDidExchangeState::Invitee(DidExchangeState::Responded(_)) => 5,
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)) => 4,
        }
    }

    pub fn step(self, message: Messages) -> VcxResult<DidExchangeStateSM> {
        println!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeStateSM { agent_info, state } = self;

        let state = match state {
            ActorDidExchangeState::Inviter(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            Messages::InvitationSent(invitation) =>
                                ActorDidExchangeState::Inviter(DidExchangeState::Invited((state, invitation).into())),
                            _ => ActorDidExchangeState::Inviter(DidExchangeState::Null(state))
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            Messages::ExchangeRequestReceived(request) =>
                                ActorDidExchangeState::Inviter(DidExchangeState::Requested((state, request).into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Inviter(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Inviter(DidExchangeState::Invited(state))
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            Messages::ExchangeResponseSent(response, agent_info) =>
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, response, agent_info).into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Inviter(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Inviter(DidExchangeState::Requested(state))
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            Messages::AckReceived(ack) =>
                                ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ack).into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Inviter(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Inviter(DidExchangeState::Responded(state))
                        }
                    }
                    DidExchangeState::Completed(state) => ActorDidExchangeState::Inviter(DidExchangeState::Completed(state))
                }
            }
            ActorDidExchangeState::Invitee(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            Messages::InvitationReceived(invitation) =>
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into())),
                            _ => ActorDidExchangeState::Invitee(DidExchangeState::Null(state))
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            Messages::ExchangeRequestSent(request) =>
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested(state.into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Invitee(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Invitee(DidExchangeState::Invited(state))
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            Messages::ExchangeResponseReceived(response) =>
                                ActorDidExchangeState::Invitee(DidExchangeState::Responded((state, response).into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Invitee(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Invitee(DidExchangeState::Requested(state))
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            Messages::AckSent(ack) =>
                                ActorDidExchangeState::Invitee(DidExchangeState::Completed((state, ack).into())),
                            Messages::ProblemReport(error) => ActorDidExchangeState::Invitee(DidExchangeState::Null((state, error).into())),
                            _ => ActorDidExchangeState::Invitee(DidExchangeState::Responded(state))
                        }
                    }
                    DidExchangeState::Completed(state) => ActorDidExchangeState::Invitee(DidExchangeState::Completed(state))
                }
            }
        };
        Ok(DidExchangeStateSM { agent_info, state })
    }

    pub fn remote_connection_info(&self) -> Option<RemoteConnectionInfo> {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(_) => None,
                    DidExchangeState::Requested(ref state) => Some(state.remote_info.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.remote_info.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.remote_info.clone()),
                },
            ActorDidExchangeState::Invitee(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(RemoteConnectionInfo::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.remote_info.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.remote_info.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.remote_info.clone()),
                }
        }
    }

    pub fn prev_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => state.prev_agent_info.as_ref(),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    Inviter,
    Invitee
}
