extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;
extern crate base64;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use messages::send_message::CreateMessagePayload;
use utils::constants::*;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use self::rmp_serde::encode;
use std::str;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct KeyDlgProofPayload {
    #[serde(rename = "agentDID")]
    pub agent_did: String,
    #[serde(rename = "agentDelegatedKey")]
    pub agent_delegated_key: String,
    #[serde(rename = "signature")]
    pub signature: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendMsgDetailPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "keyDlgProof")]
    key_proof: KeyDlgProofPayload,
    #[serde(rename = "phoneNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(rename = "includePublicDID")]
    include_public_did: bool,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct AcceptMsgDetailPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "keyDlgProof")]
    key_proof: KeyDlgProofPayload,
    sender_detail: Option<SenderDetail>,
    sender_agency_detail: Option<SenderAgencyDetail>,
    answer_status_code: Option<String>
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendInvitePayload{
    create_payload: CreateMessagePayload,
    msg_detail_payload: SendMsgDetailPayload,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct AcceptInvitePayload{
    create_payload: CreateMessagePayload,
    msg_detail_payload: AcceptMsgDetailPayload,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SendInvite {
    #[serde(rename = "to")]
    to_did: String,
    to_vk: String,
    #[serde(skip_serializing, default)]
    payload: SendInvitePayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    agent_did: String,
    agent_vk: String,
    #[serde(rename = "publicDID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    public_did: Option<String>,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInvite {
    #[serde(rename = "to")]
    to_did: String,
    to_vk: String,
    #[serde(skip_serializing, default)]
    payload: AcceptInvitePayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    agent_did: String,
    agent_vk: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SenderDetail {
    pub name: Option<String>,
    pub agent_key_dlg_proof: KeyDlgProofPayload,
    #[serde(rename = "DID")]
    pub did: String,
    pub logo_url: Option<String>,
    #[serde(rename = "verKey")]
    pub verkey: String,
    #[serde(rename = "publicDID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_did: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SenderAgencyDetail {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
    pub endpoint: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct InviteDetail {
    status_code: String,
    pub conn_req_id: String,
    pub sender_detail: SenderDetail,
    pub sender_agency_detail: SenderAgencyDetail,
    target_name: String,
    status_msg: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct MsgDetailResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    pub invite_detail: InviteDetail,
    url_to_invite_detail: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct MsgCreateResponse {
    #[serde(rename = "@type")]
    pub msg_type: MsgType,
    pub uid: String,
}

impl InviteDetail {
    pub fn new() -> InviteDetail {
        InviteDetail {
            status_code: String::new(),
            conn_req_id: String::new(),
            sender_detail: SenderDetail {
                name: Some(String::new()),
                agent_key_dlg_proof: KeyDlgProofPayload {
                    agent_did: String::new(),
                    agent_delegated_key: String::new(),
                    signature: String::new(),
                },
                did: String::new(),
                logo_url: Some(String::new()),
                verkey: String::new(),
                public_did: None,
            },
            sender_agency_detail: SenderAgencyDetail {
                did: String::new(),
                verkey: String::new(),
                endpoint: String::new(),
            },
            target_name: String::new(),
            status_msg: String::new(),
        }
    }
}

impl SendInvite{

    pub fn create() -> SendInvite {
        trace!("SendInvite::create_message >>>");

        SendInvite {
            to_did: String::new(),
            to_vk: String::new(),
            payload: SendInvitePayload {
                create_payload: CreateMessagePayload { msg_type: MsgType { name: "CREATE_MSG".to_string(), ver: "1.0".to_string(), } , mtype: "connReq".to_string(), reply_to_msg_id: None, send_msg: true} ,
                msg_detail_payload: SendMsgDetailPayload {
                    msg_type: MsgType { name: "MSG_DETAIL".to_string(), ver: "1.0".to_string(), } ,
                    key_proof: KeyDlgProofPayload { agent_did: String::new(), agent_delegated_key: String::new(), signature: String::new() , } ,
                    phone: None,
                    include_public_did: false,
                } ,
            },
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
            public_did: None,
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> &mut Self{
        match validation::validate_key_delegate(key){
            Ok(x) => {
                self.payload.msg_detail_payload.key_proof.agent_delegated_key = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn public_did(&mut self, did: Option<String>) -> &mut Self{
        if did.is_some() {
            self.payload.msg_detail_payload.include_public_did = true;
        }
        self.public_did = did.clone();
        self
    }

    pub fn phone_number(&mut self, phone_number: &Option<String>)-> &mut Self{
        if let &Some(ref p_num) = phone_number {
            match validation::validate_phone_number(p_num.as_str()) {
                Ok(x) => {
                    self.payload.msg_detail_payload.phone = Some(x);
                }
                Err(x) => {
                    self.validate_rc = x;
                }
            };
        }
        self
    }

    pub fn generate_signature(&mut self) -> Result<u32, u32> {
        let signature = format!("{}{}", self.payload.msg_detail_payload.key_proof.agent_did, self.payload.msg_detail_payload.key_proof.agent_delegated_key);
        let signature = crypto::sign(&self.to_vk, signature.as_bytes())?;
        let signature = base64::encode(&signature);
        self.payload.msg_detail_payload.key_proof.signature = signature.to_string();
        Ok(error::SUCCESS.code_num)
    }

    pub fn send_secure(&mut self) -> Result<Vec<String>, u32> {
        trace!("SendInvite::send >>>");

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_INVITE_RESPONSE.to_vec()); }

        let mut result = Vec::new();
        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let (invite, url) = parse_response(response)?;
                result.push(invite);
                result.push(url);
            },
        };

        Ok(result.to_owned())
    }
    fn print_info(&self) {
//    TODO: This could go away
        println!("\n****\n**** message pack: Send Invite");
        println!("create_payload {}", serde_json::to_string(&self.payload.create_payload).unwrap());
        println!("msg_detail_payload {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap());
        println!("self.to_vk: {}", &self.to_vk);
        println!("self.agent_did: {}", &self.agent_did);
        println!("self.agent_vk: {}", &self.agent_vk);
        debug!("connection invitation details: {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap_or("failure".to_string()));
    }
}

impl AcceptInvite{
//    TODO: This could go away
    fn print_info(&self) {
        println!("\n****\n**** message pack: Accept Invite");
        println!("create_payload {}", serde_json::to_string(&self.payload.create_payload).unwrap());
        println!("msg_detail_payload {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap());
        println!("self.to_vk: {}", &self.to_vk);
        println!("self.agent_did: {}", &self.agent_did);
        println!("self.agent_vk: {}", &self.agent_vk);
        debug!("connection invitation details: {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap_or("failure".to_string()));
    }
    pub fn create() -> AcceptInvite {
        trace!("AcceptInvite::create_message >>>");

        AcceptInvite {
            to_did: String::new(),
            to_vk: String::new(),
            payload: AcceptInvitePayload {
                create_payload: CreateMessagePayload { msg_type: MsgType { name: "CREATE_MSG".to_string(), ver: "1.0".to_string(), } , mtype: "connReqAnswer".to_string(), reply_to_msg_id: None, send_msg: true } ,
                msg_detail_payload: AcceptMsgDetailPayload
                {
                    msg_type: MsgType { name: "MSG_DETAIL".to_string(), ver: "1.0".to_string(), } ,
                    key_proof: KeyDlgProofPayload { agent_did: String::new(), agent_delegated_key: String::new(), signature: String::new() , },
                    sender_detail: None,
                    sender_agency_detail: None,
                    answer_status_code: None
                },
            },
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> &mut Self{
        match validation::validate_key_delegate(key){
            Ok(x) => {
                self.payload.msg_detail_payload.key_proof.agent_delegated_key = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn sender_details(&mut self, details: &SenderDetail) -> &mut Self {
        self.payload.msg_detail_payload.sender_detail = Some(details.clone());
        self
    }

    pub fn sender_agency_details(&mut self, details: &SenderAgencyDetail) -> &mut Self {
        self.payload.msg_detail_payload.sender_agency_detail = Some(details.clone());
        self
    }

    pub fn answer_status_code(&mut self, code: &str) -> &mut Self {
        self.payload.msg_detail_payload.answer_status_code = Some(code.to_owned());
        self
    }

    pub fn reply_to(&mut self, id: &str) -> &mut Self {
        self.payload.create_payload.reply_to_msg_id = Some(id.to_owned());
        self
    }

    pub fn generate_signature(&mut self) -> Result<u32, u32> {
        let signature = format!("{}{}", self.payload.msg_detail_payload.key_proof.agent_did, self.payload.msg_detail_payload.key_proof.agent_delegated_key);
        let signature = crypto::sign(&self.to_vk, signature.as_bytes())?;
        let signature = base64::encode(&signature);
        self.payload.msg_detail_payload.key_proof.signature = signature.to_string();
        Ok(error::SUCCESS.code_num)
    }

    pub fn send_secure(&mut self) -> Result<String, u32> {
        trace!("AcceptInvite::send >>>");

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(ACCEPT_INVITE_RESPONSE.to_vec()); }

        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                parse_send_accept_response(response)
            },
        }
    }
}


//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendInvite{
    type Msg = SendInvite;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
        self.payload.msg_detail_payload.key_proof.agent_did = self.agent_did.to_string();
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
        self.payload.msg_detail_payload.key_proof.agent_delegated_key = self.agent_vk.to_string();
    }

    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        self.generate_signature()?;
        debug!("connection invitation details: {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap_or("failure".to_string()));
        let create = encode::to_vec_named(&self.payload.create_payload).or(Err(error::UNKNOWN_ERROR.code_num))?;
        let details = encode::to_vec_named(&self.payload.msg_detail_payload).or(Err(error::UNKNOWN_ERROR.code_num))?;

        let mut bundle = Bundled::create(create);
        bundle.bundled.push(details);

        let msg = bundle.encode()?;

        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

impl GeneralMessage for AcceptInvite{
    type Msg = AcceptInvite;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
        self.payload.msg_detail_payload.key_proof.agent_did = self.agent_did.to_string();
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
        self.payload.msg_detail_payload.key_proof.agent_delegated_key = self.agent_vk.to_string();
    }

    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        self.generate_signature()?;
        debug!("connection invitation details: {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap_or("failure".to_string()));
        let create = encode::to_vec_named(&self.payload.create_payload).or(Err(error::UNKNOWN_ERROR.code_num))?;
        let details = encode::to_vec_named(&self.payload.msg_detail_payload).or(Err(error::UNKNOWN_ERROR.code_num))?;

        let mut bundle = Bundled::create(create);
        bundle.bundled.push(details);

        let msg = bundle.encode()?;

        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

fn parse_response(response: Vec<u8>) -> Result<(String, String), u32> {
    let data = unbundle_from_agency(response)?;

    if data.len() != 3 {
        error!("expected 3 messages (got {})", data.len());
        return Err(error::INVALID_MSGPACK.code_num);
    }
    debug!("invite details response: {:?}", data[1]);
    let mut de = Deserializer::new(&data[1][..]);
    let response: MsgDetailResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    debug!("Invite Details: {:?}", response.invite_detail);
    match serde_json::to_string(&response.invite_detail) {
        Ok(x) => Ok((x, response.url_to_invite_detail)),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

const ACCEPT_BUNDLE_LEN: usize = 2;
fn parse_send_accept_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    if data.len() != ACCEPT_BUNDLE_LEN {
        error!("expected {} messages (got {})",ACCEPT_BUNDLE_LEN, data.len());
        return Err(error::INVALID_MSGPACK.code_num);
    }
    debug!("create msg response: {:?}", data[0]);
    let mut de = Deserializer::new(&data[0][..]);
    let response: MsgCreateResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    Ok(response.uid.to_owned())
}

pub fn parse_invitation_acceptance_details(payload: Vec<u8>) -> Result<SenderDetail,u32> {
    #[serde(rename_all = "camelCase")]
    #[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
    struct Details {
        sender_detail: SenderDetail,
    }

    debug!("parsing invitation acceptance details: {:?}", payload);
    let mut de = Deserializer::new(&payload[..]);
    let response: Details = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => return Err(error::INVALID_MSGPACK.code_num),
    };
    Ok(response.sender_detail.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::send_invite;
    use utils::libindy::signus::create_and_store_my_did;

    #[test]
    fn test_send_invite_set_values_and_post(){
        init!("false");
        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = send_invite()
            .to(&user_did)
            .to_vk(&user_vk)
            .agent_did(&agent_did)
            .agent_vk(&agent_vk)
            .phone_number(&Some("phone".to_string()))
            .key_delegate("key")
            .msgpack().unwrap();

        assert!(msg.len() > 0);
    }

    #[test]
    fn test_parse_send_invite_response() {
        init!("indy");
        let (result, url) = parse_response(SEND_INVITE_RESPONSE.to_vec()).unwrap();

        assert_eq!(result, INVITE_DETAIL_STRING);
        assert_eq!(url, "http://localhost:9001/agency/invite/WRUzXXuFVTYkT8CjSZpFvT?uid=NjcwOWU");
    }

    #[test]
    fn test_parse_invitation_acceptance_details() {
        let payload = vec![129, 172, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, 131, 163, 68, 73, 68, 182, 67, 113, 85, 88, 113, 53, 114, 76, 105, 117, 82, 111, 100, 55, 68, 67, 52, 97, 86, 84, 97, 115, 166, 118, 101, 114, 75, 101, 121, 217, 44, 67, 70, 86, 87, 122, 118, 97, 103, 113, 65, 99, 117, 50, 115, 114, 68, 106, 117, 106, 85, 113, 74, 102, 111, 72, 65, 80, 74, 66, 111, 65, 99, 70, 78, 117, 49, 55, 113, 117, 67, 66, 57, 118, 71, 176, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, 131, 168, 97, 103, 101, 110, 116, 68, 73, 68, 182, 57, 54, 106, 111, 119, 113, 111, 84, 68, 68, 104, 87, 102, 81, 100, 105, 72, 49, 117, 83, 109, 77, 177, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, 217, 44, 66, 105, 118, 78, 52, 116, 114, 53, 78, 88, 107, 69, 103, 119, 66, 56, 81, 115, 66, 51, 109, 109, 109, 122, 118, 53, 102, 119, 122, 54, 85, 121, 53, 121, 112, 122, 90, 77, 102, 115, 74, 56, 68, 122, 169, 115, 105, 103, 110, 97, 116, 117, 114, 101, 217, 88, 77, 100, 115, 99, 66, 85, 47, 99, 89, 75, 72, 49, 113, 69, 82, 66, 56, 80, 74, 65, 43, 48, 51, 112, 121, 65, 80, 65, 102, 84, 113, 73, 80, 74, 102, 52, 84, 120, 102, 83, 98, 115, 110, 81, 86, 66, 68, 84, 115, 67, 100, 119, 122, 75, 114, 52, 54, 120, 87, 116, 80, 43, 78, 65, 68, 73, 57, 88, 68, 71, 55, 50, 50, 103, 113, 86, 80, 77, 104, 117, 76, 90, 103, 89, 67, 103, 61, 61];
        println!("payload: {:?}", payload);
        let response = parse_invitation_acceptance_details(payload).unwrap();
        println!("response: {:?}", response);
    }
}
