use serde_json;
use serde_json::Value;
use rmp_serde;

use api::VcxStateType;
use settings;
use messages;
use messages::{GeneralMessage, MessageStatusCode, RemoteMessageType, ObjectWithVersion};
use messages::invite::{InviteDetail, SenderDetail, Payload as ConnectionPayload, AcceptanceDetails};
use messages::payload::{Payloads, Thread};
use messages::get_message::{Message, MessagePayload};
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_and_store_my_did;
use utils::libindy::crypto;
use utils::json::mapped_key_rewrite;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use utils::json::KeyMatch;
use std::collections::HashMap;

lazy_static! {
    static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionOptions {
    #[serde(default)]
    connection_type: Option<String>,
    #[serde(default)]
    phone: Option<String>,
    use_public_did: Option<bool>,
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        ConnectionOptions {
            connection_type: None,
            phone: None,
            use_public_did: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Connection {
    source_id: String,
    pw_did: String,
    pw_verkey: String,
    state: VcxStateType,
    uuid: String,
    endpoint: String,
    // For QR code invitation
    invite_detail: Option<InviteDetail>,
    invite_url: Option<String>,
    agent_did: String,
    agent_vk: String,
    their_pw_did: String,
    their_pw_verkey: String,
    // used by proofs/credentials when sending to edge device
    public_did: Option<String>,
    their_public_did: Option<String>,
}


impl Connection {
    fn _connect_send_invite(&mut self, options: &ConnectionOptions) -> VcxResult<u32> {
        debug!("sending invite for connection {}", self.source_id);

        let (invite, url) =
            messages::send_invite()
                .to(&self.pw_did)?
                .to_vk(&self.pw_verkey)?
                .phone_number(options.phone.as_ref().map(String::as_str))?
                .agent_did(&self.agent_did)?
                .agent_vk(&self.agent_vk)?
                .public_did(self.public_did.as_ref().map(String::as_str))?
                .thread(&Thread::new())?
                .send_secure()
                .map_err(|err| err.extend("Cannot send invite"))?;

        self.state = VcxStateType::VcxStateOfferSent;
        self.invite_detail = Some(invite);
        self.invite_url = Some(url);

        Ok(error::SUCCESS.code_num)
    }

    pub fn delete_connection(&mut self) -> VcxResult<u32> {
        trace!("Connection::delete_connection >>>");

        messages::delete_connection()
            .to(&self.pw_did)?
            .to_vk(&self.pw_verkey)?
            .agent_did(&self.agent_did)?
            .agent_vk(&self.agent_vk)?
            .send_secure()
            .map_err(|err| err.extend("Cannot delete connection"))?;

        self.state = VcxStateType::VcxStateNone;

        Ok(error::SUCCESS.code_num)
    }

    fn _connect_accept_invite(&mut self) -> VcxResult<u32> {
        debug!("accepting invite for connection {}", self.source_id);

        let details: &InviteDetail = self.invite_detail.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::GeneralConnectionError, format!("Invite details not found for: {}", self.source_id)))?;

        messages::accept_invite()
            .to(&self.pw_did)?
            .to_vk(&self.pw_verkey)?
            .agent_did(&self.agent_did)?
            .agent_vk(&self.agent_vk)?
            .sender_details(&details.sender_detail)?
            .sender_agency_details(&details.sender_agency_detail)?
            .answer_status_code(&MessageStatusCode::Accepted)?
            .reply_to(&details.conn_req_id)?
            .thread(&self._build_thread(&details))?
            .send_secure()
            .map_err(|err| err.extend("Cannot accept invite"))?;

        self.state = VcxStateType::VcxStateAccepted;

        Ok(error::SUCCESS.code_num)
    }

    fn _build_thread(&self, invite_detail: &InviteDetail) -> Thread {
        let mut received_orders = HashMap::new();
        received_orders.insert(invite_detail.sender_detail.did.clone(), 0);
        Thread {
            thid: invite_detail.thread_id.clone(),
            pthid: None,
            sender_order: 0,
            received_orders,
        }
    }

    fn connect(&mut self, options: &ConnectionOptions) -> VcxResult<u32> {
        trace!("Connection::connect >>> options: {:?}", options);
        match self.state {
            VcxStateType::VcxStateInitialized
            | VcxStateType::VcxStateOfferSent => self._connect_send_invite(options),
            VcxStateType::VcxStateRequestReceived => self._connect_accept_invite(),
            _ => {
                warn!("connection {} in state {} not ready to connect", self.source_id, self.state as u32);
                // TODO: Refactor Error
                // TODO: Implement Correct Error
                Err(VcxError::from_msg(VcxErrorKind::GeneralConnectionError, format!("Connection {} in state {} not ready to connect", self.source_id, self.state as u32)))
            }
        }
    }

    fn get_state(&self) -> u32 {
        trace!("Connection::get_state >>>");
        self.state as u32
    }
    fn set_state(&mut self, state: VcxStateType) {
        trace!("Connection::set_state >>> state: {:?}", state);
        self.state = state;
    }

    fn get_pw_did(&self) -> &String { &self.pw_did }
    fn set_pw_did(&mut self, did: &str) { self.pw_did = did.to_string(); }

    fn get_their_pw_did(&self) -> &String { &self.their_pw_did }
    fn set_their_pw_did(&mut self, did: &str) { self.their_pw_did = did.to_string(); }

    fn set_their_public_did(&mut self, did: &str) { self.their_public_did = Some(did.to_string()); }
    fn get_their_public_did(&self) -> Option<String> { self.their_public_did.clone() }

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
        self.state != VcxStateType::VcxStateNone && self.state != VcxStateType::VcxStateAccepted
    }

    fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    fn create_agent_pairwise(&mut self) -> VcxResult<u32> {
        debug!("creating pairwise keys on agent for connection {}", self.source_id);

        let (for_did, for_verkey) = messages::create_keys()
            .for_did(&self.pw_did)?
            .for_verkey(&self.pw_verkey)?
            .send_secure()
            .map_err(|err| err.extend("Cannot create pairwise keys"))?;

        debug!("create key for connection: {} with did {:?}, vk: {:?}", self.source_id, for_did, for_verkey);
        self.set_agent_did(&for_did);
        self.set_agent_verkey(&for_verkey);

        Ok(error::SUCCESS.code_num)
    }

    fn update_agent_profile(&mut self, options: &ConnectionOptions) -> VcxResult<u32> {
        debug!("updating agent config for connection {}", self.source_id);

        if let Some(true) = options.use_public_did {
            self.public_did = Some(settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?);
        };

        if let Ok(name) = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME) {
            messages::update_data()
                .to(&self.pw_did)?
                .name(&name)?
                .logo_url(&settings::get_config_value(settings::CONFIG_INSTITUTION_LOGO_URL)?)?
                .use_public_did(&self.public_did)?
                .send_secure()
                .map_err(|err| err.extend("Cannot update agent profile"))?;
        }

        Ok(error::SUCCESS.code_num)
    }
}

