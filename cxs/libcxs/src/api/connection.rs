extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::ptr;
use connection::{build_connection, connect, to_string, get_state, release};
use std::thread;

/**
 * connection object
 */

#[no_mangle]
pub extern fn cxs_connection_create(source_id: *const c_char,
                                    did: *const c_char,
                                    their_did: *const c_char,
                                    connection_handle: *mut u32) -> u32 {
    if connection_handle.is_null() {return error::UNKNOWN_ERROR.code_num}

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::UNKNOWN_ERROR.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    let did_opt = if !did.is_null() {
        check_useful_c_str!(did, error::UNKNOWN_ERROR.code_num);
        let val = did.to_owned();
        Some(val)
    } else { None };


    let handle = build_connection(source_id_opt, did_opt, None);
    unsafe { *connection_handle = handle }

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_connect(connection_handle: u32, connection_options: *const c_char) -> u32 {
    let options = if !connection_options.is_null() {
        check_useful_c_str!(connection_options, error::UNKNOWN_ERROR.code_num);
        connection_options.to_owned()
    }
    else {
        "".to_string()
    };

    connect(connection_handle, options)
}

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_serialize(connection_handle: u32, cb: Option<extern fn(xconnection_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {

        let json_string = to_string(connection_handle);

        if json_string.is_empty() {
            warn!("could not serialize handle {}",connection_handle);
        }
            else {
                info!("serializing handle: {} with data: {}",connection_handle, json_string);
            }
        let msg = CStringUtils::string_to_cstring(json_string);

        cb(connection_handle, 0, msg.as_ptr());
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_get_state(connection_handle: u32, status: *mut u32) -> u32 {

    if status.is_null() {return error::UNKNOWN_ERROR.code_num}

    let state = get_state(connection_handle);

    unsafe { *status = state }

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
    use utils::wallet;
    use std::thread;
    use std::time::Duration;
    use api::CxsStateType;

    #[test]
    fn test_cxs_connection_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_create").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);
    }

    #[test]
    fn test_cxs_connection_create_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_create(CString::new("test_create_fails").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);

        let rc = cxs_connection_create(ptr::null(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    fn test_cxs_connection_connect() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,mockito::SERVER_URL);
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(3)
            .create();

        wallet::tests::make_wallet("test_cxs_connection_connect");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_cxs_connection_connect").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
        assert!(handle > 0);

        let rc = cxs_connection_connect(handle, CString::new("{}").unwrap().into_raw());
        assert_eq!(rc, error::SUCCESS.code_num);
        wallet::tests::delete_wallet("test_cxs_connection_connect");
        _m.assert();
    }

    #[test]
    fn test_cxs_connection_connect_fails() {
        let rc = cxs_connection_connect(0, CString::new("{}").unwrap().into_raw());
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_cxs_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_state").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let mut state: u32 = 0;
        let rc = cxs_connection_get_state(handle, &mut state);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(state,CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_cxs_connection_get_state_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_state_fails").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let rc = cxs_connection_get_state(handle, ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);

        let rc = cxs_connection_get_state(0, ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    #[allow(unused_assignments)]
    fn test_cxs_connection_get_data() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_data").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let data = cxs_connection_get_data(handle);
        let mut final_string = String::new();

        unsafe {
            let c_string = CString::from_raw(data);
            final_string = c_string.into_string().unwrap();
        }

        assert!(final_string.len() > 0);
    }

    #[test]
    fn test_cxs_connection_get_data_fails() {
        let data = cxs_connection_get_data(0);

        assert_eq!(data, ptr::null_mut());
    }

    #[test]
    fn test_cxs_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_release").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let rc = cxs_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let rc = cxs_connection_connect(handle, CString::new("{}").unwrap().into_raw());
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_init_create_and_connect(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_init_create_and_connect").unwrap().into_raw(),
                                       ptr::null_mut(),
                                       ptr::null(),
                                       &mut handle);
        assert_eq!(rc, 0);
        thread::sleep(Duration::from_millis(1500));

        let rc = cxs_connection_connect(handle, CString::new("{}").unwrap().into_raw());
        assert_eq!(rc, 0);

    }

    #[test]
    fn test_init_create_and_connect_with_did() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_init_create_and_connect_with_did").unwrap().into_raw(),
                                       CString::new("548NLfYrPxtB299RVafcjR").unwrap().into_raw(),
                                       CString::new("338NLfYrPxtB299RVafcjR").unwrap().into_raw(),
                                       &mut handle);
        thread::sleep(Duration::from_secs(1));

        let rc = cxs_connection_connect(handle, CString::new("{}").unwrap().into_raw());
        assert_eq!(rc, 0);
    }
}
