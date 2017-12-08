extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::{ CxsStateType, ProofStateType };
use utils::error;
use settings;
use proof_offer::{ ProofOffer };
use messages;
use messages::GeneralMessage;
use messages::MessageResponseCode::{ MessageAccepted };
use connection;
use utils::callback::CallbackUtils;
use std::sync::mpsc::channel;
use self::libc::c_char;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;

lazy_static! {
    static ref PROOF_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

extern {
    fn indy_verifier_verify_proof(command_handle: i32,
                                  proof_request_json: *const c_char,
                                  proof_json: *const c_char,
                                  schemas_json: *const c_char,
                                  claim_defs_jsons: *const c_char,
                                  revoc_regs_json: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                      valid: bool)>) -> i32;

}

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    source_id: String,
    handle: u32,
    requested_attrs: String,
    requested_predicates: String,
    msg_uid: String,
    ref_msg_id: String,
    requester_did: String,
    prover_did: String,
    state: CxsStateType,
    proof_state: ProofStateType,
    tid: u32,
    mid: u32,
    name: String,
    version: String,
    nonce: String,
    proof_offer: Option<ProofOffer>,
}

impl Proof {
    fn validate_proof_request(&self) -> Result<u32, u32> {
        //TODO: validate proof request
        info!("successfully validated proof {}", self.handle);
        Ok(error::SUCCESS.code_num)
    }
    
    fn validate_proof_against_request(&self) -> Result<u32, u32> {
        Ok(error::SUCCESS.code_num)
    }
    
    fn indy_validate_proof(&mut self) -> Result<u32, u32> {
// proof_request_json: initial proof request as sent by the verifier
//     {
//         "nonce": string,
//         "requested_attr1_uuid": <attr_info>,
//         "requested_attr2_uuid": <attr_info>,
//         "requested_attr3_uuid": <attr_info>,
//         "requested_predicate_1_uuid": <predicate_info>,
//         "requested_predicate_2_uuid": <predicate_info>,
//     }
// proof_json: proof json
// For each requested attribute either a proof (with optionally revealed attribute value) or
// self-attested attribute value is provided.
// Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
// There ais also aggregated proof part common for all claim proofs.
//     {
//         "requested": {
//             "requested_attr1_id": [claim_proof1_uuid, revealed_attr1, revealed_attr1_as_int],
//             "requested_attr2_id": [self_attested_attribute],
//             "requested_attr3_id": [claim_proof2_uuid]
//             "requested_attr4_id": [claim_proof2_uuid, revealed_attr4, revealed_attr4_as_int],
//             "requested_predicate_1_uuid": [claim_proof2_uuid],
//             "requested_predicate_2_uuid": [claim_proof3_uuid],
//         }
//         "claim_proofs": {
//             "claim_proof1_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
//             "claim_proof2_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no],
//             "claim_proof3_uuid": [<claim_proof>, issuer_did, schema_seq_no, revoc_reg_seq_no]
//         },
//         "aggregated_proof": <aggregated_proof>
//     }
// schemas_jsons: all schema jsons participating in the proof
//         {
//             "claim_proof1_uuid": <schema>,
//             "claim_proof2_uuid": <schema>,
//             "claim_proof3_uuid": <schema>
//         }
// claim_defs_jsons: all claim definition jsons participating in the proof
//         {
//             "claim_proof1_uuid": <claim_def>,
//             "claim_proof2_uuid": <claim_def>,
//             "claim_proof3_uuid": <claim_def>
//         }
// revoc_regs_jsons: all revocation registry jsons participating in the proof
//         {
//             "claim_proof1_uuid": <revoc_reg>,
//             "claim_proof2_uuid": <revoc_reg>,
//             "claim_proof3_uuid": <revoc_reg>
//         }
//pub extern fn indy_verifier_verify_proof(command_handle: i32,
//                                         proof_request_json: *const c_char,
//                                         proof_json: *const c_char,
//                                         schemas_json: *const c_char,
//                                         claim_defs_jsons: *const c_char,
//                                         revoc_regs_json: *const c_char,
//                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
//                                                              valid: bool)>) -> ErrorCode {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, valid | {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verifier_verify_proof_cb(cb);
        let proof_request_json = "";
        let proof_json = "";
        let schemas_json = "";
        let claim_defs_jsons =  "";
        let revoc_regs_json = "";

        unsafe {
            let indy_err = indy_verifier_verify_proof(command_handle,
                                                      CString::new(proof_request_json).unwrap().as_ptr(),
                                                      CString::new(proof_json).unwrap().as_ptr(),
                                                      CString::new(schemas_json).unwrap().as_ptr(),
                                                      CString::new(claim_defs_jsons).unwrap().as_ptr(),
                                                      CString::new(revoc_regs_json).unwrap().as_ptr(),
                                                      cb);
            if indy_err != 0 {
                return Err(self.set_invalid_proof_state(indy_err))
            }
        }

        let (err, valid) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != 0 || !valid {
            return Err(self.set_invalid_proof_state(err))
        }
        info!("Indy validated Proof Offer: {:?}", self.handle);
        self.proof_state = ProofStateType::ProofValidated;
        Ok(error::SUCCESS.code_num)
    }

