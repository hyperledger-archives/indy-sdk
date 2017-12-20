extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use proof;
use connection;
use std::thread;
use std::ptr;
use api::CxsStatus;
use api::{ CxsStateType };

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: attributes in json format prover is expected to include in proof.
///
/// # Example requested_attrs -> "[{"name":"attrName","issuer_did":"did","schema_seq_no":1}]"
///
/// requested_predicates: specific requirements regarding the prover's attributes.
///
/// # Example requested_predicates -> "[{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":1,"issuer_did":"DID"}]"
///
/// name: Name of the proof request - ex. Drivers Licence.
///
/// cb: Callback that provides proof handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn cxs_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_attrs, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_predicates, error::INVALID_OPTION.code_num);
    check_useful_c_str!(name, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(
            source_id_opt, requested_attrs, requested_predicates, name) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(command_handle: u32,
                                       proof_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

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
#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        proof::update_state(proof_handle);

        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
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
pub extern fn cxs_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match proof::to_string(proof_handle) {
            Ok(x) => {
                info!("serializing proof handle: {} with data: {}", proof_handle, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("could not serialize proof handle {}", proof_handle);
                cb(command_handle, x, ptr::null_mut());
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
/// # Examples proof_data -> {"source_id":"id","handle":1,"requested_attrs":"[{\"issuerDid\":\"did\",\"schemaSeqNo\":1,\"name\":\"\"}]","requested_predicates":"[]","msg_uid":"","requester_did":"","prover_did":"","state":1,"name":"Proof Name"}
///
/// cb: Callback that provides proof handle and provides error status
///
/// #Returns
/// Error code as a u32
#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_data, error::INVALID_OPTION.code_num);

    thread::spawn( move|| {
        let (rc, handle) = match proof::from_string(&proof_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(x) => (x, 0),
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
/// Error code as a u32
#[no_mangle]
pub extern fn cxs_proof_release(proof_handle: u32) -> u32 {
    proof::release(proof_handle)
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
pub extern fn cxs_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => x,
            Err(x) => x,
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
/// cb: Callback that provides Proof attributes and error status of sending the claim
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn cxs_get_proof(command_handle: u32,
                                        proof_handle: u32,
                                        connection_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, response_data: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    //update the state to see if proof has come
    proof::update_state(proof_handle);

    if proof::get_state(proof_handle) != CxsStateType::CxsStateAccepted as u32 {
        info!("No proof available for: {}", proof_handle);
        return error::NOT_READY.code_num;
    }

    thread::spawn(move|| {
        match proof::get_proof(proof_handle) {
            Ok(x) => {
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("could not process proof {}", proof_handle);
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}


#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }


#[cfg(test)]
mod tests {
    extern crate mockito;

    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use proof::{ create_proof };
    use proof;
    use api::CxsStateType;
    use connection;


    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";

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

    extern "C" fn get_proof_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        println!("successfully called get_proof_cb: {}", proof_string);
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_proof_serialize(0, proof_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called deserialize_cb");
        let original = "{\"source_id\":\"source id\",\"handle\":1,\"requested_attrs\":\"{\\\"attrs\\\":[{\\\"name\\\":\\\"person name\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"address_1\\\"},{\\\"schema_seq_no\\\":2,\\\"issuer_did\\\":\\\"ISSUER_DID2\\\",\\\"name\\\":\\\"address_2\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"city\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"state\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"zip\\\"}]}\",\"requested_predicates\":\"{\\\"attr_name\\\":\\\"age\\\",\\\"p_type\\\":\\\"GE\\\",\\\"value\\\":18,\\\"schema_seq_no\\\":1,\\\"issuer_did\\\":\\\"DID1\\\"}\",\"msg_uid\":\"\",\"ref_msg_id\":\"\",\"requester_did\":\"234\",\"prover_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"state\":1,\"proof_state\":0,\"name\":\"Name Data\",\"version\":\"1.0\",\"nonce\":\"123456\",\"proof\":null,\"proof_request\":null}";
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(original,new);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
    }


    extern "C" fn send_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof) {}",err)}
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_cxs_create_proof_success() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_proof_create(0,
                                    ptr::null(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
                                    Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_create_proof_fails() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_proof_create(
            0,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_proof_create(0,
                                    ptr::null(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional data").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"nonce\":\"123456\",\"version\":\"1.0\",\"handle\":1,\"msg_uid\":\"\",\"ref_msg_id\":\"\",\"name\":\"Name Data\",\"prover_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"requested_attrs\":\"{\\\"attrs\\\":[{\\\"name\\\":\\\"person name\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"address_1\\\"},{\\\"schema_seq_no\\\":2,\\\"issuer_did\\\":\\\"ISSUER_DID2\\\",\\\"name\\\":\\\"address_2\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"city\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"state\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"zip\\\"}]}\",\"requested_predicates\":\"{\\\"attr_name\\\":\\\"age\\\",\\\"p_type\\\":\\\"GE\\\",\\\"value\\\":18,\\\"schema_seq_no\\\":1,\\\"issuer_did\\\":\\\"DID1\\\"}\",\"requester_did\":\"234\",\"source_id\":\"source id\",\"state\":1,\"proof_state\":0,\"proof\":null,\"proof_request\":null}";
        cxs_proof_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_proof_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = cxs_proof_update_state(0, handle, Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_cxs_proof_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"proofRequest\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(proof::get_state(handle),CxsStateType::CxsStateInitialized as u32);

        let connection_handle = connection::create_connection("test_send_proof_request".to_owned());
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");
        assert_eq!(cxs_proof_send_request(0,handle,connection_handle,Some(send_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(proof::get_state(handle),CxsStateType::CxsStateOfferSent as u32);
        _m.assert();
    }

    #[test]
    fn test_get_proof_fails_when_not_ready_with_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        let connection_handle = connection::create_connection("test_send_proof_request".to_owned());
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");

        thread::sleep(Duration::from_millis(300));
        let rc = cxs_get_proof(0, handle, connection_handle, Some(get_proof_cb));
        assert_eq!(rc, error::NOT_READY.code_num);
        thread::sleep(Duration::from_millis(300));
    }

}
