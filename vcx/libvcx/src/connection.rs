extern crate rand;
extern crate serde_json;
extern crate rmp_serde;
extern crate serde;
extern crate libc;

use utils::libindy::wallet;
use utils::error;
use utils::libindy::signus::create_and_store_my_did;
use utils::libindy::crypto;
use utils::json::mapped_key_rewrite;
use api::VcxStateType;
use settings;
use messages::GeneralMessage;
use messages;
use messages::invite::{InviteDetail, SenderDetail};
use messages::get_message::Message;
use serde::Deserialize;
use self::rmp_serde::{encode, Deserializer};
use messages::MessageResponseCode::{ MessageAccepted };
use serde_json::Value;
use utils::json::KeyMatch;
use error::connection::ConnectionError;
use error::ToErrorCode;
use object_cache::ObjectCache;
use utils::constants::DEFAULT_SERIALIZE_VERSION;

lazy_static! {
    static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Serialize, Deserialize)]
struct ConnectionOptions {
    #[serde(default)]
    connection_type: Option<String>,
    #[serde(default)]
    phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Connection {
    source_id: String,
    pw_did: String,
    pw_verkey: String,
    state: VcxStateType,
    uuid: String,
    endpoint: String,
    // For QR code invitation
    invite_detail: Option<InviteDetail>,
    agent_did: String,
    agent_vk: String,
    their_pw_did: String,
    their_pw_verkey: String, // used by proofs/credentials when sending to edge device
}

impl Connection {
    fn _connect_send_invite(&mut self, options: Option<String>) -> Result<u32, ConnectionError> {
        debug!("sending invite for connection {}", self.source_id);

        let options_obj: ConnectionOptions = match options {
            Some(opt) => {
                match opt.trim().is_empty() {
                    true => ConnectionOptions {
                        connection_type: None,
                        phone: None
                    },
                    false => match serde_json::from_str(opt.trim()) {
                        Ok(val) => val,
                        // TODO: Refactor Error
//                        TODO: Implement Correct Error
//                        Err(_) => return Err(error::INVALID_OPTION.code_num),
                        Err(_) => return Err(ConnectionError::GeneralConnectionError()),
                    }
                }
            },
            None => {
                ConnectionOptions {
                    connection_type: None,
                    phone: None
                }
            }
        };
        match messages::send_invite()
            .to(&self.pw_did)
            .to_vk(&self.pw_verkey)
            .phone_number(&options_obj.phone)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .send_secure() {
            Err(ec) => {
                // TODO: Refactor Error
                // TODO: Implement Correct Error
                return Err(ConnectionError::CommonError(ec))
            },
            Ok(response) => {
                self.state = VcxStateType::VcxStateOfferSent;
                self.invite_detail = match parse_invite_detail(&response[0]) {
                    Ok(x) => Some(x),
                    Err(x) => {
                        error!("error when sending invite for connection {}: {}", self.source_id, x);
                        // TODO: Refactor Error
                        // TODO: Implement Correct Error
                        return Err(ConnectionError::GeneralConnectionError())
                    },
                };
                Ok(error::SUCCESS.code_num)
            }
        }
    }
    pub fn delete_connection(&mut self) -> Result<u32, ConnectionError> {
        match messages::delete_connection()
            .to(&self.pw_did)
            .to_vk(&self.pw_verkey)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .send_secure() {
            Err(ec) => {
                return Err(ConnectionError::CannotDeleteConnection())
            },
            Ok(response) => {
                self.state = VcxStateType::VcxStateNone;
                Ok(error::SUCCESS.code_num)
            }
        }

    }

    fn _connect_accept_invite(&mut self, options: Option<String>) -> Result<u32,ConnectionError> {
        debug!("accepting invite for connection {}", self.source_id);

        if let Some(ref details) = self.invite_detail {
            match messages::accept_invite()
                .to(&self.pw_did)
                .to_vk(&self.pw_verkey)
                .agent_did(&self.agent_did)
                .agent_vk(&self.agent_vk)
                .sender_details(&details.sender_detail)
                .sender_agency_details(&details.sender_agency_detail)
                .answer_status_code("MS-104")
                .reply_to(&details.conn_req_id)
                .send_secure() {
                Err(_) => {
                    // TODO: Refactor Error
                    // TODO: Implement Correct Error
                    Err(ConnectionError::GeneralConnectionError())
                },
                Ok(response) => {
                    self.state = VcxStateType::VcxStateAccepted;
                    Ok(error::SUCCESS.code_num)
                }
            }
        }
        else{
            warn!("{} can not connect without invite details", self.source_id);
            // TODO: Refactor Error
            // TODO: Implement Correct Error
            Err(ConnectionError::GeneralConnectionError())
        }
    }


    fn connect(&mut self, options: Option<String>) -> Result<u32,ConnectionError> {
        match self.state {
            VcxStateType::VcxStateInitialized
                | VcxStateType::VcxStateOfferSent => self._connect_send_invite(options),
            VcxStateType::VcxStateRequestReceived => self._connect_accept_invite(options),
            _ => {
                warn!("connection {} in state {} not ready to connect",self.source_id, self.state as u32);
                // TODO: Refactor Error
                // TODO: Implement Correct Error
                Err(ConnectionError::GeneralConnectionError())
            }
        }
    }

