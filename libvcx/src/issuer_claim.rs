extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::VcxStateType;
use utils::error;
use messages;
use settings;
use messages::GeneralMessage;
use messages::MessageResponseCode::{ MessageAccepted };
use messages::send_message::parse_msg_uid;
use connection;
use claim_request::ClaimRequest;
use utils::libindy::wallet;
use utils::openssl::encode;
use utils::httpclient;
use utils::constants::SEND_MESSAGE_RESPONSE;
use utils::libindy::anoncreds::{ libindy_issuer_create_claim };

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

static CLAIM_OFFER_ID_KEY: &str = "claim_offer_id";

#[derive(Serialize, Deserialize, Debug)]
pub struct IssuerClaim {
    source_id: String,
    #[serde(skip_serializing, default)]
    handle: u32,
    claim_attributes: String,
    msg_uid: String,
    schema_seq_no: u32,
    issuer_did: String,
    state: VcxStateType,
    pub claim_request: Option<ClaimRequest>,
    claim_name: String,
    pub claim_id: String,
    ref_msg_id: Option<String>,
    // the following 6 are pulled from the connection object
    agent_did: String, //agent_did for this relationship
    agent_vk: String,
    issued_did: String, //my_pw_did for this relationship
    issued_vk: String,
    remote_did: String, //their_pw_did for this relationship
    remote_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimOffer {
    pub msg_type: String,
    pub version: String,
    pub to_did: String,
    pub from_did: String,
    pub claim: serde_json::Map<String, serde_json::Value>,
    pub schema_seq_no: u32,
    pub issuer_did: String,
    pub claim_name: String,
    pub claim_id: String,
    pub msg_ref_id: Option<String>,
}

impl IssuerClaim {
    fn validate_claim_offer(&self) -> Result<u32, u32> {
        //TODO: validate claim_attributes against claim_def
        debug!("successfully validated issuer_claim {}", self.handle);
        Ok(error::SUCCESS.code_num)
    }

