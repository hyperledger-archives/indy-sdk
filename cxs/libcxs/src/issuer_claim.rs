extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use messages;
use settings;
use messages::GeneralMessage;
use connection;
use claim_request::ClaimRequest;
use utils::issuer_claim::CLAIM_REQ_STRING;
use self::libc::c_char;
use utils::callback::CallbackUtils;
use std::sync::mpsc::channel;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use utils::wallet;
use utils::openssl::encode;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

static DEFAULT_CLAIM_NAME: &str = "Claim";
static DEFAULT_CLAIM_ID: &str = "defaultClaimId";
static CLAIM_OFFER_ID_KEY: &str = "claim_offer_id";
static MESSAGE_TYPE_KEY: &str = "msg_type";
static MESSAGE_TYPE_CLAIM: &str = "claim";
static MESSAGES_KEY: &str = "msgs";
static CLAIM_DATA: &str =
    r#"{"address2":["101 Wilson Lane"],
        "zip":["87121"],
        "state":["UT"],
        "city":["SLC"],
        "address1":["101 Tela Lane"]
        }"#;
extern {
    fn indy_issuer_create_and_store_claim_def(command_handle: i32,
                                              wallet_handle: i32,
                                              issuer_did: *const c_char,
                                              schema_json: *const c_char,
                                              signature_type: *const c_char,
                                              create_non_revoc: bool,
                                              cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                   claim_def_json: *const c_char)>) -> i32;
    fn indy_issuer_create_claim(command_handle: i32,
                                wallet_handle: i32,
                                claim_req_json: *const c_char,
                                claim_json: *const c_char,
                                user_revoc_index: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                     revoc_reg_update_json: *const c_char, //TODO must be OPTIONAL
                                                     xclaim_json: *const c_char
                                )>)-> i32;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IssuerClaim {
    source_id: String,
    handle: u32,
    claim_attributes: String,
    msg_uid: String,
    schema_seq_no: u32,
    issuer_did: String,
    issued_did: String,
    state: CxsStateType,
    claim_request: Option<ClaimRequest>,
    claim_name: String,
    claim_id: String,
    ref_msg_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimOffer {
    msg_type: String,
    version: String,
    to_did: String,
    from_did: String,
    claim: serde_json::Map<String, serde_json::Value>,
    schema_seq_no: u32,
    issuer_did: String,
    claim_name: String,
    claim_id: String,

}

impl IssuerClaim {
    fn validate_claim_offer(&self) -> Result<u32, String> {
        //TODO: validate claim_attributes against claim_def
        Ok(error::SUCCESS.code_num)
    }

    fn send_claim_offer(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateInitialized {
            warn!("claim {} has invalid state {} for sending claimOffer", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_claim_offer", connection_handle);
            return Err(error::INVALID_CONNECTION_HANDLE.code_num);
        }

        let to_did = connection::get_pw_did(connection_handle)?;
        let claim_offer = self.generate_claim_offer(&to_did)?;
        let payload = match serde_json::to_string(&claim_offer) {
            Ok(p) => p,
            Err(_) => return Err(error::INVALID_JSON.code_num)
        };

        /* let data = connection::encrypt_payload(connection_handle, data)?; */

        match messages::send_message().to(&to_did).msg_type("claimOffer").edge_agent_payload(&payload).send() {
            Err(x) => {
                warn!("could not send claimOffer: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = get_offer_details(&response)?;
                self.issued_did = to_did;
                self.state = CxsStateType::CxsStateOfferSent;
                return Ok(error::SUCCESS.code_num);
            }
        }

    }

    fn send_claim(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateRequestReceived {
            warn!("claim {} has invalid state {} for sending claim", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_claim_offer", connection_handle);
            return Err(error::INVALID_CONNECTION_HANDLE.code_num);
        }

        let to = connection::get_pw_did(connection_handle)?;
        let attrs_with_encodings = self.create_attributes_encodings()?;
        let mut data;
        if settings::test_indy_mode_enabled() {
            data = String::from("dummytestmodedata");
        } else {
            data = match self.claim_request.clone() {
                Some(d) => create_claim_payload_using_wallet(&self.claim_id, &d, &attrs_with_encodings, wallet::get_wallet_handle())?,
                None => { warn!("Unable to create claim payload using the wallet");
                    return Err(error::INVALID_CLAIM_REQUEST.code_num)},
            };
            // append values we need for example 'from_did' and 'claim_id'
            data = match append_value(&data, CLAIM_OFFER_ID_KEY, &self.msg_uid) {
                Ok(s) => s,
                Err(_) => return Err(error::INVALID_JSON.code_num),
            };

            data = append_value(&data, "from_did", &to)?;

            data = append_value(&data, "version", "0.1")?;

            data = append_value(&data, "msg_type", "CLAIM")?;

        }

        /* let data = connection::encrypt_payload(connection_handle, data)?; */

        match messages::send_message().to(&to).ref_msg_id(&self.ref_msg_id).msg_type("claim").edge_agent_payload(&data).send() {
            Err(x) => {
                warn!("could not send claim: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = get_offer_details(&response)?;
                self.issued_did = to;
                self.state = CxsStateType::CxsStateAccepted;
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn create_attributes_encodings(&self) -> Result<String, u32> {
        let mut attributes: serde_json::Value = match serde_json::from_str(&self.claim_attributes) {
            Ok(x) => x,
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        let map = match attributes.as_object_mut() {
            Some(x) => x,
            None => {
                warn!("Invalid Json for Attribute data");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        for (attr, vec) in map.iter_mut(){
            let list = match vec.as_array_mut() {
                Some(x) => x,
                None => {
                    warn!("Invalid Json for Attribute data");
                    return Err(error::INVALID_JSON.code_num)
                }
            };
//          FIXME This is hardcode but should have logic for finding strings and integers and
//          doing a real encoding (sha256)
//            let encoded = serde_json::Value::from("1139481716457488690172217916278103335");
            let i = list[0].clone();
            let value = match i.as_str(){
                Some(v) => v,
                None => {
                    warn!("Cannot encode attribute: {}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
                    return Err(error::INVALID_ATTRIBUTES_STRUCTURE.code_num)
                },
            };
            let encoded = encode(value)?;
            let encoded_as_value: serde_json::Value = serde_json::Value::from(encoded);
            list.push(encoded_as_value);
        }

        match serde_json::to_string_pretty(&map) {
            Ok(x) => Ok(x),
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                Err(error::INVALID_JSON.code_num)
            }
        }
    }

    fn get_claim_req(&mut self, msg_uid: &str) {
        info!("Checking for outstanding claimReq for {} with uid: {}", self.handle, msg_uid);
        let response = match messages::get_messages().to(&self.issued_did).uid(msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for claim {}", self.handle);
                return
            },
        };

        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for claim {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for claim {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["typ"] == String::from("claimReq") {
                //get the followup-claim-req using refMsgId
                self.state = CxsStateType::CxsStateRequestReceived;

                let payload = match msg["edgeAgentPayload"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("claim request has no edge agent payload");
                        return
                    }
                };


                let p = String::from(payload).replace("\\\"","\"");
                self.claim_request = match ClaimRequest::from_str(&p) {
                    Ok(x) => Some(x),
                    Err(_) => {
                        warn!("invalid claim request for claim {}", self.handle);
                        return
                    }
                };
                return
            }
        }
    }

    fn get_claim_offer_status(&mut self) {
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        else if self.state != CxsStateType::CxsStateOfferSent || self.msg_uid.is_empty() || self.issued_did.is_empty() {
            return;
        }
        // state is "OfferSent" so check to see if there is a new claimReq
        let response = match messages::get_messages().to(&self.issued_did).uid(&self.msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for claim {}", self.handle);
                return
            },
        };
        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for claim {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for claim {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["statusCode"].to_string() == "\"MS-104\"" {
                //get the followup-claim-req using refMsgId
                let ref_msg_id = match msg["refMsgId"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("invalid message reference id for claim {}", self.handle);
                        return
                    }
                };
                self.ref_msg_id = ref_msg_id.to_owned();
                self.get_claim_req(ref_msg_id);
            }
        }
    }

    fn update_state(&mut self) {
        self.get_claim_offer_status();
        //There will probably be more things here once we do other things with the claim
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn get_offer_uid(&self) -> String { self.msg_uid.clone() }
    fn set_offer_uid(&mut self, uid: &str) {self.msg_uid = uid.to_owned();}
    fn set_claim_request(&mut self, claim_request:&ClaimRequest){
        self.claim_request = Some(claim_request.clone());
    }
    pub fn create_standard_issuer_claim() -> Result<IssuerClaim, u32> {
        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_claim = IssuerClaim {
            handle: 123,
            source_id: "standard_claim".to_owned(),
            schema_seq_no: 103,
            msg_uid: "1234".to_owned(),
            claim_attributes: CLAIM_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
            claim_request: Some(claim_req),
            claim_name: "Claim".to_owned(),
            claim_id: String::from(DEFAULT_CLAIM_ID),
            ref_msg_id: String::new(),
        };
        Ok(issuer_claim)
    }

    fn generate_claim_offer(&self, to_did: &str) -> Result<ClaimOffer, u32> {
        let attr_map = convert_to_map(&self.claim_attributes)?;

        Ok(ClaimOffer {
            msg_type: String::from("CLAIM_OFFER"),
            version: String::from("0.1"),
            to_did: to_did.to_owned(),
            from_did: self.issued_did.to_owned(),
            claim: attr_map,
            schema_seq_no: self.schema_seq_no.to_owned(),
            issuer_did: String::from(self.issuer_did.to_owned()),
            claim_name: String::from(self.claim_name.to_owned()),
            claim_id: String::from(self.claim_id.to_owned()),
        })
    }
}

pub fn create_claim_payload_using_wallet<'a>(claim_id: &str, claim_req: &ClaimRequest, claim_data: &str, wallet_handle: i32) -> Result< String, u32> {
    println!("claim data: {}", claim_data);
    println!("claim request: {:?}", serde_json::to_string(&claim_req));
    let (sender, receiver) = channel();

    let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
        sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
    });
    info!("wallet_handle: {}", wallet_handle);
    let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

    let claim_req_master_secret = match claim_req.blinded_ms.clone() {
        Some(ms) => ms,
        // TODO: need new error
        None => {
            error!("No Master Secret in the Claim Request!");
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    let claim_req_str = match serde_json::to_string(&claim_req) {
        Ok(s) => s,
        // TODO: need new error
        Err(x) => {
            error!("Claim Request is not properly formatted/formed: {}", x);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    unsafe {
        let err = indy_issuer_create_claim(command_handle,
                                           wallet_handle,
                                           CString::new(claim_req_str).unwrap().as_ptr(),
                                           CString::new(claim_data).unwrap().as_ptr(),
                                           -1,
                                           cb);
        if err != 0 {
            error!("could not create claim: {}", err);
            return Err(error::UNKNOWN_ERROR.code_num);
        }
    }

    let (err, revoc_reg_update_json, xclaim_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0 {
        error!("could not create claim: {}", err);
        return Err(error::UNKNOWN_ERROR.code_num);
    };

    info!("xclaim_json: {}", xclaim_json);
    // add required fields for Consumer API

    Ok(xclaim_json)
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(claim) => Ok(claim.get_offer_uid()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn issuer_claim_create(schema_seq_no: u32,
                           source_id: Option<String>,
                           issuer_did: String,
                           claim_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        claim_attributes: claim_data,
        issued_did: String::new(),
        issuer_did: issuer_did,
        state: CxsStateType::CxsStateNone,
        schema_seq_no,
        claim_request: None,
        claim_name: String::from("Claim"),
        claim_id: new_handle.to_string(),
        ref_msg_id: String::new(),
    });

    match new_issuer_claim.validate_claim_offer() {
        Ok(_) => info!("successfully validated issuer_claim {}", new_handle),
        Err(x) => return Err(x),
    };

    new_issuer_claim.state = CxsStateType::CxsStateInitialized;

    info!("inserting handle {} into claim_issuer table", new_handle);
    ISSUER_CLAIM_MAP.lock().unwrap().insert(new_handle, new_issuer_claim);

    Ok(new_handle)
}

pub fn update_state(handle: u32) {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match ISSUER_CLAIM_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_ISSUER_CLAIM_HANDLE.code_num,
    }
}

pub fn is_valid_handle(handle: u32) -> bool {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(serde_json::to_string(&c).unwrap().to_owned()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn from_string(claim_data: &str) -> Result<u32,u32> {
    let derived_claim: IssuerClaim = match serde_json::from_str(claim_data) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = derived_claim.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let claim = Box::from(derived_claim);

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, claim);
    }

    Ok(new_handle)
}

pub fn send_claim_offer(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => match c.send_claim_offer(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(x) => Err(x),
        },
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn send_claim(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => match c.send_claim(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(x) => Err(x),
        },
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

fn get_offer_details(response: &str) -> Result<String,u32> {
    if settings::test_agency_mode_enabled() {return Ok("test_mode_response".to_owned());}
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = match json["uid"].as_str() {
                Some(x) => x,
                None => {
                    info!("response had no uid");
                    return Err(error::INVALID_JSON.code_num)
                },
            };
            Ok(String::from(detail))
        },
        Err(_) => {
            info!("Connect called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

pub fn set_claim_request(handle: u32, claim_request: &ClaimRequest) -> Result<u32,u32>{
   match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
       Some(c) => {c.set_claim_request(claim_request);
                    Ok(error::SUCCESS.code_num)},
       None => Err(error::UNKNOWN_ERROR.code_num),
   }
}

pub fn append_value(original_payload: &str,key: &str,  value: &str) -> Result<String, u32> {
    use serde_json::Value;
    let mut payload_json: Value = match serde_json::from_str(original_payload) {
        Ok(s) => s,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };
    payload_json[key] = json!(&value);
    match serde_json::to_string(&payload_json) {
        Ok(s) => Ok(s),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

pub fn convert_to_map(s:&str) -> Result<serde_json::Map<String, serde_json::Value>, u32>{
    let v:serde_json::Map<String, serde_json::Value> = match serde_json::from_str(s) {
        Ok(m) => m,
        Err(_) => { warn!("{}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
            return Err(error::INVALID_ATTRIBUTES_STRUCTURE.code_num)},
    };
    Ok(v)
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    use settings;
    use connection::create_connection;
    use std::thread;
    use std::time::Duration;
    use utils::signus::SignusUtils;
    use utils::wallet::init_wallet;
    use utils::issuer_claim::tests::{put_claim_def_in_issuer_wallet, create_default_schema};
    use claim_request::ClaimRequest;
    use super::*;

    static SCHEMA: &str = r#"{{
                            "seqNo":32,
                            "data":{{
                                "name":"gvt",
                                "version":"1.0",
                                "attr_names":["address1","address2","city","state", "zip"]
                            }}
                         }}"#;

    static CLAIM_DATA: &str =
        r#"{"address2":["101 Wilson Lane"],
        "zip":["87121"],
        "state":["UT"],
        "city":["SLC"],
        "address1":["101 Tela Lane"]
        }"#;

    static X_CLAIM_JSON: &str =
        r#"{"claim":{"address1":["101 Tela Lane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101 Wilson Lane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]},"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_seq_no":48,"signature":{"non_revocation_claim":null,"primary_claim":{"a":"","e":"","m2":"","v":""}}}"#;

    fn util_put_claim_def_in_issuer_wallet(schema_seq_num: u32, wallet_handle: i32) {
        let schema = &create_default_schema(schema_seq_num);

        let stored_xclaim = String::from("");

        info!("wallet_handle: {}", wallet_handle);
        let issuer_did = &settings::get_config_value(settings::CONFIG_ENTERPRISE_DID).unwrap();

        put_claim_def_in_issuer_wallet(issuer_did, schema, wallet_handle);
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    fn stand_up_a_wallet() -> (String, i32, String) {
        let wallet_name = String::from("wallet1");
        let wallet_handle = init_wallet(&wallet_name).unwrap();
        info!("Wallet Handle: {}", wallet_handle);
        let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
        info!("Successfully used wallet handle {} to create_and_store_my_did", wallet_handle);
        (wallet_name, wallet_handle, did)
    }

    fn create_standard_issuer_claim() -> IssuerClaim {
        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_claim = IssuerClaim {
            handle: 123,
            source_id: "standard_claim".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: CLAIM_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
            claim_name: DEFAULT_CLAIM_NAME.to_owned(),
            claim_request: Some(claim_req.to_owned()),
            claim_id: String::from(DEFAULT_CLAIM_ID),
            ref_msg_id: String::new(),
        };
        issuer_claim
    }

    fn print_error_message(e: &u32) -> () {
        use utils::error::error_message;
        ::utils::logger::LoggerUtils::init();
        info!("error message: {}", error_message(e));
    }

    fn normalize_claims(c1: &str, c2: &str) -> (serde_json::Value, serde_json::Value) {
        let mut v1: serde_json::Value = serde_json::from_str(c1.clone()).unwrap();
        let mut v2: serde_json::Value = serde_json::from_str(c2.clone()).unwrap();
        v1["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        (v1, v2)
    }

    #[test]
    fn test_issuer_claim_create_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        match issuer_claim_create(0,
                                  None,
                                  "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                  "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        let claim_id = DEFAULT_CLAIM_ID;

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claimOffer\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(send_claim_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(500));
        assert_eq!(get_state(handle), CxsStateType::CxsStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "6a9u7Jt");
        _m.assert();
    }

    #[test]
    fn test_send_a_claim() {
        let test_name = "test_send_a_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, "QTrbV4raAcND4DWWzBmdsh");

        let claim_req:ClaimRequest = match ClaimRequest::from_str(&CLAIM_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with claim request"),
        };
        let issuer_did = claim_req.issuer_did;
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claim\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let mut claim = create_standard_issuer_claim();
        claim.state = CxsStateType::CxsStateRequestReceived;
        util_put_claim_def_in_issuer_wallet(48, 0);

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                info!("error message: {}", error::error_message(&x));
                assert_eq!(x, 0)
            },
        };
        _m.assert();
        assert_eq!(claim.msg_uid, "6a9u7Jt");
        assert_eq!(claim.state, CxsStateType::CxsStateAccepted);
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
        release(handle);
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_handle, handle);
        assert_eq!(new_string, string);
    }

    #[test]
    fn test_update_state_with_pending_claim_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let response = "{\"msgs\":[{\"uid\":\"6gmsuWZ\",\"typ\":\"conReq\",\"statusCode\":\"MS-102\",\"statusMsg\":\"message sent\"},\
        {\"statusCode\":\"MS-104\",\"edgeAgentPayload\":\"{\\\"attr\\\":\\\"value\\\"}\",\"sendStatusCode\":\"MSS-101\",\"typ\":\"claimOffer\",\"statusMsg\":\"message accepted\",\"uid\":\"6a9u7Jt\",\"refMsgId\":\"CKrG14Z\"},\
        {\"msg_type\":\"CLAIM_REQUEST\",\"typ\":\"claimReq\",\"edgeAgentPayload\":\"{\\\"blinded_ms\\\":{\\\"prover_did\\\":\\\"FQ7wPBUgSPnDGJnS1EYjTK\\\",\\\"u\\\":\\\"923...607\\\",\\\"ur\\\":\\\"null\\\"},\\\"version\\\":\\\"0.1\\\",\\\"mid\\\":\\\"\\\",\\\"to_did\\\":\\\"BnRXf8yDMUwGyZVDkSENeq\\\",\\\"from_did\\\":\\\"GxtnGN6ypZYgEqcftSQFnC\\\",\\\"iid\\\":\\\"cCanHnpFAD\\\",\\\"issuer_did\\\":\\\"QTrbV4raAcND4DWWzBmdsh\\\",\\\"schema_seq_no\\\":48,\\\"optional_data\\\":{\\\"terms_of_service\\\":\\\"<Large block of text>\\\",\\\"price\\\":6}}\"}]}";
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body(response)
            .expect(2)
            .create();

        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let mut claim = IssuerClaim {
            handle: 123,
            source_id: "test_has_pending_claim_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
            claim_request: Some(claim_req.to_owned()),
            claim_name: DEFAULT_CLAIM_NAME.to_owned(),
            claim_id: String::from(DEFAULT_CLAIM_ID),
            ref_msg_id: String::new(),
        };

        claim.update_state();
        _m.assert();
        assert_eq!(claim.get_state(), CxsStateType::CxsStateRequestReceived as u32);
        let claim_request = claim.claim_request.clone().unwrap();
        assert_eq!(claim_request.issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(claim_request.schema_seq_no, 48);
        claim.claim_attributes = CLAIM_DATA.to_owned();
        println!("{}", &claim.claim_attributes);
        println!("{:?}", &claim.generate_claim_offer(&claim_request.issuer_did).unwrap());
        println!("{:?}", serde_json::to_string(&claim.generate_claim_offer("QTrbV4raAcND4DWWzBmdsh").unwrap()).unwrap());
    }

    #[test]
    fn test_issuer_claim_changes_state_after_being_validated() {
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"att\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        fn get_state_from_string(s: String) -> u32 {
            let json: serde_json::Value = serde_json::from_str(&s).unwrap();
            if json["state"].is_number() {
                return json["state"].as_u64().unwrap() as u32
            }
            0
        }
        assert_eq!(get_state_from_string(string), 1);
    }

    #[test]
    fn test_issuer_claim_can_build_claim_from_correct_parts() {
        let test_name = "test_issuer_claim_can_build_from_correct_parts";
        ::utils::logger::LoggerUtils::init();
        let schema_str = SCHEMA;
        let mut issuer_claim = create_standard_issuer_claim();
        let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e".to_owned();
        issuer_claim.claim_id = String::from(DEFAULT_CLAIM_ID);
        assert_eq!(issuer_claim.claim_id, DEFAULT_CLAIM_ID);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, &issuer_did);
        wallet::init_wallet(test_name).unwrap();
        let wallet_handle = wallet::get_wallet_handle();
        SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
        util_put_claim_def_in_issuer_wallet(48, wallet_handle);

        // set the claim request issuer did to the correct (enterprise) did.
        let mut claim_req = issuer_claim.claim_request.clone().unwrap();
        claim_req.issuer_did = issuer_did.to_owned();
        println!("IssuerClaim: {}", serde_json::to_string_pretty(&issuer_claim).unwrap());
        issuer_claim.claim_request = Some(claim_req);
        let encoded_claim_data = issuer_claim.create_attributes_encodings().unwrap();
        let claim_payload = match create_claim_payload_using_wallet(&issuer_claim.claim_id, &issuer_claim.claim_request.clone().unwrap(), &encoded_claim_data, wallet::get_wallet_handle()) {
            Ok(c) => c,
            Err(_) => panic!("Error creating claim payload"),
        };
        let claim_payload_json: serde_json::Value = serde_json::from_str(&claim_payload).unwrap();
        let x_claim_json: serde_json::Value = serde_json::from_str(X_CLAIM_JSON).unwrap();

        // remove primary claims signatures
        // as they will never match
        let (n1, n2) = normalize_claims(&claim_payload, &X_CLAIM_JSON);

        assert_eq!(serde_json::to_string(&n1).unwrap(), serde_json::to_string(&n2).unwrap());

        wallet::delete_wallet(test_name).unwrap();
    }

    #[test]
    fn test_issuer_claim_request_changes_reflect_in_claim_payload() {
        // TODO: Is this duplicate of the above test?
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, "NcYxiDXkpYi6ov5FcYDi1e");
        wallet::init_wallet("test_issuer_claim_request_changes_reflect_in_claim").unwrap();
        let wallet_handle = wallet::get_wallet_handle();

        util_put_claim_def_in_issuer_wallet(48, wallet_handle);
        let issuer_claim = create_standard_issuer_claim();
        let mut claim_request = issuer_claim.claim_request.clone().unwrap();
        claim_request.issuer_did = String::from("NcYxiDXkpYi6ov5FcYDi1e");
        assert_eq!(claim_request.schema_seq_no, 48);
        info!("claim request: {:?}", serde_json::to_string(&claim_request));
        info!("claim data: {:?}", &CLAIM_DATA);
        let encoded = issuer_claim.create_attributes_encodings().unwrap();
        let claim_payload = match create_claim_payload_using_wallet( &issuer_claim.claim_id,
                                                                    &claim_request,
                                                                    &encoded,
                                                                    wallet_handle) {
            Ok(c) => c,
            Err(_) => panic!("Error creating claim payload"),
        };

        let (n1, n2) = normalize_claims(&claim_payload, &X_CLAIM_JSON);
        info!("claim_payload: {}", claim_payload);
        assert_eq!(n1, n2);
        let claim_payload_with_from_did = append_value(&claim_payload, "from_did", &settings::CONFIG_ENTERPRISE_DID);
        info!("claim_payload_with_from_did: {:?}",claim_payload_with_from_did.unwrap());

        wallet::delete_wallet("test_issuer_claim_request_changes_reflect_in_claim").unwrap();
    }

    #[test]
    fn basic_add_attribute_encoding() {
        ::utils::logger::LoggerUtils::init();
        // FIXME Make this a real test and add additional test for create_attributes_encodings
        let issuer_claim = create_standard_issuer_claim();
        match issuer_claim.create_attributes_encodings() {
            Ok(x) => {
                println!("{}", x);
                assert!(true)
            },
            Err(e) => {
                error!("Error in create_attributes_encodings test");
                assert_eq!(0, 1)
            },
        };

        let mut issuer_claim = create_standard_issuer_claim();
        match issuer_claim.claim_attributes.pop() {
            Some(brace) => assert_eq!(brace, '}'),
            None => error!("Malformed claim attributes in the issuer claim test"),
        }
        match issuer_claim.create_attributes_encodings() {
            Ok(_) => {
                error!("basic_add_attribute_encoding test should raise error.");
                assert_ne!(1, 1);
            },
            Err(e) => assert_eq!(error::INVALID_JSON.code_num, e),
        }
    }

    #[test]
    fn test_claim_offer_has_proper_fields_for_sending_message() {
        static CORRECT_CLAIM_OFFER_PAYLOAD: &str = r#"{"msg_type":"CLAIM_OFFER","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","iid":"cCanHnpFAD","mid":"","claim":{"name":["Alice"],"date_of_birth":["2000-05-17"],"height":["175"]},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f","nonce":"351590","claim_name":"Profiledetail","issuer_name":"TestEnterprise","optional_data":{"terms_of_service":"<Largeblockoftext>","price":6}}"#;
        let to_did_from_connection = "E7pKs2CKAtKQQE3z3rmx8C";
        let issuer_claim = IssuerClaim::create_standard_issuer_claim().unwrap();
        assert_eq!(issuer_claim.claim_name, DEFAULT_CLAIM_NAME);
        let claim_offer_payload = issuer_claim.generate_claim_offer(&to_did_from_connection).unwrap();
        assert_eq!(claim_offer_payload.schema_seq_no, 103);
        assert_eq!(claim_offer_payload.claim_name, issuer_claim.claim_name);
        assert_eq!(claim_offer_payload.version, "0.1");
        assert_eq!(claim_offer_payload.from_did, issuer_claim.issued_did);
        assert_eq!(claim_offer_payload.issuer_did, issuer_claim.issuer_did);
        assert_eq!(claim_offer_payload.msg_type, "CLAIM_OFFER");
        assert_eq!(claim_offer_payload.claim, convert_to_map(&issuer_claim.claim_attributes).unwrap());
    }

    #[test]
    fn test_that_test_mode_enabled_bypasses_libindy_create_claim(){
        ::utils::logger::LoggerUtils::init();
        let test_name = "test_that_TEST_MODE_ENABLED_bypasses_libindy_create_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, "QTrbV4raAcND4DWWzBmdsh");

        let claim_req = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_did = claim_req.issuer_did;

        let mut claim = create_standard_issuer_claim();
        claim.state = CxsStateType::CxsStateRequestReceived;

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");
        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                info!("error message: {}", error::error_message(&x));
                assert_eq!(x, 0)
            },
        };
        assert_eq!(claim.state, CxsStateType::CxsStateAccepted);

    }


}