    fn get_state(&self) -> u32 { self.state as u32 }
    fn set_state(&mut self, state: VcxStateType) { self.state = state; }

    fn get_pw_did(&self) -> &String { &self.pw_did }
    fn set_pw_did(&mut self, did: &str) { self.pw_did = did.to_string(); }

    fn get_their_pw_did(&self) -> &String { &self.their_pw_did }
    fn set_their_pw_did(&mut self, did: &str) { self.their_pw_did = did.to_string(); }

    fn get_agent_did(&self) -> &String { &self.agent_did }
    fn set_agent_did(&mut self, did: &str) { self.agent_did = did.to_string(); }

    fn get_pw_verkey(&self) -> &String { &self.pw_verkey }
    fn set_pw_verkey(&mut self, verkey: &str) { self.pw_verkey = verkey.to_string(); }

    fn get_their_pw_verkey(&self) -> &String { &self.their_pw_verkey }
    fn set_their_pw_verkey(&mut self, verkey: &str) { self.their_pw_verkey = verkey.to_string(); }

    fn get_agent_verkey(&self) -> &String { &self.agent_vk }
    fn set_agent_verkey(&mut self, verkey: &str) { self.agent_vk = verkey.to_string(); }

    fn get_uuid(&self) -> &String { &self.uuid }
    fn set_uuid(&mut self, uuid: &str) { self.uuid = uuid.to_string(); }

    fn get_endpoint(&self) -> &String { &self.endpoint }
    fn set_endpoint(&mut self, endpoint: &str) { self.endpoint = endpoint.to_string(); }

    fn get_invite_detail(&self) -> &Option<InviteDetail> { &self.invite_detail }
    fn set_invite_detail(&mut self, invite_detail: InviteDetail) { self.invite_detail = Some(invite_detail); }

    fn get_source_id(&self) -> &String { &self.source_id }

    fn ready_to_connect(&self) -> bool {
        if self.state == VcxStateType::VcxStateNone || self.state == VcxStateType::VcxStateAccepted {
            false
        } else {
            true
        }
    }

    fn from_str(s: &str) -> Result<Self, ConnectionError> {
        let s:Value = serde_json::from_str(&s)
            .or(Err(ConnectionError::InvalidJson()))?;
        let connection: Connection = serde_json::from_value(s["data"].clone())
            .or(Err(ConnectionError::InvalidJson()))?;
        Ok(connection)
    }

