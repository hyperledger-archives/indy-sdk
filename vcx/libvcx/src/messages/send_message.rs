extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use connection;
use settings;
use api::VcxStateType;
use utils::httpclient;
use utils::error;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use messages::*;

pub struct SendMessage {
    message: String,
    to_did: String,
    to_vk: String,
    agent_did:  String,
    agent_vk: String,
    agent_payload: String,
    payload: Vec<u8>,
    validate_rc: u32,
    ref_msg_id: Option<String>,
    status_code: String,
    uid: String,
    title: Option<String>,
    detail: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessagePayload {
    #[serde(rename = "@type")]
    pub msg_type: MsgType,
    pub mtype: String,
    #[serde(rename = "replyToMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_msg_id: Option<String>,
    pub send_msg: bool,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageDetailPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl SendMessage{

    pub fn create() -> SendMessage {
        trace!("SendMessage::create_message >>>");

        SendMessage {
            message: String::new(),
            to_did: String::new(),
            to_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            agent_payload: String::new(),
            payload: Vec::new(),
            validate_rc: error::SUCCESS.code_num,
            ref_msg_id: None,
            status_code: String::new(),
            uid: String::new(),
            title: None,
            detail: None,
        }
    }

    pub fn msg_type(&mut self, msg: &str) -> &mut Self{
        //Todo: validate msg??
        self.message = msg.to_string();
        self
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.uid = uid.to_string();
        self
    }

    pub fn status_code(&mut self, code: &str) -> &mut Self {
        //Todo: validate that it can be parsed to number??
        self.status_code = code.to_string();
        self
    }


    pub fn edge_agent_payload(&mut self, payload: &Vec<u8>) -> &mut Self {
        //todo: is this a json value, String??
        self.payload = payload.clone();
        self
    }

    pub fn ref_msg_id(&mut self, id: &str) -> &mut Self {
        self.ref_msg_id = Some(String::from(id));
        self
    }

    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        trace!("SendMessage::send >>>");

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        let mut result = Vec::new();
        debug!("sending secure message to agency");
        if settings::test_agency_mode_enabled() {
            result.push(parse_send_message_response(::utils::constants::SEND_MESSAGE_RESPONSE.to_vec())?);
            return Ok(result.to_owned());
        }

        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => result.push(parse_send_message_response(response)?),
        };
        debug!("sent message to agency");
        Ok(result.to_owned())
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_detail(&mut self, detail: &str) -> &mut Self {
        self.detail = Some(detail.to_string());
        self
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendMessage{
    type Msg = SendMessage;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }
    fn set_to_vk(&mut self, to_vk: String){ self.to_vk = to_vk; }

    fn msgpack(&mut self) -> Result<Vec<u8>, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let create = CreateMessagePayload { msg_type: MsgType { name: "CREATE_MSG".to_string(), ver: "1.0".to_string(), }, mtype: self.message.to_string(), reply_to_msg_id: self.ref_msg_id.clone(), send_msg: true};
        let detail = MessageDetailPayload { msg_type: MsgType { name: "MSG_DETAIL".to_string(), ver: "1.0".to_string(), }, msg: self.payload.clone(), title: self.title.clone(), detail: self.detail.clone(), };

        match serde_json::to_string(&detail) {
            Ok(x) => debug!("sending message: {}", x),
            Err(_) => {},
        };

        debug!("SendMessage details: {:?}", detail);
        let create = encode::to_vec_named(&create).or(Err(error::UNKNOWN_ERROR.code_num))?;
        let detail = encode::to_vec_named(&detail).or(Err(error::UNKNOWN_ERROR.code_num))?;

        let mut bundle = Bundled::create(create);
        bundle.bundled.push(detail);

        let msg = bundle.encode()?;
        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    uids: Vec<String>,
}

fn parse_send_message_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    if data.len() <= 1 {
        return Err(error::INVALID_HTTP_RESPONSE.code_num);
    }

    let mut de = Deserializer::new(&data[1][..]);
    let response: SendMessageResponse = Deserialize::deserialize(&mut de)
        .or(Err(error::INVALID_MSGPACK.code_num))?;

    debug!("messages: {:?}", response);
    serde_json::to_string(&response).or(Err(error::INVALID_JSON.code_num))
}

pub fn send_generic_message(connection_handle: u32, msg: &str, msg_type: &str, msg_title: &str) -> Result<String, u32> {

    if connection::get_state(connection_handle) != VcxStateType::VcxStateAccepted as u32 {
        return Err(error::NOT_READY.code_num)
    }

    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| error::INVALID_CONNECTION_HANDLE.code_num)?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| error::INVALID_CONNECTION_HANDLE.code_num)?;
    let did = connection::get_pw_did(connection_handle).map_err(|x| error::INVALID_CONNECTION_HANDLE.code_num)?;
    let vk = connection::get_pw_verkey(connection_handle).map_err(|x| error::INVALID_CONNECTION_HANDLE.code_num)?;
    let remote_vk = connection::get_their_pw_verkey(connection_handle).map_err(|x| error::INVALID_CONNECTION_HANDLE.code_num)?;

    let data = connection::generate_encrypted_payload(&vk, &remote_vk, &msg, msg_type)
            .map_err(|e| error::UNKNOWN_LIBINDY_ERROR.code_num)?;

    match send_message().to(&did)
        .to_vk(&vk)
        .msg_type(msg_type)
        .edge_agent_payload(&data)
        .agent_did(&agent_did)
        .agent_vk(&agent_vk)
        .set_title(&msg_title)
        .set_detail(&msg_title)
        .status_code(&MessageResponseCode::MessageAccepted.as_string())
        .send_secure() {
        Err(x) => {
            warn!("could not send message: {}", x);
            return Err(x);
        },
        Ok(response) => {
            let msg_uid = parse_msg_uid(&response[0]).map_err(|ec| error::INVALID_HTTP_RESPONSE.code_num)?;
            return Ok(msg_uid);
        }
    }
}

