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
use credential_request::CredentialRequest;
use utils::libindy::wallet;
use utils::openssl::encode;
use utils::httpclient;
use utils::constants::SEND_MESSAGE_RESPONSE;
use utils::libindy::anoncreds::{ libindy_issuer_create_credential };
use error::issuer_cred::IssuerCredError;
use utils::error::INVALID_JSON;

lazy_static! {
    static ref ISSUER_CREDENTIAL_MAP: Mutex<HashMap<u32, Box<IssuerCredential>>> = Default::default();
}

static CREDENTIAL_OFFER_ID_KEY: &str = "claim_offer_id";

#[derive(Serialize, Deserialize, Debug)]
pub struct IssuerCredential {
    source_id: String,
    #[serde(skip_serializing, default)]
    handle: u32,
    credential_attributes: String,
    msg_uid: String,
    schema_seq_no: u32,
    issuer_did: String,
    state: VcxStateType,
    pub credential_request: Option<CredentialRequest>,
    credential_name: String,
    pub credential_id: String,
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
pub struct CredentialOffer {
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

impl IssuerCredential {
    fn validate_credential_offer(&self) -> Result<u32, IssuerCredError> {
        //TODO: validate credential_attributes against credential_def
        debug!("successfully validated issuer_credential {}", self.handle);
        Ok(error::SUCCESS.code_num)
    }

    fn send_credential_offer(&mut self, connection_handle: u32) -> Result<u32, IssuerCredError> {
        debug!("sending credential offer for issuer_credential handle {} to connection handle {}", self.handle, connection_handle);
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("credential {} has invalid state {} for sending credentialOffer", self.handle, self.state as u32);
            return Err(IssuerCredError::NotReadyError())
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({})", connection_handle);
            return Err(IssuerCredError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num));
        }

