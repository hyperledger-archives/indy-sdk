use api::VcxStateType;

use v3::handlers::connection::messages::DidExchangeMessages;
use v3::messages::a2a::A2AMessage;
use v3::handlers::connection::agent::AgentInfo;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::{Response, SignedResponse};
use v3::messages::connection::problem_report::{ProblemReport, ProblemCode};
use v3::messages::connection::ping::Ping;
use v3::messages::connection::ping_response::PingResponse;
use v3::messages::ack::Ack;
use v3::messages::connection::did_doc::DidDoc;

use std::collections::HashMap;
use v3::messages::a2a::MessageId;

use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidExchangeSM {
    source_id: String,
    agent_info: AgentInfo,
    state: ActorDidExchangeState
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

impl DidExchangeState {
    pub fn code(&self) -> u32 {
        match self {
            DidExchangeState::Null(_) => VcxStateType::VcxStateInitialized as u32,
            DidExchangeState::Invited(_) => VcxStateType::VcxStateOfferSent as u32,
            DidExchangeState::Requested(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Responded(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Completed(_) => VcxStateType::VcxStateAccepted as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullState {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedState {
    invitation: Invitation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    request: Request,
    did_doc: DidDoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    response: SignedResponse,
    did_doc: DidDoc,
    prev_agent_info: AgentInfo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
    did_doc: DidDoc,
    pending_messages: HashMap<MessageId, String>
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
        RequestedState { request, did_doc: DidDoc::from(state.invitation) }
    }
}

impl From<(InvitedState, Request, SignedResponse, AgentInfo)> for RespondedState {
    fn from((state, request, response, prev_agent_info): (InvitedState, Request, SignedResponse, AgentInfo)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        RespondedState { response, did_doc: request.connection.did_doc, prev_agent_info }
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
        CompleteState { did_doc: response.connection.did_doc, pending_messages: HashMap::new() }
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
        CompleteState { did_doc: state.did_doc, pending_messages: HashMap::new() }
    }
}

impl From<(RespondedState, Ping)> for CompleteState {
    fn from((state, ping): (RespondedState, Ping)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        CompleteState { did_doc: state.did_doc, pending_messages: HashMap::new() }
    }
}

impl InvitedState {
    fn handle_connection_request(&self, request: &Request,
                                 agent_info: &AgentInfo) -> VcxResult<(SignedResponse, AgentInfo)> {
        trace!("InvitedState:handle_connection_request >>> request: {:?}, agent_info: {:?}", request, agent_info);

        request.connection.did_doc.validate()?;

        let prev_agent_info = agent_info.clone();

        // provision a new keys
        let new_agent_info: AgentInfo = agent_info.create_agent()?;

        let response = Response::create()
            .set_did(new_agent_info.pw_did.to_string())
            .set_service_endpoint(new_agent_info.agency_endpoint()?)
            .set_keys(new_agent_info.recipient_keys(), new_agent_info.routing_keys()?);

        let signed_response = response.clone()
            .set_thread_id(request.id.0.clone())
            .encode(&prev_agent_info.pw_vk)?;

        new_agent_info.send_message(&signed_response.to_a2a_message(), &request.connection.did_doc)?;

        Ok((signed_response, new_agent_info))
    }
}

impl RequestedState {
    fn handle_connection_response(&self, response: SignedResponse, agent_info: &AgentInfo) -> VcxResult<Response> {
        trace!("RequestedState:handle_connection_response >>> response: {:?}, agent_info: {:?}", response, agent_info);

        let remote_vk: String = self.did_doc.recipient_keys().get(0).cloned()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot handle Response: Remote Verkey not found"))?;

        let response: Response = response.decode(&remote_vk)?;

        if !response.thread.is_reply(&self.request.id.0) {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot handle Response: thread id does not match: {:?}", response.thread)));
        }

        let ack = Ack::create().set_thread_id(response.thread.thid.clone().unwrap_or_default());
        agent_info.send_message(&ack.to_a2a_message(), &response.connection.did_doc)?;

        Ok(response)
    }
}

impl RespondedState {
    fn handle_ping(&self, ping: &Ping, agent_info: &AgentInfo) -> VcxResult<()> {
        _handle_ping(ping, agent_info, &self.did_doc)
    }
}

impl CompleteState {
    fn handle_ping(&self, ping: &Ping, agent_info: &AgentInfo) -> VcxResult<()> {
        _handle_ping(ping, agent_info, &self.did_doc)
    }
}

fn _handle_ping(ping: &Ping, agent_info: &AgentInfo, did_doc: &DidDoc) -> VcxResult<()> {
    if ping.response_requested {
        let ping_response = PingResponse::create().set_thread_id(
            ping.thread.as_ref().and_then(|thread| thread.thid.clone()).unwrap_or(ping.id.0.clone()));
        agent_info.send_message(&ping_response.to_a2a_message(), did_doc)?;
    }
    Ok(())
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

    pub fn from(source_id: String, agent_info: AgentInfo, state: ActorDidExchangeState) -> Self {
        DidExchangeSM {
            source_id,
            agent_info,
            state,
        }
    }

    pub fn agent_info(&self) -> &AgentInfo {
        &self.agent_info
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn state(&self) -> u32 {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) | ActorDidExchangeState::Invitee(ref state) => state.code(),
        }
    }

    pub fn state_object<'a>(&'a self) -> &'a ActorDidExchangeState {
        &self.state
    }

    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("DidExchangeSM::find_message_to_handle >>> messages: {:?}", messages);

        for (uid, message) in messages {
            match self.state {
                ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) => {
                    match message {
                        request @ A2AMessage::ConnectionRequest(_) => {
                            debug!("Inviter received ConnectionRequest message");
                            return Some((uid, request));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("Inviter received ProblemReport message");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Inviter received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Invitee(DidExchangeState::Requested(ref state)) => {
                    match message {
                        response @ A2AMessage::ConnectionResponse(_) => {
                            debug!("Invitee received ConnectionResponse message");
                            return Some((uid, response));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("Invitee received ProblemReport message");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Invitee received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => {
                    match message {
                        ack @ A2AMessage::Ack(_) => {
                            debug!("Ack message received");
                            return Some((uid, ack));
                        }
                        ping @ A2AMessage::Ping(_) => {
                            debug!("Ping message received");
                            return Some((uid, ping));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("ProblemReport message received");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Unexpected message received in Responded state: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref state)) |
                ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref state)) => {
                    match message {
                        ping @ A2AMessage::Ping(_) => {
                            debug!("Ping message received");
                            return Some((uid, ping));
                        }
                        message @ _ => {
                            debug!("Unexpected message received in Completed state: {:?}", message);
                        }
                    }
                }
                _ => {
                    debug!("Unexpected message received: message: {:?}", message);
                }
            }
        }

        None
    }

    pub fn step(self, message: DidExchangeMessages) -> VcxResult<DidExchangeSM> {
        trace!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeSM { source_id, mut agent_info, state } = self;

        let state = match state {
            ActorDidExchangeState::Inviter(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            DidExchangeMessages::Connect() => {
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
                                            .set_thread_id(request.id.0.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &request.connection.did_doc).ok(); // IS is possible?
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
                                state.handle_ping(&ping, &agent_info)?;
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
                        match message {
                            DidExchangeMessages::PingReceived(ping) => {
                                state.handle_ping(&ping, &agent_info)?;
                                ActorDidExchangeState::Inviter(DidExchangeState::Completed(state))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Completed(state))
                            }
                        }
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
                            DidExchangeMessages::Connect() => {
                                agent_info = agent_info.create_agent()?;

                                let request = Request::create()
                                    .set_label(source_id.to_string())
                                    .set_did(agent_info.pw_did.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_keys(agent_info.recipient_keys(), agent_info.routing_keys()?);

                                agent_info.send_message(&request.to_a2a_message(), &DidDoc::from(state.invitation.clone()))?;
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
                                            .set_thread_id(state.request.id.0.clone());
                                        agent_info.send_message(&problem_report.to_a2a_message(), &state.did_doc).ok();
                                        ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report).into()))
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
                        match message {
                            DidExchangeMessages::PingReceived(ping) => {
                                state.handle_ping(&ping, &agent_info)?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Completed(state))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Completed(state))
                            }
                        }
                    }
                }
            }
        };
        Ok(DidExchangeSM { source_id, agent_info, state })
    }

    pub fn did_doc(&self) -> Option<DidDoc> {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(DidDoc::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.did_doc.clone()),
                },
            ActorDidExchangeState::Invitee(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(DidDoc::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.did_doc.clone()),
                }
        }
    }

    pub fn get_invitation(&self) -> Option<&Invitation> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Invited(ref state)) => Some(&state.invitation),
            _ => None
        }
    }

    pub fn remote_did(&self) -> VcxResult<String> {
        self.did_doc()
            .map(|did_doc: DidDoc| did_doc.id.clone())
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Remote Connection DID is not set"))
    }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.did_doc()
            .and_then(|did_doc| did_doc.recipient_keys().get(0).cloned())
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Remote Connection Verkey is not set"))
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Actor {
    Inviter,
    Invitee
}