    fn to_string(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }
}

pub fn is_valid_handle(handle: u32) -> bool {
    CONNECTION_MAP.has_handle(handle)
}

pub fn set_agent_did(handle: u32, did: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_agent_did(did);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_agent_did(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_agent_did().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_pw_did(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_pw_did().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_pw_did(handle: u32, did: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_pw_did(did);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_their_pw_did(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_their_pw_did().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_their_pw_did(handle: u32, did: &str) -> Result<(), ConnectionError>{
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_their_pw_did(did);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_their_pw_verkey(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_their_pw_verkey().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_their_pw_verkey(handle: u32, did: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| { cxn.set_their_pw_verkey(did); Ok(()) }).map_err(|e| {
        ConnectionError::InvalidHandle()
    })
}

pub fn get_uuid(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_uuid().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_uuid(handle: u32, uuid: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_uuid(uuid);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

// TODO: Add NO_ENDPOINT error to connection error
pub fn get_endpoint(handle: u32) -> Result<String, u32> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_endpoint().clone())
    }).or(Err(error::NO_ENDPOINT.code_num))
}

pub fn set_endpoint(handle: u32, endpoint: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_endpoint(endpoint);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_agent_verkey(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_agent_verkey().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_agent_verkey(handle: u32, verkey: &str) -> Result<(), ConnectionError>{
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_agent_verkey(verkey);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_pw_verkey(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_pw_verkey().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn set_pw_verkey(handle: u32, verkey: &str) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_pw_verkey(verkey);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_state(handle: u32) -> u32 {
    match CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_state().clone())
    }) {
        Ok(s) => s,
        Err(_) => 0,
    }
}

pub fn set_state(handle: u32, state: VcxStateType) -> Result<(), ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_state(state);
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn get_source_id(handle: u32) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_source_id().clone())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn create_agent_pairwise(handle: u32) -> Result<u32, ConnectionError> {
    debug!("creating pairwise keys on agent for connection {}", get_source_id(handle).unwrap_or_default());
    let pw_did = get_pw_did(handle)?;
    let pw_verkey = get_pw_verkey(handle)?;

    let result = messages::create_keys()
        .for_did(&pw_did)
        .for_verkey(&pw_verkey)
        .send_secure()
        .or(Err(ConnectionError::InvalidWalletSetup()))?;   // Throw a context specific error
    debug!("create key for connection: {} with did/vk: {:?}",  get_source_id(handle).unwrap_or_default(),  result);
    set_agent_did(handle,&result[0]).err();
    set_agent_verkey(handle,&result[1]).err();
    Ok(error::SUCCESS.code_num)
}

pub fn update_agent_profile(handle: u32) -> Result<u32, ConnectionError> {
    debug!("updating agent config for connection {}", get_source_id(handle).unwrap_or_default());
    let pw_did = get_pw_did(handle)?;
    if let Ok(name) = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME) {
        match messages::update_data()
            .to(&pw_did)
            .name(&name)
            .logo_url(&settings::get_config_value(settings::CONFIG_INSTITUTION_LOGO_URL).map_err(|e| ConnectionError::CommonError(e))?)
            .send_secure() {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(ec) => Err(ConnectionError::CommonError(ec)),
        }
    } else { Ok(error::SUCCESS.code_num) }
}

//
// NOTE: build_connection and create_connection are broken up to make it easier to create connections in tests
//       you can call create_connection without test_mode and you don't have to build a wallet or
//       mock the agency during the connection phase
//
fn create_connection(source_id: &str) -> Result<u32, ConnectionError> {
    // This is a new connection

    let c = Connection {
        source_id: source_id.to_string(),
        pw_did: String::new(),
        pw_verkey: String::new(),
        state: VcxStateType::VcxStateNone,
        uuid: String::new(),
        endpoint: String::new(),
        invite_detail: None,
        agent_did: String::new(),
        agent_vk: String::new(),
        their_pw_did: String::new(),
        their_pw_verkey: String::new(),
    };

    let new_handle = CONNECTION_MAP.add(c).map_err(|key| ConnectionError::CreateError(key))?;
    debug!("creating connection: {} {}", new_handle, source_id);
    Ok(new_handle)

}

fn init_connection(handle: u32) -> Result<u32, ConnectionError> {
    let (my_did, my_verkey) = match create_and_store_my_did(None) {
        Ok(y) => y,
        Err(x) => {
            error!("{} could not create DID/VK: {}", get_source_id(handle).unwrap_or_default(), x);
            return Err(ConnectionError::CommonError(x))
        },
    };

    debug!("handle: {} did: {} verkey: {}, source id: {}", handle, my_did, my_verkey, get_source_id(handle)?);
    set_pw_did(handle, &my_did).err();
    set_pw_verkey(handle, &my_verkey).err();

    match create_agent_pairwise(handle) {
        Err(err) => {
            error!("Error while Creating Agent Pairwise: {}", err);
            release(handle)?;
            return Err(err)
        },
        Ok(_) => debug!("created pairwise key on agent"),
    };

    match update_agent_profile(handle) {
        Err(x) => {
            error!("could not update profile on agent: {}", x);
            release(handle)?;
            return Err(x)
        },
        Ok(_) => debug!("updated profile on agent"),
    };

    set_state(handle, VcxStateType::VcxStateInitialized).err();

    Ok(error::SUCCESS.code_num)
}

pub fn build_connection(source_id: &str) -> Result<u32,ConnectionError> {
    let new_handle = create_connection(source_id)?;

    match init_connection(new_handle) {
        Ok(_) => Ok(new_handle),
        Err(x) => {
            release(new_handle)?;
            return Err(x)
        }
    }
}

pub fn build_connection_with_invite(source_id: &str, details: &str) -> Result<u32,ConnectionError> {
    debug!("using invite to create connection {}", source_id);

    let details:Value = serde_json::from_str(&details)
        .or(Err(ConnectionError::CommonError(error::INVALID_JSON.code_num)))?;

    let invite_details:InviteDetail = match serde_json::from_value(details.clone()) {
        Ok(x) => x,
        Err(x) => {
            // Try converting to abbreviated
            match unabbrv_event_detail(details) {
                Ok(x) => match serde_json::from_value(x) {
                    Ok(x) => x,
                    Err(x) => return Err(ConnectionError::CommonError(error::INVALID_JSON.code_num)),
                }
                Err(_) => return Err(ConnectionError::CommonError(error::INVALID_JSON.code_num)),
            }
        },
    };

    let new_handle = create_connection(source_id)?;

    match init_connection(new_handle){
        Ok(_) => (),
        Err(x) => {
            release(new_handle)?;
            return Err(x);
        }
    };

    set_their_pw_did(new_handle, invite_details.sender_detail.did.as_str()).err();
    set_their_pw_verkey(new_handle, invite_details.sender_detail.verkey.as_str()).err();

    set_invite_details(new_handle, invite_details).err();

    set_state(new_handle, VcxStateType::VcxStateRequestReceived).err();

    Ok(new_handle)
}

pub fn parse_acceptance_details(handle: u32, message: &Message) -> Result<SenderDetail, ConnectionError> {
    debug!("connection {} parsing acceptance details for message {:?}", get_source_id(handle).unwrap_or_default(), message);
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).map_err(|e| ConnectionError::CommonError(e))?;
    let payload = messages::to_u8(
        message.payload
        .as_ref()
        .ok_or(ConnectionError::CommonError(error::INVALID_MSGPACK.code_num))?
    );
    // TODO: check returned verkey
    let (_, payload) = crypto::parse_msg(&my_vk,&payload).map_err(|e| {ConnectionError::CommonError(e)})?;

    trace!("deserializing GetMsgResponse: {:?}", payload);

    let mut de = Deserializer::new(&payload[..]);
    let response: messages::get_message::GetMsgResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse outer msg: {}", x);
            return Err(ConnectionError::CommonError(error::INVALID_MSGPACK.code_num))
        },
    };

    let payload = messages::to_u8(&response.msg);
    // TODO: Refactor Error
    let details = messages::invite::parse_invitation_acceptance_details(payload).map_err(|e| {ConnectionError::CommonError(e)})?;

    Ok(details)
}

pub fn update_state(handle: u32) -> Result<u32, ConnectionError> {
    debug!("updating state for connection {}", get_source_id(handle).unwrap_or_default());
    // TODO: Refactor Error
    let pw_did = get_pw_did(handle)?;
    let pw_vk = get_pw_verkey(handle)?;
    let agent_did = get_agent_did(handle)?;
    let agent_vk = get_agent_verkey(handle)?;

    match messages::get_messages()
        .to(&pw_did)
        .to_vk(&pw_vk)
        .agent_did(&agent_did)
        .agent_vk(&agent_vk)
        .send_secure() {
        Err(x) => {
            error!("could not update state for handle {}: {}",  handle, x);
            // TODO: Refactor Error
            Err(ConnectionError::CommonError(error::POST_MSG_FAILURE.code_num))
        }
        Ok(response) => {
            debug!("connection {} update state response: {:?}", get_source_id(handle).unwrap_or_default(), response);
            if get_state(handle) == VcxStateType::VcxStateOfferSent as u32 || get_state(handle) == VcxStateType::VcxStateInitialized as u32{
                 for i in response {
                     if i.status_code == MessageAccepted.as_string() && i.msg_type == "connReqAnswer" {
                         // TODO: Refactor Error
                          let details = parse_acceptance_details(handle, &i)?;
                          set_their_pw_did(handle, &details.did).ok();
                          set_their_pw_verkey(handle, &details.verkey).ok();
                          set_state(handle, VcxStateType::VcxStateAccepted).ok();
                     }
                 }
            };

            Ok(error::SUCCESS.code_num)
            //TODO: add expiration handling
        },
    }
}
pub fn delete_connection(handle:u32) -> Result<u32, ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |t| {
        match t.delete_connection() {
            Ok(x) => Ok(x),
            Err(e) => {
                return Err(e.to_error_code())
            },
        }
    }).or(Err(ConnectionError::CannotDeleteConnection())).and(release(handle))
}

