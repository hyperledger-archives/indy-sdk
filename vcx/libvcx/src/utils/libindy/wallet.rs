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
            Err(error::INVALID_WALLET_CREATION.code_num)
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

pub fn add_record(xtype: &str, id: &str, value: &str, tags: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() { return Ok(()) }

    Wallet::add_record(get_wallet_handle(), xtype, id, value, Some(tags))
        .map_err(map_rust_indy_sdk_error_code)
}


pub fn get_record(xtype: &str, id: &str, options: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(r#"{"id":"123","type":"record type","value":"record value","tags":null}"#.to_string())
    }

    Wallet::get_record(get_wallet_handle(), xtype, id, options)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn delete_record(xtype: &str, id: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() { return Ok(()) }
    Wallet::delete_record(get_wallet_handle(), xtype, id)
        .map_err(map_rust_indy_sdk_error_code)
}


pub fn update_record_value(xtype: &str, id: &str, value: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() { return Ok(()) }
    Wallet::update_record_value(get_wallet_handle(), xtype, id, value)
        .map_err(map_rust_indy_sdk_error_code)
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

        let xtype = "type1";
        let id = "id1";
        let value = "value1";
        add_record(xtype, id, value, "{}").unwrap();

        export(handle, &dir, &backup_key).unwrap();
        dir
    }

    pub fn delete_import_wallet_path(dir: ::std::path::PathBuf) {
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }

    pub fn delete_test_wallet(name: &str) {
        match close_wallet() {
            Ok(_) => (),
            Err(_) => (),
        };

        match delete_wallet(name) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    #[test]
    fn test_wallet() {
        init!("false");
        assert!( get_wallet_handle() > 0);
        assert_eq!(error::INVALID_WALLET_CREATION.code_num, init_wallet(&String::from("")).unwrap_err());
    }

    #[test]
    fn test_wallet_import_export() {
        settings::set_defaults();
        teardown!("false");
        let exported_path = format!(r#"/tmp/{}"#, settings::DEFAULT_WALLET_NAME);

        let dir = export_test_wallet();
        let xtype = "type1";
        let id = "id1";
        let value = "value1";
        let options = "{}";

        ::api::vcx::vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: exported_path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();
        import(&import_config).unwrap();
        open_wallet(&settings::DEFAULT_WALLET_NAME).unwrap();

        // If wallet was successfully imported, there will be an error trying to add this duplicate record
        assert_eq!(add_record(xtype, id, value, "{}"), Err(error::DUPLICATE_WALLET_RECORD.code_num));
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

    #[test]
    fn test_import_wallet_fails_with_invalid_backup_key() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_defaults();
        teardown!("false");

        let exported_path = format!(r#"/tmp/{}"#, settings::DEFAULT_WALLET_NAME);
        let bad_backup = "456";

        let dir = export_test_wallet();
        ::api::vcx::vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: exported_path,
            settings::CONFIG_WALLET_BACKUP_KEY: bad_backup,
        }).to_string();
        assert_eq!(import(&import_config), Err(WalletError::CommonError(error::LIBINDY_INVALID_STRUCTURE.code_num)));

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(dir);
    }

    #[test]
    fn test_add_new_record_with_no_tag() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_new_record_with_no_tag";

        add_record(record_type, id, record, "{}").unwrap();
    }

    #[test]
    fn test_add_duplicate_record_fails() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        add_record(record_type, id, record, "{}").unwrap();
        let rc = add_record(record_type, id, record, "{}");
        assert_eq!(rc, Err(error::DUPLICATE_WALLET_RECORD.code_num));
    }

    #[test]
    fn test_add_record_with_same_id_but_different_type_success() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let record_type2 = "Type2";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        add_record(record_type, id, record, "{}").unwrap();
        add_record(record_type2, id, record, "{}").unwrap();
    }

    #[test]
    fn test_retrieve_missing_record_fails() {
        init!("false");

        let record_type = "Type";
        let id = "123";
        let options = json!({
            "retrieveType": false,
            "retrieveValue": false,
            "retrieveTags": false
        }).to_string();
        let wallet_n = "test_retrieve_missing_record_fails";

        let rc = get_record(record_type, id, &options);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_retrieve_record_success() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_retrieve_record_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let expected_retrieved_record = format!(r#"{{"type":"{}","id":"{}","value":"{}","tags":null}}"#, record_type, id, record);

        add_record(record_type, id, record, "{}").unwrap();
        let retrieved_record = get_record(record_type, id, &options).unwrap();

        assert_eq!(retrieved_record, expected_retrieved_record);
    }

    #[test]
    fn test_delete_record_fails_with_no_record() {
        init!("false");
        let wallet_n = "test_delete_record_fails_with_no_record";
        let record_type = "Type";
        let id = "123";

        let rc = delete_record(record_type, id);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_delete_record_success() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_delete_record_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();

        add_record(record_type, id, record, "{}").unwrap();
        delete_record(record_type, id).unwrap();
        let rc = get_record(record_type, id, &options);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_update_record_value_fails_with_no_initial_record() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_update_record_value_fails_with_no_initial_record";

        let rc = update_record_value(record_type, id, record);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_update_record_value_success() {
        init!("false");

        let initial_record = "Record1";
        let changed_record = "Record2";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_update_record_value_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let expected_initial_record = format!(r#"{{"type":"{}","id":"{}","value":"{}","tags":null}}"#, record_type, id, initial_record);
        let expected_updated_record = format!(r#"{{"type":"{}","id":"{}","value":"{}","tags":null}}"#, record_type, id, changed_record);

        add_record(record_type, id, initial_record, "{}").unwrap();
        let initial_record = get_record(record_type, id, &options).unwrap();
        update_record_value(record_type, id, changed_record).unwrap();
        let changed_record = get_record(record_type, id, &options).unwrap();

        assert_eq!(initial_record, expected_initial_record);
        assert_eq!(changed_record, expected_updated_record);
    }
}
