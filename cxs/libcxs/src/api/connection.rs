extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::ptr;
use std::thread;
use connection::{build_connection, connect, to_string, get_state, release, is_valid_handle, update_state, from_string};

/**
 * connection object
 */

#[no_mangle]
#[allow(unused_assignments)]
pub extern fn cxs_connection_create(command_handle: u32,
                                    source_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        match build_connection(source_id) {
            Ok(handle) => cb(command_handle, error::SUCCESS.code_num, handle),
            Err(x) => cb(command_handle, x, 0),
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_connect(command_handle:u32,
                                     connection_handle: u32,
                                     connection_options: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let options = if !connection_options.is_null() {
        check_useful_c_str!(connection_options, error::UNKNOWN_ERROR.code_num);
        connection_options.to_owned()
    }
    else {
        "".to_string()
    };

    thread::spawn(move|| {
        let rc = connect(connection_handle, options);

        cb(command_handle,rc);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_serialize(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match to_string(connection_handle) {
            Ok(json) => {
                info!("serializing handle: {} with data: {}",connection_handle, json);
                let msg = CStringUtils::string_to_cstring(json);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("could not serialize handle {}",connection_handle);
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_deserialize(command_handle: u32,
                                      connection_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(connection_data, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        let (rc, handle) = match from_string(&connection_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_update_state(command_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let rc = update_state(connection_handle);
        let state = get_state(connection_handle);
        cb(command_handle, rc, state);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_release(connection_handle: u32) -> u32 {
    release(connection_handle)
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    use super::*;
    use settings;
    use std::ffi::CString;
    use std::ptr;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use api::CxsStateType;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        if err != 0 {panic!("create_cb failed")}
        if connection_handle == 0 {panic!("invalid handle")}
        println!("successfully called create_cb")
    }

    #[test]
    fn test_cxs_connection_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_create(0,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(create_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_cxs_connection_create_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_create(0,
                                       CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);

        let rc = cxs_connection_create(0,
                                       ptr::null(),
                                       Some(create_cb));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    extern "C" fn connect_cb(command_handle: u32, err: u32) {
        assert_eq!(err, 0);
        println!("successfully called connect_cb");
    }

    #[test]
    fn test_cxs_connection_connect() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_connect(0,0, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
        let handle = build_connection("test_cxs_connection_connect".to_owned()).unwrap();
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(500));
        let rc = cxs_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state,CxsStateType::CxsStateInitialized as u32);
    }

    #[test]
    fn test_cxs_connection_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_update_state".to_owned()).unwrap();
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = cxs_connection_update_state(0,handle,Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_cxs_connection_update_state_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_update_state_fails".to_owned()).unwrap();
        assert!(handle > 0);

        let rc = cxs_connection_update_state(0,0,None);
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
    fn test_cxs_connection_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_get_data".to_owned()).unwrap();
        assert!(handle > 0);

        let data = cxs_connection_serialize(0,handle, Some(serialize_cb));
        thread::sleep(Duration::from_millis(200));
        assert_eq!(data, 0);
    }

    #[test]
    fn test_cxs_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_release".to_owned()).unwrap();
        assert!(handle > 0);

        let rc = cxs_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let rc = cxs_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called deserialize_cb");
        let original = "{\"source_id\":\"test_cxs_connection_deserialize\",\
        \"handle\":2473657597,\"pw_did\":\"\",\"pw_verkey\":\"\",\
        \"did_endpoint\":\"\",\"state\":0,\"uuid\":\"\",\"endpoint\":\"\",\
        \"invite_detail\":{\"e\":\"\",\"rid\":\"\",\"sakdp\":\"\",\
        \"sn\":\"\",\"sD\":\"\",\"lu\":\"\",\"sVk\":\"\",\"tn\":\"\"}}";
        let new = to_string(connection_handle).unwrap();
        println!("original: {}",original);
        println!("     new: {}",new);
        assert_eq!(original,new);
    }

    #[test]
    fn test_cxs_connection_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = "{\"source_id\":\"test_cxs_connection_deserialize\",\
        \"handle\":2473657597,\"pw_did\":\"\",\"pw_verkey\":\"\",\
        \"did_endpoint\":\"\",\"state\":0,\"uuid\":\"\",\"endpoint\":\"\",\
        \"invite_detail\":{\"e\":\"\",\"rid\":\"\",\"sakdp\":\"\",\
        \"sn\":\"\",\"sD\":\"\",\"lu\":\"\",\"sVk\":\"\",\"tn\":\"\"}}";

        cxs_connection_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}
