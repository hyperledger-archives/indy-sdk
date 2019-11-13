use messages::ObjectWithVersion;
use messages::get_message::Message;
use error::prelude::*;

use v3::SERIALIZE_VERSION;
use v3::handlers::connection::states::{DidExchangeSM, Actor};
use v3::handlers::connection::messages::DidExchangeMessages;
use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::connection::invite::Invitation;
use v3::handlers::connection::agent::AgentInfo;

use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    state: DidExchangeSM
}

impl Connection {
    pub fn create(source_id: &str, actor: Actor) -> Connection {
        trace!("Connection::create >>> source_id: {}, actor: {:?}", source_id, actor);

        Connection {
            state: DidExchangeSM::new(actor, source_id),
        }
    }

    pub fn state(&self) -> u32 { self.state.state() }

    pub fn agent_info(&self) -> &AgentInfo { self.state.agent_info() }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.state.remote_vk()
    }

    pub fn get_source_id(&self) -> VcxResult<String> {
        Ok(self.state.source_id().to_string())
    }

    pub fn process_invite(self, invitation: Invitation) -> VcxResult<Connection> {
        trace!("Connection::process_invite >>> invitation: {:?}", invitation);
        self.step(DidExchangeMessages::InvitationReceived(invitation))
    }

    pub fn get_invite_details(&self) -> VcxResult<String> {
        trace!("Connection::get_invite_details >>>");
        if let Some(invitation) = self.state.get_invitation() {
            return Ok(json!(invitation.to_a2a_message()).to_string());
        } else if let Some(did_doc) = self.state.did_doc() {
            return Ok(json!(Invitation::from(did_doc)).to_string());
        } else {
            Ok(json!({}).to_string())
        }
    }

    pub fn actor(&self) -> Actor {
        self.state.actor()
    }

    pub fn connect(self) -> VcxResult<Connection> {
        trace!("Connection::connect >>> source_id: {}", self.state.source_id());
        self.step(DidExchangeMessages::Connect())
    }

    pub fn update_state(mut self, message: Option<&str>) -> VcxResult<Connection> {
        trace!("Connection::update_state >>> message: {:?}", message);

        if let Some(message_) = message {
            return self.update_state_with_message(message_);
        }

        let messages = self.get_messages()?;
        let agent_info = self.agent_info().clone();

        if let Some((uid, message)) = self.state.find_message_to_handle(messages) {
            self = self.handle_message(message.into())?;
            agent_info.update_message_status(uid)?;
        };

        if let Some(prev_agent_info) = self.state.prev_agent_info().cloned() {
            let messages = prev_agent_info.get_messages()?;

            if let Some((uid, message)) = self.state.find_message_to_handle(messages) {
                self = self.handle_message(message.into())?;
                prev_agent_info.update_message_status(uid)?;
            }
        }

        Ok(self)
    }

    pub fn update_message_status(&self, uid: String) -> VcxResult<()> {
        trace!("Connection::update_message_status >>> uid: {:?}", uid);
        self.state.agent_info().update_message_status(uid)
    }

    pub fn update_state_with_message(mut self, message: &str) -> VcxResult<Connection> {
        trace!("Connection: update_state_with_message: {}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption,
                                              format!("Cannot updated state with messages: Message deserialization failed: {:?}", err)))?;

        let a2a_message = self.decode_message(&message)?;
        self = self.handle_message(a2a_message.into())?;
        self.update_message_status(message.uid)?;

        Ok(self)
    }

    pub fn get_messages(&self) -> VcxResult<HashMap<String, A2AMessage>> {
        trace!("Connection: get_messages >>>");
        self.agent_info().get_messages()
    }

    pub fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        trace!("Connection: get_message_by_id >>>");
        self.agent_info().get_message_by_id(msg_id)
    }

    pub fn handle_message(self, message: DidExchangeMessages) -> VcxResult<Connection> {
        trace!("Connection: handle_message >>> {:?}", message);
        self.step(message)
    }

    pub fn decode_message(&self, message: &Message) -> VcxResult<A2AMessage> {
        self.agent_info().decode_message(message)
    }

    pub fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        trace!("Connection::send_message >>> message: {:?}", message);

        let did_doc = self.state.did_doc()
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Cannot send message: Remote Connection information is not set"))?;

        self.agent_info().send_message(message, &did_doc)
    }

    pub fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        trace!("Connection::send_generic_message >>> message: {:?}", message);

        self.send_message(&A2AMessage::Generic(message.to_string())).map(|_| String::new())
    }

    pub fn delete(&self) -> VcxResult<()> {
        trace!("Connection: delete >>> {:?}", self.state.source_id());
        self.agent_info().delete()
    }

    pub fn from_str(data: &str) -> VcxResult<Self> {
        trace!("Connection::from_str >>> data: {:?}", data);

        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    pub fn to_string(&self) -> VcxResult<String> {
        trace!("Connection::to_string >>>");

        ObjectWithVersion::new(SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    fn step(mut self, message: DidExchangeMessages) -> VcxResult<Connection> {
        self.state = self.state.step(message)?;
        Ok(self)
    }

    pub fn add_pending_messages(&mut self, messages: HashMap<MessageId, String>) -> VcxResult<()> {
        trace!("Connection::add_pending_messages >>> messages: {:?}", messages);
        Ok(self.state.add_pending_messages(messages))
    }

    pub fn remove_pending_message(&mut self, id: MessageId) -> VcxResult<()> {
        trace!("Connection::remove_pending_message >>> id: {:?}", id);
        self.state.remove_pending_message(id)
    }
}