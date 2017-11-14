extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use proof;
use std::thread;
use std::ptr;
use api::CxsStateType;
use api::CxsStatus;

#[no_mangle]
pub extern fn cxs_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               proof_requester_did: *const c_char,
                               proof_request_data: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_requester_did, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_request_data, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    thread::spawn( move|| {
        let (rc, handle) = match proof::create_proof(source_id_opt,
                                                     proof_requester_did,
                                                     proof_request_data) {
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

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 { error::SUCCESS.code_num }

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

#[no_mangle]
pub extern fn cxs_proof_release(proof_handle: u32) -> u32 {
    proof::release(proof_handle)
}


#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> u32 { error::SUCCESS.code_num }
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
    use std::time::Duration;
    use settings;
    use connection;
    use api::CxsStateType;

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
        let original = "{\"source_id\":\"test_proof_serialize\",\"handle\":2035188318,\"proof_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"proof_requester_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"proover_did\":\"7XFh8yBzrpJQmNyZzgoTqB\",\"state\":1}";
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(original,new);
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
                                    CString::new("8XFh8yBzrpJQmNyZzgoTqB").unwrap().into_raw(),
                                    CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
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
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_proof_create(0,
                                    ptr::null(),
                                    CString::new("8XFh8yBzrpJQmNyZzgoTqB").unwrap().into_raw(),
                                    CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"source_id\":\"test_proof_serialize\",\"handle\":2035188318,\"proof_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"proof_requester_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"proover_did\":\"7XFh8yBzrpJQmNyZzgoTqB\",\"state\":1}";
        cxs_proof_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}
