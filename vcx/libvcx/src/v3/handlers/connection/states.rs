use v3::messages::connection::agent_info::AgentInfo;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::{Response, SignedResponse};
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::ack::Ack;
use v3::messages::A2AMessage;

use v3::utils::encryption_envelope::EncryptionEnvelope;
use utils::httpclient;

use messages::thread::Thread;

use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidExchangeStateSM {
    agent_info: AgentInfo,
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
    pub request: Request,
    pub remote_info: RemoteConnectionInfo,
    pub prev_agent_info: Option<AgentInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub response: Response,
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

impl From<(InvitedState, Request, AgentInfo)> for RequestedState {
    fn from((state, request, prev_agent_info): (InvitedState, Request, AgentInfo)) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RequestedState { request: request.clone(), remote_info: RemoteConnectionInfo::from(request), prev_agent_info: Some(prev_agent_info) }
    }
}

impl From<(InvitedState, Request)> for RequestedState {
    fn from((state, request): (InvitedState, Request)) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RequestedState { request, remote_info: RemoteConnectionInfo::from(state.invitation), prev_agent_info: None }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RequestedState to NullState");
        NullState {}
    }
}

impl From<(RequestedState, Response)> for RespondedState {
    fn from((state, response): (RequestedState, Response)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        let mut remote_info = RemoteConnectionInfo::from(response.clone());
        remote_info.set_label(state.remote_info.label);
        RespondedState { response, remote_info, prev_agent_info: state.prev_agent_info }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Messages {
    SendInvitation(Invitation),
    InvitationReceived(Invitation),
    ReceivedExchangeRequest(Request),
    SendExchangeRequest(Request),
    ReceivedExchangeResponse(SignedResponse),
    SendExchangeResponse(Response),
    AckReceived(Ack),
    SendAck(Ack),
    SendProblemReport(ProblemReport),
    ReceivedProblemReport(ProblemReport),
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
        trace!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeStateSM { agent_info, state } = self;

        let state = match state {
            ActorDidExchangeState::Inviter(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            Messages::SendInvitation(invitation) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Invited((state, invitation).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            Messages::ReceivedExchangeRequest(request) => {
                                request.connection.did_doc.validate()?;
                                ActorDidExchangeState::Inviter(DidExchangeState::Requested((state, request, agent_info.clone()).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            Messages::SendExchangeResponse(response) => {
                                let prev_agent_info: &AgentInfo = state.prev_agent_info.as_ref()
                                    .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot get previous AgentInfo"))?;

                                let signed_response = response.clone()
                                    .set_thread(Thread::new().set_thid(state.request.id.0.clone()))
                                    .encode(&prev_agent_info.pw_vk)?;

                                send_message(&signed_response.to_a2a_message(), &state.remote_info, &agent_info.pw_vk)?;
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, response).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            Messages::SendProblemReport(problem_report) => {
                                let problem_report = problem_report
                                    .set_thread(Thread::new().set_thid(state.request.id.0.clone()));
                                send_message(&problem_report.to_a2a_message(), &state.remote_info, &agent_info.pw_vk)?;
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => ActorDidExchangeState::Inviter(DidExchangeState::Requested(state))
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            Messages::AckReceived(ack) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ack).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded(state))
                            }
                        }
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Inviter(DidExchangeState::Completed(state))
                    }
                }
            }
            ActorDidExchangeState::Invitee(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            Messages::InvitationReceived(invitation) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            Messages::SendExchangeRequest(request) => {
                                send_message(&request.to_a2a_message(), &RemoteConnectionInfo::from(state.invitation.clone()), &agent_info.pw_vk)?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested((state, request).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            Messages::ReceivedExchangeResponse(response) => {
                                let remote_vk: String = state.remote_info.recipient_keys.get(0).cloned()
                                    .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Remote Verkey not found"))?;

                                let response: Response = response.decode(&remote_vk)?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Responded((state, response).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested(state))
                            }
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            Messages::SendAck(ack) => {
                                let ack = ack.set_thread(state.response.thread.clone());
                                send_message(&ack.to_a2a_message(), &state.remote_info, &agent_info.pw_vk)?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Completed((state, ack).into()))
                            }
                            Messages::ReceivedProblemReport(problem_report) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
                            }
                            Messages::SendProblemReport(problem_report) => {
                                let problem_report = problem_report.
                                    set_thread(state.response.thread.clone());
                                send_message(&problem_report.to_a2a_message(), &state.remote_info, &agent_info.pw_vk)?;
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Responded(state))
                            }
                        }
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Completed(state))
                    }
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
                    DidExchangeState::Invited(ref state) => Some(RemoteConnectionInfo::from(state.invitation.clone())),
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

    pub fn get_invitation(&self) -> Option<&Invitation> {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(&state.invitation),
                    DidExchangeState::Requested(ref state) => None,
                    DidExchangeState::Responded(ref state) => None,
                    DidExchangeState::Completed(ref state) => None,
                },
            ActorDidExchangeState::Invitee(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(&state.invitation),
                    DidExchangeState::Requested(ref state) => None,
                    DidExchangeState::Responded(ref state) => None,
                    DidExchangeState::Completed(ref state) => None,
                }
        }
    }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.remote_connection_info()
            .and_then(|remote_info| remote_info.recipient_keys.get(0).cloned())
            .ok_or(VcxError::from(VcxErrorKind::NotReady))
    }

    pub fn prev_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => state.prev_agent_info.as_ref(),
            _ => None
        }
    }

    pub fn actor(&self) -> Actor {
        match self.state {
            ActorDidExchangeState::Inviter(_) => Actor::Inviter,
            ActorDidExchangeState::Invitee(_) => Actor::Invitee
        }
    }

    pub fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        let remote_connection_info = self.remote_connection_info()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot get Remote Connection information"))?;

        let envelope = EncryptionEnvelope::create(&message, &self.agent_info.pw_vk, &remote_connection_info)?;

        send_message(message, &remote_connection_info, &self.agent_info.pw_vk)
    }
}

fn send_message(message: &A2AMessage, remote_connection_info: &RemoteConnectionInfo, pw_vk: &str) -> VcxResult<()> {
    let envelope = EncryptionEnvelope::create(&message, &pw_vk, &remote_connection_info)?;
    httpclient::post_message(&envelope.0, &remote_connection_info.service_endpoint)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    Inviter,
    Invitee
}
