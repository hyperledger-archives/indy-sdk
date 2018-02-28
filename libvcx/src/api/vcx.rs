extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::libindy::{wallet, pool};
use utils::error;
use utils::version_constants::{VERSION, REVISION};
use settings;
use std::thread;
use std::path::Path;

/// Initializes VCX with config file
///
/// Possible values in the Config file: ->
///
/// pool_name:
///
/// config_name
///
/// wallet_name:
///
/// wallet_type
///
/// agent_endpoint: the url to interact with the agent
///
/// enterprise_did_agency: did for enterprise pairwise relationship with an agency
///
/// agency_pairwise_did: did for the agency pairwise relationship with an enterprise
///
/// agency_pairwise_verkey: verkey for the agency pairwise relationship with an enterprise
///
/// enterprise_did_agent: did for enterprise pairwise relationship with an agent
///
/// agent_pairwise_did: did for the agent pairwise relationship with an enterprise
///
/// agent_pairwise_verkey: verkey for the agent pairwise relationship with an enterprise
///
/// enterprise_name: enterprise's name
///
/// logo_url: url for enterprise's logo
///
/// An example file is at libvcx/sample_config/config.json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// config_path: path to a config file to populate config attributes
///
/// cb: Callback that provides error status of initialization
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_init (command_handle: u32,
                        config_path:*const c_char,
                        cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    if !config_path.is_null() {
        check_useful_c_str!(config_path,error::INVALID_OPTION.code_num);

        if config_path == "ENABLE_TEST_MODE" {
            settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        } else {
            match settings::process_config_file(&config_path) {
                Err(e) => {
                    println!("Invalid configuration specified: {}", e);
                    return error::INVALID_CONFIGURATION.code_num;
                },
                Ok(_) => (),
            };
            ::utils::logger::LoggerUtils::init();
            println!("config_path: {}", config_path);
        }
    } else {
        error!("Cannot initialize with given config path: config path is null.");
        return error::INVALID_CONFIGURATION.code_num;
    }

    settings::set_defaults();

    if wallet::get_wallet_handle() > 0 {
        error!("Library was already initialized");
        return error::ALREADY_INITIALIZED.code_num;
    }

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

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

    info!("libvcx version: {}{}", VERSION, REVISION);

    info!("Initializing wallet with name: {} and pool: {}", &wallet_name, &pool_name);

    if !settings::test_indy_mode_enabled() {
        let path: String = match settings::get_config_value(settings::CONFIG_GENESIS_PATH) {
            Ok(p) => p.clone(),
            Err(e) => {
                error!("Invalid Configuration Genesis Path given");
                return e;
            },
        };

        let option_path = Some(Path::new(&path));
        match pool::create_pool_ledger_config(&pool_name, option_path.to_owned()) {
            Err(e) => {
                warn!("Pool Config Creation Error: {}", e);
                return e;
            },
            Ok(_) => {
                debug!("Pool Config Created Successfully");
                match pool::open_pool_ledger(&pool_name, None) {
                    Err(e) => {
                    warn!("Open Pool Error: {}", e);
                        return e;
                    },
                    Ok(handle) => {
                        debug!("Open Pool Successful");
                    }
                }
            }
        }
    }

    thread::spawn(move|| {
        match wallet::init_wallet(&wallet_name) {
            Err(e) => {
                warn!("Init Wallet Error {}.", e);
                cb(command_handle, e);
            },
            Ok(_) => {
                debug!("Init Wallet Successful");
                cb(command_handle, error::SUCCESS.code_num);
            },
        }
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_reset() -> u32 {

    ::schema::release_all();
    ::connection::release_all();
    ::issuer_claim::release_all();
    ::claim_def::release_all();
    ::proof::release_all();

    match wallet::close_wallet() {
        Ok(_) => {},
        Err(_) => {},
    };

    match pool::close() {
        Ok(_) => {},
        Err(_) => {},
    };

    settings::set_defaults();
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_error_message(command_handle: u32,
                                error_code: u32,
                                cb: Option<extern fn(xcommand_handle: u32, err: u32, msg: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_error_message(command_handle: {}, error_code: {})", command_handle, error_code);
    thread::spawn( move|| {
        let msg = error::error_message(&error_code);
        let c_msg = CStringUtils::string_to_cstring(msg.to_string());
        cb(command_handle, error::SUCCESS.code_num, c_msg.as_ptr());
    });

    error::SUCCESS.code_num
}


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
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::tests::remove_file_if_exists(settings::DEFAULT_GENESIS_PATH);
        settings::tests::create_default_genesis_file();

        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"config1\", \"wallet_name\":\"my_wallet\", \
        \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"evernym enterprise\",\
        \"agency_pairwise_verkey\" : \"7118p4HubxzUK1dwxcc5FU\", \"agent_pairwise_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\"}";
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }

        let result = vcx_init(0,CString::new(config_path).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
        // Leave file around or other concurrent tests will fail
        //fs::remove_file(config_path).unwrap();


        // cleanup
        wallet::delete_wallet("my_wallet").unwrap();
        settings::tests::remove_file_if_exists(settings::DEFAULT_GENESIS_PATH);

    }


    #[test]
    fn test_init_bad_path() {
        use utils::libindy::pool::get_pool_handle;
        let empty_str = CString::new("").unwrap().into_raw();
        assert_eq!(error::INVALID_OPTION.code_num,vcx_init(0,empty_str,Some(init_cb)));

        match get_pool_handle() {
            Ok(h) => {pool::close().unwrap();},
            Err(_) => {},
        };
    }

    // this test now fails, you must provide a path to a valid config
    #[test]
    fn test_init_no_config_path() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let result = vcx_init(0,ptr::null(),Some(init_cb));
        assert_eq!(result,error::INVALID_CONFIGURATION.code_num);
        thread::sleep(Duration::from_secs(1));
        wallet::delete_wallet("wallet1").unwrap();

    }

    #[test]
    fn test_reset() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let req_attr = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
        let req_predicates = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";

        wallet::init_wallet("wallet1").unwrap();
        let connection = ::connection::build_connection("h1".to_owned()).unwrap();
        let claim = ::issuer_claim::issuer_claim_create(0,None,"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"claim_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let proof = ::proof::create_proof(None,req_attr.to_owned(),req_predicates.to_owned(),"Optional".to_owned()).unwrap();
        let claimdef = ::claim_def::create_new_claimdef("SID".to_string(),"NAME".to_string(),15,"4fUDR9R7fjwELRvH9JT6HH".to_string(),false).unwrap();
        let schema = ::schema::create_new_schema("5".to_string(), "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        vcx_reset();
        assert_eq!(::connection::release(connection),error::INVALID_CONNECTION_HANDLE.code_num);
        assert_eq!(::issuer_claim::release(claim),error::INVALID_ISSUER_CLAIM_HANDLE.code_num);
        assert_eq!(::schema::release(schema),error::INVALID_SCHEMA_HANDLE.code_num);
        assert_eq!(::proof::release(proof),error::INVALID_PROOF_HANDLE.code_num);
        assert_eq!(::claim_def::release(claimdef),error::INVALID_CLAIM_DEF_HANDLE.code_num);
        assert_eq!(wallet::get_wallet_handle(), 0);
    }

    extern "C" fn error_msg_cb(handle: u32, err: u32, msg: *const c_char) {
        assert_eq!(err, 0);
        if msg.is_null() {
            panic!("error message is is null");
        }
        check_useful_c_str!(msg, ());
        println!("successfully called error_msg_cb: {}", msg);
    }

    extern "C" fn unknown_error_msg_cb(handle: u32, err: u32, msg: *const c_char) {
        assert_eq!(err, 0);
        if msg.is_null() {
            panic!("error message is is null");
        }
        check_useful_c_str!(msg, ());
        assert_eq!(msg, error::UNKNOWN_ERROR.message);
        println!("successfully called unknown_error_msg_cb: {}", msg);
    }

    #[test]
    fn test_vcx_error_message() {
        settings::set_defaults();
        assert_eq!(vcx_error_message(0, 0, Some(error_msg_cb)), 0);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_error_message_with_invalid_rc() {
        settings::set_defaults();
        assert_eq!(vcx_error_message(0, 9999999, Some(unknown_error_msg_cb)), 0);
        thread::sleep(Duration::from_millis(200));
    }
}
