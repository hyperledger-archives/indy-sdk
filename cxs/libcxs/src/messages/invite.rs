extern crate rust_base58;
extern crate serde_json;

use utils::error;
use messages::validation;
use messages::GeneralMessage;


#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct CreateKeyPayload{
    #[serde(rename = "type")]
    msg_type: String,
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

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInvitationPayload {
    #[serde(rename = "type")]
    msg_type: String,
    msg_uid: String,
    enterprise_name: String,
    logo_url: String,
    sender_did: String,
    sender_verkey: String,
    key_delegate: String,
    remote_endpoint: String,
    push_com_method: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInvitation {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: AcceptInvitationPayload,
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

}

impl AcceptInvitation{

    pub fn create() -> AcceptInvitation {
        AcceptInvitation {
            to_did: String::new(),
            payload: AcceptInvitationPayload{
                msg_type: "INVITE_ANSWERED".to_string(),
                msg_uid: String::new(),
                enterprise_name: String::new(),
                logo_url: String::new(),
                sender_did: String::new(),
                sender_verkey: String::new(),
                key_delegate: String::new(),
                remote_endpoint: String::new(),
                push_com_method: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
        }
    }

    pub fn msg_uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.msg_uid = uid.to_string();
        self
    }

    pub fn enterprise_name(&mut self, name: &str) -> &mut Self {
        self.payload.enterprise_name = name.to_string();
        self
    }

    pub fn logo_url(&mut self, url: &str)-> &mut Self {
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

    pub fn sender_did(&mut self, did: &str) -> &mut Self {
        match validation::validate_did(did){
            Ok(x) => {
                self.payload.sender_did = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn sender_verkey(&mut self, verkey: &str) -> &mut Self {
        match validation::validate_verkey(verkey){
            Ok(x) => {
                self.payload.sender_verkey = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> &mut Self {
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

    pub fn remote_endpoint(&mut self, endpoint: &str) -> &mut Self {
        //todo: is this validate URL??
        self.payload.remote_endpoint = endpoint.to_string();
        self
    }

    pub fn push_com_method(&mut self, method: &str) -> &mut Self {
        //todo: is this validate URL??
        self.payload.push_com_method = method.to_string();
        self
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for AcceptInvitation{
    type Msg = AcceptInvitation;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::{create_keys, accept_invitation, update_data, send_invite};

    #[test]
    fn test_create_key_returns_message_with_create_key_as_payload(){
        let msg = create_keys();
        let msg_payload = CreateKeyPayload{
            for_did: String::new(),
            for_verkey: String::new(),
            msg_type: "CREATE_KEY".to_string(),
            nonce: String::new(),
        };
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let nonce = "nonce";
        let msg_payload = CreateKeyPayload{
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
    fn test_create_key_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let nonce = "nonce";
        let msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey)
            .nonce(nonce)
            .serialize_message().unwrap();
        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"forDIDVerKey\\\":\\\"EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A\\\",\
            \\\"forDid\\\":\\\"11235yBzrpJQmNyZzgoTqB\\\",\
            \\\"nonce\\\":\\\"nonce\\\",\
            \\\"type\\\":\\\"CREATE_KEY\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}"
        );
    }

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

    #[test]
    fn test_accept_invitation_set_values_and_serialize(){
        let msg_uid = "123";
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let name = "name";
        let url = "https://random.com";
        let sender_did = "99Fh8yBzrpJQmNyZzgoTqB";
        let sender_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let key = "key";
        let endpoint = "https://??.com";
        let push_method = "push??";
        let msg = accept_invitation()
            .to(&to_did)
            .msg_uid(&msg_uid)
            .enterprise_name(&name)
            .logo_url(&url)
            .sender_did(&sender_did)
            .sender_verkey(&sender_verkey)
            .key_delegate(&key)
            .remote_endpoint(&endpoint)
            .push_com_method(&push_method)
            .serialize_message().unwrap();
        assert_eq!(msg, "{\"agentPayload\":\
        \"{\\\"enterpriseName\\\":\\\"name\\\",\
            \\\"keyDelegate\\\":\\\"key\\\",\
            \\\"logoUrl\\\":\\\"https://random.com\\\",\
            \\\"msgUid\\\":\\\"123\\\",\
            \\\"pushComMethod\\\":\\\"push??\\\",\
            \\\"remoteEndpoint\\\":\\\"https://??.com\\\",\
            \\\"senderDid\\\":\\\"99Fh8yBzrpJQmNyZzgoTqB\\\",\
            \\\"senderVerkey\\\":\\\"EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A\\\",\
            \\\"type\\\":\\\"INVITE_ANSWERED\\\"}\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}"

        );
    }

    #[test]
    fn test_create_key_set_invalid_did_errors_at_serialize(){
        let to_did = "Fh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let nonce = "nonce";
        let mut msg = create_keys()
            .to(to_did)
            .for_did(for_did)
            .for_verkey(for_verkey)
            .nonce(nonce).clone();

        match msg.serialize_message(){
            Ok(_) => panic!("should have had did error"),
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num)
        }
    }
}
