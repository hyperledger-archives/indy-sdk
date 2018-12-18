extern crate serde;
extern crate rmp_serde;
extern crate serde_json;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod get_message;
pub mod send_message;
pub mod update_profile;
pub mod proofs;
pub mod agent_utils;
pub mod update_connection;
pub mod update_message;

use std::u8;
use settings;
use utils::libindy::crypto;
use utils::error;
use self::rmp_serde::encode;
use self::create_key::CreateKeyMsg;
use self::update_connection::DeleteConnection;
use self::invite::{AcceptInvite, SendInvite};
use self::update_profile::UpdateProfileData;
use self::get_message::GetMessages;
use self::send_message::SendMessage;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use serde_json::Value;
use self::proofs::proof_request::{ProofRequestMessage};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct MsgInfo {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct Payload {
    #[serde(rename = "@type")]
    pub msg_info: MsgInfo,
    #[serde(rename = "@msg")]
    pub msg: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
}

pub enum MessageResponseCode {
    MessageCreate,
    MessageSent,
    MessagePending,
    MessageAccepted,
    MessageRejected,
    MessageAnswered,
}

impl MessageResponseCode {
    pub fn as_string(&self) -> String {
        match *self {
            MessageResponseCode::MessageCreate => String::from("MS-101"),
            MessageResponseCode::MessageSent => String::from("MS-102"),
            MessageResponseCode::MessagePending => String::from("MS-103"),
            MessageResponseCode::MessageAccepted => String::from("MS-104"),
            MessageResponseCode::MessageRejected => String::from("MS-105"),
            MessageResponseCode::MessageAnswered => String::from("MS-106"),
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
    msg_type: MsgType,
    uid: String,
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

    pub fn encode(&self) -> Result<Vec<u8>, u32> where T: serde::Serialize {
        let result = match encode::to_vec_named(self) {
            Ok(x) => x,
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                return Err(error::INVALID_MSGPACK.code_num);
            },
        };

        Ok(result)
    }
}

pub fn try_i8_bundle(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let mut de = Deserializer::new(&data[..]);
    let bundle: Bundled<Vec<i8>> = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(_) => {
            warn!("could not deserialize bundle with i8, will try u8");
            return Err(error::INVALID_MSGPACK.code_num);
        },
    };

    let mut new_bundle: Bundled<Vec<u8>> = Bundled { bundled: Vec::new() };
    for i in bundle.bundled {
        let mut buf: Vec<u8> = Vec::new();
        for j in i {buf.push(j as u8);}
        new_bundle.bundled.push(buf);
    }
    Ok(new_bundle)
}

pub fn to_u8(bytes: &Vec<i8>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for i in bytes {buf.push(*i as u8);}
    buf.to_owned()
}

pub fn to_i8(bytes: &Vec<u8>) -> Vec<i8> {
    let mut buf: Vec<i8> = Vec::new();
    for i in bytes {buf.push(*i as i8);}
    buf.to_owned()
}

pub fn to_json(bytes: &Vec<u8>) -> Result<Value, u32> {
    let mut de = Deserializer::new(&bytes[..]);
    match Deserialize::deserialize(&mut de) {
        Ok(x) => Ok(x),
        Err(x) => Err(error::INVALID_JSON.code_num),
    }
}

pub fn bundle_from_u8(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let bundle = match try_i8_bundle(data.clone()) {
        Ok(x) => x,
        Err(x) => {
            let mut de = Deserializer::new(&data[..]);
            let bundle: Bundled<Vec<u8>> = match Deserialize::deserialize(&mut de) {
                Ok(x) => x,
                Err(x) => {
                    error!("could not deserialize bundle with i8 or u8: {}", x);
                    return Err(error::INVALID_MSGPACK.code_num);
                },
            };
            bundle
        },
    };

    Ok(bundle)
}

pub fn extract_json_payload(data: &Vec<u8>) -> Result<String, u32> {
    let mut de = Deserializer::new(&data[..]);
    let my_payload: Payload = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("could not deserialize bundle with i8 or u8: {}", x);
            return Err(error::INVALID_MSGPACK.code_num);
            },
        };

    Ok(my_payload.msg.to_owned())
}

