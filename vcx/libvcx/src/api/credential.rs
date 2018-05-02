extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use credential;
use std::thread;
use std::ptr;

use error::ToErrorCode;

/// Create a Credential object that requests and receives a credential for an institution
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the credential, should be unique.
///
/// offer: credential offer received via "vcx_get_credential_offers"
///
/// # Example offer -> "[{"msg_type": "CREDENTIAL_OFFER","version": "0.1","to_did": "...","from_did":"...","credential": {"account_num": ["...."],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "...","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}]
///
/// cb: Callback that provides credential handle or error status
///
/// #Returns
/// Error code as a u32

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credential_create_with_offer(command_handle: u32,
                                          source_id: *const c_char,
                                          offer: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(offer, error::INVALID_OPTION.code_num);

    info!("vcx_credential_create_with_offer(command_handle: {}, source_id: {}, offer: {})",
          command_handle, source_id, offer);

    thread::spawn(move|| {
        match credential::credential_create_with_offer(&source_id, &offer) {
            Ok(x) => {
                info!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                      command_handle, source_id, error_string(0), x);
                cb(command_handle, error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                      command_handle, source_id, error_string(x), 0);
                cb(command_handle, x, 0);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Create a Credential object that requests and receives a credential for an institution
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the credential, should be unique.
///
/// connection: connection to query for credential offer
///
/// msg_id: msg_id that contains the credential offer
///
/// cb: Callback that provides credential handle or error status
///
/// #Returns
/// Error code as a u32

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credential_create_with_msgid(command_handle: u32,
                                    source_id: *const c_char,
                                    connection_handle: u32,
                                    msg_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(msg_id, error::INVALID_OPTION.code_num);

    info!("vcx_credential_create_with_msgid(command_handle: {}, source_id: {}, connection_handle: {}, msg_id: {})",
          command_handle, source_id, connection_handle, msg_id);

    thread::spawn(move|| {
        match credential::get_credential_offer(connection_handle, &msg_id) {
            Ok(offer) => {
                match credential::credential_create_with_offer(&source_id, &offer) {
                    Ok(handle) => {
                        info!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                              command_handle, source_id, error_string(0), handle);
                        cb(command_handle, error::SUCCESS.code_num, handle)
                    },
                    Err(e) => {
                        warn!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                              command_handle, source_id, error_string(e), 0);
                        cb(command_handle, e, 0);
                    },
                };
            },
            Err(e) => cb(command_handle, e.to_error_code(), 0),
        };
    });

    error::SUCCESS.code_num
}

/// Send a credential request to the connection, called after having received a credential offer
///
/// #params
/// command_handle: command handle to map callback to user context
///
/// credential_handle: credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of credential request
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_credential_send_request(command_handle: u32,
                                          credential_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !credential::is_valid_handle(credential_handle) {
        return error::INVALID_CREDENTIAL_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let source_id = credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_credential_send_request(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, credential_handle, connection_handle, source_id);

    thread::spawn(move|| {
        match credential::send_credential_request(credential_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_credential_send_request_cb(command_handle: {}, rc: {}, source_id: {:?})",
                      command_handle, x.to_string(), source_id);
                cb(command_handle,x);
            },
            Err(e) => {
                warn!("vcx_credential_send_request_cb(command_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, e.to_string(), source_id);
                cb(command_handle,e.to_error_code());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Queries agency for credential offers from the given connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection to query for credential offers.
///
/// cb: Callback that provides any credential offers and error status of query
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_credential_get_offers(command_handle: u32,
                                   connection_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_offers: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    info!("vcx_credential_get_offers(command_handle: {}, connection_handle: {})",
          command_handle, connection_handle);

    thread::spawn(move|| {
        match credential::get_credential_offer_messages(connection_handle, None) {
            Ok(x) => {
                info!("vcx_credential_get_offers_cb(command_handle: {}, rc: {}, msg: {})",
                      command_handle, x.to_string(), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_credential_get_offers_cb(command_handle: {}, rc: {}, msg: null)",
                      command_handle, x.to_string());
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the credential and updates the the state attribute.  If it detects a credential it
/// will store the credential in the wallet and update the state.
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
pub extern fn vcx_credential_update_state(command_handle: u32,
                                            credential_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !credential::is_valid_handle(credential_handle) {
        return error::INVALID_CREDENTIAL_HANDLE.code_num;
    }

    let source_id = credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_credential_update_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, credential_handle, source_id);

    thread::spawn(move|| {
        match credential::update_state(credential_handle) {
            Ok(_) => (),
            Err(e) => {
                error!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        }

        let state = match credential::get_state(credential_handle) {
            Ok(s) => {
                info!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e.to_error_code()), 0, source_id);
                cb(command_handle, e.to_error_code(), 0)
            }
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_credential_get_state(command_handle: u32,
                                  handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !credential::is_valid_handle(handle) {
        return error::INVALID_CREDENTIAL_HANDLE.code_num;
    }

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    info!("vcx_credential_get_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, handle, source_id);

    thread::spawn(move|| {
        match credential::get_state(handle) {
            Ok(s) => {
                info!("vcx_credential_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_credential_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e.to_error_code()), 0, source_id);
                cb(command_handle, e.to_error_code(), 0)
            }
        };
    });

    error::SUCCESS.code_num
}


/// Takes the credential object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides json string of the credential's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_serialize(command_handle: u32,
                                         handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !credential::is_valid_handle(handle) {
        return error::INVALID_CREDENTIAL_HANDLE.code_num;
    }

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    info!("vcx_credential_serialize(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, handle, source_id);

    thread::spawn(move|| {
        match credential::to_string(handle) {
            Ok(x) => {
                info!("vcx_credential_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                    command_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_credential_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                    command_handle, error_string(x), 0, source_id);
                cb(command_handle,x,ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an credential object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_data: json string representing a credential object
///
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_deserialize(command_handle: u32,
                                           credential_data: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_data, error::INVALID_OPTION.code_num);

    info!("vcx_credential_deserialize(command_handle: {}, credential_data: {})",
          command_handle, credential_data);

    thread::spawn(move|| {
        match credential::from_string(&credential_data) {
            Ok(x) => {
                info!("vcx_credential_deserialize_cb(command_handle: {}, rc: {}, credential_handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, credential::get_source_id(x).unwrap_or_default());

                cb(command_handle, 0, x);
            },
            Err(x) => {
                error!("vcx_credential_deserialize_cb(command_handle: {}, rc: {}, credential_handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, "");
                cb(command_handle, x, 0);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Releases the credential object by de-allocating memory
///
/// #Params
/// handle: Proof handle that was provided during creation. Used to access credential object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_credential_release(handle: u32) -> u32 {
    let source_id = credential::get_source_id(handle).unwrap_or_default();
    match credential::release(handle) {
        Ok(_) => info!("vcx_credential_release(handle: {}, rc: {}), source_id: {:?}",
                       handle, error_string(0), source_id),
        Err(e) => error!("vcx_credential_release(handle: {}, rc: {}), source_id: {:?}",
                         handle, error_string(e.to_error_code()), source_id),
    };
    error::SUCCESS.code_num
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
    use utils::constants::{DEFAULT_SERIALIZED_CREDENTIAL};

    pub const BAD_CREDENTIAL_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","credential": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    extern "C" fn create_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn bad_create_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert!(err > 0);
        assert_eq!(credential_handle, 0);
        println!("successfully called bad_create_cb")
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
    fn test_vcx_credential_create_with_offer_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_credential_create_with_offer(0,
                                               CString::new("test_create").unwrap().into_raw(),
                                               CString::new(::utils::constants::CREDENTIAL_OFFER_JSON).unwrap().into_raw(),
                                               Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credential_create_with_offer_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_credential_create_with_offer(
            0,
            CString::new("test_create").unwrap().into_raw(),
            CString::new(BAD_CREDENTIAL_OFFER).unwrap().into_raw(),
            Some(bad_create_cb)),error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credential_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = credential::credential_create_with_offer("test_vcx_credential_serialize",::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert_eq!(vcx_credential_serialize(0,
                                       handle,
                                       Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send credential(offer) {}",err)}
    }

    #[test]
    fn test_vcx_credential_send_request() {
        use utils;
        utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = credential::credential_create_with_offer("test_send_request",::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert_eq!(credential::get_state(handle).unwrap(),VcxStateType::VcxStateRequestReceived as u32);

        let connection_handle = connection::build_connection("test_send_credential_offer").unwrap();

        assert_eq!(vcx_credential_send_request(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
        println!("successfully called init_cb")
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, credential_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called deserialize_cb");
        let original = formatter(DEFAULT_SERIALIZED_CREDENTIAL);
        let new = formatter(&credential::to_string(credential_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_credential_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_CREDENTIAL;
        vcx_credential_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_offers_cb(command_handle: u32, err:u32, offers: *const c_char) {
        assert_eq!(err,0);
        check_useful_c_str!(offers, ());
        println!("successfully called get_offers_cb: {:?}", offers);
    }

    #[test]
    fn test_vcx_credential_get_new_offers(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_get_new_offers").unwrap();
        assert_eq!(error::SUCCESS.code_num as u32, vcx_credential_get_offers(0,
                                           cxn,
                                           Some(get_offers_cb)));
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_credential_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_vcx_credential_create").unwrap();
        assert_eq!(vcx_credential_create_with_msgid(0,
                                         CString::new("test_vcx_credential_create").unwrap().into_raw(),
                                         cxn,
                                         CString::new("123").unwrap().into_raw(),
                                         Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb: {}", state);
    }

    #[test]
    fn test_vcx_credential_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = credential::from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(handle > 0);
        let rc = vcx_credential_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_credential_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_credential_update_state").unwrap();
        let handle = credential::from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        //::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec());
        assert_eq!(vcx_credential_update_state(0, handle, Some(get_state_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
        assert_eq!(vcx_credential_send_request(0, handle, cxn,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}
