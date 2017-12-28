extern crate serde;
extern crate rmp_serde;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod message;
pub mod proofs;

use settings;
use utils::crypto;
use utils::wallet;
use utils::error;
use self::rmp_serde::encode;
use self::create_key::CreateKeyMsg;
use self::invite::{SendInvite, UpdateProfileData};
use self::message::{GetMessages, SendMessage};
use self::proofs::proof_request::{ProofRequestMessage};

pub enum MessageResponseCode {
    MessageCreate,
    MessageSent,
    MessagePending,
    MessageAccepted,
    MessageRejected,
    MessageAnswered,
}

impl MessageResponseCode {
    pub fn as_str(&self) -> &str {
        match *self {
            MessageResponseCode::MessageCreate => "MS-101",
            MessageResponseCode::MessageSent => "MS-102",
            MessageResponseCode::MessagePending => "MS-103",
            MessageResponseCode::MessageAccepted => "MS-104",
            MessageResponseCode::MessageRejected => "MS-105",
            MessageResponseCode::MessageAnswered => "MS-106",
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct MsgType {
    name: String,
    ver: String,
}

#[derive(Serialize, Deserialize)]
pub struct MsgResponse {
    #[serde(rename = "@type")]
    msg_type: String,
    msg_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Bundled<T> {
    bundled: Vec<T>,
}

impl<T> Bundled<T> {
    pub fn create(bundled: T) -> Bundled<T> {
        let mut vec = Vec::new();
        vec.push(bundled);
        Bundled {
            bundled: vec,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, u32>
        where T: serde::Serialize {
        match encode::to_vec_named(self) {
            Ok(x) => Ok(x),
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                Err(error::INVALID_MSGPACK.code_num)
            },
        }
    }
}

pub fn bundle_for_agency(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY).unwrap();
    let agent_vk = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY).unwrap();
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    info!("pre encryption msg: {:?}", message);
    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, &agent_vk, &message[..])?;

    let outer = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_owned(),
        msg,
    };

    let msg = Bundled::create(outer).encode()?;
    info!("pre encryption bundle: {:?}", msg);
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<u8>, u32> {

    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, &message[..])
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct Forward {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "@fwd")]
    fwd: String,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

pub trait GeneralMessage{
    type Msg;

    //todo: add version
    //todo: add encryption
    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> &mut Self {
        match validation::validate_did(to_did){
            Ok(x) => {
                self.set_to_did(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn serialize_message(&mut self) -> Result<String, u32>;
    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_validate_rc(&mut self, rc: u32);
    fn send(&mut self) -> Result<String, u32>;
    fn to_post(&self) -> Result<Vec<u8>, u32>;
    fn send_enc(&mut self) -> Result<String, u32>;

}


pub fn create_keys() -> CreateKeyMsg {
    CreateKeyMsg::create()
}

pub fn send_invite() -> SendInvite{
    SendInvite::create()
}

pub fn update_data() -> UpdateProfileData{
    UpdateProfileData::create()
}

pub fn get_messages() -> GetMessages { GetMessages::create() }

pub fn send_message() -> SendMessage { SendMessage::create() }

pub fn proof_request() -> ProofRequestMessage { ProofRequestMessage::create() }

#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use utils::httpclient;
    use serde::Deserialize;
    use self::rmp_serde::Deserializer;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct ConnectMsg {
        msg_type: MsgType,
        from_did: String,
        from_vk: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct GenericMsg {
        msg_type: MsgType,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct RegisterResponse {
        msg_type: MsgType,
    }

    pub fn parse_register_response(response: Vec<u8>) -> Result<String, u32> {

        let data = unbundle_from_agency(response)?;

        let mut de = Deserializer::new(&data[..]);
        let bundle: Bundled<RegisterResponse> = match Deserialize::deserialize(&mut de) {
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

    #[ignore]
    #[test]
    fn test_connect_register_provision() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let their_did = "BDSmVkzxRYGE4HKyMKxd1H";
        let their_vk = "B6c4yr4A5XvTmSMaLBsop2BZFT2h5ULzZvWFy6Q83Dgx";
        let my_did = "4fUDR9R7fjwELRvH9JT6HH";
        let my_vk = "2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm";
        let host = "http://3d2898b1.ngrok.io";
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,my_did);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY,my_vk);
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,host);
        settings::set_config_value(settings::CONFIG_WALLET_NAME,"my_real_wallet");
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY,their_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY,their_vk);
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());
        wallet::init_wallet("my_real_wallet").unwrap();

        /* step 1: CONNECT */
        let payload = ConnectMsg {
            msg_type: MsgType { name: "CONNECT".to_string(), ver: "1.0".to_string(), },
            from_did: my_did.to_string(),
            from_vk: my_vk.to_string(),
        };

        let data = bundle_for_agency(Bundled::create(payload).encode().unwrap(), their_did).unwrap();
        let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

        let mut de = Deserializer::new(&data[..]);
        let bundle: Bundled<ConnectMsg> = Deserialize::deserialize(&mut de).unwrap();

        println!("new did: {} new vk: {}",bundle.bundled[0].from_did, bundle.bundled[0].from_vk);

        /* step 2: SIGNUP */
        let payload = GenericMsg {
            msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
        };

        let data = bundle_for_agency(Bundled::create(payload).encode().unwrap(), their_did).unwrap();
        let response = parse_register_response(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("response: {}", response);

        /* step3: CREATE_AGENT */
        let payload = GenericMsg {
            msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
        };

        let data = bundle_for_agency(Bundled::create(payload).encode().unwrap(), their_did).unwrap();
        let response = create_key::parse_create_keys_response(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("response: {}", response);
    }

}
