use serde_json;
use serde_json::Value;

use object_cache::ObjectCache;
use api::VcxStateType;
use issuer_credential::{CredentialOffer, CredentialMessage, PaymentInfo};
use credential_request::CredentialRequest;
use messages;
use messages::{GeneralMessage, RemoteMessageType, ObjectWithVersion};
use messages::payload::{Payloads, PayloadKinds, Thread};
use messages::get_message;
use messages::get_message::MessagePayload;
use connection;
use settings;
use utils::libindy::anoncreds::{libindy_prover_create_credential_req, libindy_prover_store_credential};
use utils::libindy::anoncreds;
use utils::libindy::payments::{pay_a_payee, PaymentTxn};
use utils::error;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use error::prelude::*;

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<Credential>  = Default::default();
}

impl Default for Credential {
    fn default() -> Credential
    {
        Credential {
            source_id: String::new(),
            state: VcxStateType::VcxStateNone,
            credential_name: None,
            credential_request: None,
            agent_did: None,
            agent_vk: None,
            my_did: None,
            my_vk: None,
            their_did: None,
            their_vk: None,
            credential_offer: None,
            msg_uid: None,
            cred_id: None,
            credential: None,
            payment_info: None,
            payment_txn: None,
            thread: Some(Thread::new())
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Credential {
    source_id: String,
    state: VcxStateType,
    credential_name: Option<String>,
    credential_request: Option<CredentialRequest>,
    credential_offer: Option<CredentialOffer>,
    msg_uid: Option<String>,
    // the following 6 are pulled from the connection object
    agent_did: Option<String>,
    agent_vk: Option<String>,
    my_did: Option<String>,
    my_vk: Option<String>,
    their_did: Option<String>,
    their_vk: Option<String>,
    credential: Option<String>,
    cred_id: Option<String>,
    payment_info: Option<PaymentInfo>,
    payment_txn: Option<PaymentTxn>,
    thread: Option<Thread>
}

impl Credential {
    pub fn build_request(&self, my_did: &str, their_did: &str) -> VcxResult<CredentialRequest> {
        trace!("Credential::build_request >>> my_did: {}, their_did: {}", my_did, their_did);

        if self.state != VcxStateType::VcxStateRequestReceived {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady, format!("credential {} has invalid state {} for sending credential request", self.source_id, self.state as u32)));
        }

        let prover_did = self.my_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidDid))?;
        let credential_offer = self.credential_offer.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredential))?;

        let (cred_def_id, cred_def_json) = anoncreds::get_cred_def_json(&credential_offer.cred_def_id)?;

        /*
                debug!("storing credential offer: {}", secret!(&credential_offer));
                libindy_prover_store_credential_offer(wallet_h, &credential_offer).map_err(|ec| CredentialError::CommonError(ec))?;
        */

        let (req, req_meta) = libindy_prover_create_credential_req(&prover_did,
                                                                   &credential_offer.libindy_offer,
                                                                   &cred_def_json)
            .map_err(|err| err.extend("Cannot create credential request"))?;

        Ok(CredentialRequest {
            libindy_cred_req: req,
            libindy_cred_req_meta: req_meta,
            cred_def_id,
            tid: String::new(),
            to_did: String::from(their_did),
            from_did: String::from(my_did),
            mid: String::new(),
            version: String::from("0.1"),
            msg_ref_id: None,
        })
    }

    fn send_request(&mut self, connection_handle: u32) -> VcxResult<u32> {
        trace!("Credential::send_request >>> connection_handle: {}", connection_handle);

        debug!("sending credential request {} via connection: {}", self.source_id, connection::get_source_id(connection_handle).unwrap_or_default());
        self.my_did = Some(connection::get_pw_did(connection_handle)?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle)?);
        self.agent_did = Some(connection::get_agent_did(connection_handle)?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle)?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle)?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle)?);

        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let local_their_did = self.their_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let local_their_vk = self.their_vk.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let local_agent_did = self.agent_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let local_my_did = self.my_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let local_my_vk = self.my_vk.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;

        // if test mode, just get this.
        let cred_req: CredentialRequest = self.build_request(local_my_did, local_their_did)?;
        let cred_req_json = serde_json::to_string(&cred_req)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot serialize CredentialRequest: {}", err)))?;

        self.credential_request = Some(cred_req);

        let offer_msg_id = self.credential_offer.as_ref().and_then(|offer| offer.msg_ref_id.clone())
            .ok_or(VcxError::from(VcxErrorKind::CreateCredentialRequest))?;

        if self.payment_info.is_some() {
            let (payment_txn, _) = self.submit_payment()?;
            self.payment_txn = Some(payment_txn);
        }

        let response =
            messages::send_message()
                .to(local_my_did)?
                .to_vk(local_my_vk)?
                .msg_type(&RemoteMessageType::CredReq)?
                .agent_did(local_agent_did)?
                .agent_vk(local_agent_vk)?
                .edge_agent_payload(&local_my_vk, &local_their_vk, &cred_req_json, PayloadKinds::CredReq, self.thread.clone())?
                .ref_msg_id(Some(offer_msg_id.to_string()))?
                .send_secure()
                .map_err(|err| err.extend(format!("{} could not send proof", self.source_id)))?;

        self.msg_uid = Some(response.get_msg_uid()?);
        self.state = VcxStateType::VcxStateOfferSent;

        return Ok(error::SUCCESS.code_num);
    }

    fn _check_msg(&mut self) -> VcxResult<()> {
        let agent_did = self.agent_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let agent_vk = self.agent_vk.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let my_did = self.my_did.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let my_vk = self.my_vk.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;
        let msg_uid = self.msg_uid.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidCredentialHandle))?;

        let (_, payload) = get_message::get_ref_msg(msg_uid, my_did, my_vk, agent_did, agent_vk)?;

        let (credential, thread) = Payloads::decrypt(&my_vk, &payload)?;

        if let Some(_) = thread {
            let their_did = self.their_did.as_ref().map(String::as_str).unwrap_or("");
            self.thread.as_mut().map(|thread| thread.increment_receiver(&their_did));
        }

        let credential_msg: CredentialMessage = serde_json::from_str(&credential)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot deserialize CredentialMessage: {}", err)))?;

        let cred_req: &CredentialRequest = self.credential_request.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredential, "Cannot find CredentialRequest"))?;

        let (_, cred_def_json) = anoncreds::get_cred_def_json(&cred_req.cred_def_id)
            .map_err(|err| err.extend("Cannot get credential definition"))?;

        self.credential = Some(credential);
        self.cred_id = Some(libindy_prover_store_credential(None,
                                                            &cred_req.libindy_cred_req_meta,
                                                            &credential_msg.libindy_cred,
                                                            &cred_def_json,
                                                            match credential_msg.rev_reg_def_json.len() {
                                                                0 => None,
                                                                _ => Some(&credential_msg.rev_reg_def_json),
                                                            })?);

        self.state = VcxStateType::VcxStateAccepted;

        Ok(())
    }

    fn update_state(&mut self) {
        trace!("Credential::update_state >>>");
        match self.state {
            VcxStateType::VcxStateOfferSent => {
                //Check for messages
                let _ = self._check_msg();
            }
            VcxStateType::VcxStateAccepted => {
                //Check for revocation
            }
            _ => {
                // NOOP there is nothing the check for a changed state
            }
        }
    }

    fn get_state(&self) -> u32 {
        trace!("Credential::get_state >>>");
        self.state as u32
    }

    fn get_credential(&self) -> VcxResult<String> {
        trace!("Credential::get_credential >>>");

        if self.state != VcxStateType::VcxStateAccepted {
            return Err(VcxError::from(VcxErrorKind::InvalidState));
        }

        let credential = self.credential.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidState))?;

        Ok(self.to_cred_string(&credential))
    }

    fn get_credential_offer(&self) -> VcxResult<String> {
        trace!("Credential::get_credential_offer >>>");

        if self.state != VcxStateType::VcxStateRequestReceived {
            return Err(VcxError::from(VcxErrorKind::InvalidState));
        }

        let credential_offer = self.credential_offer.as_ref().ok_or(VcxError::from(VcxErrorKind::InvalidState))?;
        let credential_offer_json = serde_json::to_value(credential_offer)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot deserialize CredentilOffer: {}", err)))?;

        Ok(self.to_cred_offer_string(credential_offer_json))
    }

    fn get_credential_id(&self) -> String {
        self.cred_id.as_ref().map(String::as_str).unwrap_or("").to_string()
    }

    fn set_payment_info(&self, json: &mut serde_json::Map<String, Value>) {
        if let Some(ref payment_info) = self.payment_info {
            json.insert("price".to_string(), Value::String(payment_info.price.to_string()));
            json.insert("payment_address".to_string(), Value::String(payment_info.payment_addr.to_string()));
        };
    }

    fn to_cred_string(&self, cred: &str) -> String {
        let mut json = serde_json::Map::new();
        json.insert("credential_id".to_string(), Value::String(self.get_credential_id()));
        json.insert("credential".to_string(), Value::String(cred.to_string()));
        self.set_payment_info(&mut json);
        serde_json::Value::from(json).to_string()
    }

    fn to_cred_offer_string(&self, cred_offer: Value) -> String {
        let mut json = serde_json::Map::new();
        json.insert("credential_offer".to_string(), cred_offer);
        self.set_payment_info(&mut json);
        serde_json::Value::from(json).to_string()
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }

    fn get_source_id(&self) -> &String { &self.source_id }

    fn get_payment_txn(&self) -> VcxResult<PaymentTxn> {
        trace!("Credential::get_payment_txn >>>");

        match (&self.payment_txn, &self.payment_info) {
            (Some(ref payment_txn), Some(_)) => Ok(payment_txn.clone()),
            _ => Err(VcxError::from(VcxErrorKind::NoPaymentInformation))
        }
    }

    fn is_payment_required(&self) -> bool {
        self.payment_info.is_some()
    }

    fn submit_payment(&self) -> VcxResult<(PaymentTxn, String)> {
        debug!("{} submitting payment for premium credential", self.source_id);
        match &self.payment_info {
            &Some(ref pi) => {
                let address = &pi.get_address();
                let price = pi.get_price();
                let (payment_txn, receipt) = pay_a_payee(price, address)?;
                Ok((payment_txn, receipt))
            }
            &None => Err(VcxError::from(VcxErrorKind::NoPaymentInformation)),
        }
    }

    fn get_payment_info(&self) -> VcxResult<Option<PaymentInfo>> {
        trace!("Credential::get_payment_info >>>");
        Ok(self.payment_info.clone())
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Credential"))
    }

    fn from_str(data: &str) -> VcxResult<Credential> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Credential>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Credential"))
    }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(err: VcxError) -> VcxError {
    if err.kind() == VcxErrorKind::InvalidHandle {
        VcxError::from(VcxErrorKind::InvalidCredentialHandle)
    } else {
        err
    }
}

