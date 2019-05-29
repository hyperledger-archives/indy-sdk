use futures::Future;
use indy::{wallet, ErrorCode};

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error;

use std::path::Path;
use error::prelude::*;

pub static mut WALLET_HANDLE: i32 = 0;

pub fn set_wallet_handle(handle: i32) -> i32 {
    unsafe { WALLET_HANDLE = handle; }
    unsafe { WALLET_HANDLE }
}

pub fn get_wallet_handle() -> i32 { unsafe { WALLET_HANDLE } }

pub fn create_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<()> {
    trace!("creating wallet: {}", wallet_name);

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);

    let credentials = settings::get_wallet_credentials(storage_creds);

    match wallet::create_wallet(&config, &credentials)
        .wait() {
        Ok(x) => Ok(()),
        Err(x) => if x.error_code != ErrorCode::WalletAlreadyExistsError {
            warn!("could not create wallet {}: {:?}", wallet_name, x.message);
            Err(VcxError::from_msg(VcxErrorKind::WalletCreate, format!("could not create wallet {}: {:?}", wallet_name, x.message)))
        } else {
            warn!("could not create wallet {}: {:?}", wallet_name, x.message);
            Ok(())
        }
    }
}

pub fn open_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<i32> {
    trace!("open_wallet >>> wallet_name: {}", wallet_name);
    if settings::test_indy_mode_enabled() {
        return Ok(set_wallet_handle(1));
    }

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);

    let credentials = settings::get_wallet_credentials(storage_creds);

    let handle = wallet::open_wallet(&config, &credentials)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    set_wallet_handle(handle);
    Ok(handle)
}

pub fn init_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<i32> {
    if settings::test_indy_mode_enabled() {
        return Ok(set_wallet_handle(1));
    }

    create_wallet(wallet_name, wallet_type, storage_config, storage_creds)?;
    open_wallet(wallet_name, wallet_type, storage_config, storage_creds)
}

pub fn close_wallet() -> VcxResult<()> {
    trace!("close_wallet >>>");

    if settings::test_indy_mode_enabled() {
        set_wallet_handle(0);
        return Ok(());
    }
    let result = wallet::close_wallet(get_wallet_handle())
        .wait()
        .map_err(map_rust_indy_sdk_error);

    set_wallet_handle(0);
    result
}

pub fn delete_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<()> {
    trace!("delete_wallet >>> wallet_name: {}", wallet_name);

    if settings::test_indy_mode_enabled() {
        set_wallet_handle(0);
        return Ok(());
    }

    close_wallet().ok();

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);

    wallet::delete_wallet(&config, &settings::get_wallet_credentials(storage_creds))
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn add_record(xtype: &str, id: &str, value: &str, tags: Option<&str>) -> VcxResult<()> {
    trace!("add_record >>> xtype: {}, id: {}, value: {}, tags: {:?}", secret!(&xtype), secret!(&id), secret!(&value), secret!(&tags));

    if settings::test_indy_mode_enabled() { return Ok(()); }

    wallet::add_wallet_record(get_wallet_handle(), xtype, id, value, tags)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}


