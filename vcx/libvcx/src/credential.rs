extern crate rand;
extern crate serde_json;
extern crate libc;
extern crate serde;
extern crate rmp_serde;

use object_cache::ObjectCache;
use api::VcxStateType;
use utils::error;
use issuer_credential::{ CredentialOffer, CredentialMessage, PaymentInfo };

use credential_request::{ CredentialRequest };

use messages;
use messages::to_u8;
use messages::GeneralMessage;
use messages::send_message::parse_msg_uid;
use messages::extract_json_payload;

use utils::libindy::anoncreds::{libindy_prover_create_credential_req, libindy_prover_store_credential};
use utils::libindy::crypto;
use utils::libindy::anoncreds;
use utils::libindy::payments::{pay_a_payee, PaymentTxn};

use connection;

use settings;
use utils::constants::DEFAULT_SERIALIZE_VERSION;

use error::{ToErrorCode, credential::CredentialError};
use serde_json::Value;


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
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
}

impl Credential {

    pub fn build_request(&self, my_did: &str, their_did: &str) -> Result<CredentialRequest, CredentialError> {
        trace!("Credential::build_request >>> my_did: {}, their_did: {}", my_did, their_did);

        if self.state != VcxStateType::VcxStateRequestReceived { return Err(CredentialError::NotReady())}

        let prover_did = self.my_did.as_ref().ok_or(CredentialError::CommonError(error::INVALID_DID.code_num))?;
        let credential_offer = self.credential_offer.as_ref().ok_or(CredentialError::InvalidCredentialJson())?;

        let (cred_def_id, cred_def_json) = anoncreds::get_cred_def_json(&credential_offer.cred_def_id)
            .map_err(|err| CredentialError::CommonError(err))?;

/*
        debug!("storing credential offer: {}", credential_offer);
        libindy_prover_store_credential_offer(wallet_h, &credential_offer).map_err(|ec| CredentialError::CommonError(ec))?;
*/

        let (req, req_meta) = libindy_prover_create_credential_req(&prover_did,
                                                                   &credential_offer.libindy_offer,
                                                                   &cred_def_json)
            .map_err(|ec| CredentialError::CommonError(ec))?;

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

    fn send_request(&mut self, connection_handle: u32) -> Result<u32, CredentialError> {
        trace!("Credential::send_request >>> connection_handle: {}", connection_handle);

        debug!("sending credential request {} via connection: {}", self.source_id, connection::get_source_id(connection_handle).unwrap_or_default());
        self.my_did = Some(connection::get_pw_did(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
        self.agent_did = Some(connection::get_agent_did(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);

        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let local_their_did = self.their_did.as_ref().ok_or(CredentialError::InvalidHandle())?;
        let local_their_vk = self.their_vk.as_ref().ok_or(CredentialError::InvalidHandle())?;
        let local_agent_did = self.agent_did.as_ref().ok_or(CredentialError::InvalidHandle())?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(CredentialError::InvalidHandle())?;
        let local_my_did = self.my_did.as_ref().ok_or(CredentialError::InvalidHandle())?;
        let local_my_vk = self.my_vk.as_ref().ok_or(CredentialError::InvalidHandle())?;

        // if test mode, just get this.
        let req: CredentialRequest = self.build_request(local_my_did, local_their_did)?;
        self.credential_request = Some(req.clone());
        let req = serde_json::to_string(&req).or(Err(CredentialError::InvalidCredentialJson()))?;
        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &req, "CRED_REQ").map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
        let offer_msg_id = self.credential_offer.as_ref().ok_or(CredentialError::CommonError(error::CREATE_CREDENTIAL_REQUEST_ERROR.code_num))?
            .msg_ref_id.as_ref().ok_or(CredentialError::CommonError(error::CREATE_CREDENTIAL_REQUEST_ERROR.code_num))?;

        if self.payment_info.is_some() {
            let (payment_txn, _) = self.submit_payment()?;
            self.payment_txn = Some(payment_txn);
        }
        
        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("credReq")
            .agent_did(local_agent_did)
            .agent_vk(local_agent_vk)
            .edge_agent_payload(&data)
            .ref_msg_id(offer_msg_id)
            .send_secure() {
            Ok(response) => {
                self.msg_uid = Some(parse_msg_uid(&response[0]).map_err(|ec| CredentialError::CommonError(ec))?);
                self.state = VcxStateType::VcxStateOfferSent;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("{} could not send proof: {}", self.source_id, x);
                return Err(CredentialError::CommonError(x));
            }
        }
    }

    fn _check_msg(&mut self) -> Result<(), u32> {
        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let agent_did = self.agent_did.as_ref().ok_or(e_code)?;
        let agent_vk = self.agent_vk.as_ref().ok_or(e_code)?;
        let my_did = self.my_did.as_ref().ok_or(e_code)?;
        let my_vk = self.my_vk.as_ref().ok_or(e_code)?;
        let msg_uid = self.msg_uid.as_ref().ok_or(e_code)?;

        let (_, payload) = messages::get_message::get_ref_msg(msg_uid, my_did, my_vk, agent_did, agent_vk)?;

        let credential = extract_json_payload(&payload)?;

        let credential_msg: CredentialMessage = serde_json::from_str(&credential)
            .or(Err(error::INVALID_CREDENTIAL_JSON.code_num))?;

        let cred_req: &CredentialRequest = self.credential_request.as_ref()
            .ok_or(CredentialError::InvalidCredentialJson().to_error_code())?;

        let (_, cred_def_json) = anoncreds::get_cred_def_json(&cred_req.cred_def_id)?;

        self.credential = Some(credential);
        self.cred_id = Some(libindy_prover_store_credential(None,
                                                            &cred_req.libindy_cred_req_meta,
                                                            &credential_msg.libindy_cred,
                                                            &cred_def_json,
                                                            match credential_msg.rev_reg_def_json.len() {
                                                                0 => None,
                                                                _ => Some(credential_msg.rev_reg_def_json),
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
            },
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
        let state = self.state as u32;
        state
    }

    fn get_credential(&self) -> Result<String, CredentialError> {
        trace!("Credential::get_credential >>>");
        if self.state == VcxStateType::VcxStateAccepted {
            match self.credential {
                Some(ref x) => Ok(self.to_cred_string(x)),
                None => Err(CredentialError::InvalidState()),
            }
        }
        else {
            Err(CredentialError::InvalidState())
        }
    }

    fn get_credential_offer(&self) -> Result<String, CredentialError> {
        trace!("Credential::get_credential_offer >>>");
        if self.state == VcxStateType::VcxStateRequestReceived {
            match self.credential_offer {
                Some(ref x) => match serde_json::to_string(x) {
                    Ok(x) => Ok(self.to_cred_offer_string(&x)),
                    Err(_) => Err(CredentialError::InvalidCredentialJson()),
                }
                None => Err(CredentialError::InvalidState()),
            }
        }
        else {
            Err(CredentialError::InvalidState())
        }
    }

    fn get_credential_id(&self) -> String { match self.cred_id.clone() {
        Some(cid) => cid,
        None => "".to_string(),
    }}

    fn to_cred_string(&self, cred: &str) -> String {
        let payment_string = match self.payment_info {
            Some(ref x) => format!(r#","price":"{}","payment_address":"{}""#,x.price,x.payment_addr),
            None => "".to_string(),
        };
        format!(r#"{{"credential_id":"{}","credential":{}{}}}"#,self.get_credential_id(), cred, payment_string)
    }

    fn to_cred_offer_string(&self, cred_offer: &str) -> String {
        let payment_string = match self.payment_info {
            Some(ref x) => format!(r#","price":"{}","payment_address":"{}""#,x.price,x.payment_addr),
            None => "".to_string(),
        };
        format!(r#"{{"credential_offer":{}{}}}"#, cred_offer, payment_string)
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }

    fn get_source_id(&self) -> &String {&self.source_id}

    fn get_payment_txn(&self) -> Result<PaymentTxn, u32> {
        trace!("Credential::get_payment_txn >>>");
        match self.payment_txn {
            Some(ref payment_txn) if self.payment_info.is_some() => Ok(payment_txn.clone()),
            _ => Err(error::NO_PAYMENT_INFORMATION.code_num)
        }
    }

    fn set_credential_offer(&mut self, offer: CredentialOffer){
        self.credential_offer = Some(offer);
    }

    fn is_payment_required(&self) -> bool {
        self.payment_info.is_some()
    }

    fn submit_payment(&self) -> Result<(PaymentTxn, String), CredentialError> {
        debug!("{} submitting payment for premium credential", self.source_id);
        match &self.payment_info {
            &Some(ref pi) => {
                let address = &pi.get_address()?;
                let price = pi.get_price()?;
                let (payment_txn, receipt) = pay_a_payee(price, address)?;
                Ok((payment_txn, receipt))
            },
            &None => Err(CredentialError::NoPaymentInformation()),
        }
    }

    fn get_payment_info(&self) -> Result<Option<PaymentInfo>, CredentialError> {
        trace!("Credential::get_payment_info >>>");
        Ok(self.payment_info.clone())
    }

    fn to_string(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }

    fn from_str(s: &str) -> Result<Credential, CredentialError> {
        let s:Value = serde_json::from_str(&s)
            .or(Err(CredentialError::InvalidCredentialJson()))?;
        let obj: Credential = serde_json::from_value(s["data"].clone())
            .or(Err(CredentialError::InvalidCredentialJson()))?;
        Ok(obj)
    }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(code_num: u32) -> CredentialError {
    if code_num == error::INVALID_OBJ_HANDLE.code_num {
        CredentialError::InvalidHandle()
    }
    else {
        CredentialError::CommonError(code_num)
    }
}

pub fn credential_create_with_offer(source_id: &str, offer: &str) -> Result<u32, CredentialError> {
    trace!("credential_create_with_offer >>> source_id: {}, offer: {}", source_id, offer);

    let mut new_credential = _credential_create(source_id);

    let (offer, payment_info) = parse_json_offer(offer)?;
    new_credential.set_credential_offer(offer);
    new_credential.payment_info = payment_info;

    new_credential.state = VcxStateType::VcxStateRequestReceived;

    debug!("inserting credential {} into handle map", source_id);
    Ok(HANDLE_MAP.add(new_credential).map_err(|ec|CredentialError::CommonError(ec))?)
}

fn _credential_create(source_id: &str) -> Credential {

    let mut new_credential: Credential = Default::default();

    new_credential.state = VcxStateType::VcxStateInitialized;
    new_credential.set_source_id(source_id);

    new_credential
}

pub fn update_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get_mut(handle, |obj|{
        debug!("updating state for credential {} with msg_id {:?}", obj.source_id, obj.msg_uid);
        obj.update_state();
        Ok(error::SUCCESS.code_num)
    })

}

pub fn get_credential(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        debug!("getting credential {}", obj.get_source_id());
        obj.get_credential().map_err(|e| e.to_error_code())
    }).map_err(|ec| CredentialError::CommonError(ec))
}

pub fn get_payment_txn(handle: u32) -> Result<PaymentTxn, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        obj.get_payment_txn()
    }).or(Err(CredentialError::NoPaymentInformation()))
}

pub fn get_credential_offer(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        debug!("getting credential offer {}", obj.source_id);
        obj.get_credential_offer().map_err(|e| e.to_error_code())
    }).map_err(|ec| CredentialError::CommonError(ec))
}

pub fn get_credential_id(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_credential_id())
    }).map_err(|ec| CredentialError::CommonError(ec))
}