pub fn credential_create_with_offer(source_id: &str, offer: &str) -> VcxResult<u32> {
    trace!("credential_create_with_offer >>> source_id: {}, offer: {}", source_id, secret!(&offer));

    let mut new_credential = _credential_create(source_id);

    let (offer, payment_info) = parse_json_offer(offer)?;

    new_credential.credential_offer = Some(offer);
    new_credential.payment_info = payment_info;
    new_credential.state = VcxStateType::VcxStateRequestReceived;

    debug!("inserting credential {} into handle map", source_id);
    HANDLE_MAP.add(new_credential)
}

fn _credential_create(source_id: &str) -> Credential {
    let mut new_credential: Credential = Default::default();

    new_credential.state = VcxStateType::VcxStateInitialized;
    new_credential.set_source_id(source_id);

    new_credential
}

pub fn update_state(handle: u32) -> VcxResult<u32> {
    HANDLE_MAP.get_mut(handle, |obj| {
        debug!("updating state for credential {} with msg_id {:?}", obj.source_id, obj.msg_uid);
        obj.update_state();
        Ok(error::SUCCESS.code_num)
    })
}

pub fn get_credential(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        debug!("getting credential {}", obj.get_source_id());
        obj.get_credential()
    })
}

pub fn get_payment_txn(handle: u32) -> VcxResult<PaymentTxn> {
    HANDLE_MAP.get(handle, |obj| {
        obj.get_payment_txn()
    }).or(Err(VcxError::from(VcxErrorKind::NoPaymentInformation)))
}

