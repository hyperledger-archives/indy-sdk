extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use self::rmp_serde::encode;
use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use serde::Deserialize;
use self::rmp_serde::Deserializer;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct CreateKeyPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "forDID")]
    for_did: String,
    #[serde(rename = "forDIDVerKey")]
    for_verkey: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyMsg {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: CreateKeyPayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    agent_did: String,
    agent_vk: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "withPairwiseDID")]
    for_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    for_verkey: String,
}

impl CreateKeyMsg{

    pub fn create() -> CreateKeyMsg {
        CreateKeyMsg {
            to_did: String::new(),
            payload: CreateKeyPayload{
                msg_type: MsgType { name: "CREATE_KEY".to_string(), ver: "1.0".to_string(), } ,
                for_did: String::new(),
                for_verkey: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn for_did(&mut self, did: &str) ->&mut Self{
        match validation::validate_did(did){
            Ok(x) => {
                self.payload.for_did = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn for_verkey(&mut self, verkey: &str) -> &mut Self {
        match validation::validate_verkey(verkey){
            Ok(x) => {
                self.payload.for_verkey = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if settings::test_agency_mode_enabled() {
            return Ok(vec!["U5LXs4U7P9msh647kToezy".to_string(), "FktSZg8idAVzyQZrdUppK6FTrfAzW3wWVzAjJAfdUvJq".to_string()]);
        }

        let mut result = Vec::new();
        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let (did, vk) = parse_create_keys_response(response)?;
                result.push(did);
                result.push(vk);
            },
        };

        Ok(result.to_owned())
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for CreateKeyMsg  {
    type Msg = CreateKeyMsg;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }
    fn set_to_vk(&mut self, to_vk: String){ /* nothing to do here for CreateKeymsg */ }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let data = match encode::to_vec_named(&self.payload) {
            Ok(x) => x,
            Err(x) => {
                error!("could not encode create_keys msg: {}", x);
                return Err(error::INVALID_MSGPACK.code_num);
            },
        };
        debug!("create_keys inner bundle: {:?}", data);
        let msg = Bundled::create(data).encode()?;

        let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        bundle_for_agency(msg, &to_did)
    }
}

pub fn parse_create_keys_response(response: Vec<u8>) -> Result<(String, String), u32> {
    let data = unbundle_from_agency(response)?;

    debug!("create keys response inner bundle: {:?}", data[0]);
    let mut de = Deserializer::new(&data[0][..]);
    let response: CreateKeyResponse = Deserialize::deserialize(&mut de).or(Err(error::UNKNOWN_ERROR.code_num))?;

    Ok((response.for_did, response.for_verkey))
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{ CREATE_KEYS_RESPONSE, MY1_SEED, MY2_SEED, MY3_SEED };
    use utils::libindy::signus::create_and_store_my_did;
    use messages::create_keys;

    #[test]
    fn test_create_key_returns_message_with_create_key_as_payload() {
        let msg = create_keys();
        let msg_payload = CreateKeyPayload {
            for_did: String::new(),
            for_verkey: String::new(),
            msg_type: MsgType { name: "CREATE_KEY".to_string(), ver: "1.0".to_string(), } ,
        };
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let msg_payload = CreateKeyPayload {
            for_did: for_did.to_string(),
            for_verkey: for_verkey.to_string(),
            msg_type: MsgType { name: "CREATE_KEY".to_string(), ver: "1.0".to_string(), } ,
        };
        let msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey).clone();
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values_and_serialize() {
        init!("false");

        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let bytes = create_keys()
            .to(&agent_did)
            .for_did(&my_did)
            .for_verkey(&my_vk)
            .msgpack().unwrap();
        assert!(bytes.len() > 0);
    }

    #[test]
    fn test_parse_create_keys_response() {
        init!("true");

        let result = parse_create_keys_response(CREATE_KEYS_RESPONSE.to_vec()).unwrap();

        assert_eq!(result.0, "U5LXs4U7P9msh647kToezy");
        assert_eq!(result.1, "FktSZg8idAVzyQZrdUppK6FTrfAzW3wWVzAjJAfdUvJq");
    }

    #[test]
    fn test_create_key_set_invalid_did_errors(){
        let to_did = "Fh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey).clone();

        assert_eq!(msg.validate_rc, error::INVALID_DID.code_num);
    }
}