pub fn get_state(handle: u32) -> Result<u32, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

pub fn send_credential_request(handle: u32, connection_handle: u32) -> Result<u32, CredentialError> {
    HANDLE_MAP.get_mut(handle, |obj| {
        obj.send_request(connection_handle).map_err(|e| e.to_error_code())
    }).map_err(handle_err)
}

pub fn get_credential_offer_msg(connection_handle: u32, msg_id: &str) -> Result<String, CredentialError> {
    trace!("get_credential_offer_msg >>> connection_handle: {}, msg_id: {}", connection_handle, msg_id);

    let my_did = connection::get_pw_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let message = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 Some(vec![msg_id.to_string()])).map_err(|ec| CredentialError::CommonError(ec))?;

    if message[0].msg_type.eq("credOffer") {
        let (_, msg_data) = match message[0].payload {
            Some(ref data) => {
                let data = to_u8(data);
                crypto::parse_msg(&my_vk, data.as_slice()).map_err(|ec| CredentialError::CommonError(ec))?
            },
            None => return Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
        };

        let offer = extract_json_payload(&msg_data).map_err(|ec| CredentialError::CommonError(ec))?;
        let (mut offer, payment_info) = parse_json_offer(&offer)?;

        offer.msg_ref_id = Some(message[0].uid.to_owned());
        let mut payload = Vec::new();
        payload.push(json!(offer));
        if let Some(p) = payment_info { payload.push(json!(p)); }

        Ok(serde_json::to_string_pretty(&payload).or(Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num)))?)
    } else {
        Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
    }
}