        self.agent_did = connection::get_agent_did(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;
        self.agent_vk = connection::get_agent_verkey(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;
        self.issued_did = connection::get_pw_did(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;
        self.issued_vk = connection::get_pw_verkey(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;
        self.remote_vk = connection::get_their_pw_verkey(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;

        let credential_offer = self.generate_credential_offer(&self.issued_did)?;
        let payload = match serde_json::to_string(&credential_offer) {
            Ok(p) => p,
            Err(_) => return Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num))
        };

        debug!("credential offer data: {}", payload);

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        let data = connection::generate_encrypted_payload(&self.issued_vk, &self.remote_vk, &payload, "CLAIM_OFFER")
            .map_err(|x| IssuerCredError::CommonError(x))?;

        match messages::send_message().to(&self.issued_did)
            .to_vk(&self.issued_vk)
            .msg_type("claimOffer")
            .edge_agent_payload(&data)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .status_code(&MessageAccepted.as_string())
            .send_secure() {
            Err(x) => {
                warn!("could not send credentialOffer: {}", x);
                return Err(IssuerCredError::CommonError(x));
            },
            Ok(response) => {
                self.msg_uid = parse_msg_uid(&response[0]).map_err(|ec| IssuerCredError::CommonError(ec))?;
                self.state = VcxStateType::VcxStateOfferSent;
                debug!("sent credential offer for: {}", self.handle);
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn send_credential(&mut self, connection_handle: u32) -> Result<u32, IssuerCredError> {
        debug!("sending credential for issuer_credential handle {} to connection handle {}", self.handle, connection_handle);
        if self.state != VcxStateType::VcxStateRequestReceived {
            warn!("credential {} has invalid state {} for sending credential", self.handle, self.state as u32);
            return Err(IssuerCredError::NotReadyError());
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_credential_offer", connection_handle);
            return Err(IssuerCredError::InvalidHandle());
        }

        let to = connection::get_pw_did(connection_handle).map_err(|x| IssuerCredError::CommonError(x))?;
        let attrs_with_encodings = self.create_attributes_encodings()?;
        let mut data;
        if settings::test_indy_mode_enabled() {
            data = String::from("dummytestmodedata");
        } else {
            data = match self.credential_request.clone() {
                Some(d) => create_credential_payload_using_wallet(&self.credential_id, &d, &attrs_with_encodings, wallet::get_wallet_handle())?,
                None => {
                    warn!("Unable to create credential payload using the wallet");
                    return Err(IssuerCredError::InvalidCredRequest())
                },
            };
            // append values we need for example 'from_did' and 'credential_id'
            data = append_value(&data, CREDENTIAL_OFFER_ID_KEY, &self.msg_uid)?;
            data = append_value(&data, "from_did", &to)?;
            data = append_value(&data, "version", "0.1")?;
            data = append_value(&data, "msg_type", "CLAIM")?;
        }

        debug!("credential data: {}", data);

        let data = connection::generate_encrypted_payload(&self.issued_vk, &self.remote_vk, &data, "CLAIM")
            .map_err(|x| IssuerCredError::CommonError(x))?;
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
                warn!("could not send credential: {}", x);
                return Err(IssuerCredError::CommonError(x));
            },
            Ok(response) => {
                self.msg_uid = parse_msg_uid(&response[0]).map_err(|ec| IssuerCredError::CommonError(ec))?;
                self.state = VcxStateType::VcxStateAccepted;
                debug!("issued credential: {}", self.handle);
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    pub fn create_attributes_encodings(&self) -> Result<String, IssuerCredError> {
        let mut attributes: serde_json::Value = match serde_json::from_str(&self.credential_attributes) {
            Ok(x) => x,
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                return Err(IssuerCredError::CommonError(INVALID_JSON.code_num))
            }
        };

        let map = match attributes.as_object_mut() {
            Some(x) => x,
            None => {
                warn!("Invalid Json for Attribute data");
                return Err(IssuerCredError::CommonError(INVALID_JSON.code_num))
            }
        };

        for (attr, vec) in map.iter_mut(){
            let list = match vec.as_array_mut() {
                Some(x) => x,
                None => {
                    warn!("Invalid Json for Attribute data");
                    return Err(IssuerCredError::CommonError(INVALID_JSON.code_num))
                }
            };
            let i = list[0].clone();
            let value = match i.as_str(){
                Some(v) => v,
                None => {
                    warn!("Cannot encode attribute: {}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
                    return Err(IssuerCredError::CommonError(error::INVALID_ATTRIBUTES_STRUCTURE.code_num))
                },
            };
            let encoded = encode(value).map_err(|x| IssuerCredError::CommonError(x))?;
            let encoded_as_value: serde_json::Value = serde_json::Value::from(encoded);
            list.push(encoded_as_value);
        }

        match serde_json::to_string_pretty(&map) {
            Ok(x) => Ok(x),
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                Err(IssuerCredError::CommonError(INVALID_JSON.code_num))
            }
        }
    }

    // TODO: The error arm of this Result is never used in any calling functions.
    // So currently there is no way to test the error status.
    fn get_credential_offer_status(&mut self) -> Result<u32, u32> {
        debug!("updating state for credential offer: {}", self.handle);
        if self.state == VcxStateType::VcxStateRequestReceived {
            return Ok(error::SUCCESS.code_num);
        }
        else if self.state != VcxStateType::VcxStateOfferSent || self.msg_uid.is_empty() || self.issued_did.is_empty() {

            return Ok(error::SUCCESS.code_num);
        }
        let payload = messages::get_message::get_ref_msg(&self.msg_uid, &self.issued_did, &self.issued_vk, &self.agent_did, &self.agent_vk)?;

        self.credential_request = Some(parse_credential_req_payload(&payload)?);
        debug!("received credential request for credential offer: {}", self.handle);
        self.state = VcxStateType::VcxStateRequestReceived;
        Ok(error::SUCCESS.code_num)
    }

    fn update_state(&mut self) {
        self.get_credential_offer_status().unwrap_or(error::SUCCESS.code_num);
        //There will probably be more things here once we do other things with the credential
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn get_offer_uid(&self) -> &String { &self.msg_uid }
    fn set_offer_uid(&mut self, uid: &str) {self.msg_uid = uid.to_owned();}
    fn set_credential_request(&mut self, credential_request:CredentialRequest){
        self.credential_request = Some(credential_request);
    }

    fn get_source_id(&self) -> &String { &self.source_id }
    fn generate_credential_offer(&self, to_did: &str) -> Result<CredentialOffer, IssuerCredError> {
        let attr_map = convert_to_map(&self.credential_attributes)?;

        Ok(CredentialOffer {
            msg_type: String::from("CLAIM_OFFER"),
            version: String::from("0.1"),
            to_did: to_did.to_owned(),
            from_did: self.issued_did.to_owned(),
            claim: attr_map,
            schema_seq_no: self.schema_seq_no.to_owned(),
            issuer_did: String::from(self.issuer_did.to_owned()),
            claim_name: String::from(self.credential_name.to_owned()),
            claim_id: String::from(self.credential_id.to_owned()),
            msg_ref_id: None,
        })
    }
}

pub fn create_credential_payload_using_wallet<'a>(credential_id: &str, credential_req: &CredentialRequest,
                                             credential_data: &str, wallet_handle: i32) -> Result< String, IssuerCredError> {
    debug!("credential data: {}", credential_data);

    if credential_req.blinded_ms.is_none() {
        error!("No Master Secret in the Credential Request!");
        return Err(IssuerCredError::CommonError(error::INVALID_MASTER_SECRET.code_num));
    }

    let credential_req_str = match serde_json::to_string(credential_req) {
        Ok(s) => s,
        Err(x) => {
            error!("Credential Request is not properly formatted/formed: {}", x);
            return Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num));
        },
    };
    debug!("credential request: {}", credential_req_str);

    let (_, xcredential_json) = libindy_issuer_create_credential(wallet_handle,
                                                       &credential_req_str,
                                                       credential_data,
                                                       -1).map_err(|x| IssuerCredError::CommonError(x))?;
    debug!("xcredential_json: {:?}", xcredential_json);
    Ok(xcredential_json)
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get(&handle) {
        Some(credential) => Ok(credential.get_offer_uid().clone()),
        None => Err(error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num),
    }
}

fn parse_credential_req_payload(payload: &Vec<u8>) -> Result<CredentialRequest, u32> {
    debug!("parsing credentialReq payload: {:?}", payload);
    let data = messages::extract_json_payload(payload)?;

    let my_credential_req = match CredentialRequest::from_str(&data) {
         Ok(x) => x,
         Err(x) => {
             warn!("invalid json {}", x);
             return Err(error::INVALID_JSON.code_num);
         },
    };
    Ok(my_credential_req)
}

// TODO: The error arm of this Result is never thrown.  aka this method is never Err.
pub fn issuer_credential_create(schema_seq_no: u32,
                           source_id: String,
                           issuer_did: String,
                           credential_name: String,
                           credential_data: String) -> Result<u32, IssuerCredError> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let mut new_issuer_credential = Box::new(IssuerCredential {
        handle: new_handle,
        source_id,
        msg_uid: String::new(),
        credential_attributes: credential_data,
        issuer_did,
        state: VcxStateType::VcxStateNone,
        schema_seq_no,
        credential_request: None,
        credential_name,
        credential_id: new_handle.to_string(),
        ref_msg_id: None,
        issued_did: String::new(),
        issued_vk: String::new(),
        remote_did: String::new(),
        remote_vk: String::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
    });

    new_issuer_credential.validate_credential_offer()?;

    new_issuer_credential.state = VcxStateType::VcxStateInitialized;

    debug!("inserting handle {} into credential_issuer table", new_handle);
    ISSUER_CREDENTIAL_MAP.lock().unwrap().insert(new_handle, new_issuer_credential);

    Ok(new_handle)
}

pub fn update_state(handle: u32) {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => VcxStateType::VcxStateNone as u32,
    }
}

pub fn release(handle: u32) -> Result< u32, IssuerCredError> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().remove(&handle) {
        Some(t) => Ok(error::SUCCESS.code_num),
        None => Err(IssuerCredError::InvalidHandle()),
    }
}

pub fn release_all() {
    let mut map = ISSUER_CREDENTIAL_MAP.lock().unwrap();

    map.drain();
}

pub fn is_valid_handle(handle: u32) -> bool {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn to_string(handle: u32) -> Result<String, IssuerCredError> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(serde_json::to_string(&c).unwrap().to_owned()),
        None => Err(IssuerCredError::InvalidHandle()),
    }
}