pub fn get_record(xtype: &str, id: &str, options: &str) -> VcxResult<String> {
    trace!("get_record >>> xtype: {}, id: {}, options: {}", secret!(&xtype), secret!(&id), options);

    if settings::test_indy_mode_enabled() {
        return Ok(r#"{"id":"123","type":"record type","value":"record value","tags":null}"#.to_string());
    }

    wallet::get_wallet_record(get_wallet_handle(), xtype, id, options)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn delete_record(xtype: &str, id: &str) -> VcxResult<()> {
    trace!("delete_record >>> xtype: {}, id: {}", secret!(&xtype), secret!(&id));

    if settings::test_indy_mode_enabled() { return Ok(()); }

    wallet::delete_wallet_record(get_wallet_handle(), xtype, id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}


pub fn update_record_value(xtype: &str, id: &str, value: &str) -> VcxResult<()> {
    trace!("update_record_value >>> xtype: {}, id: {}, value: {}", secret!(&xtype), secret!(&id), secret!(&value));

    if settings::test_indy_mode_enabled() { return Ok(()); }

    wallet::update_wallet_record_value(get_wallet_handle(), xtype, id, value)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn export(wallet_handle: i32, path: &Path, backup_key: &str) -> VcxResult<()> {
    trace!("export >>> wallet_handle: {}, path: {:?}, backup_key: ****", wallet_handle, path);

    let export_config = json!({ "key": backup_key, "path": &path}).to_string();
    wallet::export_wallet(wallet_handle, &export_config)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn import(config: &str) -> VcxResult<()> {
    trace!("import >>> config {}", config);

    settings::process_config_string(config)?;

    let key = settings::get_config_value(settings::CONFIG_WALLET_KEY)
        .or(Err(VcxError::from(VcxErrorKind::MissingWalletKey)))?;

    let name = settings::get_config_value(settings::CONFIG_WALLET_NAME)
        .or(Err(VcxError::from(VcxErrorKind::MissingWalletName)))?;

    let exported_wallet_path = settings::get_config_value(settings::CONFIG_EXPORTED_WALLET_PATH)
        .or(Err(VcxError::from(VcxErrorKind::MissingExportedWalletPath)))?;

    let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY)
        .or(Err(VcxError::from(VcxErrorKind::MissingBackupKey)))?;

    let config = settings::get_wallet_config(&name, None, None);
    let credentials = settings::get_wallet_credentials(None);
    let import_config = json!({"key": backup_key, "path": exported_wallet_path }).to_string();

    wallet::import_wallet(&config, &credentials, &import_config)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::{get_temp_dir_path};
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
        add_record(xtype, id, value, None).unwrap();

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

        match delete_wallet(name, None, None, None) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    #[test]
    fn test_wallet() {
        init!("false");
        assert!(get_wallet_handle() > 0);
        assert_eq!(VcxErrorKind::WalletCreate, init_wallet(&String::from(""), None, None, None).unwrap_err().kind());
    }

    #[test]
    fn test_wallet_for_unknown_type() {
        init!("false");
        assert_eq!(VcxErrorKind::WalletCreate, init_wallet("test_wallet_for_unknown_type", Some("UNKNOWN_WALLET_TYPE"), None, None).unwrap_err().kind());
    }

    #[test]
    fn test_wallet_calls_fail_with_different_key_derivation() {
        teardown!("false");
        ::api::vcx::vcx_shutdown(true);
        let wallet_n = settings::DEFAULT_WALLET_NAME;
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, settings::DEFAULT_WALLET_KEY_DERIVATION);
        create_wallet(wallet_n, None, None, None).unwrap();

        // Open fails without Wallet Key Derivation set
        ::api::vcx::vcx_shutdown(false);
        assert_eq!(open_wallet(wallet_n, None, None, None).unwrap_err().kind(), VcxErrorKind::UnknownLiibndyError);

        // Open works when set
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, settings::DEFAULT_WALLET_KEY_DERIVATION);
        assert!(open_wallet(wallet_n, None, None, None).is_ok());

        // Delete fails
        ::api::vcx::vcx_shutdown(false);
        assert_eq!(delete_wallet(wallet_n, None, None, None).unwrap_err().kind(), VcxErrorKind::UnknownLiibndyError);

        // Delete works
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, settings::DEFAULT_WALLET_KEY_DERIVATION);
        assert!(delete_wallet(wallet_n, None, None, None).is_ok());
    }

    #[test]
    fn test_wallet_import_export() {
        settings::set_defaults();
        teardown!("false");

        let export_path = export_test_wallet();

        let xtype = "type1";
        let id = "id1";
        let value = "value1";
        let options = "{}";

        ::api::vcx::vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();
        import(&import_config).unwrap();
        open_wallet(&settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();

        // If wallet was successfully imported, there will be an error trying to add this duplicate record
        assert_eq!(add_record(xtype, id, value, None).unwrap_err().kind(), VcxErrorKind::DuplicationWalletRecord);
        thread::sleep(Duration::from_secs(1));
        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(export_path);
    }

    #[test]
    fn test_import_fails_with_missing_configs() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        ::api::vcx::vcx_shutdown(true);

        // Invalid json
        let res = import("").unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::InvalidJson);
        let mut config = json!({});

        // Missing wallet_key
        let res = import(&config.to_string()).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::MissingWalletKey);
        config[settings::CONFIG_WALLET_KEY] = serde_json::to_value("wallet_key1").unwrap();

        // Missing wallet name
        let res = import(&config.to_string()).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::MissingWalletName);
        config[settings::CONFIG_WALLET_NAME] = serde_json::to_value("wallet_name1").unwrap();

        // Missing exported_wallet_path
        let res = import(&config.to_string()).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::MissingExportedWalletPath);
        config[settings::CONFIG_EXPORTED_WALLET_PATH] = serde_json::to_value(
            get_temp_dir_path(Some(settings::DEFAULT_EXPORTED_WALLET_PATH)).to_str().unwrap()
        ).unwrap();

        // Missing backup_key
        let res = import(&config.to_string()).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::MissingBackupKey);
    }

    #[test]
    fn test_import_wallet_fails_with_existing_wallet() {
        settings::set_defaults();

        let export_path = export_test_wallet();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();
        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::DuplicationWallet);

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(export_path);
    }

    #[test]
    fn test_import_wallet_fails_with_invalid_path() {
        settings::set_defaults();
        let wallet_name = "test_import_wallet_fails_with_invalid_path";
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();

        let dir = export_test_wallet();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: wallet_key,
            settings::CONFIG_EXPORTED_WALLET_PATH: "DIFFERENT_PATH",
            settings::CONFIG_WALLET_BACKUP_KEY: backup_key,
        }).to_string();
        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::IOError);

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(dir);
    }

    #[test]
    fn test_import_wallet_fails_with_invalid_backup_key() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_defaults();
        teardown!("false");

        let bad_backup = "456";

        let export_path = export_test_wallet();

        ::api::vcx::vcx_shutdown(true);

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_path,
            settings::CONFIG_WALLET_BACKUP_KEY: bad_backup,
        }).to_string();
        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::LibindyInvalidStructure);

        ::api::vcx::vcx_shutdown(true);
        delete_import_wallet_path(export_path);
    }

    #[test]
    fn test_add_new_record_with_no_tag() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_new_record_with_no_tag";

        add_record(record_type, id, record, None).unwrap();
    }

    #[test]
    fn test_add_duplicate_record_fails() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        add_record(record_type, id, record, None).unwrap();
        let rc = add_record(record_type, id, record, None);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::DuplicationWalletRecord);
    }

    #[test]
    fn test_add_record_with_same_id_but_different_type_success() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let record_type2 = "Type2";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        add_record(record_type, id, record, None).unwrap();
        add_record(record_type2, id, record, None).unwrap();
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
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);

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

        add_record(record_type, id, record, None).unwrap();
        let retrieved_record = get_record(record_type, id, &options).unwrap();

        assert_eq!(retrieved_record, expected_retrieved_record);
    }

    #[test]
    fn test_delete_record_fails_with_no_record() {
        init!("false");
        let record_type = "Type";
        let id = "123";

        let rc = delete_record(record_type, id);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
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

        add_record(record_type, id, record, None).unwrap();
        delete_record(record_type, id).unwrap();
        let rc = get_record(record_type, id, &options);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
    }

    #[test]
    fn test_update_record_value_fails_with_no_initial_record() {
        init!("false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_update_record_value_fails_with_no_initial_record";

        let rc = update_record_value(record_type, id, record);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
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

        add_record(record_type, id, initial_record, None).unwrap();
        let initial_record = get_record(record_type, id, &options).unwrap();
        update_record_value(record_type, id, changed_record).unwrap();
        let changed_record = get_record(record_type, id, &options).unwrap();

        assert_eq!(initial_record, expected_initial_record);
        assert_eq!(changed_record, expected_updated_record);
    }
}