pub fn connect(handle: u32, options: Option<String>) -> Result<u32, ConnectionError> {
    CONNECTION_MAP.get_mut(handle, |t| {
        t.connect(options.clone()).map_err(|ec| ec.to_error_code())
    }).map_err(|ec| ConnectionError::CommonError(ec))
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    CONNECTION_MAP.get(handle, |t| {
        // TODO: Make this an error.to_error_code and back again?
        Ok(Connection::to_string(&t))
    }).or(Err(error::INVALID_CONNECTION_HANDLE.code_num))
}

pub fn from_string(connection_data: &str) -> Result<u32, ConnectionError> {
    let derived_connection: Connection = match Connection::from_str(connection_data) {
        Ok(x) => x,
        Err(_) => return Err(ConnectionError::CommonError(error::INVALID_JSON.code_num)),
    };


    let new_handle = CONNECTION_MAP.add(derived_connection).map_err(|ec| ConnectionError::CommonError(ec))?;
    debug!("inserting handle {} source_id {} into connection table", new_handle, get_source_id(new_handle).unwrap_or_default());

    Ok(new_handle)
}

pub fn release(handle: u32) -> Result< u32, ConnectionError> {
    match CONNECTION_MAP.release(handle) {
        Ok(_) => Ok(ConnectionError::CommonError(error::SUCCESS.code_num).to_error_code()),
        Err(_) => Err(ConnectionError::InvalidHandle())
    }
}

pub fn release_all() {
    match CONNECTION_MAP.drain() {
        Ok(_) => (),
        // TODO: This needs to be better
        Err(_) => (),
    };
}

pub fn get_invite_details(handle: u32, abbreviated:bool) -> Result<String, ConnectionError> {
    CONNECTION_MAP.get(handle, |t| {
        match abbreviated {
            false => {
                Ok(serde_json::to_string(&t.invite_detail)
                    .or(Err(ConnectionError::InviteDetailError())))
            },
            true => {
                let details = serde_json::to_value(&t.invite_detail).or(Err(ConnectionError::InviteDetailError().to_error_code()))?;
                let abbr = abbrv_event_detail(details)?;
                Ok(serde_json::to_string(&abbr).or(Err(ConnectionError::InviteDetailError())))
            },
        }
    }).or(Err(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)))?

}

