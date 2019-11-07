
use v3::handlers::connection::agent::AgentInfo;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::{Response, SignedResponse};
use v3::messages::connection::problem_report::{ProblemReport, ProblemCode};
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::connection::ping::Ping;
use v3::messages::ack::Ack;

use std::collections::HashMap;
use v3::messages::MessageId;

use messages::thread::Thread;
use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidExchangeSM {
    source_id: String,
    agent_info: AgentInfo,
    pub state: ActorDidExchangeState
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorDidExchangeState {
    Inviter(DidExchangeState),
    Invitee(DidExchangeState),
}

/// Transitions of Inviter Connection state
/// Null -> Invited
/// Invited -> Responded, Null
/// Responded -> Complete, Null
/// Completed
///
/// Transitions of Invitee Connection state
/// Null -> Invited
/// Invited -> Requested, Null
/// Requested -> Completed, Null
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    pub response: SignedResponse,
    pub remote_info: RemoteConnectionInfo,
    pub prev_agent_info: AgentInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
    pub remote_info: RemoteConnectionInfo,
    pub pending_messages: HashMap<MessageId, String>
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
        RequestedState { request, remote_info: RemoteConnectionInfo::from(state.invitation) }
    }
}

impl From<(InvitedState, Request, SignedResponse, AgentInfo)> for RespondedState {
    fn from((state, request, response, prev_agent_info): (InvitedState, Request, SignedResponse, AgentInfo)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RespondedState { response, remote_info: RemoteConnectionInfo::from(request), prev_agent_info }
    }
}

impl From<(RequestedState, ProblemReport)> for NullState {
    fn from((state, error): (RequestedState, ProblemReport)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RequestedState to NullState");
        NullState {}
    }
}

impl From<(RequestedState, Response)> for CompleteState {
    fn from((state, response): (RequestedState, Response)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        let mut remote_info = RemoteConnectionInfo::from(response);
        remote_info.set_label(state.remote_info.label);
        CompleteState { remote_info, pending_messages: HashMap::new() }
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
        CompleteState { remote_info: state.remote_info, pending_messages: HashMap::new() }
    }
}