pub fn get_credential_offer_messages(connection_handle: u32) -> Result<String, CredentialError> {
    trace!("Credential::get_credential_offer_messages >>> connection_handle: {}", connection_handle);

    debug!("checking agent for credential offers from connection {}", connection::get_source_id(connection_handle).unwrap_or_default());
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let payload = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 None).map_err(|ec| CredentialError::CommonError(ec))?;

    let mut messages = Vec::new();

    for msg in payload {
        if msg.msg_type.eq("credOffer") {
            let (_, msg_data) = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg( &my_vk, data.as_slice()).map_err(|ec| CredentialError::CommonError(ec))?
                },
                None => return Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
            };

            let offer = extract_json_payload(&msg_data).map_err(|ec| CredentialError::CommonError(ec))?;
            let (mut offer, payment_info) = parse_json_offer(&offer)?;

            offer.msg_ref_id = Some(msg.uid.to_owned());
            let mut payload = Vec::new();
            payload.push(json!(offer));
            if let Some(p) = payment_info { payload.push(json!(p)); }

            messages.push(payload);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).or(Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num)))?)
}

pub fn parse_json_offer(offer: &str) -> Result<(CredentialOffer, Option<PaymentInfo>), CredentialError> {
    let paid_offer: Value = serde_json::from_str(offer).or(Err(CredentialError::InvalidCredentialJson()))?;

    let mut payment: Option<PaymentInfo> = None;
    let mut offer: Option<CredentialOffer> = None;

    if let Some(i) = paid_offer.as_array() {
        for entry in i.iter() {
            if entry.get("libindy_offer").is_some() {
                offer = Some(serde_json::from_value(entry.clone()).or(Err(CredentialError::InvalidCredentialJson()))?);
            }

            if entry.get("payment_addr").is_some() {
                payment = Some(serde_json::from_value(entry.clone()).or(Err(CredentialError::InvalidCredentialJson()))?);
            }
        }
    }
    Ok((offer.ok_or(CredentialError::InvalidCredentialJson())?, payment))
}