    fn set_invalid_proof_state(&mut self, error:i32) -> u32 {
        error!("Error: {}, Proof offer wasn't valid {}", error, self.handle);
        self.proof_state = ProofStateType::ProofInvalid;
        error::UNKNOWN_ERROR.code_num
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateInitialized {
            warn!("proof {} has invalid state {} for sending proofRequest", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }
        self.prover_did = connection::get_pw_did(connection_handle)?;
        self.requester_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT).unwrap();
        let data_version = ".1";
        let proof_request = messages::proof_request()
            .type_version(&self.version)
            .prover_did(&self.prover_did)
            .requester_did(&self.requester_did)
            .tid(1)
            .mid(9)
            .nonce(&self.nonce)
            .proof_name(&self.name)
            .proof_data_version(data_version)
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

    fn get_proof_offer(&self) -> Result<String, u32> {
        let proof_offer = match self.proof_offer {
            Some(ref x) => x,
            None => return Err(error::INVALID_PROOF_OFFER.code_num),
        };
        proof_offer.get_attrs()
    }
    fn build_proof_offer(&mut self, msg_uid: &str) {
        info!("Checking for outstanding proofOffer for {} with uid: {}", self.handle, msg_uid);
        let msgs = match get_matching_messages(msg_uid, &self.prover_did) {
            Ok(x) => x,
            Err(err) => {
                warn!("{} {}", err, self.handle);
                return
            }
        };

        for msg in msgs {
            self.state = CxsStateType::CxsStateRequestReceived;
            // Todo: Parse proof values 
            // Todo: check/compare against request
            // Todo: Validate proof with lib-indy
            // Todo: build proof offer object and set it in proof
        }
        return
    }

    fn get_proof_request_status(&mut self) {
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        else if self.state != CxsStateType::CxsStateOfferSent || self.msg_uid.is_empty() || self.prover_did.is_empty() {
            return;
        }

        // State is proof request sent
        let msgs = match get_matching_messages(&self.msg_uid, &self.prover_did) {
            Ok(x) => x,
            Err(err) => {
                warn!("{} {}", err, self.handle);
                return
            }
        };

        for msg in msgs {
            if msg["statusCode"] == serde_json::to_value(MessageAccepted.as_str())
                .unwrap_or(serde_json::Value::Null){
                let ref_msg_id = match msg["refMsgId"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("invalid message reference id for proof {}", self.handle);
                        return
                    }
                };
                self.ref_msg_id = ref_msg_id.to_owned();
                self.build_proof_offer(ref_msg_id);
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
                    requested_attrs: String,
                    requested_predicates: String,
                    name: String) -> Result<u32, u32> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        ref_msg_id: String::new(),
        requested_attrs,
        requested_predicates,
        requester_did: String::new(),
        prover_did: String::new(),
        state: CxsStateType::CxsStateNone,
        proof_state: ProofStateType::ProofUndefined,
        tid: 0,
        mid: 0,
        name,
        version: String::from("1.0"),
        nonce: generate_nonce().to_string(),
        proof_offer: None,
    });

    new_proof.validate_proof_request()?;

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
        Some(c) => Ok(c.send_proof_request(connection_handle)?),
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

fn get_matching_messages<'a>(msg_uid:&'a str, to_did: &'a str) -> Result<Vec<serde_json::Value>, &'a str> {
    let response = match messages::get_messages().to(to_did).uid(msg_uid).send() {
            Ok(x) => x,
        Err(x) => {
            return Err("invalid response to get_messages for proof")
        },
    };

    let json: serde_json::Value = match serde_json::from_str(&response) {
        Ok(json) => json,
        Err(_) => {
            return Err("invalid json in get_messages for proof")


        },
    };

    match json["msgs"].as_array() {
        Some(array) => Ok(array.to_owned()),
        None => {
            Err("invalid msgs array returned for proof")
        },
    }
}

pub fn get_proof_offer(handle: u32) -> Result<String,u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(proof) => Ok(proof.get_proof_offer()?),
        None => Err(error::INVALID_PROOF_OFFER.code_num),
    }
}

pub fn generate_nonce() -> u32 {
    rand::random()
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate mockito;
    use std::thread;
    use std::time::Duration;
    use connection::create_connection;

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
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
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let connection_handle = create_connection("test_send_proof_request".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"proofRequest\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = match create_proof(Some("1".to_string()),
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

    
    #[test]
    fn test_send_proof_request_fails_with_no_pw() {
        //This test has 2 purposes:
        //1. when send_proof_request fails, Ok(c.send_proof_request(connection_handle)?) returns error instead of Ok(_)
        //2. Test that when no PW connection exists, send message fails on invalid did
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        //This t
        let connection_handle = create_connection("test_send_proof_request".to_owned());

        let handle = match create_proof(Some("1".to_string()),
                                            REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        thread::sleep(Duration::from_millis(500));
        match send_proof_request(handle, connection_handle) {
            Ok(x) => panic!("Should have failed in send_proof_request"),
            Err(y) => assert_eq!(y, error::INVALID_DID.code_num)
        }
    }

    #[test]
    fn test_get_proof_offer_fails_with_no_proof_offer() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(Some("1".to_string()),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(is_valid_handle(handle));

        match get_proof_offer(handle) {
            Ok(x) => {
                warn!("Should have failed with no proof");
                assert_eq!(0, 1)
            },
            Err(x) => assert_eq!(x, error::INVALID_PROOF_OFFER.code_num),

        }
    }
}