pub fn set_invite_details(handle: u32, invite_detail: InviteDetail) -> Result<(), ConnectionError>{
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_invite_detail(invite_detail.clone());
//        TODO: Verify that this is ok to do...seems not rusty.
        Ok(())
    }).or(Err(ConnectionError::InvalidHandle()))
}

pub fn parse_invite_detail(response: &str) -> Result<InviteDetail, ConnectionError> {

    let details: InviteDetail = match serde_json::from_str(response) {
        Ok(x) => x,
        Err(x) => {
            debug!("Connect called without a valid response from server: {}", x);
            return Err(ConnectionError::InviteDetailError());
        },
    };

    Ok(details)
}

// TODO: Refactor Error
// this will become a CommonError, because multiple types (Connection/Issuer Credential) use this function
// Possibly this function moves out of this file.
// On second thought, this should stick as a ConnectionError.
pub fn generate_encrypted_payload(my_vk: &str, their_vk: &str, data: &str, msg_type: &str) -> Result<Vec<u8>, ConnectionError> {
    let my_payload = messages::Payload {
        msg_info: messages::MsgInfo { name: msg_type.to_string(), ver: "1.0".to_string(), fmt: "json".to_string(), },
        msg: data.to_string(),
    };
    let bytes = match encode::to_vec_named(&my_payload) {
        Ok(x) => x,
        Err(x) => {
            error!("could not encode create_keys msg: {}", x);
            return Err(ConnectionError::InvalidMessagePack());
        },
    };
    trace!("Sending payload: {:?}", bytes);
    crypto::prep_msg(wallet::get_wallet_handle(),&my_vk, &their_vk, &bytes).map_err(|ec| ConnectionError::CommonError(ec))
}



//**********
// Code to convert InviteDetails to Abbreviated String
//**********


impl KeyMatch for (String,Option<String>){
    fn matches(&self, key: &String, context: &Vec<String>) -> bool {
        if key.eq(&self.0) {
            match context.last() {
                Some(parent) => {
                    if let Some(ref expected_parent) = self.1 {
                        return parent.eq(expected_parent);
                    }
                },
                None => {
                    return self.1.is_none();
                }
            }
        }
        false
    }
}


lazy_static!{
    static ref ABBREVIATIONS: Vec<(String, String)> = {
        vec![
        ("statusCode".to_string(),          "sc".to_string()),
        ("connReqId".to_string(),           "id".to_string()),
        ("senderDetail".to_string(),        "s".to_string()),
        ("name".to_string(),                "n".to_string()),
        ("agentKeyDlgProof".to_string(),    "dp".to_string()),
        ("agentDID".to_string(),            "d".to_string()),
        ("agentDelegatedKey".to_string(),   "k".to_string()),
        ("signature".to_string(),           "s".to_string()),
        ("DID".to_string(), "d".to_string()),
        ("logoUrl".to_string(), "l".to_string()),
        ("verKey".to_string(), "v".to_string()),
        ("senderAgencyDetail".to_string(), "sa".to_string()),
        ("endpoint".to_string(), "e".to_string()),
        ("targetName".to_string(), "t".to_string()),
        ("statusMsg".to_string(), "sm".to_string()),
        ]
    };
}

lazy_static!{
    static ref UNABBREVIATIONS: Vec<((String, Option<String>), String)> = {
        vec![
        (("sc".to_string(), None),                                  "statusCode".to_string()),
        (("id".to_string(), None),                                  "connReqId".to_string()),
        (("s".to_string(), None),                                   "senderDetail".to_string()),
        (("n".to_string(), Some("senderDetail".to_string())),       "name".to_string()),
        (("dp".to_string(), Some("senderDetail".to_string())),      "agentKeyDlgProof".to_string()),
        (("d".to_string(), Some("agentKeyDlgProof".to_string())),   "agentDID".to_string()),
        (("k".to_string(), Some("agentKeyDlgProof".to_string())),   "agentDelegatedKey".to_string()),
        (("s".to_string(), Some("agentKeyDlgProof".to_string())),   "signature".to_string()),
        (("d".to_string(), Some("senderDetail".to_string())),       "DID".to_string()),
        (("l".to_string(), Some("senderDetail".to_string())),       "logoUrl".to_string()),
        (("v".to_string(), Some("senderDetail".to_string())),       "verKey".to_string()),
        (("sa".to_string(), None),                                  "senderAgencyDetail".to_string()),
        (("d".to_string(), Some("senderAgencyDetail".to_string())), "DID".to_string()),
        (("v".to_string(), Some("senderAgencyDetail".to_string())), "verKey".to_string()),
        (("e".to_string(), Some("senderAgencyDetail".to_string())), "endpoint".to_string()),
        (("t".to_string(), None),                                   "targetName".to_string()),
        (("sm".to_string(), None),                                  "statusMsg".to_string()),
        ]
    };
}

