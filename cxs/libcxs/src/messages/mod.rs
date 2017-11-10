pub mod invite;
pub mod validation;
pub mod message;

use self::invite::{CreateKeyMsg, SendInvite, AcceptInvitation, UpdateProfileData};
use self::message::{GetMessages, SendMessage};

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    AcceptInviteMsg(AcceptInvitation),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
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
    fn set_to_did(&mut self, to_did: String);
    fn set_validate_rc(&mut self, rc: u32);
    fn send(&mut self) -> Result<String, u32>;

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

pub fn accept_invitation() -> AcceptInvitation{
    AcceptInvitation::create()
}

pub fn get_messages() -> GetMessages { GetMessages::create() }

pub fn send_message() -> SendMessage { SendMessage::create() }