impl From<(RespondedState, Ping)> for CompleteState {
    fn from((state, ping): (RespondedState, Ping)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        CompleteState { remote_info: state.remote_info, pending_messages: HashMap::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeMessages {
    SendInvitation(),
    SendExchangeRequest(),
    InvitationReceived(Invitation),
    ExchangeRequestReceived(Request),
    ExchangeResponseReceived(SignedResponse),
    AckReceived(Ack),
    ProblemReportReceived(ProblemReport),
    PingReceived(Ping),
}

impl InvitedState {
    fn handle_connection_request(&self, request: &Request,
                                 agent_info: &AgentInfo) -> VcxResult<(SignedResponse, AgentInfo)> {
        request.connection.did_doc.validate()?;

        let prev_agent_info = agent_info.clone();

        // provision a new keys
        let new_agent_info: AgentInfo = agent_info.create_agent()?;

        let response = Response::create()
            .set_did(new_agent_info.pw_did.to_string())
            .set_service_endpoint(new_agent_info.agency_endpoint()?)
            .set_keys(new_agent_info.recipient_keys(), new_agent_info.routing_keys()?);

        let signed_response = response.clone()
            .set_thread(Thread::new().set_thid(request.id.0.clone()))
            .encode(&prev_agent_info.pw_vk)?;

        new_agent_info.send_message(&signed_response.to_a2a_message(), &RemoteConnectionInfo::from(request.clone()))?;

        Ok((signed_response, new_agent_info))
    }
}

impl RequestedState {
    fn handle_connection_response(&self, response: SignedResponse, agent_info: &AgentInfo) -> VcxResult<Response> {
        let remote_vk: String = self.remote_info.recipient_keys.get(0).cloned()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Remote Verkey not found"))?;

        let response: Response = response.decode(&remote_vk)?;

        let ack = Ack::create().set_thread(response.thread.clone());
        agent_info.send_message(&ack.to_a2a_message(), &RemoteConnectionInfo::from(response.clone()))?;

        Ok(response)
    }
}

impl DidExchangeSM {
    pub fn new(actor: Actor, source_id: &str) -> Self {
        match actor {
            Actor::Inviter => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
                    state: ActorDidExchangeState::Inviter(DidExchangeState::Null(NullState {})),
                    agent_info: AgentInfo::default(),
                }
            }
            Actor::Invitee => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
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

    pub fn source_id(&self) -> &str {
        &self.source_id
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

    pub fn step(self, message: DidExchangeMessages) -> VcxResult<DidExchangeSM> {
        trace!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeSM { source_id, mut agent_info, state } = self;

        let state = match state {
            ActorDidExchangeState::Inviter(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            DidExchangeMessages::SendInvitation() => {
                                agent_info = agent_info.create_agent()?;

                                let invite: Invitation = Invitation::create()
                                    .set_label(source_id.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_recipient_keys(agent_info.recipient_keys())
                                    .set_routing_keys(agent_info.routing_keys()?);

                                ActorDidExchangeState::Inviter(DidExchangeState::Invited((state, invite).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::ExchangeRequestReceived(request) => {
                                match state.handle_connection_request(&request, &agent_info) {
                                    Ok((response, new_agent_info)) => {
                                        let prev_agent_info = agent_info.clone();
                                        agent_info = new_agent_info;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, request, response, prev_agent_info).into()))
                                    }
                                    Err(err) => {
                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::RequestProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(Thread::new().set_thid(request.id.0.clone()));

                                        agent_info.send_message(&problem_report.to_a2a_message(), &RemoteConnectionInfo::from(request))?;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        ActorDidExchangeState::Inviter(DidExchangeState::Requested(state))
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            DidExchangeMessages::AckReceived(ack) => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ack).into()))
                            }
                            DidExchangeMessages::PingReceived(ping) => {
                                if ping.response_requested {
                                    let ping = Ping::create().set_thread(
                                        ping.thread.clone()
                                            .unwrap_or(Thread::new().set_thid(ping.id.0.clone())));
                                    agent_info.send_message(&ping.to_a2a_message(), &state.remote_info)?;
                                }

                                ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ping).into()))
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
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
                            DidExchangeMessages::InvitationReceived(invitation) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::SendExchangeRequest() => {
                                agent_info = agent_info.create_agent()?;

                                let request = Request::create()
                                    .set_label(source_id.to_string())
                                    .set_did(agent_info.pw_did.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_keys(agent_info.recipient_keys(), agent_info.routing_keys()?);

                                agent_info.send_message(&request.to_a2a_message(), &RemoteConnectionInfo::from(state.invitation.clone()))?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested((state, request).into()))
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            DidExchangeMessages::ExchangeResponseReceived(response) => {
                                match state.handle_connection_response(response, &agent_info) {
                                    Ok(response) => {
                                        ActorDidExchangeState::Invitee(DidExchangeState::Completed((state, response).into()))
                                    }
                                    Err(err) => {
                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::ResponseProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(Thread::new().set_thid(state.request.id.0.clone()));

                                        agent_info.send_message(&problem_report.to_a2a_message(), &state.remote_info)?;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested(state))
                            }
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Responded(state))
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Completed(state))
                    }
                }
            }
        };
        Ok(DidExchangeSM { source_id, agent_info, state })
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
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => Some(&state.prev_agent_info),
            _ => None
        }
    }

    pub fn actor(&self) -> Actor {
        match self.state {
            ActorDidExchangeState::Inviter(_) => Actor::Inviter,
            ActorDidExchangeState::Invitee(_) => Actor::Invitee
        }
    }

    pub fn add_pending_messages(&mut self, messages: HashMap<MessageId, String>) {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref mut state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref mut state)) => {
                state.pending_messages.extend(messages)
            }
            _ => {}
        };
    }

    pub fn remove_pending_message(&mut self, id: MessageId) -> VcxResult<()> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref mut state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref mut state)) => {
                if let Some(uid) = state.pending_messages.remove(&id) {
                    return self.agent_info.update_message_status(uid);
                }
            }
            _ => {}
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    Inviter,
    Invitee
}


