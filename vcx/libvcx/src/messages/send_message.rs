use settings;
use connection;
use api::VcxStateType;
use messages::*;
use messages::message_type::MessageTypes;
use utils::{httpclient, error};

#[derive(Debug)]
pub struct SendMessageBuilder {
    mtype: CredentialExchangeMessageType,
    to_did: String,
    to_vk: String,
    agent_did: String,
    agent_vk: String,
    payload: Vec<u8>,
    ref_msg_id: Option<String>,
    status_code: MessageStatusCode,
    uid: Option<String>,
    title: Option<String>,
    detail: Option<String>,
}

impl SendMessageBuilder {
    pub fn create() -> SendMessageBuilder {
        trace!("SendMessage::create_message >>>");

        SendMessageBuilder {
            mtype: CredentialExchangeMessageType::Other(String::new()),
            to_did: String::new(),
            to_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            payload: Vec::new(),
            ref_msg_id: None,
            status_code: MessageStatusCode::Created,
            uid: None,
            title: None,
            detail: None,
        }
    }

    pub fn msg_type(&mut self, msg: &CredentialExchangeMessageType) -> Result<&mut Self, u32> {
        //Todo: validate msg??
        self.mtype = msg.clone();
        Ok(self)
    }

    pub fn uid(&mut self, uid: Option<&str>) -> Result<&mut Self, u32> {
        //Todo: validate msg_uid??
        self.uid = uid.map(String::from);
        Ok(self)
    }

    pub fn status_code(&mut self, code: &MessageStatusCode) -> Result<&mut Self, u32> {
        //Todo: validate that it can be parsed to number??
        self.status_code = code.clone();
        Ok(self)
    }

    pub fn edge_agent_payload(&mut self, payload: &[u8]) -> Result<&mut Self, u32> {
        //todo: is this a json value, String??
        self.payload = payload.to_vec();
        Ok(self)
    }

    pub fn ref_msg_id(&mut self, id: &str) -> Result<&mut Self, u32> {
        self.ref_msg_id = Some(String::from(id));
        Ok(self)
    }

    pub fn set_title(&mut self, title: &str) -> Result<&mut Self, u32> {
        self.title = Some(title.to_string());
        Ok(self)
    }

