extern crate rust_base58;
extern crate serde_json;

use settings;
use utils::httpclient;
use utils::error;
use messages::GeneralMessage;

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct GetMessagesPayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "msgType")]
    message: String,
    uid: String,
    status_code: String,
    include_edge_payload: String,
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
    validate_rc: u32,
}

impl GetMessages{

    pub fn create() -> GetMessages {
        GetMessages {
            to_did: String::new(),
            payload: GetMessagesPayload{
                msg_type: "GET_MSGS".to_string(),
                message: String::new(),
                uid: String::new(),
                status_code: String::new(),
                include_edge_payload: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn msg_type(&mut self, msg: &str) -> &mut Self{
        //Todo: validate msg??
        self.payload.message = msg.to_string();
        self
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uid = uid.to_string();
        self
    }

    pub fn status_code(&mut self, code: &str) -> &mut Self {
        //Todo: validate that it can be parsed to number??
        self.payload.status_code = code.to_string();
        self
    }


    pub fn include_edge_payload(&mut self, payload: &str) -> &mut Self {
        //todo: is this a json value, String??
        self.payload.include_edge_payload = payload.to_string();
        self
    }

    pub fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = match self.serialize_message() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for GetMessages{
    type Msg = GetMessages;

    fn set_to_did(&mut self, to_did: String){
        self.to_did = to_did;
    }
    fn set_validate_rc(&mut self, rc: u32){
        self.validate_rc = rc;
    }

    fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        self.agent_payload = json!(self.payload).to_string();
        Ok(json!(self).to_string())
    }
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendMessagePayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "msgType")]
    message: String,
    uid: String,
    status_code: String,
    edge_agent_payload: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendMessage {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: SendMessagePayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
}

impl SendMessage{

    pub fn create() -> SendMessage {
        SendMessage {
            to_did: String::new(),
            payload: SendMessagePayload{
                msg_type: "SEND_MSG".to_string(),
                message: String::new(),
                uid: String::new(),
                status_code: String::new(),
                edge_agent_payload: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn msg_type(&mut self, msg: &str) -> &mut Self{
        //Todo: validate msg??
        self.payload.message = msg.to_string();
        self
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uid = uid.to_string();
        self
    }

    pub fn status_code(&mut self, code: &str) -> &mut Self {
        //Todo: validate that it can be parsed to number??
        self.payload.status_code = code.to_string();
        self
    }


    pub fn edge_agent_payload(&mut self, payload: &str) -> &mut Self {
        //todo: is this a json value, String??
        self.payload.edge_agent_payload = payload.to_string();
        self
    }

    pub fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = match self.serialize_message() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendMessage{
    type Msg = SendMessage;

    fn set_to_did(&mut self, to_did: String){
        self.to_did = to_did;
    }
    fn set_validate_rc(&mut self, rc: u32){
        self.validate_rc = rc;
    }

    fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        self.agent_payload = json!(self.payload).to_string();
        Ok(json!(self).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::{get_messages, send_message};


    #[test]
    fn test_get_messages_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let msg_type = "message";
        let msg = match get_messages()
            .to(&to_did)
            .msg_type(&msg_type)
            .uid(&uid)
            .status_code(&status_code)
            .include_edge_payload(&payload)
            .serialize_message(){
            Ok(x) => x.to_string(),
            Err(y) => {
             println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"includeEdgePayload\\\":\\\"Some Data\\\",\
            \\\"msgType\\\":\\\"message\\\",\
            \\\"statusCode\\\":\\\"0\\\",\
            \\\"type\\\":\\\"GET_MSGS\\\",\
            \\\"uid\\\":\\\"123\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }

    #[test]
    fn test_get_messages_set_invalid_did_errors_at_serialize(){
        let to_did = "A";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let mut msg = get_messages()
            .to(&to_did)
            .uid(&uid)
            .status_code(&status_code)
            .include_edge_payload(&payload).clone();

        match msg.serialize_message(){
            Ok(_) => panic!("should have had did error"),
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num)
        }
    }

    #[test]
    fn test_send_message_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let msg_type = "message";
        let msg = match send_message()
            .to(&to_did)
            .msg_type(&msg_type)
            .uid(&uid)
            .status_code(&status_code)
            .edge_agent_payload(&payload)
            .serialize_message(){
            Ok(x) => x.to_string(),
            Err(y) => {
             println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"edgeAgentPayload\\\":\\\"Some Data\\\",\
            \\\"msgType\\\":\\\"message\\\",\
            \\\"statusCode\\\":\\\"0\\\",\
            \\\"type\\\":\\\"SEND_MSG\\\",\
            \\\"uid\\\":\\\"123\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }
}
