extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use proof;
use connection;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: attributes/claims prover must provide in proof
///
/// # Example requested_attrs -> "[{"name":"attrName","restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// requested_predicates: predicate specifications prover must provide claim for
///
/// # Example requested_predicates -> "[{"name":"attrName","p_type":"GE","p_value":9,"restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
///
/// cb: Callback that provides proof handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_attrs, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_predicates, error::INVALID_OPTION.code_num);
    check_useful_c_str!(name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    info!("vcx_proof_create(command_handle: {}, source_id: {}, requested_attrs: {}, requested_predicates: {}, name: {})",
          command_handle, source_id, requested_attrs, requested_predicates, name);

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(source_id, requested_attrs, requested_predicates, name) {
            Ok(x) => {
                info!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, proof::get_source_id(x.to_error_code()).unwrap_or_default());
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change and updates the proof state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_update_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match proof::update_state(proof_handle) {
            Ok(x) => {
                info!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), proof_handle, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x.to_string(), proof_handle, 0, source_id);
                cb(command_handle, x.to_error_code(), 0);
            }
        }
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_proof_get_state(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match proof::get_state(proof_handle) {
            Ok(x) => {
                info!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), proof_handle, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x.to_string(), proof_handle, 0, source_id);
                cb(command_handle, x.to_error_code(), 0);
            }
        }
    });

    error::SUCCESS.code_num
}

/// Takes the proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides json string of the proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_serialize(command_handle: {}, proof_handle: {}), source_id: {:?}", command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match proof::to_string(proof_handle) {
            Ok(x) => {
                info!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_data: json string representing a proof object
///
/// cb: Callback that provides proof handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_data, error::INVALID_OPTION.code_num);

    info!("vcx_proof_deserialize(command_handle: {}, proof_data: {})",
          command_handle, proof_data);

    thread::spawn( move|| {
        let (rc, handle) = match proof::from_string(&proof_data) {
            Ok(x) => {
                info!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the proof object by de-allocating memory
///
/// #Params
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_proof_release(proof_handle: u32) -> u32 {
    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    match proof::release(proof_handle) {
        Ok(x) => info!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {:?}",
                       proof_handle, error_string(0), source_id),
        Err(e) => warn!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {:?}",
                       proof_handle, error_string(e.to_error_code()), source_id),
    };
    error::SUCCESS.code_num
}

/// Sends a proof request to pairwise connection
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: provides any error status of the proof_request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_proof_send_request(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, 0, proof_handle);
                x
            },
            Err(x) => {
                warn!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, x.to_error_code(), proof_handle);
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Get Proof
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to identify proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides Proof attributes and error status of sending the credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_get_proof(command_handle: u32,
                                        proof_handle: u32,
                                        connection_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state:u32, response_data: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_get_proof(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        //update the state to see if proof has come, ignore any errors
        match proof::update_state(proof_handle) {
            Ok(_) => (),
            Err(_) => (),
        };

        match proof::get_proof(proof_handle) {
            Ok(x) => {
                info!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, 0, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, proof::get_proof_state(proof_handle).unwrap(), msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, x.to_error_code(), "null");
                cb(command_handle, x.to_error_code(), proof::get_proof_state(proof_handle).unwrap(), ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}


#[allow(unused_variables)]
pub extern fn vcx_proof_accepted(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use proof::{ create_proof };
    use proof;
    use api::VcxStateType;
    use connection;
    use api::{ ProofStateType };
    use utils::constants::*;
    use utils::libindy::return_types_u32;

    static DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";

    extern "C" fn create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        println!("successfully called serialize_cb: {}", proof_string);
    }

    extern "C" fn get_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        assert!(proof_state > 1);
        println!("successfully called get_proof_cb: {}", proof_string);
    }

    extern "C" fn no_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, error::INVALID_PROOF_HANDLE.code_num);
        assert!(proof_string.is_null());
        assert_eq!(proof_state, ProofStateType::ProofUndefined as u32);
        println!("successfully called no_proof_cb: null");
    }

    extern "C" fn verify_invalid_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        assert_eq!(proof_state, ProofStateType::ProofInvalid as u32);
        println!("successfully called verify_invalid_proof_cb");
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_proof_serialize(0, proof_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"source_id":"source id","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","msg_uid":"","ref_msg_id":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","prover_vk":"","state":2,"proof_state":0,"name":"Name Data","version":"1.0","nonce":"123456","proof":null,"proof_request":null,"remote_did":"","remote_vk":"","agent_did":"","agent_vk":""}"#;
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(expected,new);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state, VcxStateType::VcxStateInitialized as u32);
    }


    extern "C" fn send_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof) {}",err)}
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_proof_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
                                    Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_create_proof_fails() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(
            0,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional data").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let original = r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null}"#;
        assert_eq!(vcx_proof_deserialize(cb.command_handle,
                                         CString::new(PROOF_OFFER_SENT).unwrap().into_raw(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_proof_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = vcx_proof_update_state(0, handle, Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_proof_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = match create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(proof::get_state(handle).unwrap(),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        assert_eq!(vcx_proof_send_request(0,handle,connection_handle,Some(send_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(proof::get_state(handle).unwrap(),VcxStateType::VcxStateOfferSent as u32);
    }

    #[test]
    fn test_get_proof_fails_when_not_ready_with_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()).unwrap();
        assert!(handle > 0);
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();

        thread::sleep(Duration::from_millis(300));
        let rc = vcx_get_proof(0, handle, connection_handle, Some(no_proof_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_get_proof_returns_proof_with_proof_state_invalid() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();
        thread::sleep(Duration::from_millis(300));
        let proof_handle = proof::from_string(PROOF_WITH_INVALID_STATE).unwrap();
        let rc = vcx_get_proof(0, proof_handle, connection_handle, Some(verify_invalid_proof_cb));
        thread::sleep(Duration::from_millis(900));
        assert_eq!(rc, 0);
        vcx_proof_release(proof_handle);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let handle = proof::from_string(PROOF_OFFER_SENT).unwrap();
        assert!(handle > 0);
        let rc = vcx_proof_get_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        let state = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert_eq!(state, VcxStateType::VcxStateOfferSent as u32);
    }
}
