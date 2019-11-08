use messages::update_message::{UIDsByConn, update_messages as update_messages_status};
use messages::MessageStatusCode;
use messages::get_message::{Message, get_connection_messages};
use messages::update_connection::send_delete_connection_message;

use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::a2a::A2AMessage;

use v3::utils::encryption_envelope::EncryptionEnvelope;

use std::collections::HashMap;

use connection::create_agent_keys;
use utils::httpclient;
use utils::libindy::signus::create_my_did;
use settings;
use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub pw_did: String,
    pub pw_vk: String,
    pub agent_did: String,
    pub agent_vk: String
}

impl Default for AgentInfo {
    fn default() -> AgentInfo {
        AgentInfo {
            pw_did: String::new(),
            pw_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }
}

impl AgentInfo {
    pub fn create_agent(&self) -> VcxResult<AgentInfo> {
        let method_name = settings::get_config_value(settings::CONFIG_DID_METHOD).ok();
        let (pw_did, pw_vk) = create_my_did(None, method_name.as_ref().map(String::as_str))?;

        /*
            Create User Pairwise Agent in old way.
            Send Messages corresponding to V2 Protocol to avoid code changes on Agency side.
        */
        let (agent_did, agent_vk) = create_agent_keys("", &pw_did, &pw_vk)?;

        Ok(AgentInfo { pw_did, pw_vk, agent_did, agent_vk })
    }

    pub fn agency_endpoint(&self) -> VcxResult<String> {
        settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)
            .map(|str| format!("{}/agency/msg", str))
    }

    pub fn routing_keys(&self) -> VcxResult<Vec<String>> {
        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        Ok(vec![self.agent_vk.to_string(), agency_vk])
    }

    pub fn recipient_keys(&self) -> Vec<String> {
        vec![self.pw_vk.to_string()]
    }

    pub fn update_message_status(&self, uid: String) -> VcxResult<()> {
        let messages_to_update = vec![UIDsByConn {
            pairwise_did: self.pw_did.clone(),
            uids: vec![uid]
        }];

        update_messages_status(MessageStatusCode::Reviewed, messages_to_update)
    }

    pub fn get_messages(&self) -> VcxResult<HashMap<String, A2AMessage>> {
        let messages = get_connection_messages(&self.pw_did,
                                               &self.pw_vk,
                                               &self.agent_did,
                                               &self.agent_vk,
                                               None,
                                               Some(vec![MessageStatusCode::Received]))?;


        let mut a2a_messages: HashMap<String, A2AMessage> = HashMap::new();

        for message in messages {
            a2a_messages.insert(message.uid.clone(), self.decode_message(&message)?);
        }

        Ok(a2a_messages)
    }

    pub fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        let mut messages = get_connection_messages(&self.pw_did,
                                                   &self.pw_vk,
                                                   &self.agent_did,
                                                   &self.agent_vk,
                                                   Some(vec![msg_id.to_string()]),
                                                   None)?;

        let message =
            messages
                .pop()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidMessages, format!("Message not found for id: {:?}", msg_id)))?;

        let message = self.decode_message(&message)?;

        Ok(message)
    }

    pub fn decode_message(&self, message: &Message) -> VcxResult<A2AMessage> {
        EncryptionEnvelope::open(&self.pw_vk, message.payload()?)
    }

    pub fn send_message(&self, message: &A2AMessage, remote_connection_info: &RemoteConnectionInfo) -> VcxResult<()> {
        let envelope = EncryptionEnvelope::create(&message, &self.pw_vk, &remote_connection_info)?;
        httpclient::post_message(&envelope.0, &remote_connection_info.service_endpoint)?;
        Ok(())
    }

    pub fn delete(&self) -> VcxResult<()> {
        send_delete_connection_message(&self.pw_did, &self.pw_vk, &self.agent_did, &self.agent_vk)
    }
}