extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::{validation, GeneralMessage, Bundled, MsgResponse, bundle_for_agency, unbundle_from_agency};
use self::rmp_serde::encode;
use serde::Deserialize;
use self::rmp_serde::Deserializer;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
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

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
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

#[derive(Deserialize, Serialize)]
pub struct UpdateProfileResponse {
    code: String,
}

#[derive(Deserialize, Serialize)]
pub struct InviteDetails {
    msg_type: String,
    invite_details: String,
    invite_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct SendInviteResponse {
    create_response: MsgResponse,
    invite_details: InviteDetails,
    send_response: MsgResponse,
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

    fn to_post(&self) -> Result<Vec<u8>,u32> {
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
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.to_post() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post_u8(&data, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => parse_send_invite_response(response),
        }
    }
}

fn parse_send_invite_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    let mut de = Deserializer::new(&data[..]);
    let bundle: Bundled<SendInviteResponse> = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    let invite_details = &bundle.bundled[0].invite_details.invite_details;

    match serde_json::to_string(invite_details) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
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

    fn to_post(&self) -> Result<Vec<u8>,u32> {
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
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.to_post() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        match httpclient::post_u8(&data, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => parse_update_profile_response(response),
        }
    }
}

fn parse_update_profile_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    let mut de = Deserializer::new(&data[..]);
    let bundle: Bundled<UpdateProfileResponse> = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    match serde_json::to_string(&bundle.bundled) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::{update_data, send_invite};
    use utils::wallet;
    use utils::signus::SignusUtils;
    use utils::constants::*;

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
    fn test_send_invite_set_values_and_post(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let agency_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_agency").unwrap();
        let agent_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_agent").unwrap();
        let my_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_mine").unwrap();

        let (agent_did, agent_vk) = SignusUtils::create_and_store_my_did(agent_wallet, Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = SignusUtils::create_and_store_my_did(agency_wallet, Some(MY3_SEED)).unwrap();

        SignusUtils::store_their_did_from_parts(my_wallet, agent_did.as_ref(), agent_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(my_wallet, agency_did.as_ref(), agency_vk.as_ref()).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY, &my_vk);

        let msg = send_invite()
            .to(agent_did.as_ref())
            .phone_number("phone")
            .key_delegate("key")
            .to_post().unwrap();

        assert!(msg.len() > 0);

        wallet::delete_wallet("test_send_invite_set_values_and_serialize_mine").unwrap();
        wallet::delete_wallet("test_send_invite_set_values_and_serialize_agent").unwrap();
        wallet::delete_wallet("test_send_invite_set_values_and_serialize_agency").unwrap();
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

    #[test]
    fn test_update_data_set_values_and_post(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let agency_wallet = wallet::init_wallet("test_update_data_set_values_and_serialize_agency").unwrap();
        let agent_wallet = wallet::init_wallet("test_update_data_set_values_and_serialize_agent").unwrap();
        let my_wallet = wallet::init_wallet("test_update_data_set_values_and_serialize_mine").unwrap();

        let (agent_did, agent_vk) = SignusUtils::create_and_store_my_did(agent_wallet, Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = SignusUtils::create_and_store_my_did(agency_wallet, Some(MY3_SEED)).unwrap();

        SignusUtils::store_their_did_from_parts(my_wallet, agent_did.as_ref(), agent_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(my_wallet, agency_did.as_ref(), agency_vk.as_ref()).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY, &my_vk);

        let msg = update_data()
            .to(agent_did.as_ref())
            .name("name")
            .logo_url("https://random.com")
            .to_post().unwrap();
        assert!(msg.len() > 0);

        wallet::delete_wallet("test_update_data_set_values_and_serialize_mine").unwrap();
        wallet::delete_wallet("test_update_data_set_values_and_serialize_agent").unwrap();
        wallet::delete_wallet("test_update_data_set_values_and_serialize_agency").unwrap();
    }

    #[test]
    fn test_parse_send_invite_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");

        let payload = SendInviteResponse {
            create_response: MsgResponse { msg_type: "MSG_CREATED".to_string(), msg_id: "id1".to_string(), },
            invite_details: InviteDetails { msg_type: "MSG_DETAIL".to_string(), invite_details: "{\"attr\":\"value\"}".to_string(), invite_url: "url".to_string(), },
            send_response: MsgResponse { msg_type: "MSG_SENT".to_string(), msg_id: "id2".to_string(), },
        };

        let bundle = Bundled::create(payload);
        let data = encode::to_vec_named(&bundle).unwrap();
        let result = parse_send_invite_response(data).unwrap();

        println!("result: {}", result);

        assert!(result.len() > 0);
    }

    #[test]
    fn test_parse_update_profile_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");

        let payload = UpdateProfileResponse {
            code: "MS-103".to_string(),
        };

        let bundle = Bundled::create(payload);
        let data = encode::to_vec_named(&bundle).unwrap();
        let result = parse_update_profile_response(data).unwrap();

        println!("result: {}", result);

        assert!(result.len() > 0);
    }
}
