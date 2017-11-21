extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::{pool, wallet};
use utils::error;
use settings;
use std::thread;

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
pub extern fn cxs_init (command_handle: u32,
                        config_path:*const c_char,
                        cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    ::utils::logger::LoggerUtils::init();

    settings::set_defaults();

    if wallet::get_wallet_handle() > 0 {
        error!("Library was already initialized");
        return error::UNKNOWN_ERROR.code_num;
    }

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !config_path.is_null() {
        check_useful_c_str!(config_path,error::UNKNOWN_ERROR.code_num);

        if config_path == "ENABLE_TEST_MODE" {
            settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        } else {
            info!("config_path: {}", config_path);
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

    let logo_url = match settings::get_config_value(settings::CONFIG_LOGO_URL) {
        Err(x) => return x,
        Ok(v) => v,
    };

    info!("Initializing wallet with name: {} and pool: {}", &wallet_name, &pool_name);

    thread::spawn(move|| {
        /* TODO: handle pool config */
        pool::create_pool_ledger_config(&pool_name,Some(&config_name));
        let wrc = match wallet::init_wallet(&wallet_name, &pool_name, &wallet_type) {
            Ok(x) => error::SUCCESS.code_num,
            Err(x) => x,
        };

        cb(command_handle,wrc);
    });

    error::SUCCESS.code_num
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


#[cfg(test)]
mod tests {

    use super::*;
    use std::path::Path;
    use std::ffi::CString;
    use std::error::Error;
    use std::io::prelude::*;
    use std::time::Duration;
    use std::fs;
    use std::ptr;

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
        println!("successfully called init_cb")
    }

    #[test]
    fn test_init_with_file() {
        wallet::tests::delete_wallet("dummy");
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"\", \"wallet_name\":\"my_wallet\", \
        \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"evernym enterprise\",\
        \"agency_pairwise_verkey\" : \"7118p4HubxzUK1dwxcc5FU\", \"agent_pairwise_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\"}";
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        let result = cxs_init(0,CString::new(config_path).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
        // Leave file around or other concurrent tests will fail
        //fs::remove_file(config_path).unwrap();
    }


    #[test]
    fn test_init_bad_path() {
        let empty_str = CString::new("").unwrap().into_raw();
        assert_eq!(error::UNKNOWN_ERROR.code_num,cxs_init(0,empty_str,Some(init_cb)));
    }

    #[test]
    fn test_init_no_config_path() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let result = cxs_init(0,ptr::null(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
    }
}