pub fn is_valid_handle(handle: u32) -> bool {
    CONNECTION_MAP.has_handle(handle)
}

pub fn set_agent_did(handle: u32, did: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_agent_did(did))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_agent_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_agent_did().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_pw_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_pw_did().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_pw_did(handle: u32, did: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_pw_did(did))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_their_pw_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_their_pw_did().to_string())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_their_pw_did(handle: u32, did: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_their_pw_did(did))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_their_public_did(handle: u32, did: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_their_public_did(did))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_their_public_did(handle: u32) -> VcxResult<Option<String>> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_their_public_did())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_their_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_their_pw_verkey().to_string())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_their_pw_verkey(handle: u32, did: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_their_pw_verkey(did))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_uuid(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_uuid().to_string())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_uuid(handle: u32, uuid: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_uuid(uuid))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

// TODO: Add NO_ENDPOINT error to connection error
pub fn get_endpoint(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_endpoint().clone())
    }).or(Err(VcxError::from(VcxErrorKind::NoEndpoint)))
}

pub fn set_endpoint(handle: u32, endpoint: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_endpoint(endpoint))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_agent_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_agent_verkey().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_agent_verkey(handle: u32, verkey: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_agent_verkey(verkey))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_pw_verkey().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_pw_verkey(handle: u32, verkey: &str) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_pw_verkey(verkey))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_state(handle: u32) -> u32 {
    CONNECTION_MAP.get(handle, |cxn| {
        debug!("get state for connection {}", cxn.get_source_id());
        Ok(cxn.get_state().clone())
    }).unwrap_or(0)
}