pub fn parse_msg_uid(response: &str) -> Result<String,u32> {
    serde_json::from_str::<serde_json::Value>(response)
        .or(Err(error::INVALID_JSON.code_num))?["uids"]
        .as_array()
        .map_or(Err(error::INVALID_JSON.code_num), |uids| {
            Ok(uids[0]
                .as_str()
                .ok_or(error::INVALID_JSON.code_num)?
                .to_string()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::SEND_MESSAGE_RESPONSE;

    #[test]
    fn test_msgpack() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let mut message = SendMessage {
            message: "credOffer".to_string(),
            to_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            to_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            agent_payload: String::new(),
            payload: vec![1,2,3,4,5,6,7,8],
            validate_rc: 0,
            ref_msg_id: Some("123".to_string()),
            status_code: "123".to_string(),
            uid: "123".to_string(),
            title: Some("this is the title".to_string()),
            detail: Some("this is the detail".to_string()),
        };

        /* just check that it doesn't panic */
        let packed = message.msgpack().unwrap();
    }

    #[test]
    fn test_parse_send_message_response() {
        init!("true");
        let result = parse_send_message_response(SEND_MESSAGE_RESPONSE.to_vec()).unwrap();

        assert_eq!("{\"@type\":{\"name\":\"MSG_SENT\",\"ver\":\"1.0\"},\"uids\":[\"ntc2ytb\"]}", result);
    }

    #[test]
    fn test_parse_send_message_bad_response() {
        init!("true");
        let result = parse_send_message_response(::utils::constants::UPDATE_PROFILE_RESPONSE.to_vec());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_msg_uid() {

        let test_val = "devin";
        let test_json = json!({
            "uids": [test_val]
        });

        let to_str = serde_json::to_string(&test_json).unwrap();
        let uid = parse_msg_uid(&to_str).unwrap();
        assert_eq!(test_val, uid);

        let test_val = "devin";
        let test_json = json!({
            "uids": "test_val"
        });

        let to_str = serde_json::to_string(&test_json).unwrap();
        let uid = parse_msg_uid(&to_str).unwrap_err();
        assert_eq!(error::INVALID_JSON.code_num, uid);

        let test_val = "devin";
        let test_json = json!({});

        let to_str = serde_json::to_string(&test_json).unwrap();
        let uid = parse_msg_uid(&to_str).unwrap_err();
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
