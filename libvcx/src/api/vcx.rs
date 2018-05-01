extern crate libc;
extern crate serde_json;

use utils::version_constants;
use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::libindy::{wallet, pool};
use utils::error;
use settings;
use std::thread;
use std::ffi::CString;


/// Initializes VCX with config settings
///
/// example configuration is in libvcx/sample_config/config.json
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
pub extern fn vcx_init_with_config(command_handle: u32,
                                   config: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: u32, err:u32)>) -> u32 {

    check_useful_c_str!(config,error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if config == "ENABLE_TEST_MODE" {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
    } else {
        match settings::process_config_string(&config) {
            Err(e) => {
                println!("Invalid configuration specified: {}", e);
                return error::INVALID_CONFIGURATION.code_num;
            },
            Ok(_) => (),
        }
    };

    _finish_init(command_handle, cb)
}

/// Initializes VCX with config file
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

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

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
            info!("config_path: {}", config_path);
        }
    } else {
        error!("Cannot initialize with given config path: config path is null.");
        return error::INVALID_CONFIGURATION.code_num;
    }

    _finish_init(command_handle, cb)
}

fn _finish_init(command_handle: u32, cb: extern fn(xcommand_handle: u32, err: u32)) -> u32 {

    ::utils::logger::LoggerUtils::init();

    settings::set_defaults();

    if wallet::get_wallet_handle() > 0 {
        error!("Library was already initialized");
        return error::ALREADY_INITIALIZED.code_num;
    }

    info!("libvcx version: {}{}", version_constants::VERSION, version_constants::REVISION);

    thread::spawn(move|| {
        match ::utils::libindy::init_pool_and_wallet() {
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

lazy_static!{
    pub static ref VERSION_STRING: CString = CString::new(format!("{}{}", version_constants::VERSION, version_constants::REVISION)).unwrap();
}

#[no_mangle]
pub extern fn vcx_version() -> *const c_char {
    VERSION_STRING.as_ptr()
}

#[no_mangle]
pub extern fn vcx_reset() -> u32 {

    ::schema::release_all();
    ::connection::release_all();
    ::issuer_credential::release_all();
    ::credential_def::release_all();
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
pub extern fn vcx_error_c_message(error_code: u32) -> *const c_char {
    info!("vcx_error_message(error_code: {})", error_code);
    error::error_c_message(&error_code).as_ptr()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;
    use std::ptr;
    use error::*;
    use error::proof::ProofError;

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
        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"config1\", \"wallet_name\":\"my_wallet\", \
        \"agency_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"remote_to_sdk_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"sdk_to_remote_did\" : \"AB3JM851T4EQmhh8CdagSP\", \"institution_name\" : \"evernym enterprise\",\
        \"agency_verkey\" : \"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE\", \"remote_to_sdk_verkey\" : \"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE\"}";

        settings::write_config_to_file(content, config_path).unwrap();

        let result = vcx_init(0,CString::new(config_path).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
        // Leave file around or other concurrent tests will fail

        // cleanup
        wallet::delete_wallet("my_wallet").unwrap();
        settings::tests::remove_file_if_exists(settings::DEFAULT_GENESIS_PATH);
    }

    #[test]
    fn test_init_with_config() {
        let wallet_name = "test_init_with_config";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        // make sure there's a valid wallet and pool before trying to use them.
        ::utils::devsetup::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"config1\", \"wallet_name\":\"test_init_with_config\", \
        \"agency_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"remote_to_sdk_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"sdk_to_remote_did\" : \"AB3JM851T4EQmhh8CdagSP\", \"institution_name\" : \"evernym enterprise\",\
        \"agency_verkey\" : \"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE\", \"remote_to_sdk_verkey\" : \"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE\"}";

        let result = vcx_init_with_config(0,CString::new(content).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        ::utils::devsetup::cleanup_dev_env(wallet_name);
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
        wallet::delete_wallet(settings::DEFAULT_WALLET_NAME).unwrap();
    }

    #[test]
    fn test_reset() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let req_attr = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
        let req_predicates = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";

        wallet::init_wallet("wallet1").unwrap();
        let connection = ::connection::build_connection("h1").unwrap();
        let credential = ::issuer_credential::issuer_credential_create(0,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned()).unwrap();
        let proof = ::proof::create_proof("1".to_string(),req_attr.to_owned(),req_predicates.to_owned(),"Optional".to_owned()).unwrap();
        let credentialdef = ::credential_def::create_new_credentialdef("SID".to_string(),"NAME".to_string(),15,"4fUDR9R7fjwELRvH9JT6HH".to_string(),false).unwrap();
        let schema = ::schema::create_new_schema("5", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        vcx_reset();
        assert_eq!(::connection::release(connection),Err(connection::ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(::issuer_credential::release(credential),Err(issuer_cred::IssuerCredError::InvalidHandle()));
        assert_eq!(::schema::release(schema).err(),Some(schema::SchemaError::InvalidHandle()));
        assert_eq!(::proof::release(proof).err(),Some(ProofError::InvalidHandle()));
        assert_eq!(::credential_def::release(credentialdef),error::INVALID_CREDENTIAL_DEF_HANDLE.code_num);
        assert_eq!(wallet::get_wallet_handle(), 0);
    }

    #[test]
    fn test_error_c_message() {
        settings::set_defaults();
        let c_message = CStringUtils::c_str_to_string(vcx_error_c_message(0)).unwrap().unwrap();
        assert_eq!(c_message,error::SUCCESS.message);

        let c_message = CStringUtils::c_str_to_string(vcx_error_c_message(1001)).unwrap().unwrap();
        assert_eq!(c_message,error::UNKNOWN_ERROR.message);

        let c_message = CStringUtils::c_str_to_string(vcx_error_c_message(100100)).unwrap().unwrap();
        assert_eq!(c_message,error::UNKNOWN_ERROR.message);

        let c_message = CStringUtils::c_str_to_string(vcx_error_c_message(1021)).unwrap().unwrap();
        assert_eq!(c_message,error::INVALID_ATTRIBUTES_STRUCTURE.message);
    }

    #[test]
    fn test_vcx_version() {
        let return_version = CStringUtils::c_str_to_string(vcx_version()).unwrap().unwrap();
        assert!(return_version.len() > 5);
    }
}