pub fn set_state(handle: u32, state: VcxStateType) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        Ok(cxn.set_state(state))
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.get_source_id().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn create_connection(source_id: &str) -> VcxResult<u32> {
    trace!("create_connection >>> source_id: {}", source_id);

    let (pw_did, pw_verkey) = create_and_store_my_did(None)?;

    debug!("did: {} verkey: {}, source id: {}", pw_did, pw_verkey, source_id);

    let c = Connection {
        source_id: source_id.to_string(),
        pw_did,
        pw_verkey,
        state: VcxStateType::VcxStateInitialized,
        uuid: String::new(),
        endpoint: String::new(),
        invite_detail: None,
        invite_url: None,
        agent_did: String::new(),
        agent_vk: String::new(),
        their_pw_did: String::new(),
        their_pw_verkey: String::new(),
        public_did: None,
        their_public_did: None,
    };

    CONNECTION_MAP.add(c)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn create_connection_with_invite(source_id: &str, details: &str) -> VcxResult<u32> {
    debug!("create connection {} with invite {}", source_id, details);

    let details: Value = serde_json::from_str(&details)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize invite details: {}", err)))?;

    let invite_details: InviteDetail = match serde_json::from_value(details.clone()) {
        Ok(x) => x,
        Err(x) => {
            // Try converting to abbreviated
            let details = unabbrv_event_detail(details)?;
            serde_json::from_value(details)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize invite details: {}", err)))?
        }
    };

    let new_handle = create_connection(source_id)?;

    set_invite_details(new_handle, &invite_details)?;
    set_their_pw_did(new_handle, invite_details.sender_detail.did.as_str())?;
    set_their_pw_verkey(new_handle, invite_details.sender_detail.verkey.as_str())?;
    if let Some(did) = invite_details.sender_detail.public_did {
        set_their_public_did(new_handle, &did)?;
    }

    set_state(new_handle, VcxStateType::VcxStateRequestReceived)?;

    Ok(new_handle)
}

pub fn parse_acceptance_details(handle: u32, message: &Message) -> VcxResult<SenderDetail> {
    debug!("connection {} parsing acceptance details for message {:?}", get_source_id(handle).unwrap_or_default(), message);
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let payload = message.payload
        .as_ref()
        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidMessagePack, "Payload not found"))?;

    match payload {
        MessagePayload::V1(payload) => {
            // TODO: check returned verkey
            let (_, payload) = crypto::parse_msg(&my_vk, &messages::to_u8(&payload))
                .map_err(|err| err.map(VcxErrorKind::InvalidMessagePack, "Cannot decrypt connection payload"))?;

            let response: ConnectionPayload = rmp_serde::from_slice(&payload[..])
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidMessagePack, format!("Cannot parse connection payload: {}", err)))?;

            let payload = messages::to_u8(&response.msg);

            let response: AcceptanceDetails = rmp_serde::from_slice(&payload[..])
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidMessagePack, format!("Cannot deserialize AcceptanceDetails: {}", err)))?;

            Ok(response.sender_detail)
        }
        MessagePayload::V2(payload) => {
            let payload = Payloads::decrypt_payload_v2(&my_vk, &payload)?;
            let response: AcceptanceDetails = serde_json::from_str(&payload.msg)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize AcceptanceDetails: {}", err)))?;

            Ok(response.sender_detail)
        }
    }
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    debug!("updating state for connection {}", get_source_id(handle).unwrap_or_default());
    let state = get_state(handle);

    if state == VcxStateType::VcxStateInitialized as u32 || state == VcxStateType::VcxStateAccepted as u32 {
        return Ok(error::SUCCESS.code_num);
    }

    // TODO: Refactor Error
    let pw_did = get_pw_did(handle)?;
    let pw_vk = get_pw_verkey(handle)?;
    let agent_did = get_agent_did(handle)?;
    let agent_vk = get_agent_verkey(handle)?;

    let response =
        messages::get_messages()
            .to(&pw_did)?
            .to_vk(&pw_vk)?
            .agent_did(&agent_did)?
            .agent_vk(&agent_vk)?
            .send_secure()
            .map_err(|err| err.map(VcxErrorKind::PostMessageFailed, format!("Could not update state for handle {}", handle)))?;

    debug!("connection {} update state response: {:?}", get_source_id(handle).unwrap_or_default(), response);
    if get_state(handle) == VcxStateType::VcxStateOfferSent as u32 || get_state(handle) == VcxStateType::VcxStateInitialized as u32 {
        for message in response {
            if message.status_code == MessageStatusCode::Accepted && message.msg_type == RemoteMessageType::ConnReqAnswer {
                process_acceptance_message(handle, message)?;
            }
        }
    };

    Ok(error::SUCCESS.code_num)
}

