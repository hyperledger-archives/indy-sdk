extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::ptr;
use std::thread;
use connection::{get_source_id, build_connection, build_connection_with_invite, connect, to_string, get_state, release, is_valid_handle, update_state, from_string, get_invite_details};

/**
 * connection object
 */

/// -> Create a Connection object that provides a pairwise connection for an institution's user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// cb: Callback that provides connection handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_connection_create(command_handle: u32,
                                    source_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    info!("vcx_connection_create(command_handle: {}, source_id: {})", command_handle, source_id);
    thread::spawn(move|| {
        match build_connection(&source_id) {
            Ok(handle) => {
                info!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(0), handle);
                cb(command_handle, error::SUCCESS.code_num, handle)
            },
            Err(x) => {
                warn!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(x), 0);
                cb(command_handle, x, 0)
            },
        };
    });

    error::SUCCESS.code_num
}

/// -> Create a Connection object from the given invite_details that provides a pairwise connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// invite_details: Provided via the other end of the connection calling "vcx_connection_connect" or "vcx_connection_invite_details"
///
/// cb: Callback that provides connection handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_create_with_invite(command_handle: u32,
                                                source_id: *const c_char,
                                                invite_details: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(invite_details, error::INVALID_OPTION.code_num);
    info!("vcx create connection with invite called");
    thread::spawn(move|| {
        match build_connection_with_invite(&source_id, &invite_details) {
            Ok(handle) => cb(command_handle, error::SUCCESS.code_num, handle),
            Err(x) => cb(command_handle, x, 0),
        };
    });

    error::SUCCESS.code_num
}

/// Establishes connection between institution and its user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies connection object
///
/// connection_options: Provides details indicating if the connection will be established by text or QR Code
///
/// # Examples connection_options -> "{"connection_type":"SMS","phone":"123"}" OR: "{"connection_type":"QR","phone":""}"
///
/// cb: Callback that provides error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_connect(command_handle:u32,
                                     connection_handle: u32,
                                     connection_options: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, invite_details: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let options = if !connection_options.is_null() {
        check_useful_opt_c_str!(connection_options, error::INVALID_OPTION.code_num);
        connection_options.to_owned()
    }
    else {
        None
    };

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_connect(command_handle: {}, connection_handle: {}, connection_options: {:?}), source_id: {:?}",
          command_handle, connection_handle, options, source_id);

    thread::spawn(move|| {
        match connect(connection_handle, options) {
            Ok(_) => {
                match get_invite_details(connection_handle,true) {
                    Ok(x) => {
                        info!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error_string(0), x, source_id);
                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr())
                    },
                    Err(_) => {
                        warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error_string(0), "null", source_id);
                        cb(command_handle, error::SUCCESS.code_num, ptr::null_mut())
                    },
                }
            },
            Err(x) => {
                warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(x), "null", source_id);
                cb(command_handle,x, ptr::null_mut())
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes the Connection object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides json string of the connection's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_serialize(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, serialized_data: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_serialize(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match to_string(connection_handle) {
            Ok(json) => {
                info!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(0), json, source_id);
                let msg = CStringUtils::string_to_cstring(json);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a connection object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_data: json string representing a connection object
/// # Examples connection_data -> {"source_id":"1","handle":2,"pw_did":"did","pw_verkey":"verkey","did_endpoint":"","state":2,"uuid":"","endpoint":"","invite_detail":{"e":"","rid":"","sakdp":"","sn":"","sD":"","lu":"","sVk":"","tn":""}}
///
/// cb: Callback that provides claim handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_deserialize(command_handle: u32,
                                      connection_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(connection_data, error::INVALID_OPTION.code_num);

    info!("vcx_connection_deserialize(command_handle: {}, connection_data: {})", command_handle, connection_data);

    thread::spawn(move|| {
        let (rc, handle) = match from_string(&connection_data) {
            Ok(x) => {
                let source_id = get_source_id(x).unwrap_or_default();
                info!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, source_id);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                let source_id = get_source_id(x).unwrap_or_default();
                warn!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, source_id);
                (x, 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}


/// Checks for any state change in the connection and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// cb: Callback that provides most current state of the claim and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_update_state(command_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_update_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let rc = match update_state(connection_handle) {
            Ok(x) => {
                info!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), connection_handle, get_state(connection_handle), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(x), connection_handle, get_state(connection_handle), source_id);
                x
            },
        };
        let state = get_state(connection_handle);
        cb(command_handle, rc, state);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_connection_get_state(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_get_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_connection_get_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), connection_handle, get_state(connection_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, get_state(connection_handle));
    });

    error::SUCCESS.code_num
}