pub fn bundle_for_agency(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    trace!("pre encryption msg: {:?}", message);
    let msg = crypto::prep_msg(&my_vk, &agent_vk, &message[..])?;

    debug!("forwarding agency bundle to {}", did);
    let outer = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_owned(),
        msg,
    };
    let outer = encode::to_vec_named(&outer).or(Err(error::UNKNOWN_ERROR.code_num))?;

    trace!("forward bundle: {:?}", outer);
    let msg = Bundled::create(outer).encode()?;
    trace!("pre encryption bundle: {:?}", msg);
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn bundle_for_agent(message: Vec<u8>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    debug!("pre encryption msg: {:?}", message);
    let msg = crypto::prep_msg(&pw_vk, agent_vk, &message[..])?;

    /* forward to did */
    debug!("forwarding agent bundle to {}", agent_did);
    let inner = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: agent_did.to_string(),
        msg,
    };
    let inner = encode::to_vec_named(&inner).or(Err(error::UNKNOWN_ERROR.code_num))?;
    debug!("inner forward: {:?}", inner);

    let msg = Bundled::create(inner).encode()?;

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
    bundle_for_agency(msg, &to_did)
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<Vec<u8>>, u32> {

    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let (_, data) = crypto::parse_msg(&my_vk, &message[..])?;

    debug!("deserializing {:?}", data);
    let bundle:Bundled<Vec<u8>> = bundle_from_u8(data)?;

    Ok(bundle.bundled.clone())
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Forward {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "@fwd")]
    fwd: String,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

pub trait GeneralMessage{
    type Msg;

    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> &mut Self {
        match validation::validate_did(to_did){
            Ok(x) => {
                self.set_to_did(x);
                self
            },
            Err(x) => {
                warn!("could not validate recipient did");
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn to_vk(&mut self, to_vk: &str) -> &mut Self {
         match validation::validate_verkey(to_vk){
            Ok(x) => {
                self.set_to_vk(x);
                self
            },
            Err(x) => {
                warn!("could not validate recipient vk");
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn agent_did(&mut self, did: &str) -> & mut Self {
         match validation::validate_did(did){
            Ok(x) => {
                self.set_agent_did(x);
                self
            },
            Err(x) => {
                warn!("could not validate agent_did");
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn agent_vk(&mut self, to_vk: &str) -> &mut Self {
         match validation::validate_verkey(to_vk){
            Ok(x) => {
                self.set_agent_vk(x);
                self
            },
            Err(x) => {
                warn!("could not validate agent_vk");
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_agent_did(&mut self, did: String);
    fn set_agent_vk(&mut self, vk: String);
    fn set_validate_rc(&mut self, rc: u32);
    fn msgpack(&mut self) -> Result<Vec<u8>, u32>;
}

pub fn create_keys() -> CreateKeyMsg { CreateKeyMsg::create() }
pub fn send_invite() -> SendInvite { SendInvite::create() }
pub fn delete_connection() -> DeleteConnection { DeleteConnection::create() }
pub fn accept_invite() -> AcceptInvite { AcceptInvite::create() }
pub fn update_data() -> UpdateProfileData{ UpdateProfileData::create() }
pub fn get_messages() -> GetMessages { GetMessages::create() }
pub fn send_message() -> SendMessage { SendMessage::create() }
pub fn proof_request() -> ProofRequestMessage { ProofRequestMessage::create() }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -111, -36, 5, -74];

        let buf = to_u8(&vec);
        println!("new bundle: {:?}", buf);
    }

    #[test]
    fn test_to_i8() {
        let vec: Vec<u8> = vec![129, 167, 98, 117, 110, 100, 108, 101, 100, 145, 220, 19, 13];
        let buf = to_i8(&vec);
        println!("new bundle: {:?}", buf);
    }
}