pub fn process_acceptance_message(handle: u32, message: Message) -> VcxResult<u32> {
    let details = parse_acceptance_details(handle, &message)
        .map_err(|err| err.extend("Cannot parse acceptance details"))?;

    set_their_pw_did(handle, &details.did).ok();
    set_their_pw_verkey(handle, &details.verkey).ok();
    set_state(handle, VcxStateType::VcxStateAccepted).ok();

    Ok(error::SUCCESS.code_num)
}

pub fn delete_connection(handle: u32) -> VcxResult<u32> {
    CONNECTION_MAP.get_mut(handle, |t| {
        debug!("delete connection: {}", t.get_source_id());
        t.delete_connection()
    })
        .or(Err(VcxError::from(VcxErrorKind::DeleteConnection)))
        .and(release(handle))
        .and_then(|_| Ok(error::SUCCESS.code_num))
}

pub fn connect(handle: u32, options: Option<String>) -> VcxResult<u32> {
    let options_obj: ConnectionOptions =
        match options.as_ref().map(|opt| opt.trim()) {
            None => ConnectionOptions::default(),
            Some(opt) if opt.is_empty() => ConnectionOptions::default(),
            Some(opt) => {
                serde_json::from_str(&opt)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize ConnectionOptions: {}", err)))?
            }
        };

    CONNECTION_MAP.get_mut(handle, |t| {
        debug!("establish connection {}", t.get_source_id());
        t.create_agent_pairwise()?;
        t.update_agent_profile(&options_obj)?;
        t.connect(&options_obj)
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |t| {
        Connection::to_string(&t)
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn from_string(connection_data: &str) -> VcxResult<u32> {
    let derived_connection: Connection = Connection::from_str(connection_data)?;
    let handle = CONNECTION_MAP.add(derived_connection)?;
    debug!("inserting handle {} source_id {} into connection table", handle, get_source_id(handle).unwrap_or_default());
    Ok(handle)
}

pub fn release(handle: u32) -> VcxResult<()> {
    CONNECTION_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn release_all() {
    CONNECTION_MAP.drain().ok();
}

pub fn get_invite_details(handle: u32, abbreviated: bool) -> VcxResult<String> {
    debug!("get invite details for connection {}", get_source_id(handle).unwrap_or_default());

    CONNECTION_MAP.get(handle, |t| {
        match abbreviated {
            false => {
                serde_json::to_string(&t.invite_detail)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidInviteDetail, format!("Cannot serialize InviteDetail: {}", err)))
            }
            true => {
                let details = serde_json::to_value(&t.invite_detail)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidInviteDetail, format!("Cannot serialize InviteDetail: {}", err)))?;
                let abbr = abbrv_event_detail(details)?;
                serde_json::to_string(&abbr)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidInviteDetail, format!("Cannot serialize abbreviated InviteDetail: {}", err)))
            }
        }
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn set_invite_details(handle: u32, invite_detail: &InviteDetail) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |cxn| {
        cxn.set_invite_detail(invite_detail.clone());
        Ok(())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

//**********
// Code to convert InviteDetails to Abbreviated String
//**********

impl KeyMatch for (String, Option<String>) {
    fn matches(&self, key: &String, context: &Vec<String>) -> bool {
        if key.eq(&self.0) {
            match context.last() {
                Some(parent) => {
                    if let Some(ref expected_parent) = self.1 {
                        return parent.eq(expected_parent);
                    }
                }
                None => {
                    return self.1.is_none();
                }
            }
        }
        false
    }
}


lazy_static! {
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

lazy_static! {
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

fn abbrv_event_detail(val: Value) -> VcxResult<Value> {
    mapped_key_rewrite(val, &ABBREVIATIONS)
}

fn unabbrv_event_detail(val: Value) -> VcxResult<Value> {
    mapped_key_rewrite(val, &UNABBREVIATIONS)
        .map_err(|err| err.extend("Cannot unabbreviate event detail"))
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

    pub fn build_test_connection() -> u32 {
        let handle = create_connection("alice").unwrap();
        connect(handle, Some("{}".to_string())).unwrap();
        handle
    }

    pub fn create_connected_connections() -> (u32, u32) {
        let alice = create_connection("alice").unwrap();
        let my_public_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let options = json!({"use_public_did": true}).to_string();
        connect(alice, Some(options)).unwrap();
        let details = get_invite_details(alice, false).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        ::utils::devsetup::tests::set_consumer();
        let faber = create_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(faber));
        connect(faber, Some("{}".to_string())).unwrap();
        let public_did = get_their_public_did(faber).unwrap().unwrap();
        assert_eq!(my_public_did, public_did);
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        ::utils::devsetup::tests::set_institution();
        thread::sleep(Duration::from_millis(2000));
        update_state(alice, None).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(alice));
        (faber, alice)
    }

    #[test]
    fn test_build_connection_failures() {
        init!("true");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(create_connection("This Should Fail").unwrap_err().kind(), VcxErrorKind::InvalidWalletHandle);
        assert!(create_connection_with_invite("This Should Fail", "BadDetailsFoobar").is_err());
    }

    #[test]
    fn test_create_connection_agency_failure() {
        init!("indy");
        let handle = create_connection("invalid").unwrap();
        let rc = connect(handle, None);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::PostMessageFailed);
    }

    #[test]
    fn test_create_connection() {
        init!("true");
        let handle = create_connection("test_create_connection").unwrap();
        assert!(handle > 0);
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        connect(handle, Some("{}".to_string())).unwrap();
        ::utils::httpclient::set_next_u8_response(GET_MESSAGES_INVITE_ACCEPTED_RESPONSE.to_vec());
        update_state(handle, None).unwrap();
        assert_eq!(get_state(handle), VcxStateType::VcxStateAccepted as u32);
        assert_eq!(delete_connection(handle).unwrap(), 0);
        // This errors b/c we release handle in delete connection
        assert!(release(handle).is_err());
    }

    #[test]
    fn test_create_drop_create() {
        init!("true");
        let handle = create_connection("test_create_drop_create").unwrap();
        let did1 = get_pw_did(handle).unwrap();
        assert!(release(handle).is_ok());
        let handle2 = create_connection("test_create_drop_create").unwrap();
        assert_ne!(handle, handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        assert!(release(handle2).is_ok());
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);
        assert_eq!(state, VcxStateType::VcxStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        match to_string(0) {
            Ok(_) => assert_eq!(1, 0), //fail if we get here
            Err(_) => assert_eq!(0, 0),
        };
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
            invite_url: None,
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
            public_did: None,
            their_public_did: None,
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        println!("updating state, handle: {}", handle);
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        update_state(handle, None).unwrap();
        let details = get_invite_details(handle, true).unwrap();
        assert!(details.contains("\"dp\":"));
        assert_eq!(get_invite_details(12345, true).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
    }

    #[test]
    fn test_serialize_deserialize() {
        init!("true");
        let handle = create_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        assert!(release(handle).is_ok());
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        assert!(release(handle).is_ok());
        assert_eq!(first_string, second_string);
    }

    #[test]
    fn test_deserialize_existing() {
        init!("true");
        let handle = create_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        assert_eq!(first_string, second_string);
    }

    #[test]
    fn test_retry_connection() {
        init!("true");
        let handle = create_connection("test_serialize_deserialize").unwrap();
        assert!(handle > 0);
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        connect(handle, Some(String::new())).unwrap();
        connect(handle, Some(String::new())).unwrap();
    }

    #[test]
    fn test_bad_wallet_connection_fails() {
        init!("true");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(create_connection("test_bad_wallet_connection_fails").unwrap_err().kind(), VcxErrorKind::InvalidWalletHandle);
    }

    #[test]
    fn test_parse_acceptance_details() {
        init!("true");
        let test_name = "test_parse_acceptance_details";
        let handle = rand::thread_rng().gen::<u32>();

        let response = Message {
            status_code: MessageStatusCode::Accepted,
            payload: Some(MessagePayload::V1(vec![-126, -91, 64, 116, 121, 112, 101, -125, -92, 110, 97, 109, 101, -83, 99, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, -93, 118, 101, 114, -93, 49, 46, 48, -93, 102, 109, 116, -84, 105, 110, 100, 121, 46, 109, 115, 103, 112, 97, 99, 107, -92, 64, 109, 115, 103, -36, 1, 53, -48, -127, -48, -84, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, -48, -125, -48, -93, 68, 73, 68, -48, -74, 67, 113, 85, 88, 113, 53, 114, 76, 105, 117, 82, 111, 100, 55, 68, 67, 52, 97, 86, 84, 97, 115, -48, -90, 118, 101, 114, 75, 101, 121, -48, -39, 44, 67, 70, 86, 87, 122, 118, 97, 103, 113, 65, 99, 117, 50, 115, 114, 68, 106, 117, 106, 85, 113, 74, 102, 111, 72, 65, 80, 74, 66, 111, 65, 99, 70, 78, 117, 49, 55, 113, 117, 67, 66, 57, 118, 71, -48, -80, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, -48, -125, -48, -88, 97, 103, 101, 110, 116, 68, 73, 68, -48, -74, 57, 54, 106, 111, 119, 113, 111, 84, 68, 68, 104, 87, 102, 81, 100, 105, 72, 49, 117, 83, 109, 77, -48, -79, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, -48, -39, 44, 66, 105, 118, 78, 52, 116, 114, 53, 78, 88, 107, 69, 103, 119, 66, 56, 81, 115, 66, 51, 109, 109, 109, 122, 118, 53, 102, 119, 122, 54, 85, 121, 53, 121, 112, 122, 90, 77, 102, 115, 74, 56, 68, 122, -48, -87, 115, 105, 103, 110, 97, 116, 117, 114, 101, -48, -39, 88, 77, 100, 115, 99, 66, 85, 47, 99, 89, 75, 72, 49, 113, 69, 82, 66, 56, 80, 74, 65, 43, 48, 51, 112, 121, 65, 80, 65, 102, 84, 113, 73, 80, 74, 102, 52, 84, 120, 102, 83, 98, 115, 110, 81, 86, 66, 68, 84, 115, 67, 100, 119, 122, 75, 114, 52, 54, 120, 87, 116, 80, 43, 78, 65, 68, 73, 57, 88, 68, 71, 55, 50, 50, 103, 113, 86, 80, 77, 104, 117, 76, 90, 103, 89, 67, 103, 61, 61])),
            sender_did: "H4FBkUidRG8WLsWa7M6P38".to_string(),
            uid: "yzjjywu".to_string(),
            msg_type: RemoteMessageType::ConnReqAnswer,
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
            invite_url: None,
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
            public_did: None,
            their_public_did: None,
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        parse_acceptance_details(handle, &response).unwrap();

        // test that it fails
        let bad_response = Message {
            status_code: MessageStatusCode::Accepted,
            payload: None,
            // This will cause an error
            sender_did: "H4FBkUidRG8WLsWa7M6P38".to_string(),
            uid: "yzjjywu".to_string(),
            msg_type: RemoteMessageType::ConnReqAnswer,
            ref_msg_id: None,
            delivery_details: Vec::new(),
            decrypted_payload: None,
        };

        match parse_acceptance_details(handle, &bad_response) {
            Ok(_) => assert_eq!(0, 1), // we should not receive this
            // TODO: Refactor Error
            // TODO: Fix this test to be a correct Error Type
            Err(e) => assert_eq!(e.kind(), VcxErrorKind::InvalidMessagePack),
        }
    }

    #[test]
    fn test_invite_detail_abbr() {
        let invite_detail: Value = serde_json::from_str(INVITE_DETAIL_STRING).unwrap();
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
        let h1 = create_connection("rel1").unwrap();
        let h2 = create_connection("rel2").unwrap();
        let h3 = create_connection("rel3").unwrap();
        let h4 = create_connection("rel4").unwrap();
        let h5 = create_connection("rel5").unwrap();
        release_all();
        assert_eq!(release(h1).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        assert_eq!(release(h2).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        assert_eq!(release(h3).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        assert_eq!(release(h4).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        assert_eq!(release(h5).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
    }

    #[test]
    fn test_create_with_valid_invite_details() {
        init!("true");

        let details = r#"{"id":"njjmmdg","s":{"d":"JZho9BzVAEk8jJ1hwrrDiZ","dp":{"d":"JDF8UHPBTXigvtJWeeMJzx","k":"AP5SzUaHHhF5aLmyKHB3eTqUaREGKyVttwo5T4uwEkM4","s":"JHSvITBMZiTEhpK61EDIWjQOLnJ8iGQ3FT1nfyxNNlxSngzp1eCRKnGC/RqEWgtot9M5rmTC8QkZTN05GGavBg=="},"l":"https://robohash.org/123","n":"Evernym","v":"AaEDsDychoytJyzk4SuzHMeQJGCtQhQHDitaic6gtiM1"},"sa":{"d":"YRuVCckY6vfZfX9kcQZe3u","e":"52.38.32.107:80/agency/msg","v":"J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v"},"sc":"MS-101","sm":"message created","t":"there"}"#;
        let unabbrv_details = unabbrv_event_detail(serde_json::from_str(details).unwrap()).unwrap();
        let details = serde_json::to_string(&unabbrv_details).unwrap();

        let handle = create_connection_with_invite("alice", &details).unwrap();

        connect(handle, Some("{}".to_string())).unwrap();

        let handle_2 = create_connection_with_invite("alice", &details).unwrap();

        connect(handle_2, Some("{}".to_string())).unwrap();
    }

    #[test]
    fn test_process_acceptance_message() {
        init!("true");
        let handle = create_connection("test_process_acceptance_message").unwrap();
        let message = serde_json::from_str(INVITE_ACCEPTED_RESPONSE).unwrap();
        assert_eq!(error::SUCCESS.code_num, process_acceptance_message(handle, message).unwrap());
    }

    #[test]
    fn test_create_with_invalid_invite_details() {
        init!("true");
        let bad_details = r#"{"id":"mtfjmda","s":{"d":"abc"},"l":"abc","n":"Evernym","v":"avc"},"sa":{"d":"abc","e":"abc","v":"abc"},"sc":"MS-101","sm":"message created","t":"there"}"#;
        match create_connection_with_invite("alice", &bad_details) {
            Ok(_) => panic!("should have failed"),
            Err(x) => assert_eq!(x.kind(), VcxErrorKind::InvalidJson),
        };
    }

    #[test]
    fn test_connect_with_invalid_details() {
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
            invite_url: None,
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            their_pw_did: String::new(),
            their_pw_verkey: String::new(),
            public_did: None,
            their_public_did: None,
        };

        let handle = CONNECTION_MAP.add(c).unwrap();

        assert_eq!(connect(handle, Some("{}".to_string())).unwrap_err().kind(), VcxErrorKind::GeneralConnectionError);
        ;

        // from_string throws a ConnectionError
        assert_eq!(from_string("").unwrap_err().kind(), VcxErrorKind::InvalidJson);
        ;

        // release throws a connection Error
        assert_eq!(release(1234).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
    }

    #[test]
    fn test_void_functions_actually_have_results() {
        assert_eq!(set_their_pw_verkey(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_state(1, VcxStateType::VcxStateNone).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_pw_did(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_their_pw_did(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_uuid(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_endpoint(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_agent_verkey(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        let details: InviteDetail = serde_json::from_str(INVITE_DETAIL_STRING).unwrap();
        assert_eq!(set_invite_details(1, &details).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
        assert_eq!(set_pw_verkey(1, "blah").unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        ;
    }

    #[test]
    fn test_connection_release_returns_unit() {
        init!("true");
        let details = r#"{"id":"njjmmdg","s":{"d":"JZho9BzVAEk8jJ1hwrrDiZ","dp":{"d":"JDF8UHPBTXigvtJWeeMJzx","k":"AP5SzUaHHhF5aLmyKHB3eTqUaREGKyVttwo5T4uwEkM4","s":"JHSvITBMZiTEhpK61EDIWjQOLnJ8iGQ3FT1nfyxNNlxSngzp1eCRKnGC/RqEWgtot9M5rmTC8QkZTN05GGavBg=="},"l":"https://robohash.org/123","n":"Evernym","v":"AaEDsDychoytJyzk4SuzHMeQJGCtQhQHDitaic6gtiM1"},"sa":{"d":"YRuVCckY6vfZfX9kcQZe3u","e":"52.38.32.107:80/agency/msg","v":"J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v"},"sc":"MS-101","sm":"message created","t":"there"}"#;
        let handle = create_connection_with_invite("alice", &details).unwrap();
        assert_eq!(release(handle).unwrap(), ());
    }
}