pub fn get_credential_offer(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        debug!("getting credential offer {}", obj.source_id);
        obj.get_credential_offer()
    })
}

pub fn get_credential_id(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_credential_id())
    })
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

pub fn send_credential_request(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    HANDLE_MAP.get_mut(handle, |obj| {
        obj.send_request(connection_handle)
    }).map_err(handle_err)
}

pub fn get_credential_offer_msg(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
    trace!("get_credential_offer_msg >>> connection_handle: {}, msg_id: {}", connection_handle, msg_id);

    let my_did = connection::get_pw_did(connection_handle)?;
    let my_vk = connection::get_pw_verkey(connection_handle)?;
    let agent_did = connection::get_agent_did(connection_handle)?;
    let agent_vk = connection::get_agent_verkey(connection_handle)?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let message = get_message::get_connection_messages(&my_did,
                                                       &my_vk,
                                                       &agent_did,
                                                       &agent_vk,
                                                       Some(vec![msg_id.to_string()]))
        .map_err(|err| err.extend("Cannot get messages"))?;

    if message[0].msg_type != RemoteMessageType::CredOffer {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Invalid message type"));
    }

    let payload = message.get(0).and_then(|msg| msg.payload.as_ref())
        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidMessagePack, "Payload not found"))?;

    let payload = _set_cred_offer_ref_message(&payload, &my_vk, &message[0].uid)?;

    serde_json::to_string_pretty(&payload)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidMessages, format!("Cannot serialize credential offer: {}", err)))
}

