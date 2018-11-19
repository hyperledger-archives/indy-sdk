extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use utils::constants::*;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use self::rmp_serde::encode;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
struct AttrValue {
    name: String,
    value: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UpdateProfileDataPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    configs: Vec<AttrValue>,
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

#[derive(Deserialize, Serialize)]
pub struct UpdateProfileResponse {
    #[serde(rename = "@type")]
    code: MsgType,
}

impl UpdateProfileData{

    pub fn create() -> UpdateProfileData {
        UpdateProfileData {
            to_did: String::new(),
            payload: UpdateProfileDataPayload{
                msg_type: MsgType { name: "UPDATE_CONFIGS".to_string(), ver: "1.0".to_string(), } ,
                configs: Vec::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self{
        let config = AttrValue { name: "name".to_string(), value: name.to_string(), };
        self.payload.configs.push(config);
        self
    }

    pub fn logo_url(&mut self, url: &str)-> &mut Self{
        match validation::validate_url(url){
            Ok(x) => {
                let config = AttrValue { name: "logoUrl".to_string(), value: url.to_string(), };
                self.payload.configs.push(config);
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        let mut result = Vec::new();
        if settings::test_agency_mode_enabled() {
            result.push(parse_update_profile_response(UPDATE_PROFILE_RESPONSE.to_vec()).unwrap());
            return Ok(result.to_owned());
        }

        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let response = parse_update_profile_response(response)?;
                result.push(response);
            },
        };

        Ok(result.to_owned())
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for UpdateProfileData{
    type Msg = UpdateProfileData;

    fn set_agent_did(&mut self, did: String) {}
    fn set_agent_vk(&mut self, vk: String) {}
    fn set_to_did(&mut self, to_did: String){
        self.to_did = to_did;
    }
    fn set_validate_rc(&mut self, rc: u32){
        self.validate_rc = rc;
    }
    fn set_to_vk(&mut self, to_vk: String){ /* nothing to do here for CreateKeymsg */ }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let data = encode::to_vec_named(&self.payload).or(Err(error::UNKNOWN_ERROR.code_num))?;
        trace!("update profile inner bundle: {:?}", data);
        let msg = Bundled::create(data).encode()?;

        let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        bundle_for_agency(msg, &to_did)
    }
}

fn parse_update_profile_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    let mut de = Deserializer::new(&data[0][..]);

    let response: UpdateProfileResponse = Deserialize::deserialize(&mut de)
        .or(Err(error::UNKNOWN_ERROR.code_num))?;

    serde_json::to_string(&response).or(Err(error::INVALID_JSON.code_num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::update_data;
    use utils::libindy::signus::create_and_store_my_did;

    #[test]
    fn test_update_data_post() {
        init!("true");
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let name = "name";
        let url = "https://random.com";
        let msg = update_data()
            .to(to_did)
            .name(&name)
            .logo_url(&url)
            .msgpack().unwrap();
    }

    #[test]
    fn test_update_data_set_values_and_post() {
        init!("false");
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = update_data()
            .to(agent_did.as_ref())
            .name("name")
            .logo_url("https://random.com")
            .msgpack().unwrap();
        assert!(msg.len() > 0);
    }

    #[test]
    fn test_parse_update_profile_response() {
        init!("indy");

        let result = parse_update_profile_response(UPDATE_PROFILE_RESPONSE.to_vec()).unwrap();

        assert_eq!(result, UPDATE_PROFILE_RESPONSE_STR);
    }
}
