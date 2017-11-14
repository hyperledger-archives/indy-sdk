extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use settings;

lazy_static! {
    static ref PROOF_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    source_id: String,
    handle: u32,
    proof_attributes: String,
    msg_uid: String,
    proof_requester_did: String,
    proover_did: String,
    state: CxsStateType,
}

impl Proof {
    fn validate_proof_request(&self) -> Result<u32, String> {
        //TODO: validate proof request
        Ok(error::SUCCESS.code_num)
    }

    fn get_proof_request_state(&mut self) {
        return
    }

    fn get_state(&self) -> u32 {let state = self.state as u32; state}
}

fn find_proof(source_id: &str) -> Result<u32,u32> {
    for (handle, proof) in PROOF_MAP.lock().unwrap().iter() { //TODO this could be very slow with lots of objects
        if proof.source_id == source_id {
            return Ok(*handle);
        }
    };

    Err(0)
}

pub fn create_proof(source_id: Option<String>,
                    requester_did: String,
                    proof_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        proof_attributes: proof_data,
        proof_requester_did: requester_did,
        proover_did: String::new(),
        state: CxsStateType::CxsStateNone,
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

fn get_offer_details(response: &str) -> Result<String, u32> {
    if settings::test_mode_enabled() {return Ok("test_mode_response".to_owned());}
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["uid"];
            Ok(detail.to_string())
        },
        Err(_) => {
            info!("Connect called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate mockito;

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
                           "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1),
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = match create_proof(None,
                           "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                           "{\"attr\":\"value\"}".to_owned()) {
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
                                        "{\"attr\":\"value\"}".to_owned()) {
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
                                        "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(release(handle), 0);
        assert!(!is_valid_handle(handle));
    }
}