pub fn get_credential_offer_messages(connection_handle: u32) -> VcxResult<String> {
    trace!("Credential::get_credential_offer_messages >>> connection_handle: {}", connection_handle);

    debug!("checking agent for credential offers from connection {}", connection::get_source_id(connection_handle).unwrap_or_default());
    let my_did = connection::get_pw_did(connection_handle)?;
    let my_vk = connection::get_pw_verkey(connection_handle)?;
    let agent_did = connection::get_agent_did(connection_handle)?;
    let agent_vk = connection::get_agent_verkey(connection_handle)?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let payload = get_message::get_connection_messages(&my_did,
                                                       &my_vk,
                                                       &agent_did,
                                                       &agent_vk,
                                                       None)
        .map_err(|err| err.extend("Cannot get messages"))?;

    let mut messages = Vec::new();

    for msg in payload {
        if msg.msg_type == RemoteMessageType::CredOffer {
            let payload = msg.payload
                .ok_or(VcxError::from(VcxErrorKind::InvalidMessages))?;

            let payload = _set_cred_offer_ref_message(&payload, &my_vk, &msg.uid)?;

            messages.push(payload);
        }
    }

    serde_json::to_string_pretty(&messages)
        .or(Err(VcxError::from(VcxErrorKind::InvalidMessages)))
}

fn _set_cred_offer_ref_message(payload: &MessagePayload, my_vk: &str, msg_id: &str) -> VcxResult<Vec<Value>> {
    let (offer, thread) = Payloads::decrypt(my_vk, payload)?;

    let (mut offer, payment_info) = parse_json_offer(&offer)?;

    offer.msg_ref_id = Some(msg_id.to_owned());
    if let Some(tr) = thread {
        offer.thread_id = tr.thid.clone();
    }

    let mut payload = Vec::new();
    payload.push(json!(offer));
    if let Some(p) = payment_info { payload.push(json!(p)); }

    Ok(payload)
}

