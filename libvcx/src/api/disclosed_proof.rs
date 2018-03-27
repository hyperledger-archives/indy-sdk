extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use disclosed_proof;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/// Create a proof for fulfilling a corresponding proof request
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's identification for the proof, should be unique.
///
/// req: proof request received via "vcx_get_proof_requests"
///
/// # Example proof_request {"@topic":{"mid":9,"tid":1},"@type":{"name":"PROOF_REQUEST","version":"1.0"},"msg_ref_id":"ymy5nth","proof_request_data":{"name":"Account Certificate","nonce":"838186471541979035208225","requested_attrs":{"business_2":{"name":"business","schema_seq_no":52},"email_1":{"name":"email","schema_seq_no":52},"name_0":{"name":"name","schema_seq_no":52}},"requested_predicates":{},"version":"0.1"}}
///
/// cb: Callback that provides proof handle or error status
///
/// #Returns
/// Error code as u32

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_disclosed_proof_create_with_request(command_handle: u32,
                                                      source_id: *const c_char,
                                                      proof_req: *const c_char,
                                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_req, error::INVALID_OPTION.code_num);

    info!("vcx_disclosed_proof_create_with_request(command_handle: {}, source_id: {}, proof_req: {})",
          command_handle, source_id, proof_req);

    thread::spawn(move|| {
        match disclosed_proof::create_proof(source_id, proof_req){
            Ok(x) => {
                info!("vcx_disclosed_proof_create_with_request_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(0), x);
                cb(command_handle, 0, x);
            },
            Err(x) => {
                error!("vcx_disclosed_proof_create_with_request_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(x.to_error_code()), 0);
                cb(command_handle, x.to_error_code(), 0);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Send a proof to the connection, called after having received a proof request
///
/// #params
/// command_handle: command handle to map callback to API user context.
///
/// proof_handle: proof handle that was provided duration creation.  Used to identify proof object.
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of proof send request
///
/// #Returns
/// Error code as u32

#[no_mangle]
pub extern fn vcx_disclosed_proof_send_proof(command_handle: u32,
                                             proof_handle: u32,
                                             connection_handle: u32,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return error::INVALID_DISCLOSED_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_disclosed_proof_send_proof(command_handle: {}, proof_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, proof_handle, connection_handle, source_id);

    thread::spawn(move|| {
        let err = match disclosed_proof::send_proof(proof_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_disclosed_proof_send_proof_cb(command_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, error_string(0), source_id);
                cb(command_handle,x);
            },
            Err(x) => {
                error!("vcx_disclosed_proof_send_proof_cb(command_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), source_id);
                cb(command_handle,x.to_error_code());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Queries agency for proof requests from the given connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection to query for proof requests.
///
/// cb: Callback that provides any proof requests and error status of query
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_disclosed_proof_get_requests(command_handle: u32,
                                               connection_handle: u32,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, requests: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    info!("vcx_disclosed_proof_get_requests(command_handle: {}, connection_handle: {})",
          command_handle, connection_handle);

    thread::spawn(move|| {
        match disclosed_proof::get_proof_request_messages(connection_handle, None) {
            Ok(x) => {
                info!("vcx_disclosed_proof_get_requests_cb(command_handle: {}, rc: {}, msg: {})",
                      command_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_disclosed_proof_get_requests_cb(command_handle: {}, rc: {}, msg: {})",
                      command_handle, error_string(0), x);
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_disclosed_proof_get_state(command_handle: u32,
                                            proof_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return error::INVALID_DISCLOSED_PROOF_HANDLE.code_num;
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_disclosed_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    thread::spawn(move|| {
        match disclosed_proof::get_state(proof_handle) {
            Ok(s) => {
                info!("vcx_disclosed_proof_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_disclosed_proof_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_disclosed_proof_update_state(command_handle: u32,
                                               proof_handle: u32,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return error::INVALID_DISCLOSED_PROOF_HANDLE.code_num;
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_disclosed_proof_update_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    thread::spawn(move|| {
        match disclosed_proof::update_state(proof_handle) {
            Ok(s) => {
                info!("vcx_disclosed_proof_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_disclosed_proof_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        };
    });

    error::SUCCESS.code_num
}

/// Takes the disclosed proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Proof handle that was provided during creation. Used to identify the disclosed proof object
///
/// cb: Callback that provides json string of the disclosed proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_serialize(command_handle: u32,
                                            proof_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return error::INVALID_DISCLOSED_PROOF_HANDLE.code_num;
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_disclosed_proof_serialize(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    thread::spawn(move|| {
        match disclosed_proof::to_string(proof_handle) {
            Ok(x) => {
                info!("vcx_disclosed_proof_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                      command_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num,msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_disclosed_proof_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                       command_handle, error_string(x), 0, source_id);
                cb(command_handle,x,ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an disclosed proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// data: json string representing a disclosed proof object
///
///
/// cb: Callback that provides handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_deserialize(command_handle: u32,
                                              proof_data: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_data, error::INVALID_OPTION.code_num);

    info!("vcx_disclosed_proof_deserialize(command_handle: {}, proof_data: {})",
          command_handle, proof_data);

    thread::spawn(move|| {
        match disclosed_proof::from_string(&proof_data) {
            Ok(x) => {
                info!("vcx_disclosed_proof_deserialize_cb(command_handle: {}, rc: {}, proof_handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, disclosed_proof::get_source_id(x).unwrap_or_default());

                cb(command_handle, 0, x);
            },
            Err(x) => {
                error!("vcx_disclosed_proof_deserialize_cb(command_handle: {}, rc: {}, proof_handle: {}), source_id: {:?}",
                       command_handle, error_string(x.to_error_code()), 0, "");
                cb(command_handle, x.to_error_code(), 0);
            },
        };
    });

    error::SUCCESS.code_num
}


/// Releases the disclosed proof object by de-allocating memory
///
/// #Params
/// handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_release(handle: u32) -> u32 {
    let source_id = disclosed_proof::get_source_id(handle).unwrap_or_default();
    match disclosed_proof::release(handle) {
        Ok(_) => {
            info!("vcx_disclosed_proof_release(handle: {}, rc: {}), source_id: {:?}",
                  handle, error_string(0), source_id);
            error::SUCCESS.code_num
        },
        Err(e) => {
            error!("vcx_disclosed_proof_release(handle: {}, rc: {}), source_id: {:?}",
                   handle, error_string(e), source_id);
            e
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use settings;
    use connection;
    use api::VcxStateType;
    use utils::constants::{DEFAULT_SERIALIZED_PROOF};

    pub const BAD_PROOF_REQUEST: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    extern "C" fn create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn bad_create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, error::INVALID_JSON.code_num);
        assert_eq!(proof_handle, 0);
        println!("successfully called bad_create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        println!("successfully called serialize_cb: {}", proof_string);
    }

    #[test]
    fn test_vcx_proof_create_with_request_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_disclosed_proof_create_with_request(0,
                                               CString::new("test_create").unwrap().into_raw(),
                                               CString::new(::utils::constants::PROOF_REQUEST_JSON).unwrap().into_raw(),
                                               Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_create_with_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_disclosed_proof_create_with_request(
            0,
            CString::new("test_create").unwrap().into_raw(),
            CString::new(BAD_PROOF_REQUEST).unwrap().into_raw(),
            Some(bad_create_cb)),error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_disclosed_proof_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = disclosed_proof::create_proof("1".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        assert_eq!(vcx_disclosed_proof_serialize(0,
                                       handle,
                                       Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_proof_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof {}",err)}
    }

    #[test]
    fn test_vcx_send_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = disclosed_proof::create_proof("1".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        assert_eq!(disclosed_proof::get_state(handle).unwrap(),VcxStateType::VcxStateRequestReceived as u32);

        let connection_handle = connection::build_connection("test_send_proof").unwrap();

        assert_eq!(vcx_disclosed_proof_send_proof(0,handle,connection_handle,Some(send_proof_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
        println!("successfully called init_cb")
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called deserialize_cb");
        let original = formatter(DEFAULT_SERIALIZED_PROOF);
        let new = formatter(&disclosed_proof::to_string(proof_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_proof_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_PROOF;
        vcx_disclosed_proof_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_requests_cb(command_handle: u32, err:u32, requests: *const c_char) {
        assert_eq!(err,0);
        check_useful_c_str!(requests, ());
        assert!(requests.len() > 20);
    }

    #[test]
    fn test_vcx_proof_get_requests(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_get_new_requests").unwrap();
        ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec());
        assert_eq!(error::SUCCESS.code_num as u32, vcx_disclosed_proof_get_requests(0,
                                           cxn,
                                           Some(get_requests_cb)));
        thread::sleep(Duration::from_millis(300));
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb: {}", state);
    }

    #[test]
    fn test_vcx_proof_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = disclosed_proof::from_string(DEFAULT_SERIALIZED_PROOF).unwrap();
        assert!(handle > 0);
        let rc = vcx_disclosed_proof_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }
}