    fn send_claim_offer(&mut self, connection_handle: u32) -> Result<u32, u32> {
        debug!("sending claim offer for issuer_claim handle {} to connection handle {}", self.handle, connection_handle);
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("claim {} has invalid state {} for sending claimOffer", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({})", connection_handle);
            return Err(error::INVALID_CONNECTION_HANDLE.code_num);
        }

        self.agent_did = connection::get_agent_did(connection_handle)?;
        self.agent_vk = connection::get_agent_verkey(connection_handle)?;
        self.issued_did = connection::get_pw_did(connection_handle)?;
        self.issued_vk = connection::get_pw_verkey(connection_handle)?;
        self.remote_vk = connection::get_their_pw_verkey(connection_handle)?;

        let claim_offer = self.generate_claim_offer(&self.issued_did)?;
        let payload = match serde_json::to_string(&claim_offer) {
            Ok(p) => p,
            Err(_) => return Err(error::INVALID_JSON.code_num)
        };

        debug!("claim offer data: {}", payload);

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        let data = connection::generate_encrypted_payload(&self.issued_vk, &self.remote_vk, &payload, "CLAIM_OFFER")?;

        match messages::send_message().to(&self.issued_did)
            .to_vk(&self.issued_vk)
            .msg_type("claimOffer")
            .edge_agent_payload(&data)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .status_code(&MessageAccepted.as_string())
            .send_secure() {
            Err(x) => {
                warn!("could not send claimOffer: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = parse_msg_uid(&response[0])?;
                self.state = VcxStateType::VcxStateOfferSent;
                debug!("sent claim offer for: {}", self.handle);
                return Ok(error::SUCCESS.code_num);
            }
        }

    }

    fn send_claim(&mut self, connection_handle: u32) -> Result<u32, u32> {
        debug!("sending claim for issuer_claim handle {} to connection handle {}", self.handle, connection_handle);
        if self.state != VcxStateType::VcxStateRequestReceived {
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
            data = append_value(&data, CLAIM_OFFER_ID_KEY, &self.msg_uid)?;
            data = append_value(&data, "from_did", &to)?;
            data = append_value(&data, "version", "0.1")?;
            data = append_value(&data, "msg_type", "CLAIM")?;
        }

        debug!("claim data: {}", data);
        let data = connection::generate_encrypted_payload(&self.issued_vk, &self.remote_vk, &data, "CLAIM")?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(&self.issued_did)
            .to_vk(&self.issued_vk)
            .msg_type("claim")
            .status_code((&MessageAccepted.as_string()))
            .edge_agent_payload(&data)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .send_secure() {
            Err(x) => {
                warn!("could not send claim: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = parse_msg_uid(&response[0])?;
                self.state = VcxStateType::VcxStateAccepted;
                debug!("issued claim: {}", self.handle);
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    pub fn create_attributes_encodings(&self) -> Result<String, u32> {
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

    fn get_claim_offer_status(&mut self) -> Result<u32, u32> {
        debug!("updating state for claim offer: {}", self.handle);
        if self.state == VcxStateType::VcxStateRequestReceived {
            return Ok(error::SUCCESS.code_num);
        }
        else if self.state != VcxStateType::VcxStateOfferSent || self.msg_uid.is_empty() || self.issued_did.is_empty() {

            return Ok(error::SUCCESS.code_num);
        }
        let payload = messages::get_message::get_ref_msg(&self.msg_uid, &self.issued_did, &self.issued_vk, &self.agent_did, &self.agent_vk)?;

        self.claim_request = Some(parse_claim_req_payload(&payload)?);
        debug!("received claim request for claim offer: {}", self.handle);
        self.state = VcxStateType::VcxStateRequestReceived;
        Ok(error::SUCCESS.code_num)
    }

    fn update_state(&mut self) {
        self.get_claim_offer_status().unwrap_or(error::SUCCESS.code_num);
        //There will probably be more things here once we do other things with the claim
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn get_offer_uid(&self) -> String { self.msg_uid.clone() }
    fn set_offer_uid(&mut self, uid: &str) {self.msg_uid = uid.to_owned();}
    fn set_claim_request(&mut self, claim_request:ClaimRequest){
        self.claim_request = Some(claim_request);
    }

    fn get_source_id(&self) -> String { self.source_id.clone() }

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
            msg_ref_id: None,
        })
    }
}

pub fn create_claim_payload_using_wallet<'a>(claim_id: &str, claim_req: &ClaimRequest, claim_data: &str, wallet_handle: i32) -> Result< String, u32> {
    debug!("claim data: {}", claim_data);

    if claim_req.blinded_ms.is_none() {
        error!("No Master Secret in the Claim Request!");
        return Err(error::INVALID_MASTER_SECRET.code_num);
    }

    let claim_req_str = match serde_json::to_string(claim_req) {
        Ok(s) => s,
        Err(x) => {
            error!("Claim Request is not properly formatted/formed: {}", x);
            return Err(error::INVALID_JSON.code_num);
        },
    };
    debug!("claim request: {}", claim_req_str);

    let (_, xclaim_json) = libindy_issuer_create_claim(wallet_handle,
                                                       claim_req_str,
                                                       claim_data.to_string(),
                                                       -1)?;
    debug!("xclaim_json: {:?}", xclaim_json);
    Ok(xclaim_json)
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(claim) => Ok(claim.get_offer_uid()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

fn parse_claim_req_payload(payload: &Vec<u8>) -> Result<ClaimRequest, u32> {
    debug!("parsing claimReq payload: {:?}", payload);
    let data = messages::extract_json_payload(payload)?;

    let my_claim_req = match ClaimRequest::from_str(&data) {
         Ok(x) => x,
         Err(x) => {
             warn!("invalid json {}", x);
             return Err(error::INVALID_JSON.code_num);
         },
    };
    Ok(my_claim_req)
}

pub fn issuer_claim_create(schema_seq_no: u32,
                           source_id: String,
                           issuer_did: String,
                           claim_name: String,
                           claim_data: String) -> Result<u32, u32> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let mut new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        source_id,
        msg_uid: String::new(),
        claim_attributes: claim_data,
        issuer_did,
        state: VcxStateType::VcxStateNone,
        schema_seq_no,
        claim_request: None,
        claim_name,
        claim_id: new_handle.to_string(),
        ref_msg_id: None,
        issued_did: String::new(),
        issued_vk: String::new(),
        remote_did: String::new(),
        remote_vk: String::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
    });

    new_issuer_claim.validate_claim_offer()?;

    new_issuer_claim.state = VcxStateType::VcxStateInitialized;

    debug!("inserting handle {} into claim_issuer table", new_handle);
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
        None => VcxStateType::VcxStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match ISSUER_CLAIM_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_ISSUER_CLAIM_HANDLE.code_num,
    }
}

pub fn release_all() {
    let mut map = ISSUER_CLAIM_MAP.lock().unwrap();

    map.drain();
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

    let new_handle = rand::thread_rng().gen::<u32>();
    let source_id = derived_claim.source_id.clone();
    let claim = Box::from(derived_claim);

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        debug!("inserting handle {} with source_id {:?} into claim_issuer table",
               new_handle, source_id);
        m.insert(new_handle, claim);
    }