pub fn parse_json_offer(offer: &str) -> VcxResult<(CredentialOffer, Option<PaymentInfo>)> {
    let paid_offer: Value = serde_json::from_str(offer)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize offer: {}", err)))?;

    let mut payment: Option<PaymentInfo> = None;
    let mut offer: Option<CredentialOffer> = None;

    if let Some(i) = paid_offer.as_array() {
        for entry in i.iter() {
            if entry.get("libindy_offer").is_some() {
                offer = Some(serde_json::from_value(entry.clone())
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize offer: {}", err)))?);
            }

            if entry.get("payment_addr").is_some() {
                payment = Some(serde_json::from_value(entry.clone())
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize payment address: {}", err)))?);
            }
        }
    }
    Ok((offer.ok_or(VcxError::from(VcxErrorKind::InvalidJson))?, payment))
}

pub fn release(handle: u32) -> VcxResult<()> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn release_all() {
    HANDLE_MAP.drain().ok();
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        Credential::to_string(&obj)
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

pub fn from_string(credential_data: &str) -> VcxResult<u32> {
    let credential: Credential = Credential::from_str(credential_data)?;
    let new_handle = HANDLE_MAP.add(credential)?;

    debug!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}

pub fn is_payment_required(handle: u32) -> VcxResult<bool> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.is_payment_required())
    }).map_err(handle_err)
}

pub fn submit_payment(handle: u32) -> VcxResult<(PaymentTxn, String)> {
    HANDLE_MAP.get_mut(handle, |obj| {
        obj.submit_payment()
    }).map_err(handle_err)
}

pub fn get_payment_information(handle: u32) -> VcxResult<Option<PaymentInfo>> {
    HANDLE_MAP.get(handle, |obj| {
        obj.get_payment_info()
    }).map_err(handle_err)
}