pub fn from_string(credential_data: &str) -> Result<u32,u32> {
    let derived_credential: IssuerCredential = match serde_json::from_str(credential_data) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = rand::thread_rng().gen::<u32>();
    let source_id = derived_credential.source_id.clone();
    let credential = Box::from(derived_credential);

    {
        let mut m = ISSUER_CREDENTIAL_MAP.lock().unwrap();
        debug!("inserting handle {} with source_id {:?} into credential_issuer table",
               new_handle, source_id);
        m.insert(new_handle, credential);
    }

    Ok(new_handle)
}

pub fn send_credential_offer(handle: u32, connection_handle: u32) -> Result<u32,IssuerCredError> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => Ok(c.send_credential_offer(connection_handle)?),
        None => Err(IssuerCredError::InvalidHandle()),
    }
}

pub fn send_credential(handle: u32, connection_handle: u32) -> Result<u32,IssuerCredError> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => Ok(c.send_credential(connection_handle)?),
        None => Err(IssuerCredError::InvalidHandle()),
    }
}

fn get_offer_details(response: &str) -> Result<String, IssuerCredError> {
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = match json["uid"].as_str(){
                Some(x) => x,
                None => {
                    warn!("response had no uid");
                    return Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num))
                },
            };
            Ok(String::from(detail))
        },
        Err(_) => {
            warn!("get_messages called without a valid response from server");
            Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num))
        },
    }
}

