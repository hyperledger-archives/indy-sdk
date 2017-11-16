extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use issuer_claim;
use std::thread;
use std::ptr;

/**
 * claim object
 */

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_create_claim(command_handle: u32,
                                      source_id: *const c_char,
                                      schema_seq_no: u32,
                                      issuer_did: *const c_char,
                                      claim_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);
    check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    thread::spawn(move|| {
        let (rc, handle) = match issuer_claim::issuer_claim_create(schema_seq_no, source_id_opt, issuer_did, claim_data) {
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

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }


    thread::spawn(move|| {
        let err = match issuer_claim::send_claim_offer(claim_handle, connection_handle) {
            Ok(x) => x,
            Err(x) => x,
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_issuer_claim_update_state(command_handle: u32,
                                            claim_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    thread::spawn(move|| {
        issuer_claim::update_state(claim_handle);

        cb(command_handle, error::SUCCESS.code_num, issuer_claim::get_state(claim_handle));
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_accept_claim(claim_handle: u32) -> u32 { error::SUCCESS.code_num }

#[no_mangle]
pub extern fn cxs_issuer_send_claim(command_handle: u32,
                                    claim_handle: u32,
                                    connection_handle: u32,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match issuer_claim::send_claim(claim_handle, connection_handle) {
            Ok(x) => x,
            Err(x) => x,
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables)]
pub extern fn cxs_issuer_terminate_claim(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }

#[no_mangle]
pub extern fn cxs_issuer_claim_serialize(command_handle: u32,
                                         claim_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match issuer_claim::to_string(claim_handle) {
            Ok(x) => {
                info!("serializing handle: {} with data: {}",claim_handle, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num,msg.as_ptr());
            },
            Err(x) => {
                warn!("could not serialize handle {}",claim_handle);
                cb(command_handle,x,ptr::null_mut());
            },
        };
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
        let (rc, handle) = match issuer_claim::from_string(&claim_data) {
            Ok(x) => (error::SUCCESS.code_num, x),
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_claim_issuer_release(claim_handle: u32) -> u32 { issuer_claim::release(claim_handle) }


#[cfg(test)]
mod tests {
    extern crate mockito;
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;
    use settings;
    use connection;
    use api::CxsStateType;

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
        assert_eq!(cxs_issuer_create_claim(0,
                                           ptr::null(),
                                           32,
                                           CString::new("8XFh8yBzrpJQmNyZzgoTqB").unwrap().into_raw(),
                                           CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
                                           Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_issuer_create_claim_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(
            0,
            ptr::null(),
            32,
            ptr::null(),
            ptr::null(),
            Some(create_cb)),error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_issuer_claim_serialize(0,claim_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_issuer_claim_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(cxs_issuer_create_claim(0,
                                           ptr::null(),
                                           32,
                                           CString::new("8XFh8yBzrpJQmNyZzgoTqB").unwrap().into_raw(),
                                           CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send claim(offer) {}",err)}
    }

    #[test]
    fn test_cxs_issuer_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claimOffer\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let original = "{\"source_id\":\"test_cxs_issuer_send_claim_offer\",\"handle\":456,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"schema_seq_no\":32,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"issued_did\":\"\",\"state\":1}";
        let handle = issuer_claim::from_string(original).unwrap();
        assert_eq!(issuer_claim::get_state(handle),CxsStateType::CxsStateInitialized as u32);

        let connection_handle = connection::create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        assert_eq!(cxs_issuer_send_claim_offer(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        _m.assert();
    }

    #[test]
    fn test_cxs_issuer_send_claim() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let original = "{\"source_id\":\"test_cxs_issuer_send_claim\",\"handle\":123,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"schema_seq_no\":32,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"issued_did\":\"\",\"state\":3}";
        let handle = issuer_claim::from_string(original).unwrap();
        assert_eq!(issuer_claim::get_state(handle),CxsStateType::CxsStateRequestReceived as u32);

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claimOffer\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let connection_handle = connection::create_connection("test_send_claim".to_owned());
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        assert_eq!(cxs_issuer_send_claim(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        _m.assert();
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called deserialize_cb");
        let original = "{\"source_id\":\"test_claim_serialize\",\"handle\":261385873,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"schema_seq_no\":32,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"issued_did\":\"\",\"state\":1,\"claim_request\":null}";
        let new = issuer_claim::to_string(claim_handle).unwrap();
        assert_eq!(original,new);
    }

    #[test]
    fn test_cxs_issuer_claim_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = "{\"source_id\":\"test_claim_serialize\",\"handle\":261385873,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"schema_seq_no\":32,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"issued_did\":\"\",\"state\":1,\"claim_request\":null}";
        cxs_issuer_claim_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}