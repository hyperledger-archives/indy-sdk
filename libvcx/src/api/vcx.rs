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
        settings::set_defaults();
    } else {
        match settings::process_config_string(&config) {
            Err(e) => {
                println!("Invalid configuration specified: {}", e);
                return e;
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
            settings::set_defaults();
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
    match ::utils::libindy::payments::init_payments() {
        Ok(_) => (),
        Err(x) => return x,
    };

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

    ::schema::release_all();
    ::connection::release_all();
    ::issuer_credential::release_all();
    ::credential_def::release_all();
    ::proof::release_all();
    ::disclosed_proof::release_all();
    ::credential::release_all();

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

    settings::clear_config();
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

#[no_mangle]
pub extern fn vcx_mint_tokens(number_of_addresses: u32, tokens_per_address: u32) {
    let ledger_fees = r#"{"101":2, "102":3}"#;
    info!("vcx_mint_tokens(number_of_addresses: {}, tokens_per_address: {})", number_of_addresses, tokens_per_address);
    ::utils::libindy::payments::mint_tokens(Some(number_of_addresses), Some(tokens_per_address)).unwrap_or_default();
    ::utils::libindy::payments::set_ledger_fees(Some(ledger_fees.to_string())).unwrap_or_default();
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;
    use std::ptr;

    extern "C" fn init_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("create_cb failed: {}", err)}
    }

    #[test]
    fn test_init_with_file() {
        let wallet_name = "test_init_with_file";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let config_path = "/tmp/test_init.json";
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_init_with_file",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"<KEY_IS_NOT_SET>"
        }).to_string();

        settings::write_config_to_file(&content, config_path).unwrap();

        let result = vcx_init(0,CString::new(config_path).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_init_with_config() {
        let wallet_name = "test_init_with_config";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_init_with_config",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"<KEY_IS_NOT_SET>"
        }).to_string();

        let result = vcx_init_with_config(0,CString::new(content).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_vcx_init_with_default_values() {
        let wallet_name = "test_vcx_init_with_default_values";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = "{}".to_string();

        let result = vcx_init_with_config(0,CString::new(content).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_vcx_init_called_twice_fails() {
        let wallet_name = "test_vcx_init_called_twice_fails";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = "{}";

        let result = vcx_init_with_config(0,CString::new(content).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        // Repeat call
        let result = vcx_init_with_config(0,CString::new(content).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,error::ALREADY_INITIALIZED.code_num);
        thread::sleep(Duration::from_secs(1));

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_vcx_init_called_twice_passes_after_shutdown() {
        let wallet_name = "test_vcx_init_called_twice_fails";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = json!({"wallet_name": wallet_name, "pool_name":"pool232"});

        let result = vcx_init_with_config(0,CString::new(content.to_string()).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        //Assert config values were set correctly
        assert_eq!(settings::get_config_value("wallet_name").unwrap(), wallet_name.to_string());
        assert_eq!(settings::get_config_value("pool_name").unwrap(), "pool232".to_string());

        //Verify shutdown was successful
        vcx_shutdown(true);
        assert_eq!(settings::get_config_value("wallet_name"), Err(error::INVALID_CONFIGURATION.code_num));
        assert_eq!(settings::get_config_value("pool_name"), Err(error::INVALID_CONFIGURATION.code_num));

        // Init for the second time works
        ::utils::devsetup::tests::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        let result = vcx_init_with_config(0,CString::new(content.to_string()).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));

        vcx_shutdown(true);
    }

    #[test]
    fn test_init_fails_with_open_wallet() {
        let wallet_name = "test_init_fails_with_open_wallet";
        ::utils::devsetup::tests::setup_dev_env(wallet_name);


        let config_path = "/tmp/test_init.json";
        let content = json!({
            "pool_name" : "pool1",
            "config_name":"config1",
            "wallet_name":"test_init_fails_with_open_wallet",
            "agency_did" : "72x8p4HubxzUK1dwxcc5FU",
            "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
            "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
            "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "institution_name" : "evernym enterprise",
            "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "genesis_path":"/tmp/pool1.txn",
            "wallet_key":"<KEY_IS_NOT_SET>"
        }).to_string();

        settings::write_config_to_file(&content, config_path).unwrap();

        let result = vcx_init(0,CString::new(config_path).unwrap().into_raw(),Some(init_cb));
        assert_eq!(result,error::ALREADY_INITIALIZED.code_num);
        thread::sleep(Duration::from_secs(1));
        // Leave file around or other concurrent tests will fail

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
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
        ::utils::devsetup::tests::set_institution_dev_config("test_shutdown");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_WALLET_KEY,"default");
        settings::set_config_value(settings::CONFIG_LINK_SECRET_ALIAS, settings::DEFAULT_LINK_SECRET_ALIAS);

        let data = r#"["name","male"]"#;
        let connection = ::connection::build_connection("h1").unwrap();
        let issuer_credential = ::issuer_credential::issuer_credential_create("cred_id".to_string(),"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let proof = ::proof::create_proof("1".to_string(),"[]".to_string(), "[]".to_string(),"Optional".to_owned()).unwrap();
        let credentialdef = ::credential_def::create_new_credentialdef("SID".to_string(),"NAME".to_string(),"4fUDR9R7fjwELRvH9JT6HH".to_string(), "id".to_string(), "tag".to_string(),"{}".to_string() ).unwrap();
        let schema = ::schema::create_new_schema("5",  "VsKV7grR1BUE29mG2Fm2kX".to_string(),"name".to_string(), "0.1".to_string(), data.to_string()).unwrap();
        let disclosed_proof = ::disclosed_proof::create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        let credential = ::credential::credential_create_with_offer("name", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();

        vcx_shutdown(true);
        assert_eq!(::connection::release(connection),Err(::error::connection::ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(::issuer_credential::release(issuer_credential),Err(::error::issuer_cred::IssuerCredError::InvalidHandle()));
        assert_eq!(::schema::release(schema).err(),Some(::error::schema::SchemaError::InvalidHandle()));
        assert_eq!(::proof::release(proof).err(),Some(::error::proof::ProofError::InvalidHandle()));
        assert_eq!(::credential_def::release(credentialdef),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
        assert_eq!(::credential::release(credential), Err(::error::credential::CredentialError::CommonError(error::INVALID_CREDENTIAL_HANDLE.code_num)));
        assert_eq!(::disclosed_proof::release(disclosed_proof), Result::Err(error::INVALID_DISCLOSED_PROOF_HANDLE.code_num));
        assert_eq!(wallet::get_wallet_handle(), 0);

        ::utils::devsetup::tests::setup_dev_env("test_shutdown");
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
