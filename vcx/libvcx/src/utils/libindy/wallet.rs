extern crate libc;
extern crate serde_json;

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use utils::error;
use error::wallet::WalletError;
use indy::wallet::Wallet;
use indy::ErrorCode;
use std::path::Path;
pub static mut WALLET_HANDLE: i32 = 0;

pub fn get_wallet_handle() -> i32 { unsafe { WALLET_HANDLE } }

pub fn create_wallet(wallet_name: &str) -> Result<(), u32> {
    trace!("creating wallet: {}", wallet_name);

    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    match Wallet::create(&config, &settings::get_wallet_credentials()) {
        Ok(x) => Ok(()),
        Err(x) => if x != ErrorCode::WalletAlreadyExistsError && x != ErrorCode::Success {
            warn!("could not create wallet {}: {:?}", wallet_name, x);
            Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        } else {
            warn!("could not create wallet {}: {:?}", wallet_name, x);
            Ok(())
        }
    }
}

pub fn open_wallet(wallet_name: &str) -> Result<i32, u32> {
    trace!("opening wallet: {}", wallet_name);
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    let handle = Wallet::open(&config, &settings::get_wallet_credentials()).map_err(map_rust_indy_sdk_error_code)?;
    unsafe { WALLET_HANDLE = handle; }
    Ok(handle)
}

pub fn init_wallet(wallet_name: &str) -> Result<i32, u32> {
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    create_wallet(wallet_name)?;
    open_wallet(wallet_name)
}

pub fn close_wallet() -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        unsafe { WALLET_HANDLE = 0; }
        return Ok(());
    }
    let result = Wallet::close(get_wallet_handle()).map_err(map_rust_indy_sdk_error_code);
    unsafe { WALLET_HANDLE = 0; }
    result
}

pub fn delete_wallet(wallet_name: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        unsafe { WALLET_HANDLE = 0;}
        return Ok(())
    }

    match close_wallet() {
        Ok(_) => (),
        Err(x) => (),
    };

    let config = format!(r#"{{"id":"{}"}}"#, wallet_name);

    Wallet::delete(&config,&settings::get_wallet_credentials()).map_err(map_rust_indy_sdk_error_code)
}

pub fn export(wallet_handle: i32, path: &Path, backup_key: &str) -> Result<(), WalletError> {
    let export_config = json!({ "key": backup_key, "path": &path}).to_string();
    match Wallet::export(wallet_handle, &export_config) {
        Ok(_) => Ok(()),
        Err(e) => Err(WalletError::CommonError(map_rust_indy_sdk_error_code(e))),
    }
}

