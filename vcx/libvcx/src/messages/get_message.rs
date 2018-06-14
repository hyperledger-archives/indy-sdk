extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use messages::MessageResponseCode::{ MessageAccepted, MessagePending };
use utils::libindy::crypto;

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct GetMessagesPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "excludePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessages {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: GetMessagesPayload,
    #[serde(skip_serializing, default)]
    to_vk: String,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    #[serde(skip_serializing, default)]
    agent_did: String,
    #[serde(skip_serializing, default)]
    agent_vk: String,
}

impl GetMessages{

    pub fn create() -> GetMessages {
        GetMessages {
            to_did: String::new(),
            to_vk: String::new(),
            payload: GetMessagesPayload{
                msg_type: MsgType { name: "GET_MSGS".to_string(), ver: "1.0".to_string(), },
                uids: None,
                exclude_payload: None,
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uids = Some(uid.to_string());
        self
    }

    pub fn include_edge_payload(&mut self, payload: &str) -> &mut Self {
        //todo: is this a json value, String??
        self.payload.exclude_payload = Some(payload.to_string());
        self
    }

    pub fn send_secure(&mut self) -> Result<Vec<Message>, u32> {
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT).unwrap());

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post_u8(&data, &url) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => if settings::test_agency_mode_enabled() && response.len() == 0 {
                return Ok(Vec::new());
            } else {
                parse_get_messages_response(response)
            },
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for GetMessages{
    type Msg = GetMessages;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }
    fn set_to_vk(&mut self, to_vk: String){ self.to_vk = to_vk; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let data = encode::to_vec_named(&self.payload).unwrap();
        trace!("get_message content: {:?}", data);

        let msg = Bundled::create(data).encode()?;

        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMsgType {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMsgResponse {
    #[serde(rename = "@type")]
    pub msg_type: GetMsgType,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDetails {
    to: String,
    status_code: String,
    last_updated_date_time: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub status_code: String,
    pub payload: Option<Vec<i8>>,
    #[serde(rename = "senderDID")]
    pub sender_did: String,
    pub uid: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub ref_msg_id: Option<String>,
    #[serde(skip_deserializing)]
    pub delivery_details: Vec<DeliveryDetails>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            status_code: String::new(),
            payload: None,
            sender_did: String::new(),
            uid: String::new(),
            msg_type: String::new(),
            ref_msg_id: None,
            delivery_details: Vec::new(), 
        }    
    }    
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    msgs: Vec<Message>,
}

fn parse_get_messages_response(response: Vec<u8>) -> Result<Vec<Message>, u32> {
    let data = unbundle_from_agency(response)?;

    trace!("get_message response: {:?}", data[0]);
    let mut de = Deserializer::new(&data[0][..]);
    let response: GetMessagesResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);

            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    Ok(response.msgs.to_owned())
}

pub fn get_matching_message(msg_uid:&str, pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<get_message::Message, u32> {

    match get_messages()
        .to(&pw_did)
        .to_vk(&pw_vk)
        .agent_did(&agent_did)
        .agent_vk(&agent_vk)
        .uid(msg_uid)
        .send_secure() {
        Err(x) => {
            error!("could not post get_messages: {}", x);
            Err(error::POST_MSG_FAILURE.code_num)
        },
        Ok(response) => {
            if response.len() == 0 {
                Ok(get_message::Message::new())    
            } else {
                trace!("message returned: {:?}", response[0]);
                Ok(response[0].to_owned())
            }
        },
    }
}

pub fn get_all_message(pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<Message>, u32> {
    match get_messages()
        .to(&pw_did)
        .to_vk(&pw_vk)
        .agent_did(&agent_did)
        .agent_vk(&agent_vk)
        .send_secure() {
        Err(x) => {
            error!("could not post get_messages: {}", x);
            Err(error::POST_MSG_FAILURE.code_num)
        },
        Ok(response) => Ok(response)
    }
}

pub fn get_ref_msg(msg_id: &str, pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    let message = get_matching_message(msg_id, pw_did, pw_vk, agent_did, agent_vk)?;
    trace!("checking for ref_msg: {:?}", message);
    let msg_id;
    if message.status_code == MessageAccepted.as_string() && !message.ref_msg_id.is_none() {
        msg_id = message.ref_msg_id.unwrap()
    }
    else {
        return Err(error::NOT_READY.code_num);
    }

    let message = get_matching_message(&msg_id, pw_did, pw_vk, agent_did, agent_vk)?;

    trace!("checking for pending message: {:?}", message);

    // this will work for both claimReq and proof types
    if message.status_code == MessagePending.as_string() && !message.payload.is_none() {
        let data = to_u8(message.payload.as_ref().unwrap());
        crypto::parse_msg(wallet::get_wallet_handle(), &pw_vk, &data)
    }
    else {
        Err(error::INVALID_HTTP_RESPONSE.code_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::GET_MESSAGES_RESPONSE;

    #[test]
    fn test_parse_get_messages_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let result = parse_get_messages_response(GET_MESSAGES_RESPONSE.to_vec()).unwrap();
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn test_build_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let delivery_details1 = DeliveryDetails {
            to: "3Xk9vxK9jeiqVaCPrEQ8bg".to_string(),
            status_code: "MDS-101".to_string(),
            last_updated_date_time: "2017-12-14T03:35:20.444Z[UTC]".to_string(),
        };

        let delivery_details2 = DeliveryDetails {
            to: "3Xk9vxK9jeiqVaCPrEQ8bg".to_string(),
            status_code: "MDS-101".to_string(),
            last_updated_date_time: "2017-12-14T03:35:20.500Z[UTC]".to_string(),
        };

        let msg1 = Message {
            status_code: MessageResponseCode::MessageAccepted.as_string(),
            payload: Some(vec![-9, 108, 97, 105, 109, 45, 100, 97, 116, 97]),
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "mmi3yze".to_string(),
            msg_type: "connReq".to_string(),
            ref_msg_id: None,
            delivery_details: vec![delivery_details1],
        };
        let msg2 = Message {
            status_code: MessageResponseCode::MessageCreate.as_string(),
            payload: None,
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "zjcynmq".to_string(),
            msg_type: "claimOffer".to_string(),
            ref_msg_id: None,
            delivery_details: Vec::new(),
        };
        let response = GetMessagesResponse {
            msg_type: MsgType { name: "MSGS".to_string(), ver: "1.0".to_string(), },
            msgs: vec![msg1, msg2],
        };

        let data = encode::to_vec_named(&response).unwrap();

        println!("generated response: {:?}", data);
        let bundle = Bundled::create(data).encode().unwrap();
        println!("bundle: {:?}", bundle);
        let result = parse_get_messages_response(bundle).unwrap();
        println!("response: {:?}", result);

    }
}
