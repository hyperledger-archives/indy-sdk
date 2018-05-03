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
    settings::log_settings();

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

/// Reset libvcx to a pre-configured state, releasing/deleting any handles and freeing memory
///
/// libvcx will be inoperable and must be initialized again with vcx_init_with_config
///
/// #Params
/// delete: specify whether wallet/pool should be deleted
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_shutdown(delete: bool) -> u32 {

    match wallet::close_wallet() {
        Ok(_) => {},
        Err(_) => {},
    };

    match pool::close() {
        Ok(_) => {},
        Err(_) => {},
    };

    if delete {
        match settings::get_config_value(settings::CONFIG_WALLET_NAME) {
            Ok(w) => match wallet::delete_wallet(&w) {
                Ok(_) => (),
                Err(_) => (),
            },
            Err(_) => (),
        };

        match settings::get_config_value(settings::CONFIG_POOL_NAME) {
            Ok(p) => match pool::delete(&p) {
                Ok(_) => (),
                Err(_) => (),
            }
            Err(_) => (),
        }
    }

    settings::set_defaults();
    info!("vcx_shutdown(delete: {})", delete);
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_error_c_message(error_code: u32) -> *const c_char {
    info!("vcx_error_message(error_code: {})", error_code);
    error::error_c_message(&error_code).as_ptr()
}

#[no_mangle]
pub extern fn vcx_update_institution_info(name: *const c_char, logo_url: *const c_char) -> u32 {
    check_useful_c_str!(name, error::INVALID_CONFIGURATION.code_num);
    check_useful_c_str!(logo_url, error::INVALID_CONFIGURATION.code_num);
    info!("vcx_update_institution_info(name: {}, logo_url: {})", name, logo_url);

    settings::set_config_value(::settings::CONFIG_INSTITUTION_NAME, &name);
    settings::set_config_value(::settings::CONFIG_INSTITUTION_LOGO_URL, &logo_url);

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;
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
    fn test_shutdown() {
        ::utils::devsetup::setup_dev_env("test_shutdown");
        vcx_shutdown(true);
        ::utils::devsetup::setup_dev_env("test_shutdown");
        vcx_shutdown(true);
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

    #[test]
    fn test_vcx_update_institution_info() {
        settings::set_defaults();
        let new_name = "new_name";
        let new_url = "http://www.evernym.com";
        assert_ne!(new_name, &settings::get_config_value(::settings::CONFIG_INSTITUTION_NAME).unwrap());
        assert_ne!(new_url, &settings::get_config_value(::settings::CONFIG_INSTITUTION_LOGO_URL).unwrap());

        assert_eq!(error::SUCCESS.code_num, vcx_update_institution_info(CString::new(new_name.to_string()).unwrap().into_raw(),
                                                                        CString::new(new_url.to_string()).unwrap().into_raw()));

        assert_eq!(new_name, &settings::get_config_value(::settings::CONFIG_INSTITUTION_NAME).unwrap());
        assert_eq!(new_url, &settings::get_config_value(::settings::CONFIG_INSTITUTION_LOGO_URL).unwrap());
        ::settings::set_defaults();
    }
}
