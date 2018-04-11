extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use settings;
use issuer_credential;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/**
 * credential object
 */

/// Create a Issuer Credential object that provides a credential for an enterprise's user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_seq_no: integer number corresponding to credential's schema number on the ledger
///
/// issuer_did: did corresponding to entity issuing a credential. Needs to have Trust Anchor permissions on ledger
///
/// credential_data: data attributes offered to person in the credential
///
/// # Example credential_data -> "{"state":["UT"]}"
///
/// credential_name: Name of the credential - ex. Drivers Licence
///
/// cb: Callback that provides credential handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_create_credential(command_handle: u32,
                                      source_id: *const c_char,
                                      schema_seq_no: u32,
                                      issuer_did: *const c_char,
                                      credential_data: *const c_char,
                                      credential_name: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_data, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    let issuer_did: String = if !issuer_did.is_null() {
        check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);
        issuer_did.to_owned()
    } else {
        match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
            Ok(x) => x,
            Err(x) => return x
        }
    };

    info!("vcx_issuer_create_credential(command_handle: {}, source_id: {}, schema_seq_no: {}, issuer_did: {}, credential_data: {}, credential_name: {})",
          command_handle,
          source_id,
          schema_seq_no,
          issuer_did,
          credential_data,
          credential_name);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_credential::issuer_credential_create(schema_seq_no, source_id, issuer_did, credential_name, credential_data) {
            Ok(x) => {
                info!("vcx_issuer_create_credential_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, issuer_credential::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_create_credential_cb(command_handle: {}, rc: {}, handle: {}, source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Send a credential offer to user showing what will be included in the actual credential
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of credential offer
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_credential_offer(command_handle: u32,
                                          credential_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_send_credential(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, credential_handle, connection_handle, source_id);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match issuer_credential::send_credential_offer(credential_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, credential_handle, error_string(x), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {}, source_id: {:?})",
                      command_handle, credential_handle, error_string(x.to_error_code()), source_id);
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the credential and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_update_state(command_handle: u32,
                                            credential_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_update_state(command_handle: {}, credential_handle: {})",
          command_handle, credential_handle);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    thread::spawn(move|| {
        issuer_credential::update_state(credential_handle);

        info!("vcx_issuer_credential_update_state_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
              command_handle, credential_handle, error_string(0), issuer_credential::get_state(credential_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, issuer_credential::get_state(credential_handle));
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_issuer_credential_get_state(command_handle: u32,
                                         credential_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_get_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, credential_handle, source_id);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_issuer_credential_get_state_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
              command_handle, credential_handle, error_string(0), issuer_credential::get_state(credential_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, issuer_credential::get_state(credential_handle));
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_get_credential_request(credential_handle: u32, credential_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_accept_credential(credential_handle: u32) -> u32 { error::SUCCESS.code_num }

/// Send Credential that was requested by user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of sending the credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_credential(command_handle: u32,
                                    credential_handle: u32,
                                    connection_handle: u32,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_send_credential(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, credential_handle, connection_handle, source_id);
    thread::spawn(move|| {
        let err = match issuer_credential::send_credential(credential_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {})",
                      command_handle, credential_handle, error_string(x));
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {})",
                      command_handle, credential_handle, error_string(x.to_error_code()));
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables)]
pub extern fn vcx_issuer_terminate_credential(credential_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }

/// Takes the credential object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides json string of the credential's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_serialize(command_handle: u32,
                                         credential_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_serialize(credential_serialize(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, credential_handle, source_id);
    thread::spawn(move|| {
        match issuer_credential::to_string(credential_handle) {
            Ok(x) => {
                info!("vcx_issuer_credential_serialize_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, credential_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num,msg.as_ptr());
            },
            Err(x) => {
                info!("vcx_issuer_credential_serialize_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}, source_id: {:?})",
                      command_handle, credential_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle,x.to_error_code(),ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an issuer credential object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_data: json string representing a credential object
///
/// # Examples credential_data -> {"source_id":"1","handle":2,"credential_attributes":"{\"state\":[\"UT\"]}","msg_uid":"","schema_seq_no":1234,"issuer_did":"DID","issued_did":"","state":1,"credential_request":"","credential_name":"Credential","credential_id":"123","ref_msg_id":""}
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_deserialize(command_handle: u32,
                                      credential_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_data, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_credential_deserialize(command_handle: {}, credential_data: {})", command_handle, credential_data);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_credential::from_string(&credential_data) {
            Ok(x) => {
                info!("vcx_issuer_credential_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, issuer_credential::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_credential_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, issuer_credential::get_source_id(x));
                (x, 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the issuer credential object by deallocating memory
///
/// #Params
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_release(credential_handle: u32) -> u32 {
    info!("(vcx_issuer_credential_release credential_handle: {}, source_id: {:?})", credential_handle, issuer_credential::get_source_id(credential_handle).unwrap_or_default());
    match issuer_credential::release(credential_handle) {
        Ok(x) => x,
        Err(e) => e.to_error_code(),
    }
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;
    use settings;
    use connection;
    use api::VcxStateType;
    use utils::constants::{DEFAULT_SERIALIZED_ISSUER_CREDENTIAL, CREDENTIAL_REQ_STRING};
    use credential_request::CredentialRequest;
    use error::issuer_cred::IssuerCredError;

    static DEFAULT_CREDENTIAL_NAME: &str = "Credential Name Default";
    static DEFAULT_DID: &str = "8XFh8yBzrpJQmNyZzgoTqB";
    static DEFAULT_ATTR: &str = "{\"attr\":\"value\"}";
    static DEFAULT_SCHEMA_SEQ_NO: u32 = 32;
    static ISSUER_CREDENTIAL_STATE_ACCEPTED: &str = r#"{"credential_id":"a credential id","credential_name":"credential name","source_id":"test_vcx_issuer_send_credential","handle":123,"credential_attributes":"{\"state\":[\"UT\"],\"zip\":[\"84000\"],\"city\":[\"Draper\"],\"address2\":[\"Suite 3\"],\"address1\":[\"123 Main St\"]}","msg_uid":"","schema_seq_no":32,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","issued_did":"VsKV7grR1BUE29mG2Fm2kX","issued_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","remote_did":"VsKV7grR1BUE29mG2Fm2kX","remote_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","agent_did":"VsKV7grR1BUE29mG2Fm2kX","agent_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","state":3,"ref_msg_id":"abc123"}"#;
    extern "C" fn create_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, credential_string: *const c_char) {
        assert_eq!(err, 0);
        if credential_string.is_null() {
            panic!("credential_string is null");
        }
        check_useful_c_str!(credential_string, ());
        println!("successfully called serialize_cb: {}", credential_string);
    }

    #[test]
    fn test_vcx_issuer_create_credential_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           32,
                                           ptr::null(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_create_credential_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(
            0,
            CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
            32,
            ptr::null(),
            ptr::null(),
            CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
            Some(create_cb)),error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_issuer_credential_serialize(0,credential_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_credential_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           DEFAULT_SCHEMA_SEQ_NO,
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send credential(offer) {}",err)}
    }

    #[test]
    fn test_vcx_issuer_send_credential_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = issuer_credential::from_string(DEFAULT_SERIALIZED_ISSUER_CREDENTIAL).unwrap();
        assert_eq!(issuer_credential::get_state(handle),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_credential_offer").unwrap();

        assert_eq!(vcx_issuer_send_credential_offer(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    #[test]
    fn test_vcx_issuer_send_a_credential() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);

        let test_name = "test_vcx_issuer_send_a_credential";

        let handle = issuer_credential::from_string(ISSUER_CREDENTIAL_STATE_ACCEPTED).unwrap();

        /* align credential request and credential def ***********************************/
        let mut credential_request = match CredentialRequest::from_str(CREDENTIAL_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with credential request"),
        };
        // set credential request to have the same did as enterprise did (and sam as credential def)
        credential_request.issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).clone().unwrap();
        credential_request.schema_seq_no = 15;
        issuer_credential::set_credential_request(handle, credential_request).unwrap();
        assert_eq!(issuer_credential::get_state(handle),VcxStateType::VcxStateRequestReceived as u32);
        /**********************************************************************/

        // create connection
        let connection_handle = connection::build_connection("test_send_credential").unwrap();

        // send the credential
        assert_eq!(vcx_issuer_send_credential(0, handle, connection_handle, Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }
    extern "C" fn deserialize_cb(command_handle: u32, err: u32, credential_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called deserialize_cb");
        let serialized_issuer_credential = r#"{"source_id":"test_credential_serialize","credential_attributes":"{\"attr\":\"value\"}","msg_uid":"","schema_seq_no":32,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","state":1,"credential_request":null,"credential_name":"credential name","credential_id":"1737199584","ref_msg_id":"abc123","agent_did":"","agent_vk":"","issued_did":"","issued_vk":"","remote_did":"","remote_vk":""}"#;
        let original = formatter(&serialized_issuer_credential);
        let new = formatter(&issuer_credential::to_string(credential_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_issuer_credential_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_ISSUER_CREDENTIAL;
        vcx_issuer_credential_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_create_credential_arguments_correct(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           DEFAULT_SCHEMA_SEQ_NO,
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_issuer_credential_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = issuer_credential::from_string(DEFAULT_SERIALIZED_ISSUER_CREDENTIAL).unwrap();
        assert!(handle > 0);
        let rc = vcx_issuer_credential_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_errors(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let credential_request = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let invalid_handle = 1234388;
        assert_eq!(issuer_credential::set_credential_request(invalid_handle, credential_request), Err(IssuerCredError::InvalidHandle()));
    }

}