/// Gets the current connection details
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// abbreviated: abbreviated connection details for QR codes or not
///
/// cb: Callback that provides the json string of details
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_invite_details(command_handle: u32,
                                            connection_handle: u32,
                                            abbreviated: bool,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, details: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_invite_details(command_handle: {}, connection_handle: {}, abbreviated: {}), source_id: {:?}",
          command_handle, connection_handle, abbreviated, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match get_invite_details(connection_handle, abbreviated){
            Ok(str) => {
                info!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(0), str, source_id);
                let msg = CStringUtils::string_to_cstring(str);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            }
        }
    });

    error::SUCCESS.code_num
}

/// Releases the connection object by de-allocating memory
///
/// #Params
/// connection_handle: was provided during creation. Used to identify connection object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_release(connection_handle: u32) -> u32 {
    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_release(connection_handle: {}), source_id: {:?}", connection_handle, source_id);
    release(connection_handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings;
    use std::ffi::CString;
    use std::ptr;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use api::VcxStateType;
    use utils::httpclient;
    use utils::constants::GET_MESSAGES_RESPONSE;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        if err != 0 {panic!("create_cb failed")}
        if connection_handle == 0 {panic!("invalid handle")}
        println!("successfully called create_cb")
    }

    #[test]
    fn test_vcx_connection_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = vcx_connection_create(0,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(create_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_vcx_connection_create_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = vcx_connection_create(0,
                                       CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);

        let rc = vcx_connection_create(0,
                                       ptr::null(),
                                       Some(create_cb));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    extern "C" fn connect_cb(command_handle: u32, err: u32, details: *const c_char) {        if err != 0 {panic!("connect failed: {}", err);}
        println!("successfully called connect_cb");
    }

    #[test]
    fn test_vcx_connection_connect() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = vcx_connection_connect(0,0, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
        let handle = build_connection("test_vcx_connection_connect").unwrap();
        assert!(handle > 0);
        let rc = vcx_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        thread::sleep(Duration::from_millis(500));
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state,VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_vcx_connection_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_vcx_connection_update_state").unwrap();
        assert!(handle > 0);
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        let rc = vcx_connection_update_state(0,handle,Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_connection_update_state_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = vcx_connection_update_state(0,0,None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, claim_string: *const c_char) {
        assert_eq!(err, 0);
        if claim_string.is_null() {
            panic!("claim_string is empty");
        }
        check_useful_c_str!(claim_string, ());
        println!("successfully called serialize_cb: {}", claim_string);
    }

    #[test]
    #[allow(unused_assignments)]
    fn test_vcx_connection_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_vcx_connection_get_data").unwrap();
        assert!(handle > 0);

        let data = vcx_connection_serialize(0,handle, Some(serialize_cb));
        thread::sleep(Duration::from_millis(200));
        assert_eq!(data, 0);
    }

    #[test]
    fn test_vcx_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_vcx_connection_release").unwrap();
        assert!(handle > 0);

        let rc = vcx_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let rc = vcx_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called deserialize_cb");
        let string = r#"{"source_id":"test_vcx_connection_deserialialize_succeeds","pw_did":"8XFh8yBzrpJQmNyZzgoTqB","pw_verkey":"EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A","state":1,"uuid":"","endpoint":"","invite_detail":{"statusCode":"","connReqId":"","senderDetail":{"name":"","agentKeyDlgProof":{"agentDID":"","agentDelegatedKey":"","signature":""},"DID":"","logoUrl":"","verKey":""},"senderAgencyDetail":{"DID":"","verKey":"","endpoint":""},"targetName":"","statusMsg":""},"agent_did":"U5LXs4U7P9msh647kToezy","agent_vk":"FktSZg8idAVzyQZrdUppK6FTrfAzW3wWVzAjJAfdUvJq","their_pw_did":"","their_pw_verkey":""}"#;

        let new = to_string(connection_handle).unwrap();
        println!("original: {}",string);
        println!("     new: {}",new);
        assert_eq!(string,new);
    }

    #[test]
    fn test_vcx_connection_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = r#"{"source_id":"test_vcx_connection_deserialialize_succeeds","pw_did":"8XFh8yBzrpJQmNyZzgoTqB","pw_verkey":"EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A","did_endpoint":"","state":1,"uuid":"","endpoint":"","invite_detail":{"statusCode":"","connReqId":"","senderDetail":{"name":"","agentKeyDlgProof":{"agentDID":"","agentDelegatedKey":"","signature":""},"DID":"","logoUrl":"","verKey":""},"senderAgencyDetail":{"DID":"","verKey":"","endpoint":""},"targetName":"","statusMsg":""},"agent_did":"U5LXs4U7P9msh647kToezy","agent_vk":"FktSZg8idAVzyQZrdUppK6FTrfAzW3wWVzAjJAfdUvJq","their_pw_did":"","their_pw_verkey":""}"#;

        vcx_connection_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_vcx_connection_update_state").unwrap();
        assert!(handle > 0);
        let rc = vcx_connection_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }
}
