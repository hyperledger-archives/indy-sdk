extern crate rmp_serde;
extern crate serde_json;

use error::{ToErrorCode, messages};
use serde::Deserialize;
use self::rmp_serde::{encode, Deserializer};
use messages::{Bundled, MsgType, bundle_for_agent, unbundle_from_agency, GeneralMessage};
use utils::{error, httpclient};
use settings;
use utils::constants::DELETE_CONNECTION_RESPONSE;


#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct DeleteConnection {
    #[serde(rename = "to")]
    to_did: String,
    to_vk: String,
    #[serde(skip_serializing, default)]
    payload: DeleteConnectionPayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    agent_did: String,
    agent_vk: String,
}



#[derive(Clone,Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct DeleteConnectionPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    status_code: String,

}

impl DeleteConnectionPayload {
    pub fn create() -> DeleteConnectionPayload {
        DeleteConnectionPayload {
            msg_type: MsgType {
                name: "UPDATE_CONN_STATUS".to_string(),
                ver: "1.0".to_string()
            },
            status_code: "CS-103".to_string(),
        }
    }

    pub fn deserialize(data: Vec<u8>) ->
    Result< DeleteConnectionPayload,
        messages::MessageError> {
        let mut de = Deserializer::new(&data[..]);
        let message: Self = match Deserialize::deserialize(&mut de) {
            Ok(x) => x,
            Err(x) => {
                return Err(messages::MessageError::MessagePackError())
            },
        };
        Ok(message)

    }
}
impl DeleteConnection {
    pub fn create() -> DeleteConnection {
        DeleteConnection {
            to_did: String::new(),
            to_vk: String::new(),
            payload: DeleteConnectionPayload::create(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }
    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(DELETE_CONNECTION_RESPONSE.to_vec()); }

        let mut result = Vec::new();
        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let response = self.parse_response_as_delete_connection_payload(&response)?;
                result.push(response);
            },
        };

        Ok(result.to_owned())
    }

    pub fn parse_response_as_delete_connection_payload(&self, response: &Vec<u8> ) -> Result<String, u32> {
        if settings::test_agency_mode_enabled() {
            let data = response.clone();
            return Ok(serde_json::to_string(&DeleteConnectionPayload::deserialize(data.to_owned()).unwrap()).unwrap())
        }
        let data = unbundle_from_agency(response.clone())?;
        let response = DeleteConnectionPayload::deserialize(data[0].to_owned())
            .map_err(|e| e.to_error_code())?;
        serde_json::to_string(&response).or(Err(error::INVALID_JSON.code_num))
    }

    fn print_info(&self) {
        println!("\n****\n**** message pack: Delete Connection");
        println!("payload {}", serde_json::to_string(&self.payload).unwrap());
        println!("self.to_vk: {}", &self.to_vk);
        println!("self.agent_did: {}", &self.agent_did);
        println!("self.agent_vk: {}", &self.agent_vk);
        debug!("connection invitation details: {}", serde_json::to_string(&self.payload).unwrap_or("failure".to_string()));
    }
}

//TODO Every GeneralMessage extension, duplicates code
impl GeneralMessage for DeleteConnection{
    type Msg = DeleteConnection;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
    }

    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let payload = encode::to_vec_named(&self.payload).or(Err(error::INVALID_JSON.code_num))?;

        let bundle = Bundled::create(payload);

        let msg = bundle.encode()?;

        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_delete_connection_payload(){
        let payload = vec![130, 165, 64, 116, 121, 112, 101, 130, 164, 110, 97, 109, 101, 179, 67, 79, 78, 78, 95, 83, 84, 65, 84, 85, 83, 95, 85, 80, 68, 65, 84, 69, 68, 163, 118, 101, 114, 163, 49, 46, 48, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 166, 67, 83, 45, 49, 48, 51];
        let msg_str = r#"{ "@type": { "name": "CONN_STATUS_UPDATED", "ver": "1.0" }, "statusCode": "CS-103" }"#;
        let delete_connection_payload: DeleteConnectionPayload = serde_json::from_str(msg_str).unwrap();
        assert_eq!(delete_connection_payload, DeleteConnectionPayload::deserialize(payload.clone()).unwrap());
        let delete_connection: DeleteConnection = DeleteConnection::create();
    }
}