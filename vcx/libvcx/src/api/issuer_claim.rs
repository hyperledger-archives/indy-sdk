extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use settings;
use issuer_claim;
use std::thread;
use std::ptr;

/**
 * claim object
 */

/// Create a Issuer Claim object that provides a claim for an enterprise's user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_seq_no: integer number corresponding to claim's schema number on the ledger
///
/// issuer_did: did corresponding to entity issuing a claim. Needs to have Trust Anchor permissions on ledger
///
/// claim_data: data attributes offered to person in the claim
///
/// # Example claim_data -> "{"state":["UT"]}"
///
/// claim_name: Name of the claim - ex. Drivers Licence
///
/// cb: Callback that provides claim handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_create_claim(command_handle: u32,
                                      source_id: *const c_char,
                                      schema_seq_no: u32,
                                      issuer_did: *const c_char,
                                      claim_data: *const c_char,
                                      claim_name: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_name, error::INVALID_OPTION.code_num);

    let issuer_did: String = if !issuer_did.is_null() {
        check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);
        issuer_did.to_owned()
    } else {
        match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
            Ok(x) => x,
            Err(x) => return x
        }
    };

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    info!("vcx_issuer_create_claim(command_handle: {}, source_id: {:?}, schema_seq_no: {}, issuer_did: {}, claim_data: {}, claim_name: {})",
          command_handle,
          source_id_opt,
          schema_seq_no,
          issuer_did,
          claim_data,
          claim_name);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_claim::issuer_claim_create(schema_seq_no, source_id_opt, issuer_did, claim_name, claim_data) {
            Ok(x) => {
                info!("vcx_issuer_create_claim_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(0), x);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_create_claim_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(x), 0);
                (x, 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Send a claim offer to user showing what will be included in the actual claim
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_handle: Claim handle that was provided during creation. Used to identify claim object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of claim offer
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_claim_offer(command_handle: u32,
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

    info!("vcx_issuer_send_claim(command_handle: {}, claim_handle: {}, connection_handle: {})",
          command_handle, claim_handle, connection_handle);

    thread::spawn(move|| {
        let err = match issuer_claim::send_claim_offer(claim_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_claim_cb(command_handle: {}, claim_handle: {}, rc: {})",
                      command_handle, claim_handle, error_string(x));
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_claim_cb(command_handle: {}, claim_handle: {}, rc: {})",
                      command_handle, claim_handle, error_string(x));
                x
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the claim and updates the the state attribute
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
pub extern fn vcx_issuer_claim_update_state(command_handle: u32,
                                            claim_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_claim_update_state(command_handle: {}, claim_handle: {})",
          command_handle, claim_handle);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    thread::spawn(move|| {
        issuer_claim::update_state(claim_handle);

        info!("vcx_issuer_claim_update_state_cb(command_handle: {}, claim_handle: {}, rc: {}, state: {})",
              command_handle, claim_handle, error_string(0), issuer_claim::get_state(claim_handle));
        cb(command_handle, error::SUCCESS.code_num, issuer_claim::get_state(claim_handle));
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_issuer_claim_get_state(command_handle: u32,
                                         claim_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_claim_get_state(command_handle: {}, claim_handle: {})",
          command_handle, claim_handle);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_issuer_claim_get_state_cb(command_handle: {}, claim_handle: {}, rc: {}, state: {})",
              command_handle, claim_handle, error_string(0), issuer_claim::get_state(claim_handle));
        cb(command_handle, error::SUCCESS.code_num, issuer_claim::get_state(claim_handle));
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_accept_claim(claim_handle: u32) -> u32 { error::SUCCESS.code_num }

/// Send Claim that was requested by user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_handle: Claim handle that was provided during creation. Used to identify claim object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of sending the claim
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_claim(command_handle: u32,
                                    claim_handle: u32,
                                    connection_handle: u32,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_send_claim(command_handle: {}, claim_handle: {}, connection_handle: {})",
          command_handle, claim_handle, connection_handle);

    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match issuer_claim::send_claim(claim_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_claim_cb(command_handle: {}, claim_handle: {}, rc: {})",
                      command_handle, claim_handle, error_string(x));
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_claim_cb(command_handle: {}, claim_handle: {}, rc: {})",
                      command_handle, claim_handle, error_string(x));
                x
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables)]
pub extern fn vcx_issuer_terminate_claim(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }

/// Takes the claim object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_handle: Claim handle that was provided during creation. Used to identify claim object
///
/// cb: Callback that provides json string of the claim's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_claim_serialize(command_handle: u32,
                                         claim_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_claim_serialize(claim_serialize(command_handle: {}, claim_handle: {})",
          command_handle, claim_handle);
    if !issuer_claim::is_valid_handle(claim_handle) {
        return error::INVALID_ISSUER_CLAIM_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match issuer_claim::to_string(claim_handle) {
            Ok(x) => {
                info!("vcx_issuer_claim_serialize_cb(command_handle: {}, claim_handle: {}, rc: {}, state: {})",
                      command_handle, claim_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num,msg.as_ptr());
            },
            Err(x) => {
                info!("vcx_issuer_claim_serialize_cb(command_handle: {}, claim_handle: {}, rc: {}, state: {})",
                      command_handle, claim_handle, error_string(x), "null");
                cb(command_handle,x,ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an issuer claim object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claim_data: json string representing a claim object
///
/// # Examples claim_data -> {"source_id":"1","handle":2,"claim_attributes":"{\"state\":[\"UT\"]}","msg_uid":"","schema_seq_no":1234,"issuer_did":"DID","issued_did":"","state":1,"claim_request":"","claim_name":"Claim","claim_id":"123","ref_msg_id":""}
///
/// cb: Callback that provides claim handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_claim_deserialize(command_handle: u32,
                                      claim_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claim_data, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_claim_deserialize(command_handle: {}, claim_data: {})", command_handle, claim_data);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_claim::from_string(&claim_data) {
            Ok(x) => {
                info!("vcx_issuer_claim_deserialize_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(0), x);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_claim_deserialize_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(x), 0);
                (x, 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the issuer claim object by deallocating memory
///
/// #Params
/// claim_handle: Claim handle that was provided during creation. Used to identify claim object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_claim_release(claim_handle: u32) -> u32 {
    info!("(vcx_issuer_claim_release claim_handle: {})", claim_handle);
    issuer_claim::release(claim_handle)
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
    use utils::constants::{DEFAULT_SERIALIZED_ISSUER_CLAIM, CLAIM_REQ_STRING};
    use api::vcx::vcx_init;

    static DEFAULT_CLAIM_NAME: &str = "Claim Name Default";
    static DEFAULT_DID: &str = "8XFh8yBzrpJQmNyZzgoTqB";
    static DEFAULT_ATTR: &str = "{\"attr\":\"value\"}";
    static DEFAULT_SCHEMA_SEQ_NO: u32 = 32;
    static ISSUER_CLAIM_STATE_ACCEPTED: &str = r#"{"claim_id":"a claim id","claim_name":"claim name","source_id":"test_vcx_issuer_send_claim","handle":123,"claim_attributes":"{\"state\":[\"UT\"],\"zip\":[\"84000\"],\"city\":[\"Draper\"],\"address2\":[\"Suite 3\"],\"address1\":[\"123 Main St\"]}","msg_uid":"","schema_seq_no":32,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","issued_did":"VsKV7grR1BUE29mG2Fm2kX","issued_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","remote_did":"VsKV7grR1BUE29mG2Fm2kX","remote_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","agent_did":"VsKV7grR1BUE29mG2Fm2kX","agent_vk":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW","state":3,"ref_msg_id":"abc123"}"#;
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
    fn test_vcx_issuer_create_claim_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_claim(0,
                                           ptr::null(),
                                           32,
                                           ptr::null(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CLAIM_NAME).unwrap().into_raw(),
                                           Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_create_claim_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_claim(
            0,
            ptr::null(),
            32,
            ptr::null(),
            ptr::null(),
            CString::new(DEFAULT_CLAIM_NAME).unwrap().into_raw(),
            Some(create_cb)),error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_issuer_claim_serialize(0,claim_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_claim_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_claim(0,
                                           ptr::null(),
                                           DEFAULT_SCHEMA_SEQ_NO,
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CLAIM_NAME).unwrap().into_raw(),
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send claim(offer) {}",err)}
    }

    #[test]
    fn test_vcx_issuer_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = issuer_claim::from_string(DEFAULT_SERIALIZED_ISSUER_CLAIM).unwrap();
        assert_eq!(issuer_claim::get_state(handle),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_claim_offer".to_owned()).unwrap();

        assert_eq!(vcx_issuer_send_claim_offer(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
        println!("successfully called init_cb")
    }

    #[test]
    fn test_vcx_issuer_send_a_claim() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);
        use claim_request::ClaimRequest;

        let test_name = "test_vcx_issuer_send_a_claim";

        //let result = vcx_init(0,ptr::null(),Some(init_cb));
        thread::sleep(Duration::from_secs(1));

        let handle = issuer_claim::from_string(ISSUER_CLAIM_STATE_ACCEPTED).unwrap();

        /* align claim request and claim def ***********************************/
        let mut claim_request = match ClaimRequest::from_str(CLAIM_REQ_STRING) {
            Ok(x) => x,
            Err(_) => panic!("error with claim request"),
        };
        // set claim request to have the same did as enterprise did (and sam as claim def)
        claim_request.issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).clone().unwrap();
        claim_request.schema_seq_no = 15;
        issuer_claim::set_claim_request(handle, claim_request).unwrap();
        assert_eq!(issuer_claim::get_state(handle),VcxStateType::VcxStateRequestReceived as u32);
        /**********************************************************************/

        // create connection
        let connection_handle = connection::build_connection("test_send_claim".to_owned()).unwrap();

        // send the claim
        assert_eq!(vcx_issuer_send_claim(0, handle, connection_handle, Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }
    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claim_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called deserialize_cb");
        let original = formatter(DEFAULT_SERIALIZED_ISSUER_CLAIM);
        let new = formatter(&issuer_claim::to_string(claim_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_issuer_claim_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_ISSUER_CLAIM;
        vcx_issuer_claim_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    // TODO: Need to get this test working
    /*
    #[test]
    fn test_vcx_issue_claim_fails_without_claim_def_in_wallet(){

        let test_name = "test_vcx_issue_claim_fails_without_claim_def_in_wallet";
        let schema_seq_num = 32 as u32;

        let result = vcx_init(0,ptr::null(),Some(init_cb));
        thread::sleep(Duration::from_secs(1));

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENCY_ENDPOINT, mockito::SERVER_URL);
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID,"8XFh8yBzrpJQmNyZzgoTqB");

        let original_issuer_claim_str = "{\"source_id\":\"test_vcx_issue_claim_fails_without_claim_def_in_wallet\",\"handle\":123,\"claim_attributes\":\"{\\\"attr\\\":\\\"value\\\"}\",\"msg_uid\":\"\",\"schema_seq_no\":32,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"issued_did\":\"\",\"state\":3}";
        let handle = issuer_claim::from_string(original_issuer_claim_str).unwrap();
        let connection_handle = connection::create_connection(test_name.to_owned());
        /* align claim request and claim def ***********************************/
        let mut claim_request = create_claim_request_from_str(CLAIM_REQ_STRING);
        // set claim request to have the same did as enterprise did (and sam as claim def)
        claim_request.issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).clone().unwrap();
        // set claim request to have the same sequence number as the schema sequence number
        claim_request.schema_seq_no = schema_seq_num as i32;
        assert_eq!(claim_request.schema_seq_no, schema_seq_num as i32);
        issuer_claim::set_claim_request(handle, &claim_request).unwrap();
        assert_eq!(issuer_claim::get_state(handle),VcxStateType::VcxStateRequestReceived as u32);
        let schema = create_default_schema(schema_seq_num);
        let wallet_name = create_dummy_wallet(test_name);
//        put_claim_def_in_issuer_wallet(&settings::get_config_value(
//            settings::CONFIG_INSTITUTION_DID).unwrap(), &schema, get_wallet_handle());
        /**********************************************************************/
        connection::set_pw_did(connection_handle, "8XFh8yBzrpJQmNyZzgoTqB");

        let command_handle = 0;
        // create closure for send claim


        // wait for response, response should be error

        assert_eq!(vcx_issuer_send_claim(command_handle, handle, connection_handle, Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }
    */

    // TODO: Need to get this test working
    /*
    #[test]
    fn test_calling_send_claim_without_claim_request_errors(){
        assert_eq!(0,1);
    }
    */

    #[test]
    fn test_create_claim_arguments_correct(){
        let result = vcx_init(0,ptr::null(),Some(init_cb));
        thread::sleep(Duration::from_secs(1));

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);
        assert_eq!(vcx_issuer_create_claim(0,
                                           ptr::null(),
                                           DEFAULT_SCHEMA_SEQ_NO,
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CLAIM_NAME).unwrap().into_raw(),
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_issuer_claim_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = issuer_claim::from_string(DEFAULT_SERIALIZED_ISSUER_CLAIM).unwrap();
        assert!(handle > 0);
        let rc = vcx_issuer_claim_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

}
