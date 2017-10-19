extern crate libc;

use self::libc::c_char;
use api::CxsStatus;
use utils::cstring::CStringUtils;
use utils::{pool, wallet};
use utils::error;
use std::ptr;
use settings;
use connection::{build_connection, connect, to_string, get_state, release};

/// Possible values in the Config file:
///
/// pool_name:
/// config_name
/// wallet_name:
/// wallet_type
/// agent_endpoint: the url to interact with the agent
/// enterprise_did_agency: did for enterprise pairwise relationship with an agency
/// agency_pairwise_did: did for the agency pairwise relationship with an enterprise
/// agency_pairwise_verkey: verkey for the agency pairwise relationship with an enterprise
/// enterprise_did_agent: did for enterprise pairwise relationship with an agent
/// agent_pairwise_did: did for the agent pairwise relationship with an enterprise
/// agent_pairwise_verkey: verkey for the agent pairwise relationship with an enterprise
/// enterprise_name: enterprise's name
/// logo_url: url for enterprise's logo
/// A example file is at libcxs/sample_config/config.json
#[no_mangle]
pub extern fn cxs_init (config_path:*const c_char) -> u32 {

    //TODO: ensure routine is NOT idempotent, return error if already initialized
    ::utils::logger::LoggerUtils::init();

    settings::set_defaults();

    if !config_path.is_null() {
        check_useful_c_str!(config_path,error::UNKNOWN_ERROR.code_num);

        if config_path == "ENABLE_TEST_MODE" {
            settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        } else {
            match settings::process_config_file(&config_path) {
                Err(_) => {
                    error!("Invalid configuration specified");
                    return error::INVALID_CONFIGURATION.code_num;
                },
                Ok(_) => info!("Successfully parsed config: {}", config_path),
            };
        }
    }

    let config_name = match settings::get_config_value(settings::CONFIG_POOL_CONFIG_NAME) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let pool_name = match settings::get_config_value(settings::CONFIG_POOL_NAME) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let wallet_name = match settings::get_config_value(settings::CONFIG_WALLET_NAME) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let wallet_type = match settings::get_config_value(settings::CONFIG_WALLET_TYPE) {
        Err(x) => return x,
        Ok(v) => v,
    };

    match pool::create_pool_config(&pool_name, &config_name) {
        0 => 0,
        x => return x,
    };

    match wallet::init_wallet(&pool_name, &wallet_name, &wallet_type) {
        0 => 0,
        x => return x,
    };


    let agency_pairwise_did = match settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_DID) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let agent_pairwise_did = match settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_DID) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let agency_ver_key = match settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let agent_ver_key = match settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let enterprise_did_agency = match settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENCY) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let enterprise_did_agent = match settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT) {
        Err(x) => return x,
        Ok(v) => v,
    };

    let enterprise_name = match settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME) {
        Err(x) => return x,
        Ok(v) => v,
    };
    //
//        let logo_url = match settings::get_config_value(settings::CONFIG_LOGO_URL) {
//            Err(x) => return x,
//            Ok(v) => v,
//        };

    return error::SUCCESS.code_num
}




/**
 * Schema object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_create(schema_data: *const c_char, schema_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_commit(schema_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_schema_get_data(schema_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_schema_get_sequence_no(schema_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }


/**
 * claimdef object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_create(schema_handle: u32, claimdef_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }


/**
 * connection object
 */

