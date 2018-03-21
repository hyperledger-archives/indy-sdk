extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use claim;
use std::thread;
use std::ptr;


/// Create a Claim object that requests and receives a claim for an institution
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the user, should be unique.
///
/// offer: claim offer received via "vcx_get_claim_offers"
///
/// # Example offer -> "[{"msg_type": "CLAIM_OFFER","version": "0.1","to_did": "...","from_did":"...","claim": {"account_num": ["...."],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "...","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}]
///
/// cb: Callback that provides claim handle or error status
///
/// #Returns
/// Error code as a u32

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_claim_create_with_offer(command_handle: u32,
                                          source_id: *const c_char,
                                          offer: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(offer, error::INVALID_OPTION.code_num);

    info!("vcx_claim_create_with_offer(command_handle: {}, source_id: {}, offer: {})",
          command_handle, source_id, offer);

    thread::spawn(move|| {
        match claim::claim_create_with_offer(&source_id, &offer) {
            Ok(x) => {
                info!("vcx_claim_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                      command_handle, source_id, error_string(0), x);
                cb(command_handle, error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_claim_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                      command_handle, source_id, error_string(x), 0);
                cb(command_handle, x, 0);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Send a claim request to the connection, called after having received a claim offer
///
/// #params
/// command_handle: command handle to map callback to user context
///
/// claim_handle: claim handle that ws provided during creation. Used to identify claim object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of claim request
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_claim_send_request(command_handle: u32,
                                          claim_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !claim::is_valid_handle(claim_handle) {
        return error::INVALID_CLAIM_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let source_id = claim::get_source_id(claim_handle).unwrap_or_default();
    info!("vcx_claim_send_request(command_handle: {}, claim_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, claim_handle, connection_handle, source_id);

    thread::spawn(move|| {
        match claim::send_claim_request(claim_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_claim_send_request_cb(command_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, error_string(0), source_id);
                cb(command_handle,x);
            },
            Err(x) => {
                warn!("vcx_claim_send_request_cb(command_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, error_string(x), source_id);
                cb(command_handle,x);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Queries agency for claim offers from the given connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection to query for claim offers.
///
/// cb: Callback that provides any claim offers and error status of query
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_claim_get_offers(command_handle: u32,
                                   connection_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_offers: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    info!("vcx_claim_get_offers(command_handle: {}, connection_handle: {})",
          command_handle, connection_handle);

    thread::spawn(move|| {
        match claim::get_claim_offer_messages(connection_handle, None) {
            Ok(x) => {
                info!("vcx_claim_get_offers_cb(command_handle: {}, rc: {}, msg: {})",
                      command_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_claim_get_offers_cb(command_handle: {}, rc: {}, msg: null)",
                      command_handle, error_string(x));
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the claim and updates the the state attribute.  If it detects a claim it
/// will store the claim in the wallet and update the state.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_handle: Claim handle that was provided during creation. Used to identify claim object
///
/// cb: Callback that provides most current state of the claim and error status of request
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_claim_update_state(command_handle: u32,
                                            claim_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !claim::is_valid_handle(claim_handle) {
        return error::INVALID_CLAIM_HANDLE.code_num;
    }

    let source_id = claim::get_source_id(claim_handle).unwrap_or_default();
    info!("vcx_claim_update_state(command_handle: {}, claim_handle: {}), source_id: {:?}",
          command_handle, claim_handle, source_id);

    thread::spawn(move|| {
        match claim::update_state(claim_handle) {
            Ok(_) => (),
            Err(e) => {
                error!("vcx_claim_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        }

        let state = match claim::get_state(claim_handle) {
            Ok(s) => {
                info!("vcx_claim_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_claim_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_claim_get_state(command_handle: u32,
                                  handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !claim::is_valid_handle(handle) {
        return error::INVALID_CLAIM_HANDLE.code_num;
    }

    let source_id = claim::get_source_id(handle).unwrap_or_default();
    info!("vcx_claim_get_state(command_handle: {}, claim_handle: {}), source_id: {:?}",
          command_handle, handle, source_id);

    thread::spawn(move|| {
        match claim::get_state(handle) {
            Ok(s) => {
                info!("vcx_claim_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            },
            Err(e) => {
                error!("vcx_claim_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(e), 0, source_id);
                cb(command_handle, e, 0)
            }
        };
    });

    error::SUCCESS.code_num
}


/// Takes the claim object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Claim handle that was provided during creation. Used to identify claim object
///
/// cb: Callback that provides json string of the claim's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claim_serialize(command_handle: u32,
                                         handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !claim::is_valid_handle(handle) {
        return error::INVALID_CLAIM_HANDLE.code_num;
    }

    let source_id = claim::get_source_id(handle).unwrap_or_default();
    info!("vcx_claim_serialize(command_handle: {}, claim_handle: {}), source_id: {:?}",
          command_handle, handle, source_id);

    thread::spawn(move|| {
        match claim::to_string(handle) {
            Ok(x) => {
                info!("vcx_claim_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                    command_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                error!("vcx_claim_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                    command_handle, error_string(x), 0, source_id);
                cb(command_handle,x,ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an claim object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_data: json string representing a claim object
///
///
/// cb: Callback that provides claim handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claim_deserialize(command_handle: u32,
                                           claim_data: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);

    info!("vcx_claim_deserialize(command_handle: {}, claim_data: {})",
          command_handle, claim_data);

    thread::spawn(move|| {
        match claim::from_string(&claim_data) {
            Ok(x) => {
                info!("vcx_claim_deserialize_cb(command_handle: {}, rc: {}, claim_handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, claim::get_source_id(x).unwrap_or_default());

                cb(command_handle, 0, x);
            },
            Err(x) => {
                error!("vcx_claim_deserialize_cb(command_handle: {}, rc: {}, claim_handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, "");
                cb(command_handle, x, 0);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Releases the claim object by de-allocating memory
///
/// #Params
/// handle: Proof handle that was provided during creation. Used to access claim object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claim_release(handle: u32) -> u32 {
    let source_id = claim::get_source_id(handle).unwrap_or_default();
    match claim::release(handle) {
        Ok(_) => {
            info!("vcx_claim_release(handle: {}, rc: {}), source_id: {:?}",
                  handle, error_string(0), source_id);
            error::SUCCESS.code_num
        },
        Err(e) => {
            error!("vcx_claim_release(handle: {}, rc: {}), source_id: {:?}",
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
    use utils::constants::{DEFAULT_SERIALIZED_CLAIM};

    pub const BAD_CLAIM_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    extern "C" fn create_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn bad_create_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, error::INVALID_JSON.code_num);
        assert_eq!(claim_handle, 0);
        println!("successfully called bad_create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, claim_string: *const c_char) {
        assert_eq!(err, 0);
        if claim_string.is_null() {
            panic!("claim_string is null");
        }
        check_useful_c_str!(claim_string, ());
        println!("successfully called serialize_cb: {}", claim_string);
    }

    #[test]
    fn test_vcx_claim_create_with_offer_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_claim_create_with_offer(0,
                                               CString::new("test_create").unwrap().into_raw(),
                                               CString::new(::utils::constants::CLAIM_OFFER_JSON).unwrap().into_raw(),
                                               Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_claim_create_with_offer_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_claim_create_with_offer(
            0,
            CString::new("test_create").unwrap().into_raw(),
            CString::new(BAD_CLAIM_OFFER).unwrap().into_raw(),
            Some(bad_create_cb)),error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_claim_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = claim::claim_create_with_offer("test_vcx_claim_serialize",::utils::constants::CLAIM_OFFER_JSON).unwrap();
        assert_eq!(vcx_claim_serialize(0,
                                       handle,
                                       Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send claim(offer) {}",err)}
    }

    #[test]
    fn test_vcx_claim_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = claim::claim_create_with_offer("test_send_request",::utils::constants::CLAIM_OFFER_JSON).unwrap();
        assert_eq!(claim::get_state(handle).unwrap(),VcxStateType::VcxStateRequestReceived as u32);

        let connection_handle = connection::build_connection("test_send_claim_offer").unwrap();

        assert_eq!(vcx_claim_send_request(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
        println!("successfully called init_cb")
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called deserialize_cb");
        let original = formatter(DEFAULT_SERIALIZED_CLAIM);
        let new = formatter(&claim::to_string(claim_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_claim_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_CLAIM;
        vcx_claim_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_offers_cb(command_handle: u32, err:u32, offers: *const c_char) {
        assert_eq!(err,0);
        check_useful_c_str!(offers, ());
        println!("successfully called get_offers_cb: {:?}", offers);
    }

    #[test]
    fn test_vcx_claim_get_new_offers(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_get_new_offers").unwrap();
        ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CLAIM_OFFER_RESPONSE.to_vec());
        assert_eq!(error::SUCCESS.code_num as u32, vcx_claim_get_offers(0,
                                           cxn,
                                           Some(get_offers_cb)));
        thread::sleep(Duration::from_millis(300));
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb: {}", state);
    }

    #[test]
    fn test_vcx_claim_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = claim::from_string(DEFAULT_SERIALIZED_CLAIM).unwrap();
        assert!(handle > 0);
        let rc = vcx_claim_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_claim_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cxn = ::connection::build_connection("test_claim_update_state").unwrap();
        let handle = claim::from_string(DEFAULT_SERIALIZED_CLAIM).unwrap();
        //::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CLAIM_OFFER_RESPONSE.to_vec());
        assert_eq!(vcx_claim_update_state(0, handle, Some(get_state_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
        assert_eq!(vcx_claim_send_request(0, handle, cxn,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

}