fn abbrv_event_detail(val: Value) -> Result<Value, u32> {
    mapped_key_rewrite(val, &ABBREVIATIONS)
}

fn unabbrv_event_detail(val: Value) -> Result<Value, u32> {
    mapped_key_rewrite(val, &UNABBREVIATIONS)
}



#[cfg(test)]
pub mod tests {
    use utils::constants::*;
    use utils::httpclient;
    use messages::get_message::*;
    use std::thread;
    use std::time::Duration;
    use utils::constants::INVITE_DETAIL_STRING;
    use super::*;
    use rand::Rng;

    pub fn create_connected_connections() -> (u32, u32) {
        let alice = build_connection("alice").unwrap();
        connect(alice, Some("{}".to_string())).unwrap();
        let details = get_invite_details(alice, false).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        ::utils::devsetup::tests::set_consumer();
        let faber = build_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(faber));
        connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        ::utils::devsetup::tests::set_institution();
        thread::sleep(Duration::from_millis(2000));
        update_state(alice).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(alice));
        (faber, alice)
    }

    #[test]
    fn test_build_connection_failures(){
        init!("true");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        assert_eq!(build_connection("This Should Fail").err(),
                   Some(ConnectionError::CommonError(error::INVALID_WALLET_HANDLE.code_num)));
        assert!(build_connection_with_invite("This Should Fail", "BadDetailsFoobar").is_err());
    }
    #[test]
    fn test_create_connection() {
        init!("true");
        let handle = build_connection("test_create_connection").unwrap();
        assert!(handle > 0);
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        connect(handle, Some("{}".to_string())).unwrap();
        assert_eq!(delete_connection(handle).unwrap(), 0);
        // This errors b/c we release handle in delete connection
        assert!(release(handle).is_err());
    }

    #[test]
    fn test_create_drop_create() {
        init!("true");
        let handle = build_connection("test_create_drop_create").unwrap();
        let did1 = get_pw_did(handle).unwrap();
        assert!(release(handle).is_ok());
        let handle2 = build_connection("test_create_drop_create").unwrap();
        assert_ne!(handle,handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        assert!(release(handle2).is_ok());
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        assert_eq!(rc.err(),
                   Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);
        assert_eq!(state, VcxStateType::VcxStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        match to_string(0) {
            Ok(_) => assert_eq!(1,0), //fail if we get here
            Err(_) => assert_eq!(0,0),
        };
    }

    #[test]
    fn test_parse_invite_details() {
        let invite = parse_invite_detail(INVITE_DETAIL_STRING).unwrap();
        assert_eq!(invite.sender_detail.verkey,"ESE6MnqAyjRigduPG454vfLvKhMbmaZjy9vqxCnSKQnp");
        assert_eq!(parse_invite_detail(BAD_INVITE_DETAIL_STRING).err(), Some(ConnectionError::InviteDetailError()));
    }

    #[test]
    fn test_get_qr_code_data() {
        init!("true");
        let test_name = "test_get_qr_code_data";
        let c = Connection {
            source_id: test_name.to_string(),
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            state: VcxStateType::VcxStateOfferSent,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: Some(InviteDetail::new()),
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        println!("updating state, handle: {}", handle);
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        update_state(handle).unwrap();
        let details = get_invite_details(handle, true).unwrap();
        assert!(details.contains("\"dp\":"));
        assert_eq!(get_invite_details(12345, true).err(),
                   Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
    }

    #[test]
    fn test_serialize_deserialize() {
        init!("true");
        let handle = build_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        assert!(release(handle).is_ok());
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        assert!(release(handle).is_ok());
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_deserialize_existing() {
        init!("true");
        let handle = build_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_retry_connection() {
        init!("true");
        let handle = build_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        connect(handle,Some(String::new())).unwrap();
        connect(handle, Some(String::new())).unwrap();
    }

    #[test]
    fn test_bad_wallet_connection_fails() {
        init!("true");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        assert_eq!(build_connection("test_bad_wallet_connection_fails").unwrap_err().to_error_code(),error::INVALID_WALLET_HANDLE.code_num);
    }

    #[test]
    fn test_parse_acceptance_details() {
        init!("true");
        let test_name = "test_parse_acceptance_details";
        let handle = rand::thread_rng().gen::<u32>();

        let response = Message {
            status_code: MessageAccepted.as_string(),
            payload: Some(vec![-126, -91, 64, 116, 121, 112, 101, -125, -92, 110, 97, 109, 101, -83, 99, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, -93, 118, 101, 114, -93, 49, 46, 48, -93, 102, 109, 116, -84, 105, 110, 100, 121, 46, 109, 115, 103, 112, 97, 99, 107, -92, 64, 109, 115, 103, -36, 1, 53, -48, -127, -48, -84, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, -48, -125, -48, -93, 68, 73, 68, -48, -74, 67, 113, 85, 88, 113, 53, 114, 76, 105, 117, 82, 111, 100, 55, 68, 67, 52, 97, 86, 84, 97, 115, -48, -90, 118, 101, 114, 75, 101, 121, -48, -39, 44, 67, 70, 86, 87, 122, 118, 97, 103, 113, 65, 99, 117, 50, 115, 114, 68, 106, 117, 106, 85, 113, 74, 102, 111, 72, 65, 80, 74, 66, 111, 65, 99, 70, 78, 117, 49, 55, 113, 117, 67, 66, 57, 118, 71, -48, -80, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, -48, -125, -48, -88, 97, 103, 101, 110, 116, 68, 73, 68, -48, -74, 57, 54, 106, 111, 119, 113, 111, 84, 68, 68, 104, 87, 102, 81, 100, 105, 72, 49, 117, 83, 109, 77, -48, -79, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, -48, -39, 44, 66, 105, 118, 78, 52, 116, 114, 53, 78, 88, 107, 69, 103, 119, 66, 56, 81, 115, 66, 51, 109, 109, 109, 122, 118, 53, 102, 119, 122, 54, 85, 121, 53, 121, 112, 122, 90, 77, 102, 115, 74, 56, 68, 122, -48, -87, 115, 105, 103, 110, 97, 116, 117, 114, 101, -48, -39, 88, 77, 100, 115, 99, 66, 85, 47, 99, 89, 75, 72, 49, 113, 69, 82, 66, 56, 80, 74, 65, 43, 48, 51, 112, 121, 65, 80, 65, 102, 84, 113, 73, 80, 74, 102, 52, 84, 120, 102, 83, 98, 115, 110, 81, 86, 66, 68, 84, 115, 67, 100, 119, 122, 75, 114, 52, 54, 120, 87, 116, 80, 43, 78, 65, 68, 73, 57, 88, 68, 71, 55, 50, 50, 103, 113, 86, 80, 77, 104, 117, 76, 90, 103, 89, 67, 103, 61, 61]),
            sender_did: "H4FBkUidRG8WLsWa7M6P38".to_string(),
            uid: "yzjjywu".to_string(),
            msg_type: "connReqAnswer".to_string(),
            ref_msg_id: None,
            delivery_details: Vec::new(),
            decrypted_payload: None,
        };

        let c = Connection {
            source_id: test_name.to_string(),
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            state: VcxStateType::VcxStateOfferSent,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: None,
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        parse_acceptance_details(handle, &response).unwrap();

        // test that it fails
        let bad_response = Message {
            status_code: MessageAccepted.as_string(),
            payload: None, // This will cause an error
            sender_did: "H4FBkUidRG8WLsWa7M6P38".to_string(),
            uid: "yzjjywu".to_string(),
            msg_type: "connReqAnswer".to_string(),
            ref_msg_id: None,
            delivery_details: Vec::new(),
            decrypted_payload: None,
        };

        match parse_acceptance_details(handle, &bad_response) {
            Ok(_) => assert_eq!(0,1), // we should not receive this
            // TODO: Refactor Error
            // TODO: Fix this test to be a correct Error Type
            Err(e) => assert_eq!(e, ConnectionError::CommonError(1019)),
        }
    }

    #[test]
    fn test_invite_detail_abbr() {
        let invite_detail:Value = serde_json::from_str(INVITE_DETAIL_STRING).unwrap();
        let abbr = abbrv_event_detail(invite_detail).unwrap();

        let abbr_obj = abbr.as_object().unwrap();
        assert_eq!(abbr_obj.get("sc").unwrap(), "MS-101")
    }

    #[test]
    fn test_invite_detail_abbr2() {
        let un_abbr = json!({
  "statusCode":"MS-102",
  "connReqId":"yta2odh",
  "senderDetail":{
    "name":"ent-name",
    "agentKeyDlgProof":{
      "agentDID":"N2Uyi6SVsHZq1VWXuA3EMg",
      "agentDelegatedKey":"CTfF2sZ5q4oPcBvTP75pgx3WGzYiLSTwHGg9zUsJJegi",
      "signature":"/FxHMzX8JaH461k1SI5PfyxF5KwBAe6VlaYBNLI2aSZU3APsiWBfvSC+mxBYJ/zAhX9IUeTEX67fj+FCXZZ2Cg=="
    },
    "DID":"F2axeahCaZfbUYUcKefc3j",
    "logoUrl":"ent-logo-url",
    "verKey":"74xeXSEac5QTWzQmh84JqzjuXc8yvXLzWKeiqyUnYokx"
  },
  "senderAgencyDetail":{
    "DID":"BDSmVkzxRYGE4HKyMKxd1H",
    "verKey":"6yUatReYWNSUfEtC2ABgRXmmLaxCyQqsjLwv2BomxsxD",
    "endpoint":"52.38.32.107:80/agency/msg"
  },
  "targetName":"there",
  "statusMsg":"message sent"
});

        let abbr = json!({
  "sc":"MS-102",
  "id": "yta2odh",
  "s": {
    "n": "ent-name",
    "dp": {
      "d": "N2Uyi6SVsHZq1VWXuA3EMg",
      "k": "CTfF2sZ5q4oPcBvTP75pgx3WGzYiLSTwHGg9zUsJJegi",
      "s":
        "/FxHMzX8JaH461k1SI5PfyxF5KwBAe6VlaYBNLI2aSZU3APsiWBfvSC+mxBYJ/zAhX9IUeTEX67fj+FCXZZ2Cg==",
    },
    "d": "F2axeahCaZfbUYUcKefc3j",
    "l": "ent-logo-url",
    "v": "74xeXSEac5QTWzQmh84JqzjuXc8yvXLzWKeiqyUnYokx",
  },
  "sa": {
    "d": "BDSmVkzxRYGE4HKyMKxd1H",
    "v": "6yUatReYWNSUfEtC2ABgRXmmLaxCyQqsjLwv2BomxsxD",
    "e": "52.38.32.107:80/agency/msg",
  },
  "t": "there",
  "sm":"message sent"
});
        let processed = abbrv_event_detail(un_abbr.clone()).unwrap();
        assert_eq!(processed, abbr);
        let unprocessed = unabbrv_event_detail(processed).unwrap();
        assert_eq!(unprocessed, un_abbr);
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let h1 = build_connection("rel1").unwrap();
        let h2 = build_connection("rel2").unwrap();
        let h3 = build_connection("rel3").unwrap();
        let h4 = build_connection("rel4").unwrap();
        let h5 = build_connection("rel5").unwrap();
        release_all();
        assert_eq!(release(h1).err(),Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(release(h2).err(),Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(release(h3).err(),Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(release(h4).err(),Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(release(h5).err(),Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
    }

    #[test]
    fn test_create_with_valid_invite_details() {
        init!("true");

        let details = r#"{"id":"njjmmdg","s":{"d":"JZho9BzVAEk8jJ1hwrrDiZ","dp":{"d":"JDF8UHPBTXigvtJWeeMJzx","k":"AP5SzUaHHhF5aLmyKHB3eTqUaREGKyVttwo5T4uwEkM4","s":"JHSvITBMZiTEhpK61EDIWjQOLnJ8iGQ3FT1nfyxNNlxSngzp1eCRKnGC/RqEWgtot9M5rmTC8QkZTN05GGavBg=="},"l":"https://robohash.org/123","n":"Evernym","v":"AaEDsDychoytJyzk4SuzHMeQJGCtQhQHDitaic6gtiM1"},"sa":{"d":"YRuVCckY6vfZfX9kcQZe3u","e":"52.38.32.107:80/agency/msg","v":"J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v"},"sc":"MS-101","sm":"message created","t":"there"}"#;
        let unabbrv_details = unabbrv_event_detail(serde_json::from_str(details).unwrap()).unwrap();
        let details = serde_json::to_string(&unabbrv_details).unwrap();

        let handle = build_connection_with_invite("alice",&details).unwrap();

        connect(handle,Some("{}".to_string())).unwrap();

        let handle_2 = build_connection_with_invite("alice",&details).unwrap();

        connect(handle_2,Some("{}".to_string())).unwrap();
    }

    #[test]
    fn test_create_with_invalid_invite_details() {
        init!("true");
        let bad_details = r#"{"id":"mtfjmda","s":{"d":"abc"},"l":"abc","n":"Evernym","v":"avc"},"sa":{"d":"abc","e":"abc","v":"abc"},"sc":"MS-101","sm":"message created","t":"there"}"#;
        match build_connection_with_invite("alice",&bad_details) {
            Ok(_) => panic!("should have failed"),
            Err(x) => assert_eq!(x, ConnectionError::CommonError(error::INVALID_JSON.code_num)),
        };
    }

    #[test]
    fn test_connect_with_invalid_details() {
        use error::connection::ConnectionError;
        init!("true");
        let test_name = "test_connect_with_invalid_details";

        let c = Connection {
            source_id: test_name.to_string(),
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            state: VcxStateType::VcxStateRequestReceived,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: None,
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        assert_eq!(connect(handle, Some("{}".to_string())).err(), Some(ConnectionError::CommonError(error::CONNECTION_ERROR.code_num)));

        // from_string throws a ConnectionError
        assert_eq!(from_string("").err(), Some(ConnectionError::CommonError(1016)));

        // release throws a connection Error
        assert_eq!(release(1234).err(),
                   Some(ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));

    }

    #[test]
    fn test_void_functions_actually_have_results(){
        assert_eq!(set_their_pw_verkey(1, "blah").err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_state(1, VcxStateType::VcxStateNone).err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_pw_did(1, "blah").err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_their_pw_did(1,"blah").err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_uuid(1,"blah").err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_endpoint(1,"blah").err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_agent_verkey(1,"blah").err(), Some(ConnectionError::InvalidHandle()));
        let details: InviteDetail = serde_json::from_str(INVITE_DETAIL_STRING).unwrap();
        assert_eq!(set_invite_details(1, details).err(), Some(ConnectionError::InvalidHandle()));
        assert_eq!(set_pw_verkey(1, "blah").err(), Some(ConnectionError::InvalidHandle()));
    }
}