#[no_mangle]
pub extern fn cxs_connection_create(recipient_info: *const c_char, connection_handle: *mut u32) -> u32 {
    check_useful_c_str!(recipient_info, error::UNKNOWN_ERROR.code_num);

    if connection_handle.is_null() {return error::UNKNOWN_ERROR.code_num}

    let handle = build_connection(recipient_info.to_owned());

    unsafe { *connection_handle = handle }

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_connect(connection_handle: u32, connection_type: *const c_char) -> u32 {
    connect(connection_handle)
}

#[no_mangle]
pub extern fn cxs_connection_get_data(connection_handle: u32) -> *mut c_char {
    let json_string = to_string(connection_handle);

    if json_string.is_empty() {
        return ptr::null_mut()
    }
    else {
        let msg = CStringUtils::string_to_cstring(json_string);

        msg.into_raw()
    }
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

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_connection_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }


/**
 * claim object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_create(claimdef_handle: u32, claim_data: *const c_char, claim_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_claim_set_connection(claim_handle: u32, connection_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send_offer(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_send(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_claim_terminate(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claim_get_state(claim_handle: u32, status: *mut c_char) -> u32 { error::SUCCESS.code_num }


/**
 * proof object
 */

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(proof_request_data: *mut c_char, proof_handle: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(proof_handle: u32, connection_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(proof_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_state(proof_handle: u32, status: *mut c_char) -> u32 { error::SUCCESS.code_num }




#[cfg(test)]
mod tests {

    use super::*;
    use api::CxsStateType;
    use std::path::Path;
    use std::ffi::CString;
    use std::error::Error;
    use std::io::prelude::*;
    use utils::wallet;
    use utils::pool;
    use std::thread;
    use std::time::Duration;
    use mockito;
    use std::fs;

    #[test]
    fn test_init_with_file() {
        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"my_config\", \"wallet_name\":\"my_wallet\", \
        \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"enterprise\",\
        \"agency_pairwise_verkey\" : \"7118p4HubxzUK1dwxcc5FU\", \"agent_pairwise_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\"}";
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        let result = cxs_init(CString::new(config_path).unwrap().into_raw());
        assert_eq!(result,0);
        // Leave file around or other concurrent tests will fail
//        fs::remove_file(config_path).unwrap();
        wallet::tests::delete_wallet("my_wallet");
        pool::delete_pool_config("my_config");
    }


    #[test]
    fn test_init_bad_path() {
        let empty_str = CString::new("").unwrap().into_raw();
        assert_eq!(error::UNKNOWN_ERROR.code_num,cxs_init(empty_str));
    }

    #[test]
    fn test_init_no_config_path() {
        let result = cxs_init(ptr::null());
        assert_eq!(result,0);
        wallet::tests::delete_wallet("wallet1");
        pool::delete_pool_config("config1");
    }


    #[test]
    fn test_cxs_connection_create() {
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_create").unwrap().into_raw(), &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);
    }

    #[test]
    fn test_cxs_connection_create_fails() {
        let rc = cxs_connection_create(CString::new("test_create_fails").unwrap().into_raw(), ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);

        let rc = cxs_connection_create(ptr::null(),ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    fn test_cxs_connection_connect() {
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,mockito::SERVER_URL);
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(3)
            .create();

        wallet::tests::make_wallet("test_cxs_connection_connect");
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_cxs_connection_connect").unwrap().into_raw(), &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(2));
        assert!(handle > 0);

        let rc = cxs_connection_connect(handle, CString::new("QR").unwrap().into_raw());
        assert_eq!(rc, error::SUCCESS.code_num);
        wallet::tests::delete_wallet("test_cxs_connection_connect");
        _m.assert();
    }

    #[test]
    fn test_cxs_connection_connect_fails() {
        let rc = cxs_connection_connect(0, CString::new("QR").unwrap().into_raw());
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_cxs_connection_get_state() {
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_state").unwrap().into_raw(), &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let mut state: u32 = 0;
        let rc = cxs_connection_get_state(handle, &mut state);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(state,CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_cxs_connection_get_state_fails() {
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_state_fails").unwrap().into_raw(), &mut handle);
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
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_get_data").unwrap().into_raw(), &mut handle);
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
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_release").unwrap().into_raw(), &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let rc = cxs_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let rc = cxs_connection_connect(handle, CString::new("QR").unwrap().into_raw());
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

}
