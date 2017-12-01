extern crate serde;
extern crate rmp_serde;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod message;

use settings;
use utils::crypto;
use utils::wallet;
use utils::error;
use self::rmp_serde::encode;
use self::create_key::CreateKeyMsg;
use self::invite::{SendInvite, UpdateProfileData};
use self::message::{GetMessages, SendMessage};

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
}

#[derive(Serialize, Deserialize)]
pub struct MsgResponse {
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
}

pub fn bundle_for_agency(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY).unwrap();
    let agent_vk = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY).unwrap();
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, &agent_vk, &message[..])?;

    let outer = Forward {
        msg_type: "{\"name\":\"FWD\",\"ver\":\"0.1\"}".to_owned(),
        fwd: did.to_owned(),
        msg,
    };

    let bundle = Bundled::create(outer);
    let msg = match encode::to_vec_named(&bundle) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not convert bundle to messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<u8>, u32> {
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, &message[..])
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct Forward {
    #[serde(rename = "@type")]
    msg_type: String,
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
