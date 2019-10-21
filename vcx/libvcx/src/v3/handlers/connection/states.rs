use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::problem_report::ProblemReport;
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
    pub invitation: Invitation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    pub request: Request
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub response: Response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {}



impl From<(NullState, Invitation)> for InvitedState {
    fn from((state, invitation): (NullState, Invitation)) -> InvitedState {
        InvitedState { invitation }
    }
}

impl From<(InvitedState, ProblemReport)> for NullState {
    fn from((state, error): (InvitedState, ProblemReport)) -> NullState {
        NullState {}
    }
}

impl From<(InvitedState, Request)> for RequestedState {
    fn from((state, request): (InvitedState, Request)) -> RequestedState {
        RequestedState { request }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        NullState {}
    }
}

impl From<(RequestedState, Response)> for RespondedState {
    fn from((state, response): (RequestedState, Response)) -> RespondedState {
        RespondedState { response }
    }
}

impl From<(RespondedState, ProblemReport)> for InvitedState {
    fn from((state, error): (RespondedState, ProblemReport)) -> InvitedState {
        InvitedState { invitation: Invitation::create() }
    }
}

impl From<(RespondedState, Ack)> for CompleteState {
    fn from((state, response): (RespondedState, Ack)) -> CompleteState {
        CompleteState {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Messages {
    Error(ProblemReport),
    Invitation(Invitation),
    ExchangeRequest(Request),
    ExchangeResponse(Response),
    Ack(Ack)
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
                    Messages::Invitation(invitation) => DidExchangeState::Invited((state, invitation).into()),
                    _ => DidExchangeState::Null(state)
                }
            }
            DidExchangeState::Invited(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Null((state, error).into()),
                    Messages::ExchangeRequest(request) => DidExchangeState::Requested((state, request).into()),
                    _ => DidExchangeState::Invited(state)
                }
            }
            DidExchangeState::Requested(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Null((state, error).into()),
                    Messages::ExchangeResponse(response) => DidExchangeState::Responded((state, response).into()),
                    _ => DidExchangeState::Requested(state)
                }
            }
            DidExchangeState::Responded(state) => {
                match message {
                    Messages::Error(error) => DidExchangeState::Invited((state, error).into()),
                    Messages::Ack(ack) => DidExchangeState::Complete((state, ack).into()),
                    _ => DidExchangeState::Responded(state)
                }
            }
            DidExchangeState::Complete(state) => DidExchangeState::Complete(state)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Actor {
    Inviter,
    Invitee
}