    pub fn set_detail(&mut self, detail: &str) -> Result<&mut Self, u32> {
        self.detail = Some(detail.to_string());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> Result<SendResponse, u32> {
        trace!("SendMessage::send >>>");

        if settings::test_agency_mode_enabled() {
            return SendMessageBuilder::parse_response(::utils::constants::SEND_MESSAGE_RESPONSE.to_vec());
        }

        let data = self.prepare()?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        let result = SendMessageBuilder::parse_response(response)?;

        Ok(result)
    }

    fn parse_response(response: Vec<u8>) -> Result<SendResponse, u32> {
        match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 => {
                let mut messages = parse_response_from_agency(&response)?;
                if messages.len() <= 1 {
                    return Err(error::INVALID_HTTP_RESPONSE.code_num);
                }
                let response: MessageSent = MessageSent::from_a2a_message(messages.remove(1))?;
                Ok(SendResponse { uid: response.uid, uids: response.uids })
            }
            settings::ProtocolTypes::V2 => {
                let mut messages = parse_response_from_agency(&response)?;
                let response: CredentialExchangeResponse = CredentialExchangeResponse::from_a2a_message(messages.remove(0))?;
                Ok(SendResponse { uid: Some(response.uid.clone()), uids: if response.sent { vec![response.uid] } else { vec![] } })
            }
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendMessageBuilder {
    type Msg = SendMessageBuilder;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String) { self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }

    fn prepare(&mut self) -> Result<Vec<u8>, u32> {
        let messages =
            match settings::get_protocol_type() {
                settings::ProtocolTypes::V1 => {
                    let create = CreateMessage {
                        msg_type: MessageTypes::build(A2AMessageKinds::CreateMessage),
                        mtype: self.mtype.clone(),
                        reply_to_msg_id: self.ref_msg_id.clone(),
                        send_msg: true,
                        uid: self.uid.clone()
                    };
                    let detail = GeneralMessageDetail {
                        msg_type: MessageTypes::build(A2AMessageKinds::MessageDetail),
                        msg: self.payload.clone(),
                        title: self.title.clone(),
                        detail: self.detail.clone()
                    };
                    vec![A2AMessage::CreateMessage(create), A2AMessage::MessageDetail(MessageDetail::General(detail))]
                }
                settings::ProtocolTypes::V2 => {
                    let message = CredentialExchange {
                        msg_type: MessageTypes::build(A2AMessageKinds::CredentialExchange),
                        mtype: self.mtype.clone(),
                        reply_to_msg_id: self.ref_msg_id.clone(),
                        send_msg: true,
                        uid: self.uid.clone(),
                        msg: self.payload.clone(),
                        title: self.title.clone(),
                        detail: self.detail.clone(),
                    };
                    vec![A2AMessage::CredentialExchange(message)]
                }
            };

        prepare_message_for_agent(messages, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Debug, PartialEq)]
pub struct SendResponse {
    uid: Option<String>,
    uids: Vec<String>,
}

impl SendResponse {
    pub fn get_msg_uid(&self) -> Result<String, u32> {
        self.uids
            .get(0)
            .map(|uid| uid.to_string())
            .ok_or(error::INVALID_JSON.code_num)
    }
}

pub fn send_generic_message(connection_handle: u32, msg: &str, msg_type: &str, msg_title: &str) -> Result<String, u32> {
    if connection::get_state(connection_handle) != VcxStateType::VcxStateAccepted as u32 {
        return Err(error::NOT_READY.code_num);
    }

    let agent_did = connection::get_agent_did(connection_handle).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))?;
    let did = connection::get_pw_did(connection_handle).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))?;
    let vk = connection::get_pw_verkey(connection_handle).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))?;
    let remote_vk = connection::get_their_pw_verkey(connection_handle).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))?;

    let data = connection::generate_encrypted_payload(&vk, &remote_vk, &msg, PayloadKinds::Other(msg_type.to_string()))
        .or(Err(error::UNKNOWN_LIBINDY_ERROR.code_num))?;

    let response =
        send_message()
            .to(&did)?
            .to_vk(&vk)?
            .msg_type(&CredentialExchangeMessageType::Other(msg_type.to_string()))?
            .edge_agent_payload(&data)?
            .agent_did(&agent_did)?
            .agent_vk(&agent_vk)?
            .set_title(&msg_title)?
            .set_detail(&msg_title)?
            .status_code(&MessageStatusCode::Accepted)?
            .send_secure()
            .map_err(|err| {
                warn!("could not send message: {}", err);
                err
            })?;

    let msg_uid = response.get_msg_uid()?;
    return Ok(msg_uid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::SEND_MESSAGE_RESPONSE;

    #[test]
    fn test_msgpack() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let mut message = SendMessageBuilder {
            mtype: CredentialExchangeMessageType::CredOffer,
            to_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            to_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            payload: vec![1, 2, 3, 4, 5, 6, 7, 8],
            ref_msg_id: Some("123".to_string()),
            status_code: MessageStatusCode::Created,
            uid: Some("123".to_string()),
            title: Some("this is the title".to_string()),
            detail: Some("this is the detail".to_string()),
        };

        /* just check that it doesn't panic */
        let packed = message.prepare().unwrap();
    }

    #[test]
    fn test_parse_send_message_response() {
        init!("true");
        let result = SendMessageBuilder::parse_response(SEND_MESSAGE_RESPONSE.to_vec()).unwrap();
        let expected = SendResponse {
            uid: None,
            uids: vec!["ntc2ytb".to_string()],
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_send_message_bad_response() {
        init!("true");
        let result = SendMessageBuilder::parse_response(::utils::constants::UPDATE_PROFILE_RESPONSE.to_vec());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_msg_uid() {
        let test_val = "devin";
        let response = SendResponse {
            uid: None,
            uids: vec![test_val.to_string()],
        };

        let uid = response.get_msg_uid().unwrap();
        assert_eq!(test_val, uid);

        let test_val = "devin";
        let response = SendResponse {
            uid: None,
            uids: vec![],
        };

        let uid = response.get_msg_uid().unwrap_err();
        assert_eq!(error::INVALID_JSON.code_num, uid);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_send_generic_message() {
        init!("agency");
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();

        match send_generic_message(alice, "this is the message", "type", "title") {
            Ok(x) => println!("message id: {}", x),
            Err(x) => panic!("paniced! {}", x),
        };
        ::utils::devsetup::tests::set_consumer();
        let all_messages = get_message::download_messages(None, None, None).unwrap();
        println!("{}", serde_json::to_string(&all_messages).unwrap());
        teardown!("agency");
    }

    #[test]
    fn test_send_generic_message_fails_with_invalid_connection() {
        init!("true");
        let handle = ::connection::tests::build_test_connection();

        match send_generic_message(handle, "this is the message", "type", "title") {
            Ok(x) => panic!("test shoudl fail: {}", x),
            Err(x) => assert_eq!(x, error::NOT_READY.code_num),
        };
    }
}
