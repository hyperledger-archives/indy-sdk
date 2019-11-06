use messages;
use messages::ObjectWithVersion;
use messages::get_message::Message;
use error::prelude::*;
use messages::update_connection::send_delete_connection_message;

use v3::handlers::connection::states::*;
use v3::messages::{A2AMessage, MessageId};
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::agent_info::AgentInfo;

use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    state: DidExchangeSM
}

impl Connection {
    const SERIALIZE_VERSION: &'static str = "2.0";

    pub fn create(source_id: &str, actor: Actor) -> Connection {
        trace!("Connection::create >>> source_id: {}, actor: {:?}", source_id, actor);

        Connection {
            state: DidExchangeSM::new(actor, source_id),
        }
    }

    pub fn state(&self) -> u32 { self.state.state() }

    fn actor(&self) -> Actor { self.state.actor() }

    pub fn agent_info(&self) -> &AgentInfo { self.state.agent_info() }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.state.remote_vk()
    }

    pub fn get_source_id(&self) -> VcxResult<String> {
        Ok(self.state.source_id().to_string())
    }

    pub fn process_invite(self, invitation: Invitation) -> VcxResult<Connection> {
        self.step(Messages::InvitationReceived(invitation))
    }

    pub fn get_invite_details(&self) -> VcxResult<String> {
        if let Some(invitation) = self.state.get_invitation() {
            return Ok(json!(invitation.to_a2a_message()).to_string());
        } else if let Some(remote_info) = self.state.remote_connection_info() {
            return Ok(json!(remote_info).to_string());
        } else {
            Ok(json!({}).to_string())
        }
    }

    pub fn connect(self) -> VcxResult<Connection> {
        trace!("Connection::connect >>> source_id: {}", self.state.source_id());

        let message = match self.actor() {
            Actor::Inviter => Messages::SendInvitation(),
            Actor::Invitee => Messages::SendExchangeRequest()
        };

        self.step(message)
    }

    pub fn update_state(mut self, message: Option<&str>) -> VcxResult<Connection> {
        trace!("Connection: update_state");

        match message {
            Some(message_) => {
                self = self.update_state_with_message(message_)?
            }
            None => {
                let messages = self.get_messages()?;
                let agent_info = self.agent_info().clone();

                if let Some((uid, message)) = self.find_message_to_handle(messages) {
                    self = self.handle_message(message)?;
                    agent_info.update_message_status(uid)?;
                };

                if let Some(prev_agent_info) = self.state.prev_agent_info().cloned() {
                    let messages = prev_agent_info.get_messages()?;

                    if let Some((uid, message)) = self.find_message_to_handle(messages) {
                        self = self.handle_message(message)?;
                        prev_agent_info.update_message_status(uid)?;
                    }
                }
            }
        };

        Ok(self)
    }

    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, Messages)> {
        trace!("Prover::get_message_to_handle >>> messages: {:?}", messages);

        for (uid, message) in messages {
            match self.state.state {
                ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) => {
                    match message {
                        A2AMessage::ConnectionRequest(request) => {
                            debug!("Inviter received ConnectionRequest message");
                            return Some((uid, Messages::ExchangeRequestReceived(request)));
                        }
                        A2AMessage::ConnectionProblemReport(problem_report) => {
                            debug!("Inviter received ProblemReport message");
                            return Some((uid, Messages::ProblemReportReceived(problem_report)));
                        }
                        message @ _ => {
                            debug!("Inviter received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Invitee(DidExchangeState::Requested(ref state)) => {
                    match message {
                        A2AMessage::ConnectionResponse(response) => {
                            debug!("Invitee received ConnectionResponse message");
                            return Some((uid, Messages::ExchangeResponseReceived(response)));
                        }
                        A2AMessage::ConnectionProblemReport(problem_report) => {
                            debug!("Invitee received ProblemReport message");
                            return Some((uid, Messages::ProblemReportReceived(problem_report)));
                        }
                        message @ _ => {
                            debug!("Invitee received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => {
                    match message {
                        A2AMessage::Ack(ack) => {
                            debug!("Ack message received");
                            return Some((uid, Messages::AckReceived(ack)));
                        }
                        A2AMessage::Ping(ping) => {
                            debug!("Ping message received");
                            return Some((uid, Messages::PingReceived(ping)));
                        }
                        A2AMessage::ConnectionProblemReport(problem_report) => {
                            debug!("ProblemReport message received");
                            return Some((uid, Messages::ProblemReportReceived(problem_report)));
                        }
                        message @ _ => {
                            debug!("Unexpected message received in Responded state: {:?}", message);
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

    pub fn update_message_status(&self, uid: String) -> VcxResult<()> {
        self.state.agent_info().update_message_status(uid)
    }

    pub fn update_state_with_message(mut self, message: &str) -> VcxResult<Connection> {
        trace!("Connection: update_state_with_message: {}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let mut messages: HashMap<String, A2AMessage> = HashMap::new();
        messages.insert(message.uid.clone(), self.decode_message(&message)?);

        if let Some((uid, message)) = self.find_message_to_handle(messages) {
            self = self.handle_message(message)?;
            self.update_message_status(uid)?;
        }

        Ok(self)
    }

    pub fn get_messages(&self) -> VcxResult<HashMap<String, A2AMessage>> {
        trace!("Connection: get_messages");
        self.agent_info().get_messages()
    }

    pub fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        trace!("Connection: get_message_by_id");

        let agent_info = self.agent_info();

        let mut messages = messages::get_message::get_connection_messages(&agent_info.pw_did,
                                                                          &agent_info.pw_vk,
                                                                          &agent_info.agent_did,
                                                                          &agent_info.agent_vk,
                                                                          Some(vec![msg_id.to_string()]),
                                                                          None)?;

        let message =
            messages
                .pop()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidMessages, format!("Message not found for id: {:?}", msg_id)))?;

        let message = self.decode_message(&message)?;

        Ok(message)
    }

    pub fn handle_message(self, message: Messages) -> VcxResult<Connection> {
        trace!("Connection: handle_message: {:?}", message);
        self.step(message)
    }

    pub fn decode_message(&self, message: &Message) -> VcxResult<A2AMessage> {
        self.agent_info().decode_message(message)
    }

    pub fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        self.state.send_message(message)
    }

    pub fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        self.send_message(&A2AMessage::Generic(message.to_string())).map(|_| String::new())
    }

    pub fn delete(&mut self) -> VcxResult<()> {
        trace!("Connection: delete: {:?}", self.state.source_id());
        send_delete_connection_message(&self.agent_info().pw_did, &self.agent_info().pw_vk, &self.agent_info().agent_did, &self.agent_info().agent_vk)
    }

    pub fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    pub fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    fn step(mut self, message: Messages) -> VcxResult<Connection> {
        self.state = self.state.step(message)?;
        Ok(self)
    }

    pub fn add_pending_messages(&mut self, messages: HashMap<MessageId, String>) -> VcxResult<()> {
        Ok(self.state.add_pending_messages(messages))
    }

    pub fn remove_pending_message(&mut self, id: MessageId) -> VcxResult<()> {
        self.state.remove_pending_message(id)
    }
}