extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;

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
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

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

    fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        self.agent_payload = json!(self.payload).to_string();
        Ok(json!(self).to_string())
    }

    fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = self.serialize_message()?;

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }

    fn set_to_vk(&mut self, to_vk: String){ self.to_vk = to_vk; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let data = encode::to_vec_named(&self.payload).unwrap();
        info!("get_message content: {:?}", data);

        let msg = Bundled::create(data).encode()?;

        bundle_for_agent(msg, &self.agent_did, &self.agent_vk)
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
    pub delivery_details: Vec<DeliveryDetails>,
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

    info!("get_message response: {:?}", data[0]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use messages::get_messages;
    use utils::constants::GET_MESSAGES_RESPONSE;

    #[test]
    fn test_get_messages_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let payload = "Some Data";
        let msg = match get_messages()
            .to(&to_did)
            .uid("123")
            .serialize_message(){
            Ok(x) => x.to_string(),
            Err(y) => {
             println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\"{\\\"@type\\\":{\\\"name\\\":\\\"GET_MSGS\\\",\\\"ver\\\":\\\"1.0\\\"},\\\"uids\\\":\\\"123\\\"}\",\"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }

    #[test]
    fn test_get_messages_set_some_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let payload = "Some Data";
        let msg = match get_messages()
            .to(&to_did)
            .serialize_message(){
            Ok(x) => x.to_string(),
            Err(y) => {
             println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\"{\\\"@type\\\":{\\\"name\\\":\\\"GET_MSGS\\\",\\\"ver\\\":\\\"1.0\\\"}}\",\"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }

    #[test]
    fn test_get_messages_set_invalid_did_errors_at_serialize(){
        let to_did = "A";
        let payload = "Some Data";
        let mut msg = get_messages()
            .to(&to_did)
            .uid("123")
            .include_edge_payload(&payload).clone();

        match msg.serialize_message(){
            Ok(_) => panic!("should have had did error"),
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num)
        }
    }

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