pub fn import(config: &str) -> Result<(), WalletError> {
    settings::process_config_string(config).map_err(|e| WalletError::CommonError(e))?;

    let key = settings::get_config_value(settings::CONFIG_WALLET_KEY)
        .map_err(|e| WalletError::CommonError(e))?;

    let name = settings::get_config_value(settings::CONFIG_WALLET_NAME)
        .map_err(|e| WalletError::CommonError(error::MISSING_WALLET_NAME.code_num))?;

    let exported_wallet_path = settings::get_config_value(settings::CONFIG_EXPORTED_WALLET_PATH)
        .or(Err(WalletError::CommonError(error::MISSING_EXPORTED_WALLET_PATH.code_num)))?;

    let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY)
        .or(Err(WalletError::CommonError(error::MISSING_BACKUP_KEY.code_num)))?;

    let credentials = json!({"key": key, "storage":"{}"}).to_string();
    let import_config = json!({"key": backup_key, "path": exported_wallet_path }).to_string();
    let config = format!(r#"{{"id":"{}"}}"#, name);

    match Wallet::import(&config, &credentials, &import_config) {
        Ok(_) => Ok(()),
        Err(e) => Err(WalletError::CommonError(map_rust_indy_sdk_error_code(e))),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use std::ptr;
    use utils::cstring::CStringUtils;
    use utils::devsetup::tests::setup_wallet_env;
    use std::{fs, env};

    pub fn export_test_wallet() -> ::std::path::PathBuf {
        let filename_str = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let mut dir = env::temp_dir();
        dir.push(filename_str);
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }

        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();
        let handle = setup_wallet_env(&wallet_name).unwrap();

        let xtype = CStringUtils::string_to_cstring("type1".to_string());
        let id = CStringUtils::string_to_cstring("id1".to_string());
        let value = CStringUtils::string_to_cstring("value1".to_string());
        ::api::wallet::vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(::api::wallet::tests::indy_generic_no_msg_cb));

        export(handle, &dir, &backup_key).unwrap();
        dir
    }

    pub fn delete_import_wallet_path(dir: ::std::path::PathBuf) {
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }

    #[test]
    fn test_wallet() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_name = String::from("walletUnique");
        let mut wallet_handle = init_wallet(&wallet_name).unwrap();
        assert!( wallet_handle > 0);
        assert_eq!(error::UNKNOWN_LIBINDY_ERROR.code_num, init_wallet(&String::from("")).unwrap_err());

        thread::sleep(Duration::from_secs(1));
        delete_wallet("walletUnique").unwrap();
        let handle = get_wallet_handle();
        let wallet_name2 = String::from("wallet2");
        wallet_handle = init_wallet(&wallet_name2).unwrap();
        assert!(wallet_handle > 0);

        thread::sleep(Duration::from_secs(1));
        assert_ne!(handle, get_wallet_handle());
        delete_wallet("wallet2").unwrap();
    }

    #[test]
    fn test_wallet_import_export() {

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let exported_path = format!(r#"/tmp/{}"#, wallet_name);
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();

        let dir = export_test_wallet();
        let xtype = CStringUtils::string_to_cstring("type1".to_string());
        let id = CStringUtils::string_to_cstring("id1".to_string());
        let value = CStringUtils::string_to_cstring("value1".to_string());
        let options = CStringUtils::string_to_cstring("{}".to_string());

        ::api::vcx::vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: wallet_key,
            settings::CONFIG_EXPORTED_WALLET_PATH: exported_path,
            settings::CONFIG_WALLET_BACKUP_KEY: backup_key,
        }).to_string();
        import(&import_config).unwrap();
        open_wallet(&wallet_name).unwrap();

        // If wallet was successfully imported, there will be an error trying to add this duplicate record
        ::api::wallet::vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(::api::wallet::tests::duplicate_record_cb));
        thread::sleep(Duration::from_secs(1));
        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(dir);
    }

    #[test]
    fn test_import_fails_with_missing_configs() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        ::api::vcx::vcx_shutdown(true);

        // Invalid json
        assert_eq!(import(""), Err(WalletError::CommonError(error::INVALID_JSON.code_num)));
        let mut config = json!({});

        // Missing wallet_key
        assert_eq!(import(&config.to_string()), Err(WalletError::CommonError(error::MISSING_WALLET_KEY.code_num)));
        config[settings::CONFIG_WALLET_KEY] = serde_json::to_value("wallet_key1").unwrap();

        // Missing wallet name
        assert_eq!(import(&config.to_string()), Err(WalletError::CommonError(error::MISSING_WALLET_NAME.code_num)));
        config[settings::CONFIG_WALLET_NAME] = serde_json::to_value("wallet_name1").unwrap();

        // Missing exported_wallet_path
        assert_eq!(import(&config.to_string()), Err(WalletError::CommonError(error::MISSING_EXPORTED_WALLET_PATH.code_num)));
        config[settings::CONFIG_EXPORTED_WALLET_PATH] = serde_json::to_value(settings::DEFAULT_EXPORTED_WALLET_PATH).unwrap();

        // Missing backup_key
        assert_eq!(import(&config.to_string()), Err(WalletError::CommonError(error::MISSING_BACKUP_KEY.code_num)));

    }

    #[test]
    fn test_import_wallet_fails_with_existing_wallet() {
        settings::set_defaults();
        let wallet_name = "test_import_wallet_fails_with_existing_wallet";
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        let exported_path = format!(r#"/tmp/{}"#, wallet_name);
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();

        let dir = export_test_wallet();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: wallet_key,
            settings::CONFIG_EXPORTED_WALLET_PATH: exported_path,
            settings::CONFIG_WALLET_BACKUP_KEY: backup_key,
        }).to_string();
        assert_eq!(import(&import_config), Err(WalletError::CommonError(error::WALLET_ALREADY_EXISTS.code_num)));

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(dir);
    }

    #[test]
    fn test_import_wallet_fails_with_invalid_path(){
        settings::set_defaults();
        let wallet_name = "test_import_wallet_fails_with_invalid_path";
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        let exported_path = format!(r#"/tmp/{}"#, wallet_name);
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();

        let dir = export_test_wallet();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: wallet_key,
            settings::CONFIG_EXPORTED_WALLET_PATH: "DIFFERENT_PATH",
            settings::CONFIG_WALLET_BACKUP_KEY: backup_key,
        }).to_string();
        assert_eq!(import(&import_config), Err(WalletError::CommonError(error::IOERROR.code_num)));

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(dir);
    }

}
