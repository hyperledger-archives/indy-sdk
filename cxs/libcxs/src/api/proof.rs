extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use proof;
use connection;
use std::thread;
use std::ptr;
use api::CxsStatus;

#[no_mangle]
pub extern fn cxs_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_attrs, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_predicates, error::INVALID_OPTION.code_num);
    check_useful_c_str!(name, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(
            source_id_opt, requested_attrs, requested_predicates, name) {
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
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        proof::update_state(proof_handle);

        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
    });

    error::SUCCESS.code_num
}

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

#[no_mangle]
pub extern fn cxs_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => x,
            Err(x) => x,
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

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
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use proof::{ create_proof };
    use proof;
    use api::CxsStateType;
    use connection;


    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static EXPECTED_ATTRS: &'static str = "{\"Test0\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_1\",\"schema_seq_no\":1},\"Test1\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\",\"schema_seq_no\":1},\"Test2\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"city\",\"schema_seq_no\":1},\"Test3\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"state\",\"schema_seq_no\":1},\"Test4\":{\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"zip\",\"schema_seq_no\":1}";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";

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
        let original = "{\"source_id\":\"source id\",\"handle\":1,\"requested_attrs\":\"{\\\"attrs\\\":[{\\\"name\\\":\\\"person name\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"address_1\\\"},{\\\"schema_seq_no\\\":2,\\\"issuer_did\\\":\\\"ISSUER_DID2\\\",\\\"name\\\":\\\"address_2\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"city\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"state\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"zip\\\"}]}\",\"requested_predicates\":\"{\\\"attr_name\\\":\\\"age\\\",\\\"p_type\\\":\\\"GE\\\",\\\"value\\\":18,\\\"schema_seq_no\\\":1,\\\"issuer_did\\\":\\\"DID1\\\"}\",\"msg_uid\":\"\",\"requester_did\":\"234\",\"prover_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"state\":1,\"tid\":33,\"mid\":22,\"name\":\"Name Data\"}";
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(original,new);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
    }


    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof(offer) {}",err)}
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
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
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
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_proof_create(0,
                                    ptr::null(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional data").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"handle\":1,\"mid\":22,\"msg_uid\":\"\",\"name\":\"Name Data\",\"prover_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"requested_attrs\":\"{\\\"attrs\\\":[{\\\"name\\\":\\\"person name\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"address_1\\\"},{\\\"schema_seq_no\\\":2,\\\"issuer_did\\\":\\\"ISSUER_DID2\\\",\\\"name\\\":\\\"address_2\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"city\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"state\\\"},{\\\"schema_seq_no\\\":1,\\\"name\\\":\\\"zip\\\"}]}\",\"requested_predicates\":\"{\\\"attr_name\\\":\\\"age\\\",\\\"p_type\\\":\\\"GE\\\",\\\"value\\\":18,\\\"schema_seq_no\\\":1,\\\"issuer_did\\\":\\\"DID1\\\"}\",\"requester_did\":\"234\",\"source_id\":\"source id\",\"state\":1,\"tid\":33}";
        cxs_proof_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_proof_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = cxs_proof_update_state(0, handle, Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_cxs_proof_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"indy");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"proofRequest\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(proof::get_state(handle),CxsStateType::CxsStateInitialized as u32);

        let connection_handle = connection::create_connection("test_send_proof_request".to_owned());
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");
        assert_eq!(cxs_proof_send_request(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(proof::get_state(handle),CxsStateType::CxsStateOfferSent as u32);
        _m.assert();
    }

}