    Ok(new_handle)
}

pub fn send_claim_offer(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => Ok(c.send_claim_offer(connection_handle)?),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn send_claim(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => Ok(c.send_claim(connection_handle)?),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn set_claim_request(handle: u32, claim_request: ClaimRequest) -> Result<u32,u32>{
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => {c.set_claim_request(claim_request);
            Ok(error::SUCCESS.code_num)},
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
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

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(c.get_source_id()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

#[cfg(test)]
pub mod tests {
    use settings;
    use connection::build_connection;
    use utils::libindy::{ set_libindy_rc };
    use utils::libindy::signus::SignusUtils;
    use utils::libindy::anoncreds::libindy_create_and_store_claim_def;
    use claim_request::ClaimRequest;
    use utils::constants::*;
    use super::*;

    static DEFAULT_CLAIM_NAME: &str = "Claim";
    static DEFAULT_CLAIM_ID: &str = "defaultClaimId";
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
        r#"{"claim":{"address1":["101 Tela Lane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101 Wilson Lane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]},"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_seq_no":15,"signature":{"non_revocation_claim":null,"primary_claim":{"a":"","e":"","m2":"","v":""}}}"#;

    pub fn util_put_claim_def_in_issuer_wallet(schema_seq_num: u32, wallet_handle: i32) {
        let stored_xclaim = String::from("");

        let issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        libindy_create_and_store_claim_def(wallet_handle, issuer_did, SCHEMAS_JSON.to_string(), None, false).unwrap();
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    pub fn create_standard_issuer_claim() -> IssuerClaim {
        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_claim = IssuerClaim {
            handle: 123,
            source_id: "standard_claim".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: CLAIM_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            claim_name: DEFAULT_CLAIM_NAME.to_owned(),
            claim_request: Some(claim_req.to_owned()),
            claim_id: String::from(DEFAULT_CLAIM_ID),
            ref_msg_id: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        issuer_claim
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
                                  "1".to_string(),
                                  "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                  "claim_name".to_string(),
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
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "claim_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_claim_offer").unwrap();

        let claim_id = DEFAULT_CLAIM_ID;

        let handle = issuer_claim_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "claim_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();

        assert_eq!(send_claim_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_retry_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_claim_offer").unwrap();

        let claim_id = DEFAULT_CLAIM_ID;

        let handle = issuer_claim_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "claim_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        assert_eq!(send_claim_offer(handle, connection_handle), Err(error::TIMEOUT_LIBINDY_ERROR.code_num));
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "");

        // Can retry after initial failure
        assert_eq!(send_claim_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_send_a_claim() {
        let test_name = "test_send_a_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let claim_req:ClaimRequest = match ClaimRequest::from_str(&CLAIM_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with claim request"),
        };
        let issuer_did = claim_req.issuer_did;

        let mut claim = create_standard_issuer_claim();
        claim.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_claim_offer").unwrap();

        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                println!("error message: {}", error::error_message(&x));
                assert_eq!(x, 0)
            },
        };
        assert_eq!(claim.msg_uid, "ntc2ytb");
        assert_eq!(claim.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_claim_can_be_resent_after_failure() {

        let test_name = "test_send_a_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let claim_req:ClaimRequest = match ClaimRequest::from_str(&CLAIM_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with claim request"),
        };
        let issuer_did = claim_req.issuer_did;

        let mut claim = create_standard_issuer_claim();
        claim.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_claim_offer").unwrap();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 1),
            Err(x) => {
                assert_eq!(x, error::TIMEOUT_LIBINDY_ERROR.code_num)
            },
        };
        assert_eq!(claim.msg_uid, "1234");
        assert_eq!(claim.state, VcxStateType::VcxStateRequestReceived);
        // Retry sending the claim, use the mocked http. Show that you can retry sending the claim
        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                assert_eq!(x, 0)
            },
        };
        assert_eq!(claim.msg_uid, "ntc2ytb");
        assert_eq!(claim.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "claim_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
        release(handle);
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_string, string);
    }

    #[test]
    fn test_update_state_with_pending_claim_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_update_state_with_pending_claim_request").unwrap();
        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let mut claim = IssuerClaim {
            handle: 123,
            source_id: "test_has_pending_claim_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            claim_request: Some(claim_req.to_owned()),
            claim_name: DEFAULT_CLAIM_NAME.to_owned(),
            claim_id: String::from(DEFAULT_CLAIM_ID),
            ref_msg_id: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };

        httpclient::set_next_u8_response(CLAIM_REQ_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_CLAIM_RESPONSE.to_vec());

        claim.update_state();
        assert_eq!(claim.get_state(), VcxStateType::VcxStateRequestReceived as u32);
        let claim_request = claim.claim_request.clone().unwrap();
        assert_eq!(claim_request.issuer_did, "2hoqvcwupRTUNkXn6ArYzs");
        assert_eq!(claim_request.schema_seq_no, 15);
        claim.claim_attributes = CLAIM_DATA.to_owned();
        println!("{}", &claim.claim_attributes);
        println!("{:?}", &claim.generate_claim_offer(&claim_request.issuer_did).unwrap());
        println!("{:?}", serde_json::to_string(&claim.generate_claim_offer("QTrbV4raAcND4DWWzBmdsh").unwrap()).unwrap());
    }