#[cfg(test)]
pub mod test {
    use super::*;

    use v3::test::source_id;
    use v3::test::setup::{TestModeSetup, AgencyModeSetup};
    use v3::messages::connection::invite::tests::_invitation;
    use v3::messages::connection::request::tests::_request;
    use v3::messages::connection::response::tests::_signed_response;
    use v3::messages::connection::problem_report::tests::_problem_report;
    use v3::messages::connection::ping::tests::_ping;
    use v3::messages::ack::tests::_ack;

    pub mod inviter {
        use super::*;

        pub fn inviter_sm() -> DidExchangeSM {
            DidExchangeSM::new(Actor::Inviter, &source_id())
        }

        impl DidExchangeSM {
            fn to_inviter_invited_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self
            }

            fn to_inviter_responded_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                self
            }

            fn to_inviter_completed_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                self = self.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                self
            }
        }

        mod new {
            use super::*;

            #[test]
            fn test_inviter_new() {
                let _setup = TestModeSetup::init();

                let inviter_sm = inviter_sm();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), inviter_sm.state);
                assert_eq!(source_id(), inviter_sm.source_id());
            }
        }

        mod step {
            use super::*;

            #[test]
            fn test_did_exchange_init() {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = inviter_sm();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_exchange_request_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Responded(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invalid_exchange_request_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                let mut request = _request();
                request.connection.did_doc = DidDoc::default();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(request)).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_ack_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();


                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_ping_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Responded(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_any_message_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }
        }

        mod find_message_to_handle {
            use super::*;

            #[test]
            fn test_find_message_to_handle_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm();

                // No messages
                {
                    let messages = map!(
                    "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                    "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                    "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                    "key_4".to_string() => A2AMessage::Ping(_ping()),
                    "key_5".to_string() => A2AMessage::Ack(_ack())
                );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_invited_state();

                // Connection Request
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::ConnectionRequest(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_responded_state();

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_1", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }

                // Ack
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::Ack(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_completed_state();

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }
            }
        }

        mod get_state {
            use super::*;

            #[test]
            fn test_get_state() {
                let _setup = TestModeSetup::init();

                assert_eq!(VcxStateType::VcxStateInitialized as u32, inviter_sm().state());
                assert_eq!(VcxStateType::VcxStateOfferSent as u32, inviter_sm().to_inviter_invited_state().state());
                assert_eq!(VcxStateType::VcxStateRequestReceived as u32, inviter_sm().to_inviter_responded_state().state());
                assert_eq!(VcxStateType::VcxStateAccepted as u32, inviter_sm().to_inviter_completed_state().state());
            }
        }
    }

    pub mod invitee {
        use super::*;

        use v3::messages::connection::did_doc::tests::_service_endpoint;
        use ::utils::libindy::tests::create_key;

        pub fn invitee_sm() -> DidExchangeSM {
            DidExchangeSM::new(Actor::Invitee, &source_id())
        }

        impl DidExchangeSM {
            pub fn to_invitee_invited_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                self
            }

            pub fn to_invitee_requested_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self
            }

            pub fn to_invitee_completed_state(mut self, wallet_handle: i32) -> DidExchangeSM {
                let key = create_key(wallet_handle, Some(::utils::libindy::tests::test_setup::TRUSTEE_SEED));
                let invitation = Invitation::default().set_recipient_keys(vec![key.clone()]);

                self = self.step(DidExchangeMessages::InvitationReceived(invitation)).unwrap();
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeResponseReceived(_response(&key))).unwrap();
                self = self.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                self
            }
        }

        fn _response(key: &str) -> SignedResponse {
            Response::default()
                .set_service_endpoint(_service_endpoint())
                .set_keys(vec![key.to_string()], vec![])
                .set_thread_id(_request().id.0.clone())
                .encode(&key).unwrap()
        }

        mod new {
            use super::*;

            #[test]
            fn test_invitee_new() {
                let _setup = TestModeSetup::init();

                let invitee_sm = invitee_sm();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), invitee_sm.state);
                assert_eq!(source_id(), invitee_sm.source_id());
            }
        }

        mod step {
            use super::*;

            #[test]
            fn test_did_exchange_init() {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = invitee_sm();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invite_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_response_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let key = create_key(_setup.wallet_handle, Some(::utils::libindy::tests::test_setup::TRUSTEE_SEED));

                let mut did_exchange_sm = invitee_sm();

                let invitation = Invitation::default().set_recipient_keys(vec![key.clone()]);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(invitation)).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeResponseReceived(_response(&key))).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invalid_response_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                let mut signed_response = _signed_response();
                signed_response.connection_sig.signature = String::from("other");

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeResponseReceived(signed_response)).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_messages_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let key = create_key(_setup.wallet_handle, Some(::utils::libindy::tests::test_setup::TRUSTEE_SEED));

                let mut did_exchange_sm = invitee_sm();

                let invitation = Invitation::default().set_recipient_keys(vec![key.clone()]);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(invitation)).unwrap();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeResponseReceived(_response(&key))).unwrap();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }
        }

        mod find_message_to_handle {
            use super::*;

            #[test]
            fn test_find_message_to_handle_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_invited_state();

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_requested_state();

                // Connection Response
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionResponse(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_completed_state() {
                let setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_completed_state(setup.wallet_handle);

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }
            }
        }

        mod get_state {
            use super::*;

            #[test]
            fn test_get_state() {
                let _setup = TestModeSetup::init();

                assert_eq!(VcxStateType::VcxStateInitialized as u32, invitee_sm().state());
                assert_eq!(VcxStateType::VcxStateOfferSent as u32, invitee_sm().to_invitee_invited_state().state());
                assert_eq!(VcxStateType::VcxStateRequestReceived as u32, invitee_sm().to_invitee_requested_state().state());
            }
        }
    }
}