#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use utils::httpclient;
    use api::VcxStateType;
    use serde_json::Value;

    pub const BAD_CREDENTIAL_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    use utils::constants::{DEFAULT_SERIALIZED_CREDENTIAL,
                           DEFAULT_SERIALIZED_CREDENTIAL_PAYMENT_REQUIRED};
    use utils::libindy::payments::build_test_address;

    pub fn create_credential(offer: &str) -> Credential {
        let mut credential = _credential_create("source_id");
        let (offer, payment_info) = ::credential::parse_json_offer(offer).unwrap();
        credential.credential_offer = Some(offer);
        credential.payment_info = payment_info;
        credential.state = VcxStateType::VcxStateRequestReceived;
        credential.my_did = Some(settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap());
        credential
    }

    fn create_credential_with_price(price: u64) -> Credential {
        let mut cred: Credential = Credential::from_str(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        cred.payment_info = Some(PaymentInfo {
            payment_required: "one-time".to_string(),
            payment_addr: build_test_address("OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j"),
            price,
        });
        cred
    }

    #[test]
    fn test_credential_defaults() {
        let credential = Credential::default();
        assert_eq!(credential.build_request("test1", "test2").unwrap_err().kind(), VcxErrorKind::NotReady);
    }

    #[test]
    fn test_credential_create_with_offer() {
        let handle = credential_create_with_offer("test_credential_create_with_offer", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_credential_create_with_bad_offer() {
        match credential_create_with_offer("test_credential_create_with_bad_offer", BAD_CREDENTIAL_OFFER) {
            Ok(_) => panic!("should have failed with bad credential offer"),
            Err(x) => assert_eq!(x.kind(), VcxErrorKind::InvalidJson)
        };
    }

    #[test]
    fn test_credential_serialize_deserialize() {
        let handle = credential_create_with_offer("test_credential_serialize_deserialize", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        let credential_string = to_string(handle).unwrap();
        release(handle).unwrap();
        assert_eq!(release(handle).unwrap_err().kind(), VcxErrorKind::InvalidCredentialHandle);
        let handle = from_string(&credential_string).unwrap();
        let cred1: Credential = Credential::from_str(&credential_string).unwrap();
        assert_eq!(cred1.get_state(), 3);
        let cred2: Credential = Credential::from_str(&to_string(handle).unwrap()).unwrap();
        assert!(!cred1.is_payment_required());
        assert_eq!(cred1, cred2);
        let handle = from_string(DEFAULT_SERIALIZED_CREDENTIAL_PAYMENT_REQUIRED).unwrap();
        let payment_required_credential: Credential = Credential::from_str(&to_string(handle).unwrap()).unwrap();
        assert!(payment_required_credential.is_payment_required())
    }

    #[test]
    fn full_credential_test() {
        init!("true");

        let connection_h = connection::tests::build_test_connection();
        let offers = get_credential_offer_messages(connection_h).unwrap();
        let offers: Value = serde_json::from_str(&offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();

        let c_h = credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(c_h).unwrap());

        send_credential_request(c_h, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, get_state(c_h).unwrap());

        assert_eq!(get_credential_id(c_h).unwrap(), "");
        httpclient::set_next_u8_response(::utils::constants::CREDENTIAL_RESPONSE.to_vec());
        httpclient::set_next_u8_response(::utils::constants::UPDATE_CREDENTIAL_RESPONSE.to_vec());
        update_state(c_h).unwrap();
        assert_eq!(get_state(c_h).unwrap(), VcxStateType::VcxStateAccepted as u32);
        assert_eq!(get_credential_id(c_h).unwrap(), "cred_id"); // this is set in test mode
        assert!(get_credential(c_h).unwrap().len() > 100);
        let serialized = to_string(c_h).unwrap();
    }

    #[test]
    fn test_get_credential_offer() {
        init!("true");
        let connection_h = connection::tests::build_test_connection();
        let offer = get_credential_offer_messages(connection_h).unwrap();
        let o: serde_json::Value = serde_json::from_str(&offer).unwrap();
        let credential_offer: CredentialOffer = serde_json::from_str(&o[0][0].to_string()).unwrap();
        assert!(offer.len() > 50);
    }

    #[test]
    fn test_pay_for_credential_with_sufficient_funds() {
        init!("true");
        let cred = create_credential_with_price(1);
        assert!(cred.is_payment_required());
        let payment = serde_json::to_string(&cred.submit_payment().unwrap().0).unwrap();
        assert!(payment.len() > 50);
    }

    #[test]
    fn test_pay_for_non_premium_credential() {
        init!("true");
        let cred: Credential = Credential::from_str(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(cred.payment_info.is_none());
        assert_eq!(cred.submit_payment().unwrap_err().kind(), VcxErrorKind::NoPaymentInformation);
    }

    #[test]
    fn test_pay_for_credential_with_insufficient_funds() {
        init!("true");
        let cred = create_credential_with_price(10000000000);
        assert!(cred.submit_payment().is_err());
    }

    #[test]
    fn test_pay_for_credential_with_handle() {
        init!("true");
        let handle = from_string(DEFAULT_SERIALIZED_CREDENTIAL_PAYMENT_REQUIRED).unwrap();
        submit_payment(handle).unwrap();
        get_payment_information(handle).unwrap();
        let handle2 = from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(!is_payment_required(handle2).unwrap());
        let invalid_handle = 12345;
        assert_eq!(is_payment_required(invalid_handle).unwrap_err().kind(), VcxErrorKind::InvalidCredentialHandle);
    }

    #[test]
    fn test_get_credential() {
        init!("true");
        let handle = from_string(::utils::constants::DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        let offer_string = get_credential_offer(handle).unwrap();
        let handle = from_string(::utils::constants::FULL_CREDENTIAL_SERIALIZED).unwrap();
        let cred_string = get_credential(handle).unwrap();
    }

    #[test]
    fn test_submit_payment_through_credential_request() {
        init!("true");
        use utils::libindy::payments::get_wallet_token_info;
        let balance = get_wallet_token_info().unwrap().get_balance();
        assert!(balance > 0);
        let mut cred = create_credential_with_price(5);
        assert!(cred.send_request(1234).is_err());
        let new_balance = get_wallet_token_info().unwrap().get_balance();
        assert_eq!(new_balance, balance);
    }

    #[test]
    fn test_get_cred_offer_returns_json_string_with_cred_offer_json_nested() {
        init!("true");
        let handle = from_string(::utils::constants::DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        let offer_string = get_credential_offer(handle).unwrap();
        let offer_value: serde_json::Value = serde_json::from_str(&offer_string).unwrap();

        let offer_struct: CredentialOffer = serde_json::from_value(offer_value["credential_offer"].clone()).unwrap();

    }
}
