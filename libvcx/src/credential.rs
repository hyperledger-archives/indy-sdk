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
use utils::libindy::wallet;
use utils::libindy::crypto;

use credential_def::retrieve_credential_def;
use connection;

use settings;
use utils::httpclient;
use utils::constants::{ SEND_MESSAGE_RESPONSE };

use error::ToErrorCode;
use error::credential::CredentialError;

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
            link_secret_alias: Some(String::from("main")), //TODO this should not be hardcoded
            msg_uid: None,
            cred_id: None,
            credential: None,
            payment_info: None,
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
    link_secret_alias: Option<String>,
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
}

impl Credential {

    fn _build_request(&self, my_did: &str, their_did: &str) -> Result<CredentialRequest, CredentialError> {
        if self.state != VcxStateType::VcxStateRequestReceived { return Err(CredentialError::NotReady())}

        let prover_did = self.my_did.as_ref().ok_or(CredentialError::CommonError(error::INVALID_DID.code_num))?;
        let credential_offer = self.credential_offer.as_ref().ok_or(CredentialError::InvalidCredentialJson())?;

        let (cred_def_id, cred_def_json) = retrieve_credential_def(&credential_offer.cred_def_id)
            .map_err(|err| CredentialError::CommonError(err.to_error_code()))?;

/*
        debug!("storing credential offer: {}", credential_offer);
        libindy_prover_store_credential_offer(wallet_h, &credential_offer).map_err(|ec| CredentialError::CommonError(ec))?;
*/

        let (req, req_meta) = libindy_prover_create_credential_req(&prover_did,
                                                                   &credential_offer.libindy_offer,
                                                                   &cred_def_json,
                                                                   self.link_secret_alias.clone())
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
        })
    }

    fn send_request(&mut self, connection_handle: u32) -> Result<u32, CredentialError> {
        debug!("sending credential request via connection: {}", connection_handle);
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
        let req: CredentialRequest = self._build_request(local_my_did, local_their_did)?;
        self.credential_request = Some(req.clone());
        let req = serde_json::to_string(&req).or(Err(CredentialError::InvalidCredentialJson()))?;
        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &req, "CLAIM_REQ").map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
        let offer_msg_id = self.credential_offer.as_ref().unwrap().msg_ref_id.as_ref().ok_or(CredentialError::CommonError(error::CREATE_CREDENTIAL_REQUEST_ERROR.code_num))?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("claimReq")
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
                warn!("could not send proof: {}", x);
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

        let payload = messages::get_message::get_all_message(my_did,
                                                         my_vk,
                                                         agent_did,
                                                         agent_vk)?;

        for msg in payload {
            if msg.msg_type.eq("claim") {
                match msg.payload {
                    Some(ref data) => {
                        let data = to_u8(data);
                        let data = crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice())?;

                        let credential = extract_json_payload(&data)?;

                        let credential_msg: CredentialMessage = serde_json::from_str(&credential)
                            .or(Err(error::INVALID_CREDENTIAL_JSON.code_num)).unwrap();

                        let cred_req: &CredentialRequest = self.credential_request.as_ref()
                            .ok_or(CredentialError::InvalidCredentialJson().to_error_code())?;

                        let (_, cred_def_json) = ::credential_def::retrieve_credential_def(&cred_req.cred_def_id)
                            .map_err(|err| CredentialError::CommonError(err.to_error_code()).to_error_code())?;

                        self.credential = Some(credential);
                        self.cred_id = Some(libindy_prover_store_credential(None,
                                                                      &cred_req.libindy_cred_req_meta,
                                                                      &credential_msg.libindy_cred,
                                                                      &cred_def_json,
                                                                      None)?);
                        self.state = VcxStateType::VcxStateAccepted;
                    },
                    None => return Err(error::INVALID_HTTP_RESPONSE.code_num)
                };
            }
        }
        Ok(())
    }

    fn update_state(&mut self) {
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
        let state = self.state as u32;
        state
    }


    // TODO have this be VCXState, not u32
    fn get_credential(&self) -> Result<String, CredentialError> {
        match self.state {
            VcxStateType::VcxStateAccepted => match self.credential.clone() {
                Some(ref c) => match self.cred_id.clone() {
                    Some(id) => Ok(format!(r#"{{"{}":{}}}"#, id, c)),
                    None => Err(CredentialError::InvalidState()),
                }
                None => Err(CredentialError::InvalidState()),
            }
            _ => Err(CredentialError::InvalidState()),
        }
    }

    fn get_credential_id(&self) -> String { match self.cred_id.clone() {
        Some(cid) => cid,
        None => "".to_string(),
    }}


    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }

    fn get_source_id(&self) -> &String {&self.source_id}

    fn set_credential_offer(&mut self, offer: CredentialOffer){
        self.credential_offer = Some(offer);
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
    let mut new_credential = _credential_create(source_id);

    let (offer, payment_info) = parse_json_offer(offer)?;
    new_credential.set_credential_offer(offer);
    new_credential.payment_info = payment_info;

    new_credential.state = VcxStateType::VcxStateRequestReceived;

    debug!("inserting credential into handle map");
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
        obj.update_state();
        Ok(error::SUCCESS.code_num)
    })

}

