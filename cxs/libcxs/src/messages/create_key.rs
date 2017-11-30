extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::{Bundled, GeneralMessage, validation, bundle_for_agency};
use self::rmp_serde::encode;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct CreateKeyPayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "forDID")]
    for_did: String,
    #[serde(rename = "forDIDVerKey")]
    for_verkey: String,
    nonce: String,
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
}

impl CreateKeyMsg{

    pub fn create() -> CreateKeyMsg {
        CreateKeyMsg {
            to_did: String::new(),
            payload: CreateKeyPayload{
                msg_type: "CREATE_KEY".to_string(),
                for_did: String::new(),
                for_verkey: String::new(),
                nonce: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
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

    pub fn nonce(&mut self, nonce: &str) -> &mut Self {
        match validation::validate_nonce(nonce){
            Ok(x) => {
                self.payload.nonce = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for CreateKeyMsg  {
    type Msg = CreateKeyMsg;

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

        let json_msg = match self.serialize_message() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

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
            Ok(response) => parse_create_keys_response(&response),
        }
    }
}

fn parse_create_keys_response(response: &Vec<u8>) -> Result<String, u32> {
    Ok(String::new().to_owned())
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::*;
    use utils::signus::SignusUtils;
    use messages::{create_keys, Forward};
    use utils::wallet;
    use utils::crypto;
    use serde::Deserialize;
    use self::rmp_serde::Deserializer;

    #[test]
    fn test_create_key_returns_message_with_create_key_as_payload() {
        let msg = create_keys();
        let msg_payload = CreateKeyPayload {
            for_did: String::new(),
            for_verkey: String::new(),
            msg_type: "CREATE_KEY".to_string(),
            nonce: String::new(),
        };
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let nonce = "nonce";
        let msg_payload = CreateKeyPayload {
            for_did: for_did.to_string(),
            for_verkey: for_verkey.to_string(),
            msg_type: "CREATE_KEY".to_string(),
            nonce: nonce.to_string(),
        };
        let msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey)
            .nonce(nonce).clone();
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values_and_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let agency_wallet = wallet::init_wallet("test_create_key_set_values_and_serialize_agency", POOL, "Default").unwrap();
        let agent_wallet = wallet::init_wallet("test_create_key_set_values_and_serialize_agent", POOL, "Default").unwrap();
        let my_wallet = wallet::init_wallet("test_create_key_set_values_and_serialize_mine", POOL, "Default").unwrap();

        let (agent_did, agent_vk) = SignusUtils::create_and_store_my_did(agent_wallet, Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = SignusUtils::create_and_store_my_did(agency_wallet, Some(MY3_SEED)).unwrap();

        SignusUtils::store_their_did_from_parts(my_wallet, agent_did.as_ref(), agent_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(my_wallet, agency_did.as_ref(), agency_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(agent_wallet, my_did.as_ref(), my_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(agency_wallet, my_did.as_ref(), my_vk.as_ref()).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY, &my_vk);
        let nonce = "nonce";
        let bytes = create_keys()
            .to(&agent_did)
            .for_did(&my_did)
            .for_verkey(&my_vk)
            .nonce(nonce)
            .to_post().unwrap();
        assert!(bytes.len() > 0);

        let (_, msg) = crypto::parse_msg(agency_wallet,&agency_vk,&bytes[..]).unwrap();

        let mut de = Deserializer::new(&msg[..]);
        let bundle: Bundled<Forward> = Deserialize::deserialize(&mut de).unwrap();

        println!("first unbundle");
        println!("{:?}", bundle);

        let msg = bundle.bundled[0].msg.clone();
        println!("msg: {:?}", msg);
        let (_, msg) = crypto::parse_msg(agent_wallet,&agent_vk,&msg[..]).unwrap();

        let mut de = Deserializer::new(&msg[..]);
        let bundle: Bundled<CreateKeyPayload> = Deserialize::deserialize(&mut de).unwrap();

        println!("second unbundle");
        println!("{:?}", bundle);

        wallet::delete_wallet("test_create_key_set_values_and_serialize_mine").unwrap();
        wallet::delete_wallet("test_create_key_set_values_and_serialize_agent").unwrap();
        wallet::delete_wallet("test_create_key_set_values_and_serialize_agency").unwrap();
    }

    #[test]
    fn test_create_key_set_invalid_did_errors(){
        let to_did = "Fh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let nonce = "nonce";
        let msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey)
            .nonce(nonce).clone();

        assert_eq!(msg.validate_rc, error::INVALID_DID.code_num);
    }
}

