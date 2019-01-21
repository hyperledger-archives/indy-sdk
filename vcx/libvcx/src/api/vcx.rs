extern crate libc;
extern crate serde_json;

use utils::version_constants;
use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::libindy::{wallet, pool};
use utils::error;
use settings;
use std::ffi::CString;
use utils::threadpool::spawn;


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
    info!("vcx_init_with_config >>>");

    check_useful_c_str!(config,error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    trace!("vcx_init(command_handle: {}, config: {:?})",
           command_handle, config);

    if config == "ENABLE_TEST_MODE" {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_defaults();
    } else {
        match settings::process_config_string(&config) {
            Err(e) => {
                error!("Invalid configuration specified: {}", e);
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
    info!("vcx_init >>>");

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    trace!("vcx_init(command_handle: {}, config_path: {:?})",
           command_handle, config_path);


    if !config_path.is_null() {
        check_useful_c_str!(config_path,error::INVALID_OPTION.code_num);

        if config_path == "ENABLE_TEST_MODE" {
            settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
            settings::set_defaults();
        } else {
            match settings::process_config_file(&config_path) {
                Err(e) => {
                    return error::INVALID_CONFIGURATION.code_num;
                },
                Ok(_) => {
                    match settings::validate_payment_method() {
                        Ok(_) => (),
                        Err(e) => return e
                    }
                },
            };
        }
    } else {
        error!("Cannot initialize with given config path: config path is null.");
        return error::INVALID_CONFIGURATION.code_num;
    }

    _finish_init(command_handle, cb)
}

fn _finish_init(command_handle: u32, cb: extern fn(xcommand_handle: u32, err: u32)) -> u32 {

    ::utils::threadpool::init();

    settings::log_settings();

    if wallet::get_wallet_handle() > 0 {
        error!("Library was already initialized");
        return error::ALREADY_INITIALIZED.code_num;
    }
    // Wallet name was already validated
    let wallet_name = match settings::get_config_value(settings::CONFIG_WALLET_NAME) {
        Ok(x) => x,
        Err(_) => {
            trace!("Using default wallet: {}", settings::DEFAULT_WALLET_NAME.to_string());
            settings::set_config_value(settings::CONFIG_WALLET_NAME, settings::DEFAULT_WALLET_NAME);
            settings::DEFAULT_WALLET_NAME.to_string()
        }
    };

    let wallet_type = settings::get_config_value(settings::CONFIG_WALLET_TYPE).ok();

    trace!("libvcx version: {}{}", version_constants::VERSION, version_constants::REVISION);

    spawn(move|| {
        if settings::get_config_value(settings::CONFIG_GENESIS_PATH).is_ok() {
            match ::utils::libindy::init_pool() {
                Ok(_) => (),
                Err(e) => {
                    error!("Init Pool Error {}.", e);
                    return Ok(cb(command_handle, e))
                },
            }
        }

        match wallet::open_wallet(&wallet_name, wallet_type.as_ref().map(String::as_str)) {
            Ok(_) => {
                debug!("Init Wallet Successful");
                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(e) => {
                error!("Init Wallet Error {}.", e);
                cb(command_handle, e);
            }
        }
        Ok(())
    });

    error::SUCCESS.code_num
}

lazy_static!{
    pub static ref VERSION_STRING: CString = CString::new(format!("{}{}", version_constants::VERSION, version_constants::REVISION)).unwrap();
}

#[no_mangle]
pub extern fn vcx_version() -> *const c_char {
    info!("vcx_version >>>");
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
    info!("vcx_shutdown >>>");
    trace!("vcx_shutdown(delete: {})", delete);

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
        let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)
            .unwrap_or(settings::DEFAULT_POOL_NAME.to_string());

        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME)
            .unwrap_or(settings::DEFAULT_WALLET_NAME.to_string());

        let wallet_type = settings::get_config_value(settings::CONFIG_WALLET_TYPE).ok();

        match wallet::delete_wallet(&wallet_name, wallet_type.as_ref().map(String::as_str)) {
            Ok(_) => (),
            Err(_) => (),
        };

        match pool::delete(&pool_name) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    settings::clear_config();
    trace!("vcx_shutdown(delete: {})", delete);
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_error_c_message(error_code: u32) -> *const c_char {
    info!("vcx_error_c_message >>>");
    trace!("vcx_error_message(error_code: {})", error_code);
    error::error_c_message(&error_code).as_ptr()
}

#[no_mangle]
pub extern fn vcx_update_institution_info(name: *const c_char, logo_url: *const c_char) -> u32 {
    info!("vcx_update_institution_info >>>");

    check_useful_c_str!(name, error::INVALID_CONFIGURATION.code_num);
    check_useful_c_str!(logo_url, error::INVALID_CONFIGURATION.code_num);
    trace!("vcx_update_institution_info(name: {}, logo_url: {})", name, logo_url);

    settings::set_config_value(::settings::CONFIG_INSTITUTION_NAME, &name);
    settings::set_config_value(::settings::CONFIG_INSTITUTION_LOGO_URL, &logo_url);

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_mint_tokens(seed: *const c_char, fees: *const c_char) {
    info!("vcx_mint_tokens >>>");

    let seed = if !seed.is_null() {
        check_useful_opt_c_str!(seed, ());
        seed.to_owned()
    }
    else {
        None
    };

    let fees = if !fees.is_null() {
        check_useful_opt_c_str!(fees, ());
        fees.to_owned()
    }
    else {
        None
    };
    trace!("vcx_mint_tokens(seed: {:?}, fees: {:?})", seed, fees);

    ::utils::libindy::payments::mint_tokens_and_set_fees(None, None, fees, seed).unwrap_or_default();
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;
    use std::ptr;
    use std::thread;
    use utils::{
        libindy::{
            wallet::{import, tests::export_test_wallet, tests::delete_import_wallet_path},
            pool::get_pool_handle
        },
        get_temp_dir_path
    };
    use api::return_types_u32;

    fn create_config_util(logging: Option<&str>) -> String {
        json!({"agency_did" : "72x8p4HubxzUK1dwxcc5FU",
               "remote_to_sdk_did" : "UJGjM6Cea2YVixjWwHN9wq",
               "sdk_to_remote_did" : "AB3JM851T4EQmhh8CdagSP",
               "sdk_to_remote_verkey" : "888MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
               "institution_name" : "evernym enterprise",
               "agency_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
               "remote_to_sdk_verkey" : "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
               "genesis_path": get_temp_dir_path(Some("pool1.txn")).to_str().unwrap(),
               "payment_method": "null"}).to_string()
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_with_file() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let config_path_buf = get_temp_dir_path(Some("test_init.json"));
        let config_path = config_path_buf.to_str().unwrap();
        let content = create_config_util(Some("true"));
        settings::write_config_to_file(&content, config_path).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init(cb.command_handle,
                            CString::new(config_path).unwrap().into_raw(),
                            Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
        // Assert pool was initialized
        assert_ne!(get_pool_handle().unwrap(), 0);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_with_file_no_payment_method() {
        init!("false");
        settings::clear_config();

        let config_path_buf = get_temp_dir_path(Some("test_init.json"));
        let config_path = config_path_buf.to_str().unwrap();
        let content = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        }).to_string();

        settings::write_config_to_file(&content, config_path).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init(cb.command_handle,
                            CString::new(config_path).unwrap().into_raw(),
                            Some(cb.get_callback())),
                   error::MISSING_PAYMENT_METHOD.code_num);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_with_config() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = create_config_util(None);

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                          CString::new(content).unwrap().into_raw(),
                                          Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
        // Assert pool was initialized
        assert_ne!(get_pool_handle().unwrap(), 0);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_fails_when_open_pool_fails() {
        settings::set_defaults();
        vcx_shutdown(true);
        use std::fs;
        use std::io::Write;
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY,settings::DEFAULT_WALLET_KEY);

        // Write invalid genesis.txn
        let mut f = fs::File::create(get_temp_dir_path(Some(::utils::constants::GENESIS_PATH)).to_str().unwrap()).unwrap();
        f.write_all("{}".as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let wallet_name = "test_init_fails_when_open_pool_fails";
        wallet::create_wallet(wallet_name, None).unwrap();

        let content = create_config_util(None);

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let rc = cb.receive(Some(Duration::from_secs(10)));
        thread::sleep(Duration::from_secs(1));
        assert!(rc.is_err());
        assert_eq!(get_pool_handle(), Err(error::NO_POOL_OPEN.code_num));
        assert_eq!(wallet::get_wallet_handle(), 0);
        wallet::delete_wallet(wallet_name, None).unwrap();
    }

    #[test]
    fn test_init_can_be_called_with_no_pool_config() {
        init!("false");
        wallet::close_wallet().unwrap();

        let content = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        }).to_string();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // assert that pool was never initialized
        assert!(get_pool_handle().is_err());
    }

    #[test]
    fn test_init_fails_with_no_wallet_key() {
        settings::set_defaults();
        vcx_shutdown(true);
        let content = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
        }).to_string();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::MISSING_WALLET_KEY.code_num);
    }

    #[test]
    fn test_config_with_no_wallet_uses_default() {
        init!("false");

        vcx_shutdown(false);
        thread::sleep(Duration::from_secs(1));
        assert!(settings::get_config_value(settings::CONFIG_WALLET_NAME).is_err());

        let content = json!({
            "wallet_key": "key",
        }).to_string();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let err = cb.receive(Some(Duration::from_secs(10)));
        // Assert default wallet name
        assert_eq!(settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap(), settings::DEFAULT_WALLET_NAME);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_init_with_default_values() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = "{}".to_string();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_init_called_twice_fails() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = "{}";

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // Repeat call
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::ALREADY_INITIALIZED.code_num);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_init_called_twice_passes_after_shutdown() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();

        let content = format!(r#"{{"wallet_name":"{}"}}"#, settings::DEFAULT_WALLET_NAME);

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content.clone()).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        //Assert config values were set correctly
        assert_eq!(settings::get_config_value("wallet_name").unwrap(), settings::DEFAULT_WALLET_NAME);

        //Verify shutdown was successful
        vcx_shutdown(true);
        assert_eq!(settings::get_config_value("wallet_name"), Err(error::INVALID_CONFIGURATION.code_num));

        // Init for the second time works
        ::utils::devsetup::tests::setup_ledger_env();
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        vcx_shutdown(true);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_fails_with_open_wallet() {
        init!("ledger");

        let config_path_buf = get_temp_dir_path(Some("test_init.json"));
        let config_path = config_path_buf.to_str().unwrap();
        let content = create_config_util(None);
        settings::write_config_to_file(&content, config_path).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init(cb.command_handle,
                                        CString::new(config_path).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::ALREADY_INITIALIZED.code_num);
    }

    #[test]
    fn test_init_after_importing_wallet_success() {
        settings::set_defaults();
        teardown!("false");

        let export_path = export_test_wallet();

        vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_WALLET_KEY_DERIVATION: settings::DEFAULT_WALLET_KEY_DERIVATION,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
        }).to_string();
        import(&import_config).unwrap();

        let content = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        }).to_string();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        delete_import_wallet_path(export_path);
        vcx_shutdown(true);
    }

    #[test]
    fn test_init_with_imported_wallet_fails_with_different_params() {
        settings::set_defaults();
        teardown!("false");

        let export_path = export_test_wallet();

        vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_WALLET_KEY_DERIVATION: settings::DEFAULT_WALLET_KEY_DERIVATION,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();
        import(&import_config).unwrap();

        let content = json!({
            "wallet_name": "different_wallet_name",
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        }).to_string();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(error::WALLET_NOT_FOUND.code_num));

        delete_import_wallet_path(export_path);
        settings::set_config_value(settings::CONFIG_WALLET_NAME,settings::DEFAULT_WALLET_NAME);
        vcx_shutdown(true);
    }

    #[test]
    fn test_import_after_init_fails() {
        settings::set_defaults();
        teardown!("false");

        let export_path = export_test_wallet();

        vcx_shutdown(false);

        let content = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "wallet_key_derivation": settings::DEFAULT_WALLET_KEY_DERIVATION,
        }).to_string();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();
        assert_eq!(import(&import_config), Err(::error::wallet::WalletError::CommonError(error::WALLET_ALREADY_EXISTS.code_num)));

        delete_import_wallet_path(export_path);
        vcx_shutdown(true);
    }

    #[test]
    fn test_init_bad_path() {
        use utils::libindy::pool::get_pool_handle;
        init!("false");
        let config_path = "";
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init(cb.command_handle,
                                        CString::new(config_path).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::INVALID_OPTION.code_num);

        match get_pool_handle() {
            Ok(h) => {pool::close().unwrap();},
            Err(_) => {},
        };
    }

    // this test now fails, you must provide a path to a valid config
    #[test]
    fn test_init_no_config_path() {
        init!("true");
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init(cb.command_handle,
                                        ptr::null(),
                                        Some(cb.get_callback())),
                   error::INVALID_CONFIGURATION.code_num);
    }

    #[test]
    fn test_shutdown_with_no_previous_config() {
        vcx_shutdown(true);
        vcx_shutdown(false);
    }

    #[test]
    fn test_shutdown() {
        init!("true");

        let data = r#"["name","male"]"#;
        let connection = ::connection::tests::build_test_connection();
        let credentialdef = ::credential_def::create_new_credentialdef("SID".to_string(),"NAME".to_string(),"4fUDR9R7fjwELRvH9JT6HH".to_string(), "id".to_string(), "tag".to_string(),"{}".to_string() ).unwrap();
        let issuer_credential = ::issuer_credential::issuer_credential_create(credentialdef,"1".to_string(),"8XFh8yBzrpJQmNyZzgoTqB".to_owned(),"credential_name".to_string(),"{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let proof = ::proof::create_proof("1".to_string(),"[]".to_string(), "[]".to_string(),r#"{"support_revocation":false}"#.to_string(), "Optional".to_owned()).unwrap();
        let schema = ::schema::create_new_schema("5",  "VsKV7grR1BUE29mG2Fm2kX".to_string(),"name".to_string(), "0.1".to_string(), data.to_string()).unwrap();
        let disclosed_proof = ::disclosed_proof::create_proof("id",::utils::constants::PROOF_REQUEST_JSON).unwrap();
        let credential = ::credential::credential_create_with_offer("name", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();

        vcx_shutdown(true);
        assert_eq!(::connection::release(connection),Err(::error::connection::ConnectionError::CommonError(error::INVALID_CONNECTION_HANDLE.code_num)));
        assert_eq!(::issuer_credential::release(issuer_credential),Err(::error::issuer_cred::IssuerCredError::InvalidHandle()));
        assert_eq!(::schema::release(schema).err(),Some(::error::schema::SchemaError::InvalidHandle()));
        assert_eq!(::proof::release(proof).err(),Some(::error::proof::ProofError::InvalidHandle()));
        assert_eq!(::credential_def::release(credentialdef),Err(::error::cred_def::CredDefError::InvalidHandle()));
        assert_eq!(::credential::release(credential), Err(::error::credential::CredentialError::CommonError(error::INVALID_CREDENTIAL_HANDLE.code_num)));
        assert_eq!(::disclosed_proof::release(disclosed_proof), Result::Err(error::INVALID_DISCLOSED_PROOF_HANDLE.code_num));
        assert_eq!(wallet::get_wallet_handle(), 0);
    }

    #[test]
    fn test_error_c_message() {
        init!("true");
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
        init!("true");
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

    // This test is ignored because it sets up logging, which can only be done
    // once per process.
    #[ignore]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_with_logging_config() {
        init!("ledger");
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        let content = create_config_util(Some("debug"));
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle,
                                        CString::new(content).unwrap().into_raw(),
                                        Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_ne!(get_pool_handle().unwrap(), 0);
        debug!("This statement should log");
    }
}
