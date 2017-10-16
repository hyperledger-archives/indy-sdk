extern crate libc;

use self::libc::c_char;
use api::CxsStatus;
use utils::cstring::CStringUtils;
use utils::{pool, wallet};
use utils::error;
use std::ptr;
use settings;
use connection::{build_connection, connect, to_string, get_state, release};

#[no_mangle]
pub extern fn cxs_init (config_path:*const c_char) -> u32 {

    ::utils::logger::LoggerUtils::init();

    settings::set_defaults();

    if !config_path.is_null() {
        check_useful_c_str!(config_path,error::UNKNOWN_ERROR.code_num);

        match settings::process_config_file(&config_path) {
            Err(_) => {
                error!("Invalid configuration specified");
                return error::INVALID_CONFIGURATION.code_num;
            },
            Ok(_) => info!("Successfully parsed config: {}",config_path),
        };
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

    let agency_pairwise_did = match settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_DID) {
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

    let logo_url = match settings::get_config_value(settings::CONFIG_LOGO_URL) {
        Err(x) => return x,
        Ok(v) => v,
    };

    match pool::create_pool_config(&pool_name, &config_name) {
        0 => 0,
        x => return x,
    };

    match wallet::create_wallet(&pool_name, &wallet_name, &wallet_type) {
        0 => 0,
        x => return x,
    };

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
pub extern fn cxs_connection_connect(connection_handle: u32) -> u32 {
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
        \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \
        \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"enterprise\",  \"logo_url\" : \"https://s19.postimg.org/ykyz4x8jn/evernym.png\"}";
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        let result = cxs_init(CString::new(config_path).unwrap().into_raw());
        assert_eq!(result,0);
        // Leave file around or other concurrent tests will fail
        //fs::remove_file(config_path).unwrap();
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
        let mut handle: u32 = 0;
        let rc = cxs_connection_create(CString::new("test_connect").unwrap().into_raw(), &mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);

        let rc = cxs_connection_connect(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_cxs_connection_connect_fails() {
        let rc = cxs_connection_connect(0);
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
        assert_eq!(state,CxsStateType::CxsStateInitialized as u32);
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
        let rc = cxs_connection_connect(handle);
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }
}
