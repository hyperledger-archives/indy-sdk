extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::{validation, GeneralMessage, Bundled, bundle_for_agency};
use self::rmp_serde::encode;


#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UpdateProfileDataPayload{
    #[serde(rename = "type")]
    msg_type: String,
    name: String,
    logo_url: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileData {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: UpdateProfileDataPayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendInvitePayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "keyDlgProof")]
    key_delegate: String,
    phone_number: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendInvite {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: SendInvitePayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
}

impl SendInvite{

    pub fn create() -> SendInvite {
        SendInvite {
            to_did: String::new(),
            payload: SendInvitePayload{
                msg_type: "SEND_INVITE".to_string(),
                key_delegate: String::new(),
                phone_number: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> &mut Self{
        match validation::validate_key_delegate(key){
            Ok(x) => {
                self.payload.key_delegate = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn phone_number(&mut self, p_num: &str)-> &mut Self{
        match validation::validate_phone_number(p_num){
            Ok(x) => {
                self.payload.phone_number = x;
                 self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendInvite{
    type Msg = SendInvite;

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

    fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = self.serialize_message()?;

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }

    fn set_to_vk(&mut self, to_vk: String){ /* nothing to do here for SendInvite */ }

    fn to_post(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let bundle = Bundled::create(self.payload.clone());

        let msg = match encode::to_vec_named(&bundle) {
                        Ok(x) => x,
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                return Err(error::INVALID_MSGPACK.code_num)
            },
        };

        bundle_for_agency(msg, self.to_did.as_ref())
    }

    fn send_enc(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.to_post() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post_u8(&data, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => parse_send_invite_response(&response),
        }
    }
}

fn parse_send_invite_response(response: &Vec<u8>) -> Result<String, u32> {
    Ok(String::new().to_owned())
}

impl UpdateProfileData{

    pub fn create() -> UpdateProfileData {
        UpdateProfileData {
            to_did: String::new(),
            payload: UpdateProfileDataPayload{
                msg_type: "UPDATE_PROFILE_DATA".to_string(),
                name: String::new(),
                logo_url: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self{
        self.payload.name = name.to_string();
        self
    }

    pub fn logo_url(&mut self, url: &str)-> &mut Self{
        match validation::validate_url(url){
            Ok(x) => {
                self.payload.logo_url = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for UpdateProfileData{
    type Msg = UpdateProfileData;

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

    fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = self.serialize_message()?;

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }

    fn set_to_vk(&mut self, to_vk: String){ /* nothing to do here for CreateKeymsg */ }

    fn to_post(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let bundle = Bundled::create(self.payload.clone());

        let msg = match encode::to_vec_named(&bundle) {
                        Ok(x) => x,
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                return Err(error::INVALID_MSGPACK.code_num)
            },
        };

        bundle_for_agency(msg, self.to_did.as_ref())
    }

    fn send_enc(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.to_post() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post_u8(&data, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => parse_update_profile_response(&response),
        }
    }
}

fn parse_update_profile_response(response: &Vec<u8>) -> Result<String, u32> {
    Ok(String::new().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::{update_data, send_invite};

    #[test]
    fn test_send_invite_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let phone = "phone";
        let key = "key";
        let msg = send_invite()
            .to(to_did)
            .phone_number(&phone)
            .key_delegate(&key)
            .serialize_message().unwrap();

        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"keyDlgProof\\\":\\\"key\\\",\
            \\\"phoneNumber\\\":\\\"phone\\\",\
            \\\"type\\\":\\\"SEND_INVITE\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}"

        );
    }

    #[test]
    fn test_update_data_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let name = "name";
        let url = "https://random.com";
        let msg = update_data()
            .to(to_did)
            .name(&name)
            .logo_url(&url)
            .serialize_message().unwrap();
        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"logoUrl\\\":\\\"https://random.com\\\",\
            \\\"name\\\":\\\"name\\\",\
            \\\"type\\\":\\\"UPDATE_PROFILE_DATA\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");

    }
}