pub fn set_credential_request(handle: u32, credential_request: CredentialRequest) -> Result<u32,IssuerCredError>{
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => {c.set_credential_request(credential_request);
            Ok(error::SUCCESS.code_num)},
        None => Err(IssuerCredError::InvalidHandle()),
    }
}

pub fn append_value(original_payload: &str,key: &str,  value: &str) -> Result<String, IssuerCredError> {
    use serde_json::Value;
    let mut payload_json: Value = match serde_json::from_str(original_payload) {
        Ok(s) => s,
        Err(_) => return Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num)),
    };
    payload_json[key] = json!(&value);
    match serde_json::to_string(&payload_json) {
        Ok(s) => Ok(s),
        Err(_) => return Err(IssuerCredError::CommonError(error::INVALID_JSON.code_num)),
    }
}

pub fn convert_to_map(s:&str) -> Result<serde_json::Map<String, serde_json::Value>, IssuerCredError>{
    let v:serde_json::Map<String, serde_json::Value> = match serde_json::from_str(s) {
        Ok(m) => m,
        Err(_) => {
            warn!("{}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
            return Err(IssuerCredError::CommonError(error::INVALID_ATTRIBUTES_STRUCTURE.code_num))
        },
    };
    Ok(v)
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    match ISSUER_CREDENTIAL_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(c.get_source_id().clone()),
        None => Err(error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use settings;
    use connection::build_connection;
    use utils::libindy::{ set_libindy_rc };
    use utils::libindy::signus::SignusUtils;
    use utils::libindy::anoncreds::libindy_create_and_store_credential_def;
    use credential_request::CredentialRequest;
    use utils::constants::*;
    use error::issuer_cred::IssuerCredError;
    use error::ToErrorCode;

    static DEFAULT_CREDENTIAL_NAME: &str = "Credential";
    static DEFAULT_CREDENTIAL_ID: &str = "defaultCredentialId";
    static SCHEMA: &str = r#"{{
                            "seqNo":32,
                            "data":{{
                                "name":"gvt",
                                "version":"1.0",
                                "attr_names":["address1","address2","city","state", "zip"]
                            }}
                         }}"#;

    static CREDENTIAL_DATA: &str =
        r#"{"address2":["101 Wilson Lane"],
        "zip":["87121"],
        "state":["UT"],
        "city":["SLC"],
        "address1":["101 Tela Lane"]
        }"#;

    static X_CREDENTIAL_JSON: &str =
        r#"{"claim":{"address1":["101 Tela Lane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101 Wilson Lane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]},"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_seq_no":15,"signature":{"non_revocation_claim":null,"primary_claim":{"a":"","e":"","m2":"","v":""}}}"#;

    pub fn util_put_credential_def_in_issuer_wallet(schema_seq_num: u32, wallet_handle: i32) {
        let stored_xcredential = String::from("");

        let issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        libindy_create_and_store_credential_def(wallet_handle, &issuer_did, SCHEMAS_JSON, None, false).unwrap();
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    pub fn create_standard_issuer_credential() -> IssuerCredential {
        let credential_req:CredentialRequest = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let issuer_credential = IssuerCredential {
            handle: 123,
            source_id: "standard_credential".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            credential_attributes: CREDENTIAL_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            credential_name: DEFAULT_CREDENTIAL_NAME.to_owned(),
            credential_request: Some(credential_req.to_owned()),
            credential_id: String::from(DEFAULT_CREDENTIAL_ID),
            ref_msg_id: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        issuer_credential
    }

    fn normalize_credentials(c1: &str, c2: &str) -> (serde_json::Value, serde_json::Value) {
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
    fn test_issuer_credential_create_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        match issuer_credential_create(0,
                                  "1".to_string(),
                                  "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                  "credential_name".to_string(),
                                  "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let handle = issuer_credential_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "credential_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_credential_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_credential_offer").unwrap();

        let credential_id = DEFAULT_CREDENTIAL_ID;

        let handle = issuer_credential_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "credential_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();

        assert_eq!(send_credential_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_retry_send_credential_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_credential_offer").unwrap();

        let credential_id = DEFAULT_CREDENTIAL_ID;

        let handle = issuer_credential_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "credential_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        assert_eq!(send_credential_offer(handle, connection_handle), Err(IssuerCredError::CommonError(error::TIMEOUT_LIBINDY_ERROR.code_num)));
        assert_eq!(get_state(handle), VcxStateType::VcxStateInitialized as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "");

        // Can retry after initial failure
        assert_eq!(send_credential_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_send_a_credential() {
        let test_name = "test_send_a_credential";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let credential_req:CredentialRequest = match CredentialRequest::from_str(&CREDENTIAL_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with credential request"),
        };
        let issuer_did = credential_req.issuer_did;

        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_credential_offer").unwrap();

        match credential.send_credential(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                println!("error message: {}", error::error_message(&x.to_error_code()));
                assert_eq!(x.to_error_code(), 0)
            },
        };
        assert_eq!(credential.msg_uid, "ntc2ytb");
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_credential_can_be_resent_after_failure() {

        let test_name = "test_send_a_credential";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let credential_req:CredentialRequest = match CredentialRequest::from_str(&CREDENTIAL_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with credential request"),
        };
        let issuer_did = credential_req.issuer_did;

        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_credential_offer").unwrap();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        match credential.send_credential(connection_handle) {
            Ok(_) => assert_eq!(0, 1),
            Err(x) => {
                assert_eq!(x, IssuerCredError::CommonError(error::TIMEOUT_LIBINDY_ERROR.code_num))
            },
        };
        assert_eq!(credential.msg_uid, "1234");
        assert_eq!(credential.state, VcxStateType::VcxStateRequestReceived);
        // Retry sending the credential, use the mocked http. Show that you can retry sending the credential
        match credential.send_credential(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                assert_eq!(x.to_error_code(), 0)
            },
        };
        assert_eq!(credential.msg_uid, "ntc2ytb");
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = issuer_credential_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "credential_name".to_string(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
        assert!(release(handle).is_ok());
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_string, string);
    }

    #[test]
    fn test_update_state_with_pending_credential_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_update_state_with_pending_credential_request").unwrap();
        let credential_req:CredentialRequest = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let mut credential = IssuerCredential {
            handle: 123,
            source_id: "test_has_pending_credential_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            credential_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            credential_request: Some(credential_req.to_owned()),
            credential_name: DEFAULT_CREDENTIAL_NAME.to_owned(),
            credential_id: String::from(DEFAULT_CREDENTIAL_ID),
            ref_msg_id: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };

        httpclient::set_next_u8_response(CREDENTIAL_REQ_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_CREDENTIAL_RESPONSE.to_vec());

        credential.update_state();
        assert_eq!(credential.get_state(), VcxStateType::VcxStateRequestReceived as u32);
        let credential_request = credential.credential_request.clone().unwrap();
        assert_eq!(credential_request.issuer_did, "2hoqvcwupRTUNkXn6ArYzs");
        assert_eq!(credential_request.schema_seq_no, 15);
        credential.credential_attributes = CREDENTIAL_DATA.to_owned();
        println!("{}", &credential.credential_attributes);
        println!("{:?}", &credential.generate_credential_offer(&credential_request.issuer_did).unwrap());
        println!("{:?}", serde_json::to_string(&credential.generate_credential_offer("QTrbV4raAcND4DWWzBmdsh").unwrap()).unwrap());
    }

    #[test]
    fn test_issuer_credential_changes_state_after_being_validated() {
        set_default_and_enable_test_mode();
        let handle = issuer_credential_create(0,
                                         "1".to_string(),
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "credential_name".to_string(),
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
    fn test_issuer_credential_can_build_credential_from_correct_parts() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e".to_owned();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &issuer_did);
        let schema_str = SCHEMA;
        let mut issuer_credential = create_standard_issuer_credential();
        issuer_credential.credential_id = String::from(DEFAULT_CREDENTIAL_ID);
        assert_eq!(issuer_credential.credential_id, DEFAULT_CREDENTIAL_ID);
        let handle = wallet::init_wallet("correct_parts").unwrap();
        println!("handle: {}", handle);
        SignusUtils::create_and_store_my_did(handle, None).unwrap();
        util_put_credential_def_in_issuer_wallet(15, handle);

        // set the credential request issuer did to the correct (enterprise) did.
        let mut credential_req = issuer_credential.credential_request.clone().unwrap();
        credential_req.issuer_did = issuer_did.to_owned();
        println!("IssuerCredential: {}", serde_json::to_string_pretty(&issuer_credential).unwrap());
        issuer_credential.credential_request = Some(credential_req);
        let encoded_credential_data = issuer_credential.create_attributes_encodings().unwrap();
        let credential_payload = match create_credential_payload_using_wallet(&issuer_credential.credential_id, &issuer_credential.credential_request.clone().unwrap(), &encoded_credential_data, wallet::get_wallet_handle()) {
            Ok(c) => c,
            Err(_) => panic!("Error creating credential payload"),
        };
        let credential_payload_json: serde_json::Value = serde_json::from_str(&credential_payload).unwrap();
        let x_credential_json: serde_json::Value = serde_json::from_str(X_CREDENTIAL_JSON).unwrap();

        // remove primary credentials signatures
        // as they will never match
        let (n1, n2) = normalize_credentials(&credential_payload, &X_CREDENTIAL_JSON);

        assert_eq!(serde_json::to_string(&n1).unwrap(), serde_json::to_string(&n2).unwrap());

        wallet::delete_wallet("correct_parts").unwrap();
    }

    #[test]
    fn test_issuer_credential_request_changes_reflect_in_credential_payload() {
        // TODO: Is this duplicate of the above test?
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "NcYxiDXkpYi6ov5FcYDi1e");
        wallet::init_wallet("test_issuer_credential_request_changes_reflect_in_credential").unwrap();
        let wallet_handle = wallet::get_wallet_handle();

        util_put_credential_def_in_issuer_wallet(15, wallet_handle);
        let issuer_credential = create_standard_issuer_credential();
        let mut credential_request = issuer_credential.credential_request.clone().unwrap();
        credential_request.issuer_did = String::from("NcYxiDXkpYi6ov5FcYDi1e");
        assert_eq!(credential_request.schema_seq_no, 15);
        println!("credential request: {:?}", serde_json::to_string(&credential_request));
        println!("credential data: {:?}", &CREDENTIAL_DATA);
        let encoded = issuer_credential.create_attributes_encodings().unwrap();
        let credential_payload = match create_credential_payload_using_wallet( &issuer_credential.credential_id,
                                                                     &credential_request,
                                                                     &encoded,
                                                                     wallet_handle) {
            Ok(c) => c,
            Err(_) => panic!("Error creating credential payload"),
        };

        let (n1, n2) = normalize_credentials(&credential_payload, &X_CREDENTIAL_JSON);
        debug!("credential_payload: {}", credential_payload);
        assert_eq!(n1, n2);
        let credential_payload_with_from_did = append_value(&credential_payload, "from_did", &settings::CONFIG_INSTITUTION_DID);
        debug!("credential_payload_with_from_did: {:?}",credential_payload_with_from_did.unwrap());

        wallet::delete_wallet("test_issuer_credential_request_changes_reflect_in_credential").unwrap();
    }

    #[test]
    fn basic_add_attribute_encoding() {
        // FIXME Make this a real test and add additional test for create_attributes_encodings
        let issuer_credential = create_standard_issuer_credential();
        match issuer_credential.create_attributes_encodings() {
            Ok(x) => {
                println!("{}", x);
                assert!(true)
            },
            Err(e) => {
                error!("Error in create_attributes_encodings test");
                assert_eq!(0, 1)
            },
        };

        let mut issuer_credential = create_standard_issuer_credential();
        match issuer_credential.credential_attributes.pop() {
            Some(brace) => assert_eq!(brace, '}'),
            None => error!("Malformed credential attributes in the issuer credential test"),
        }
        match issuer_credential.create_attributes_encodings() {
            Ok(_) => {
                error!("basic_add_attribute_encoding test should raise error.");
                assert_ne!(1, 1);
            },
            Err(e) => assert_eq!(e, IssuerCredError::CommonError(error::INVALID_JSON.code_num)),
        }
    }

    #[test]
    fn test_that_test_mode_enabled_bypasses_libindy_create_credential(){
        let test_name = "test_that_TEST_MODE_ENABLED_bypasses_libindy_create_credential";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_connection("test_send_credential_offer").unwrap();

        match credential.send_credential(connection_handle) {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => {
                println!("error message: {}", error::error_message(&x.to_error_code()));
                assert_eq!(x.to_error_code(), 0)
            },
        };
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);

    }

    #[test]
    fn test_release_all() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let h1 = issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h2 = issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h3 = issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h4 = issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let h5 = issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        release_all();
        assert_eq!(release(h1),Err(IssuerCredError::InvalidHandle()));
        assert_eq!(release(h2),Err(IssuerCredError::InvalidHandle()));
        assert_eq!(release(h3),Err(IssuerCredError::InvalidHandle()));
        assert_eq!(release(h4),Err(IssuerCredError::InvalidHandle()));
        assert_eq!(release(h5),Err(IssuerCredError::InvalidHandle()));
    }

    #[test]
    fn test_errors(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let invalid_handle = 478620;
        assert_eq!(to_string(invalid_handle).err(), Some(IssuerCredError::InvalidHandle()));
        assert_eq!(release(invalid_handle).err(), Some(IssuerCredError::InvalidHandle()));
    }
}