pub fn get_credential(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        obj.get_credential().map_err(|e| e.to_error_code())
    }).map_err(|ec| CredentialError::CommonError(ec))
}

fn get_credential_id(handle: u32) -> Result<String, CredentialError> {
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

pub fn get_credential_offer(connection_handle: u32, msg_id: &str) -> Result<String, CredentialError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let message = messages::get_message::get_matching_message(msg_id,
                                                              &my_did,
                                                              &my_vk,
                                                              &agent_did,
                                                              &agent_vk).map_err(|ec| CredentialError::CommonError(ec))?;

    if message.msg_type.eq("claimOffer") {
        let msg_data = match message.payload {
            Some(ref data) => {
                let data = to_u8(data);
                crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice()).map_err(|ec| CredentialError::CommonError(ec))?
            },
            None => return Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
        };

        let offer = extract_json_payload(&msg_data).map_err(|ec| CredentialError::CommonError(ec))?;
        let (mut offer, payment_info) = parse_json_offer(&offer)?;

        offer.msg_ref_id = Some(message.uid.to_owned());
        let mut payload = Vec::new();
        payload.push(json!(offer));
        if payment_info.is_some() { payload.push(json!(payment_info.unwrap())); }

        Ok(serde_json::to_string_pretty(&payload).unwrap())
    } else {
        Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
    }
}

pub fn get_credential_offer_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, CredentialError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| CredentialError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec()); }

    let payload = messages::get_message::get_all_message(&my_did,
                                                     &my_vk,
                                                     &agent_did,
                                                     &agent_vk).map_err(|ec| CredentialError::CommonError(ec))?;

    let mut messages = Vec::new();

    for msg in payload {
        if msg.msg_type.eq("claimOffer") {
            let msg_data = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice()).map_err(|ec| CredentialError::CommonError(ec))?
                },
                None => return Err(CredentialError::CommonError(error::INVALID_MESSAGES.code_num))
            };

            let offer = extract_json_payload(&msg_data).map_err(|ec| CredentialError::CommonError(ec))?;
            let (mut offer, payment_info) = parse_json_offer(&offer)?;

            offer.msg_ref_id = Some(msg.uid.to_owned());
            let mut payload = Vec::new();
            payload.push(json!(offer));
            if payment_info.is_some() { payload.push(json!(payment_info.unwrap())); }

            messages.push(payload);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).unwrap())
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
    if offer.is_some() {
        Ok((offer.unwrap(), payment))
    }
    else {
        Err(CredentialError::InvalidCredentialJson())
    }
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
        serde_json::to_string(&obj).map_err(|e|{
            warn!("Unable to serialize: {:?}", e);
            error::SERIALIZATION_ERROR.code_num
        })
    })
}

pub fn get_source_id(handle: u32) -> Result<String, CredentialError> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

pub fn from_string(credential_data: &str) -> Result<u32, u32> {
    let credential: Credential = match serde_json::from_str(credential_data) {
        Ok(x) => x,
        Err(y) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = HANDLE_MAP.add(credential)?;

    debug!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use utils::httpclient;
    use api::VcxStateType;
    use serde_json::Value;

    pub const BAD_CREDENTIAL_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    #[test]
    fn test_credential_defaults() {
        let credential = Credential::default();
        assert_eq!(credential._build_request("test1","test2").err(), Some(CredentialError::NotReady()));
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
        let cred1: Credential = serde_json::from_str(&credential_string).unwrap();
        assert_eq!(cred1.get_state(), 3);
        let cred2: Credential = serde_json::from_str(&to_string(handle).unwrap()).unwrap();
        assert_eq!(cred1, cred2);
    }

    #[test]
    fn full_credential_test(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        wallet::init_wallet("full_credential_test").unwrap();

        let connection_h = connection::build_connection("test_send_credential_offer").unwrap();
        let offers = get_credential_offer_messages(connection_h, None).unwrap();
        let offers:Value = serde_json::from_str(&offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();

        let c_h = credential_create_with_offer("TEST_CREDENTIAL", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(c_h).unwrap());

        send_credential_request(c_h, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, get_state(c_h).unwrap());

        assert_eq!(get_credential_id(c_h).unwrap(), "");
        httpclient::set_next_u8_response(::utils::constants::CREDENTIAL_RESPONSE.to_vec());
        update_state(c_h).unwrap();
        assert_eq!(get_state(c_h).unwrap(), VcxStateType::VcxStateAccepted as u32);
        assert_eq!(get_credential_id(c_h).unwrap(), "cred_id"); // this is set in test mode
        assert!(get_credential(c_h).unwrap().len() > 100);
        wallet::delete_wallet("full_credential_test").unwrap();
    }

    #[test]
    fn test_get_credential_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let test_name = "test_get_credential_offer";
        wallet::init_wallet(test_name).unwrap();
        let connection_h = connection::build_connection("test_get_credential_offer").unwrap();
        let offer = get_credential_offer_messages(connection_h, None).unwrap();
        assert!(offer.len() > 50);
        wallet::delete_wallet(test_name).unwrap();
    }
}
