extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use issuer_claim::{issuer_claim_create, to_string, from_string, send_claim_offer, release};
use std::thread;

/**
 * claim object
 */

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_create_claim(command_handle: u32,
                                      source_id: *const c_char,
                                      claimdef_handle: u32,
                                      claim_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    thread::spawn(move|| {
        let (rc, handle) = match issuer_claim_create(claimdef_handle, source_id_opt, claim_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_issuer_send_claim_offer(command_handle: u32,
                                          claim_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        let err = match send_claim_offer(claim_handle, connection_handle) {
            Ok(x) => x,
            Err(x) => x,
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_accept_claim(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_send_claim(claim_handle: u32, connection_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_issuer_terminate_claim(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }

#[no_mangle]
pub extern fn cxs_issuer_claim_serialize(claim_handle: u32, cb: Option<extern fn(xclaim_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        let (claim_string, err) = match to_string(claim_handle) {
            Ok(x) => {
                info!("serializing handle: {} with data: {}",claim_handle, x);
                (x, error::SUCCESS.code_num)
            },
            Err(_) => {
                warn!("could not serialize handle {}",claim_handle);
                (String::new(), error::UNKNOWN_ERROR.code_num)
            },
        };

        let request_result_string = CStringUtils::string_to_cstring(claim_string);

        cb(claim_handle, err, request_result_string.as_ptr());
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_issuer_claim_deserialize(command_handle: u32,
                                      claim_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        let (rc, handle) = match from_string(&claim_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_claim_issuer_release(claim_handle: u32) -> u32 { release(claim_handle) }


#[cfg(test)]
mod tests {

    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;
    use settings;
    use connection::build_connection;

    extern "C" fn create_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_cb")
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
    fn test_cxs_issuer_create_claim_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(0, ptr::null(), 32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_issuer_create_claim_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(0, ptr::null(),32,ptr::null(),Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_issuer_claim_serialize(claim_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_issuer_claim_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(0, ptr::null(),32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send claim offer")}
    }

    extern "C" fn create_and_send_offer_cb(command_handle: u32, err: u32, claim_handle: u32) {
        if err != 0 {panic!("failed to create claim handle in create_and_send_offer_cb!")}

        let connection_handle = build_connection("test_send_claim_offer".to_owned());
        thread::sleep(Duration::from_millis(500));
        if cxs_issuer_send_claim_offer(command_handle, claim_handle, connection_handle, Some(send_offer_cb)) != error::SUCCESS.code_num {
            panic!("failed to send claim offer");
        }
    }

    #[test]
    fn test_cxs_issuer_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(0, ptr::null(),32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),Some(create_and_send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called deserialize_cb");
        let original = "{\"source_id\":\"test_cxs_issuer_claim_deserialize_succeeds\",\"handle\":422325509,\"claim_def\":32,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"issued_did\":\"\",\"state\":1}";
        let new = to_string(claim_handle).unwrap();
        assert_eq!(original,new);
    }

    #[test]
    fn test_cxs_issuer_claim_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = "{\"source_id\":\"test_cxs_issuer_claim_deserialize_succeeds\",\"handle\":422325509,\"claim_def\":32,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"issued_did\":\"\",\"state\":1}";
        cxs_issuer_claim_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}