    #[test]
    fn test_issuer_claim_changes_state_after_being_validated() {
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "claim_name".to_string(),
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
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e".to_owned();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &issuer_did);
        let schema_str = SCHEMA;
        let mut issuer_claim = create_standard_issuer_claim();
        issuer_claim.claim_id = String::from(DEFAULT_CLAIM_ID);
        assert_eq!(issuer_claim.claim_id, DEFAULT_CLAIM_ID);
        let handle = wallet::init_wallet("correct_parts").unwrap();
        println!("handle: {}", handle);
        SignusUtils::create_and_store_my_did(handle, None).unwrap();
        util_put_claim_def_in_issuer_wallet(15, handle);

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

        wallet::delete_wallet("correct_parts").unwrap();
    }

    #[test]
    fn test_issuer_claim_request_changes_reflect_in_claim_payload() {
        // TODO: Is this duplicate of the above test?
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "NcYxiDXkpYi6ov5FcYDi1e");
        wallet::init_wallet("test_issuer_claim_request_changes_reflect_in_claim").unwrap();
        let wallet_handle = wallet::get_wallet_handle();

        util_put_claim_def_in_issuer_wallet(15, wallet_handle);
        let issuer_claim = create_standard_issuer_claim();
        let mut claim_request = issuer_claim.claim_request.clone().unwrap();
        claim_request.issuer_did = String::from("NcYxiDXkpYi6ov5FcYDi1e");
        assert_eq!(claim_request.schema_seq_no, 15);
        println!("claim request: {:?}", serde_json::to_string(&claim_request));
        println!("claim data: {:?}", &CLAIM_DATA);
        let encoded = issuer_claim.create_attributes_encodings().unwrap();
        let claim_payload = match create_claim_payload_using_wallet( &issuer_claim.claim_id,
                                                                     &claim_request,
                                                                     &encoded,
                                                                     wallet_handle) {
            Ok(c) => c,
            Err(_) => panic!("Error creating claim payload"),
        };

        let (n1, n2) = normalize_claims(&claim_payload, &X_CLAIM_JSON);
        debug!("claim_payload: {}", claim_payload);
        assert_eq!(n1, n2);
        let claim_payload_with_from_did = append_value(&claim_payload, "from_did", &settings::CONFIG_INSTITUTION_DID);
        debug!("claim_payload_with_from_did: {:?}",claim_payload_with_from_did.unwrap());

        wallet::delete_wallet("test_issuer_claim_request_changes_reflect_in_claim").unwrap();
    }

    #[test]
    fn basic_add_attribute_encoding() {
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
    fn test_that_test_mode_enabled_bypasses_libindy_create_claim(){
        let test_name = "test_that_TEST_MODE_ENABLED_bypasses_libindy_create_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let mut claim = create_standard_issuer_claim();
        claim.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_claim_offer").unwrap();

        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                println!("error message: {}", error::error_message(&x));
                assert_eq!(x, 0)
            },
        };
        assert_eq!(claim.state, VcxStateType::VcxStateAccepted);

    }

    #[test]
    fn test_release_all() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let h1 = issuer_claim_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h2 = issuer_claim_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h3 = issuer_claim_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h4 = issuer_claim_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h5 = issuer_claim_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        release_all();
        assert_eq!(release(h1),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
        assert_eq!(release(h2),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
        assert_eq!(release(h3),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
        assert_eq!(release(h4),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
        assert_eq!(release(h5),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
    }
}
