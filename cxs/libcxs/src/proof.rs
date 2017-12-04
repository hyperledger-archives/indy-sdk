extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use settings;
use messages;
use messages::GeneralMessage;
use connection;

lazy_static! {
    static ref PROOF_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

static DEFAULT_PROOF_NAME: &str = "Proof";

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    source_id: String,
    handle: u32,
    requested_attrs: String,
    requested_predicates: String,
    msg_uid: String,
    requester_did: String,
    prover_did: String,
    state: CxsStateType,
    tid: u32,
    mid: u32,
    name: String,
}

impl Proof {
    fn validate_proof_request(&self) -> Result<u32, String> {
        //TODO: validate proof request
        Ok(error::SUCCESS.code_num)
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateInitialized {
            warn!("proof {} has invalid state {} for sending proofRequest", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }
        self.prover_did = connection::get_pw_did(connection_handle)?;
        let from_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT).unwrap();
        //TODO: call to libindy to encrypt payload
        //TODO: Set expiration date
        let proof_request = messages::proof_request()
            .type_version("1.0")
            .prover_did(&self.prover_did)
            .requester_did(&self.requester_did)
            .tid(1)
            .mid(9)
            .nonce("123432421212")
            .proof_name(&self.name)
            .proof_data_version(".1")
            .requested_attrs(&self.requested_attrs)
//            .requested_predicates(&self.requested_predicates)
            .serialize_message()?;

        match messages::send_message().to(&self.prover_did).msg_type("proofReq").edge_agent_payload(&proof_request).send() {
            Ok(response) => {
                self.msg_uid = get_offer_details(&response)?;
                self.state = CxsStateType::CxsStateOfferSent;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proofReq: {}", x);
                return Err(x);
            }
        }
    }

    fn get_proof_offer(&mut self, msg_uid: &str) {
        return
    }

    fn get_proof_request_status(&mut self) {
        // If proof received Todo: fix states to make sense for proofs
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        //If proof request not sent
        else if self.state != CxsStateType::CxsStateOfferSent || self.msg_uid.is_empty() || self.prover_did.is_empty() {
            return;
        }

        // State is proof request sent
        let response = match messages::get_messages().to(&self.prover_did).uid(&self.msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for proof {}", self.handle);
                return
            },
        };
        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for proof {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for proof {}", self.handle);
                return
            },
        };

        for msg in msgs {
            //Todo: Find out what message will look like for proof offer??
            //Todo: This will see if there is a proof offer from user
            if msg["statusCode"].to_string() == "\"Don't hit yet\"" {
                let ref_msg_id = match msg["refMsgId"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("invalid message reference id for proof {}", self.handle);
                        return
                    }
                };
                self.get_proof_offer(ref_msg_id);
            }
        }


    }

    fn update_state(&mut self) {
        self.get_proof_request_status();
    }

    fn get_state(&self) -> u32 {let state = self.state as u32; state}

    fn get_offer_uid(&self) -> String { self.msg_uid.clone() }
}

pub fn create_proof(source_id: Option<String>,
                    requester_did: String,
                    requested_attrs: String,
                    requested_predicates: String,
                    name: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        requested_attrs,
        requested_predicates,
        requester_did,
        prover_did: String::new(),
        state: CxsStateType::CxsStateNone,
        tid: 0,
        mid: 0,
        name,
    });

    match new_proof.validate_proof_request() {
        Ok(_) => info!("successfully validated proof {}", new_handle),
        Err(x) => return Err(x),
    };

    new_proof.state = CxsStateType::CxsStateInitialized;

    {
        let mut m = PROOF_MAP.lock().unwrap();
        info!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, new_proof);
    }

    Ok(new_handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn update_state(handle: u32) {
    match PROOF_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_PROOF_HANDLE.code_num,
    }
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num)
    }
}

pub fn from_string(proof_data: &str) -> Result<u32, u32> {

    let derived_proof: Proof = match serde_json::from_str(proof_data) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let new_handle = derived_proof.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let proof = Box::from(derived_proof);

    {
        let mut m = PROOF_MAP.lock().unwrap();
        info!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, proof);
    }

    Ok(new_handle)
}

pub fn send_proof_request(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match PROOF_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => match c.send_proof_request(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(x) => Err(x),
        },
        None => Err(error::INVALID_PROOF_HANDLE.code_num),
    }
}

fn get_offer_details(response: &str) -> Result<String, u32> {
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
            info!("Proof called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(proof) => Ok(proof.get_offer_uid()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate mockito;
    use std::thread;
    use std::time::Duration;
    use connection::create_connection;

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static EXPECTED_ATTRS: &'static str = "{\"Test0\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_1\",\"schema_seq_no\":1},\"Test1\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\",\"schema_seq_no\":1},\"Test2\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"city\",\"schema_seq_no\":1},\"Test3\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"state\",\"schema_seq_no\":1},\"Test4\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"zip\",\"schema_seq_no\":1}";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";


    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }


    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_create_proof_succeeds() {
        set_default_and_enable_test_mode();

        match create_proof(None,
                           "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                           REQUESTED_ATTRS.to_owned(),
                           REQUESTED_PREDICATES.to_owned(),
                            "Optional".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1),
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = match create_proof(None,
                           "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let proof_string = to_string(handle).unwrap();
        assert!(!proof_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(None,
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let proof_data = to_string(handle).unwrap();
        assert!(!proof_data.is_empty());
        release(handle);
        let new_handle = from_string(&proof_data).unwrap();
        let new_proof_data = to_string(new_handle).unwrap();
        assert_eq!(new_handle,handle);
        assert_eq!(new_proof_data,proof_data);
    }

    #[test]
    fn test_release_proof() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(Some("1".to_string()),
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(release(handle), 0);
        assert!(!is_valid_handle(handle));
    }

    #[test]
    fn test_send_proof_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let connection_handle = create_connection("test_send_proof_request".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"proofRequest\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = match create_proof(Some("1".to_string()),
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        thread::sleep(Duration::from_millis(500));
        assert_eq!(send_proof_request(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(500));
        assert_eq!(get_state(handle), CxsStateType::CxsStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "6a9u7Jt");
        _m.assert();
    }

}