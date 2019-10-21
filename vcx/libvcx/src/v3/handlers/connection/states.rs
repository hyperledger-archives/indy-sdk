use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::ack::Ack;

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
    pub remote_info: Option<RemoteConnectionInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    pub remote_info: RemoteConnectionInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub remote_info: RemoteConnectionInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
    pub remote_info: RemoteConnectionInfo
}

impl From<NullState> for InvitedState {
    fn from(state: NullState) -> InvitedState {
        InvitedState {
            remote_info: None
        }
    }
}

impl From<(NullState, Invitation)> for InvitedState {
    fn from((state, invitation): (NullState, Invitation)) -> InvitedState {
        InvitedState {
            remote_info: Some(RemoteConnectionInfo::from(invitation))
        }
    }
}

impl From<(InvitedState, ProblemReport)> for NullState {
    fn from((state, error): (InvitedState, ProblemReport)) -> NullState {
        NullState {}
    }
}

impl From<(InvitedState, Request)> for RequestedState {
    fn from((state, request): (InvitedState, Request)) -> RequestedState {
        RequestedState { remote_info: RemoteConnectionInfo::from(request) }
    }
}

impl From<InvitedState> for RequestedState {
    fn from(state: InvitedState) -> RequestedState {
        RequestedState { remote_info: state.remote_info.unwrap() }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        NullState {}
    }
}

impl From<RequestedState> for RespondedState {
    fn from(state: RequestedState) -> RespondedState {
        RespondedState {
            remote_info: state.remote_info,
        }
    }
}

impl From<(RequestedState, Response)> for RespondedState {
    fn from((state, response): (RequestedState, Response)) -> RespondedState {
        let mut remote_info = RemoteConnectionInfo::from(response);
        remote_info.label = state.remote_info.label.clone();
        RespondedState { remote_info }
    }
}

impl From<(RespondedState, ProblemReport)> for InvitedState {
    fn from((state, error): (RespondedState, ProblemReport)) -> InvitedState {
        InvitedState { remote_info: Some(state.remote_info) }
    }
}

impl From<(RespondedState, Ack)> for CompleteState {
    fn from((state, ack): (RespondedState, Ack)) -> CompleteState {
        CompleteState { remote_info: state.remote_info }
    }
}

impl From<RespondedState> for CompleteState {
    fn from(state: RespondedState) -> CompleteState {
        CompleteState { remote_info: state.remote_info }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Messages {
    Error(ProblemReport),
    InvitationSent(Invitation),
    InvitationReceived(Invitation),
    ExchangeRequestSent(Request),
    ExchangeRequestReceived(Request),
    ExchangeResponseSent(Response),
    ExchangeResponseReceived(Response),
    AckSent(Ack),
    AckReceived(Ack),
}

impl DidExchangeState {
    pub fn state(&self) -> u32 {
        match self {
            DidExchangeState::Null(_) => 1,
            DidExchangeState::Invited(_) => 2,
            DidExchangeState::Requested(_) => 3,
            DidExchangeState::Responded(_) => 5, // for backward compatibility
            DidExchangeState::Complete(_) => 4,
        }
    }

    pub fn step(&self, message: Messages) -> DidExchangeState {
        let state = self.clone();
        match state {
            DidExchangeState::Null(state) => {
                match message {
                    Messages::InvitationSent(invitation) => DidExchangeState::Invited(state.into()),
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
                    Messages::ExchangeResponseSent(response) => DidExchangeState::Responded(state.into()),
                    Messages::ExchangeResponseReceived(response) => DidExchangeState::Responded((state, response).into()),
                    _ => DidExchangeState::Requested(state)
                }
            }
            DidExchangeState::Responded(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Invited((state, error).into()),
                    Messages::AckSent(ack) => DidExchangeState::Complete(state.into()),
                    Messages::AckReceived(ack) => DidExchangeState::Complete((state, ack).into()),
                    _ => DidExchangeState::Responded(state)
                }
            }
            DidExchangeState::Complete(state) => DidExchangeState::Complete(state)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    Inviter,
    Invitee
}
