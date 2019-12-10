use messages::get_message::Message;
use error::prelude::*;

use v3::handlers::connection::states::{DidExchangeSM, Actor, ActorDidExchangeState};
use v3::handlers::connection::messages::DidExchangeMessages;
use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::connection::invite::Invitation;
use v3::messages::trust_ping::ping::Ping;
use v3::handlers::connection::agent::AgentInfo;

use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    connection_sm: DidExchangeSM
}

impl Connection {
    pub fn create(source_id: &str) -> Connection {
        trace!("Connection::create >>> source_id: {}", source_id);

        Connection {
            connection_sm: DidExchangeSM::new(Actor::Inviter, source_id),
        }
    }

    pub fn from_parts(source_id: String, agent_info: AgentInfo, state: ActorDidExchangeState) -> Connection {
        Connection { connection_sm: DidExchangeSM::from(source_id, agent_info, state) }
    }

    pub fn create_with_invite(source_id: &str, invitation: Invitation) -> VcxResult<Connection> {
        trace!("Connection::create_with_invite >>> source_id: {}", source_id);

        let mut connection = Connection {
            connection_sm: DidExchangeSM::new(Actor::Invitee, source_id),
        };

        connection.process_invite(invitation)?;

        Ok(connection)
    }

    pub fn source_id(&self) -> String { self.connection_sm.source_id().to_string() }

    pub fn state(&self) -> u32 { self.connection_sm.state() }

    pub fn agent_info(&self) -> &AgentInfo { self.connection_sm.agent_info() }

    pub fn remote_did(&self) -> VcxResult<String> {
        self.connection_sm.remote_did()
    }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.connection_sm.remote_vk()
    }

    pub fn state_object<'a>(&'a self) -> &'a ActorDidExchangeState {
        &self.connection_sm.state_object()
    }

    pub fn get_source_id(&self) -> String {
        self.connection_sm.source_id().to_string()
    }

    pub fn process_invite(&mut self, invitation: Invitation) -> VcxResult<()> {
        trace!("Connection::process_invite >>> invitation: {:?}", invitation);
        self.step(DidExchangeMessages::InvitationReceived(invitation))
    }

    pub fn get_invite_details(&self) -> VcxResult<String> {
        trace!("Connection::get_invite_details >>>");
        if let Some(invitation) = self.connection_sm.get_invitation() {
            return Ok(json!(invitation.to_a2a_message()).to_string());
        } else if let Some(did_doc) = self.connection_sm.did_doc() {
            return Ok(json!(Invitation::from(did_doc)).to_string());
        } else {
            Ok(json!({}).to_string())
        }
    }

    pub fn actor(&self) -> Actor {
        self.connection_sm.actor()
    }

    pub fn connect(&mut self) -> VcxResult<()> {
        trace!("Connection::connect >>> source_id: {}", self.connection_sm.source_id());
        self.step(DidExchangeMessages::Connect())
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        trace!("Connection::update_state >>> message: {:?}", message);

        if let Some(message_) = message {
            return self.update_state_with_message(message_);
        }

        let messages = self.get_messages()?;
        let agent_info = self.agent_info().clone();

        if let Some((uid, message)) = self.connection_sm.find_message_to_handle(messages) {
            self.handle_message(message.into())?;
            agent_info.update_message_status(uid)?;
        };

        if let Some(prev_agent_info) = self.connection_sm.prev_agent_info().cloned() {
            let messages = prev_agent_info.get_messages()?;

            if let Some((uid, message)) = self.connection_sm.find_message_to_handle(messages) {
                self.handle_message(message.into())?;
                prev_agent_info.update_message_status(uid)?;
            }
        }

        Ok(())
    }

    pub fn update_message_status(&self, uid: String) -> VcxResult<()> {
        trace!("Connection::update_message_status >>> uid: {:?}", uid);
        self.connection_sm.agent_info().update_message_status(uid)
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Connection: update_state_with_message: {}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption,
                                              format!("Cannot updated state with messages: Message deserialization failed: {:?}", err)))?;

        let a2a_message = self.decode_message(&message)?;
        self.handle_message(a2a_message.into())?;
        self.update_message_status(message.uid)?;

        Ok(())
    }

    pub fn get_messages(&self) -> VcxResult<HashMap<String, A2AMessage>> {
        trace!("Connection: get_messages >>>");
        self.agent_info().get_messages()
    }

    pub fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        trace!("Connection: get_message_by_id >>>");
        self.agent_info().get_message_by_id(msg_id)
    }

    pub fn handle_message(&mut self, message: DidExchangeMessages) -> VcxResult<()> {
        trace!("Connection: handle_message >>> {:?}", message);
        self.step(message)
    }

    pub fn decode_message(&self, message: &Message) -> VcxResult<A2AMessage> {
        self.agent_info().decode_message(message)
    }

    pub fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        trace!("Connection::send_message >>> message: {:?}", message);

        let did_doc = self.connection_sm.did_doc()
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Cannot send message: Remote Connection information is not set"))?;

        self.agent_info().send_message(message, &did_doc)
    }

    pub fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        trace!("Connection::send_generic_message >>> message: {:?}", message);

        self.send_message(&A2AMessage::Generic(message.to_string())).map(|_| String::new())
    }

    pub fn send_ping(&self, comment: Option<&str>) -> VcxResult<()> {
        trace!("Connection::send_ping >>> comment: {:?}", comment);

        let mut ping = Ping::create().request_response();
        if let Some(comment) = comment {
            ping = ping.set_comment(comment.to_string());
        }

        self.send_message(&A2AMessage::Ping(ping))
    }

    pub fn delete(&self) -> VcxResult<()> {
        trace!("Connection: delete >>> {:?}", self.connection_sm.source_id());
        self.agent_info().delete()
    }

    fn step(&mut self, message: DidExchangeMessages) -> VcxResult<()> {
        self.connection_sm = self.connection_sm.clone().step(message)?;
        Ok(())
    }

    pub fn add_pending_messages(&mut self, messages: HashMap<MessageId, String>) -> VcxResult<()> {
        trace!("Connection::add_pending_messages >>> messages: {:?}", messages);
        Ok(self.connection_sm.add_pending_messages(messages))
    }

    pub fn remove_pending_message(&mut self, id: MessageId) -> VcxResult<()> {
        trace!("Connection::remove_pending_message >>> id: {:?}", id);
        self.connection_sm.remove_pending_message(id)
    }
}