pub fn release(handle: u32) -> Result<(), CredentialError> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn release_all() {
    match HANDLE_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj|{
        Ok(Credential::to_string(&obj))
    })
}

pub fn get_source_id(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

pub fn from_string(credential_data: &str) -> Result<u32, u32> {
    let credential: Credential = match Credential::from_str(credential_data) {
        Ok(x) => x,
        Err(y) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = HANDLE_MAP.add(credential)?;

    debug!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}

pub fn is_payment_required(handle: u32) -> Result<bool, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.is_payment_required())
    }).map_err(handle_err)
}

pub fn submit_payment(handle: u32) -> Result<(PaymentTxn, String), CredentialError> {
    HANDLE_MAP.get_mut(handle, |obj| {
        obj.submit_payment().map_err(|e| e.to_error_code())
    }).map_err(handle_err)

}

pub fn get_payment_information(handle: u32) -> Result<Option<PaymentInfo>, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        obj.get_payment_info().map_err(|e| e.to_error_code())
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

    fn create_credential_with_price(price:u64) -> Credential{
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
        assert_eq!(credential.build_request("test1","test2").err(), Some(CredentialError::NotReady()));
    }

    #[test]
    fn test_credential_create_with_offer() {
        let handle = credential_create_with_offer("test_credential_create_with_offer", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_credential_create_with_bad_offer() {
        match credential_create_with_offer("test_credential_create_with_bad_offer",BAD_CREDENTIAL_OFFER) {
            Ok(_) => panic!("should have failed with bad credential offer"),
            Err(x) => assert_eq!(x.to_error_code(),error::INVALID_JSON.code_num),
        };
    }

    #[test]
    fn test_credential_serialize_deserialize() {
        let handle = credential_create_with_offer("test_credential_serialize_deserialize", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        let credential_string = to_string(handle).unwrap();
        release(handle).unwrap();
        assert_eq!(release(handle).err(), Some(CredentialError::InvalidHandle()));
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
    fn full_credential_test(){
        init!("true");

        let connection_h = connection::tests::build_test_connection();
        let offers = get_credential_offer_messages(connection_h).unwrap();
        let offers:Value = serde_json::from_str(&offers).unwrap();
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
        assert_eq!(cred.submit_payment().err(), Some(CredentialError::NoPaymentInformation()));
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
        assert_eq!(is_payment_required(invalid_handle).err(), Some(CredentialError::InvalidHandle()));
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
}
