extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use messages::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SendMessage {
    message: String,
    to_did: String,
    to_vk: String,
    agent_did:  String,
    agent_vk: String,
    agent_payload: String,
    payload: Vec<u8>,
    validate_rc: u32,
    ref_msg_id: String,
    status_code: String,
    uid: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct CreateMessagePayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    mtype: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct MessageDetailPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SendMessagePayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
}

impl SendMessage{

    pub fn create() -> SendMessage {
        SendMessage {
            message: String::new(),
            to_did: String::new(),
            to_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            agent_payload: String::new(),
            payload: Vec::new(),
            validate_rc: error::SUCCESS.code_num,
            ref_msg_id: String::new(),
            status_code: String::new(),
            uid: String::new(),
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
        self.ref_msg_id = String::from(id);
        self
    }

    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        let mut result = Vec::new();
        info!("sending secure message to agency");
        match httpclient::post_u8(&data, &url) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let string: String = if settings::test_agency_mode_enabled() && response.len() == 0 {
                    String::new()
                } else {
                    parse_send_message_response(response)?
                };
                result.push(string);
            },
        };
        info!("sent message to agency");
        Ok(result.to_owned())
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

        let create = CreateMessagePayload { msg_type: MsgType { name: "CREATE_MSG".to_string(), ver: "1.0".to_string(), }, mtype: self.message.to_string(), };
        let detail = MessageDetailPayload { msg_type: MsgType { name: "MSG_DETAIL".to_string(), ver: "1.0".to_string(), }, msg: self.payload.clone(), };
        let send = SendMessagePayload { msg_type: MsgType { name: "SEND_MSG".to_string(), ver: "1.0".to_string(), }, };

        let create = encode::to_vec_named(&create).unwrap();
        let detail = encode::to_vec_named(&detail).unwrap();
        let send = encode::to_vec_named(&send).unwrap();

        debug!("SendMessage details: {:?}", detail);
        let mut bundle = Bundled::create(create);
        bundle.bundled.push(detail);
        bundle.bundled.push(send);

        let msg = bundle.encode()?;
        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    uid: String,
}

fn parse_send_message_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    let mut de = Deserializer::new(&data[1][..]);
    let response: SendMessageResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    info!("messages: {:?}", response);
    match serde_json::to_string(&response) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::SEND_CLAIM_OFFER_RESPONSE;

    #[test]
    fn test_msgpack() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let mut message = SendMessage {
            message: "claimOffer".to_string(),
            to_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            to_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            agent_payload: String::new(),
            payload: vec![1,2,3,4,5,6,7,8],
            validate_rc: 0,
            ref_msg_id: "123".to_string(),
            status_code: "123".to_string(),
            uid: "123".to_string(),
        };

        /* just check that it doesn't panic */
        let packed = message.msgpack().unwrap();
    }

    #[test]
    fn test_parse_send_message_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let result = parse_send_message_response(SEND_CLAIM_OFFER_RESPONSE.to_vec()).unwrap();

        assert_eq!("{\"@type\":{\"name\":\"MSG_SENT\",\"ver\":\"1.0\"},\"uid\":\"ntc2ytb\"}", result);
    }
}
