use futures::Future;
use indy::{wallet, ErrorCode};

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error;

use error::prelude::*;
use indy::{WalletHandle, INVALID_WALLET_HANDLE};

pub static mut WALLET_HANDLE: WalletHandle = INVALID_WALLET_HANDLE;

pub fn set_wallet_handle(handle: WalletHandle) -> WalletHandle {
    unsafe { WALLET_HANDLE = handle; }
    unsafe { WALLET_HANDLE }
}

pub fn get_wallet_handle() -> WalletHandle { unsafe { WALLET_HANDLE } }

pub fn reset_wallet_handle() { set_wallet_handle(INVALID_WALLET_HANDLE); }

pub fn create_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<()> {
    trace!("creating wallet: {}", wallet_name);

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);
    let credentials = settings::get_wallet_credentials(storage_creds);

    match wallet::create_wallet(&config, &credentials)
        .wait() {
        Ok(()) => Ok(()),
        Err(err) if err.error_code == ErrorCode::WalletAlreadyExistsError => {
            warn!("wallet \"{}\" already exists. skipping creation", wallet_name);
            Ok(())
        }
        Err(err) => {
            warn!("could not create wallet {}: {:?}", wallet_name, err.message);
            Err(VcxError::from_msg(VcxErrorKind::WalletCreate, format!("could not create wallet {}: {:?}", wallet_name, err.message)))
        }
    }
}

pub fn open_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<WalletHandle> {
    trace!("open_wallet >>> wallet_name: {}", wallet_name);
    if settings::mock_indy_test_mode_enabled() {
        return Ok(set_wallet_handle(WalletHandle(1)));
    }

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);
    let credentials = settings::get_wallet_credentials(storage_creds);

    let handle = wallet::open_wallet(&config, &credentials)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    set_wallet_handle(handle);

    Ok(handle)
}

pub fn init_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<WalletHandle> {
    if settings::mock_indy_test_mode_enabled() {
        return Ok(set_wallet_handle(WalletHandle(1)));
    }

    create_wallet(wallet_name, wallet_type, storage_config, storage_creds)?;
    open_wallet(wallet_name, wallet_type, storage_config, storage_creds)
}

pub fn close_wallet() -> VcxResult<()> {
    trace!("close_wallet >>>");

    if settings::mock_indy_test_mode_enabled() {
        set_wallet_handle(INVALID_WALLET_HANDLE);
        return Ok(());
    }

    wallet::close_wallet(get_wallet_handle())
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    reset_wallet_handle();
    Ok(())
}

pub fn delete_wallet(wallet_name: &str, wallet_type: Option<&str>, storage_config: Option<&str>, storage_creds: Option<&str>) -> VcxResult<()> {
    trace!("delete_wallet >>> wallet_name: {}", wallet_name);

    close_wallet().ok();

    let config = settings::get_wallet_config(wallet_name, wallet_type, storage_config);
    let credentials = settings::get_wallet_credentials(storage_creds);

    wallet::delete_wallet(&config, &credentials)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn add_record(xtype: &str, id: &str, value: &str, tags: Option<&str>) -> VcxResult<()> {
    trace!("add_record >>> xtype: {}, id: {}, value: {}, tags: {:?}", secret!(&xtype), secret!(&id), secret!(&value), secret!(&tags));

    if settings::mock_indy_test_mode_enabled() { return Ok(()); }

    wallet::add_wallet_record(get_wallet_handle(), xtype, id, value, tags)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn get_record(xtype: &str, id: &str, options: &str) -> VcxResult<String> {
    trace!("get_record >>> xtype: {}, id: {}, options: {}", secret!(&xtype), secret!(&id), options);

    if settings::mock_indy_test_mode_enabled() {
        return Ok(r#"{"id":"123","type":"record type","value":"record value","tags":null}"#.to_string());
    }

    wallet::get_wallet_record(get_wallet_handle(), xtype, id, options)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn delete_record(xtype: &str, id: &str) -> VcxResult<()> {
    trace!("delete_record >>> xtype: {}, id: {}", secret!(&xtype), secret!(&id));

    if settings::mock_indy_test_mode_enabled() { return Ok(()); }

    wallet::delete_wallet_record(get_wallet_handle(), xtype, id)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}


pub fn update_record_value(xtype: &str, id: &str, value: &str) -> VcxResult<()> {
    trace!("update_record_value >>> xtype: {}, id: {}, value: {}", secret!(&xtype), secret!(&id), secret!(&value));

    if settings::mock_indy_test_mode_enabled() { return Ok(()); }

    wallet::update_wallet_record_value(get_wallet_handle(), xtype, id, value)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn export(wallet_handle: WalletHandle, path: &str, backup_key: &str) -> VcxResult<()> {
    trace!("export >>> wallet_handle: {:?}, path: {:?}, backup_key: ****", wallet_handle, path);

    let export_config = json!({ "key": backup_key, "path": &path}).to_string();
    wallet::export_wallet(wallet_handle, &export_config)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn import(config: &str) -> VcxResult<()> {
    trace!("import >>> config {}", config);

    let config: serde_json::Value = serde_json::from_str(config)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot parse config: {}", err)))?;

    config.get(settings::CONFIG_WALLET_KEY)
        .ok_or(VcxError::from(VcxErrorKind::MissingWalletKey))?;

    let name = config[settings::CONFIG_WALLET_NAME].as_str()
        .ok_or(VcxError::from(VcxErrorKind::MissingWalletName))?;

    let exported_wallet_path = config[settings::CONFIG_EXPORTED_WALLET_PATH].as_str()
        .ok_or(VcxError::from(VcxErrorKind::MissingExportedWalletPath))?;

    let backup_key =  config[settings::CONFIG_WALLET_BACKUP_KEY].as_str()
        .ok_or(VcxError::from(VcxErrorKind::MissingBackupKey))?;

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
    use utils::get_temp_dir_path;
    use utils::devsetup::{SetupLibraryWallet, SetupDefaults, TempFile, SetupEmpty};

    fn _record() -> (&'static str, &'static str, &'static str) {
        ("type1", "id1", "value1")
    }

    pub fn export_test_wallet() -> (TempFile, String) {
        let wallet_name = "export_test_wallet";

        let export_file = TempFile::prepare_path(wallet_name);

        let handle = init_wallet(wallet_name, None, None, None).unwrap();

        let backup_key = settings::get_config_value(settings::CONFIG_WALLET_BACKUP_KEY).unwrap();

        let (type_, id, value) = _record();
        add_record(type_, id, value, None).unwrap();

        export(handle, &export_file.path, &backup_key).unwrap();

        close_wallet().unwrap();

        (export_file, wallet_name.to_string())
    }

    #[test]
    fn test_wallet() {
        let _setup = SetupLibraryWallet::init();

        assert_ne!(get_wallet_handle(), INVALID_WALLET_HANDLE);
        assert_eq!(VcxErrorKind::WalletCreate, init_wallet(&String::from(""), None, None, None).unwrap_err().kind());
    }

    #[test]
    fn test_wallet_for_unknown_type() {
        let _setup = SetupDefaults::init();

        assert_eq!(VcxErrorKind::WalletCreate, init_wallet("test_wallet_for_unknown_type", Some("UNKNOWN_WALLET_TYPE"), None, None).unwrap_err().kind());
    }

    #[test]
    fn test_wallet_calls_fail_with_different_key_derivation() {
        let _setup = SetupDefaults::init();

        let wallet_n = settings::DEFAULT_WALLET_NAME;

        settings::set_defaults();
        create_wallet(wallet_n, None, None, None).unwrap();

        settings::clear_config();

        // Open fails without Wallet Key Derivation set
        assert_eq!(open_wallet(wallet_n, None, None, None).unwrap_err().kind(), VcxErrorKind::UnknownLiibndyError);

        ::settings::clear_config();

        // Open works when set
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, settings::DEFAULT_WALLET_KEY_DERIVATION);
        assert!(open_wallet(wallet_n, None, None, None).is_ok());

        ::settings::clear_config();

        // Delete fails
        assert_eq!(delete_wallet(wallet_n, None, None, None).unwrap_err().kind(), VcxErrorKind::UnknownLiibndyError);

        // Delete works
        settings::set_defaults();
        delete_wallet(wallet_n, None, None, None).unwrap()
    }

    #[test]
    fn test_wallet_import_export() {
        let _setup = SetupDefaults::init();

        let (export_wallet_path, wallet_name) = export_test_wallet();

        delete_wallet(&wallet_name, None, None, None).unwrap();

        let (type_, id, value) = _record();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name.as_str(),
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_wallet_path.path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();

        import(&import_config).unwrap();
        open_wallet(&wallet_name, None, None, None).unwrap();

        // If wallet was successfully imported, there will be an error trying to add this duplicate record
        assert_eq!(add_record(type_, id, value, None).unwrap_err().kind(), VcxErrorKind::DuplicationWalletRecord);

        delete_wallet(&wallet_name, None, None, None).unwrap();
    }

    #[test]
    fn test_import_fails_with_missing_configs() {
        let _setup = SetupEmpty::init();

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
            get_temp_dir_path(settings::DEFAULT_EXPORTED_WALLET_PATH).to_str().unwrap()
        ).unwrap();

        // Missing backup_key
        let res = import(&config.to_string()).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::MissingBackupKey);
    }

    #[test]
    fn test_import_wallet_fails_with_existing_wallet() {
        let _setup = SetupDefaults::init();

        let (export_wallet_path, wallet_name) = export_test_wallet();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_wallet_path.path,
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();

        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::DuplicationWallet);

        delete_wallet(&wallet_name, None, None, None).unwrap();
    }

    #[test]
    fn test_import_wallet_fails_with_invalid_path() {
        let _setup = SetupDefaults::init();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: "DIFFERENT_PATH",
            settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
        }).to_string();

        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::IOError);
    }

    #[test]
    fn test_import_wallet_fails_with_invalid_backup_key() {
        let _setup = SetupDefaults::init();

        let bad_backup = "456";

        let (export_wallet_path, wallet_name) = export_test_wallet();

        delete_wallet(&wallet_name, None, None, None).unwrap();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: export_wallet_path.path,
            settings::CONFIG_WALLET_BACKUP_KEY: bad_backup,
        }).to_string();
        let res = import(&import_config).unwrap_err();
        assert_eq!(res.kind(), VcxErrorKind::LibindyInvalidStructure);
    }

    #[test]
    fn test_add_new_record_with_no_tag() {
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, record) = _record();

        add_record(record_type, id, record, None).unwrap();
    }

    #[test]
    fn test_add_duplicate_record_fails() {
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, record) = _record();

        add_record(record_type, id, record, None).unwrap();

        let rc = add_record(record_type, id, record, None);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::DuplicationWalletRecord);
    }

    #[test]
    fn test_add_record_with_same_id_but_different_type_success() {
        let _setup = SetupLibraryWallet::init();

        let (_, id, record) = _record();

        let record_type = "Type";
        let record_type2 = "Type2";

        add_record(record_type, id, record, None).unwrap();
        add_record(record_type2, id, record, None).unwrap();
    }

    #[test]
    fn test_retrieve_missing_record_fails() {
        let _setup = SetupLibraryWallet::init();

        let record_type = "Type";
        let id = "123";
        let options = json!({
            "retrieveType": false,
            "retrieveValue": false,
            "retrieveTags": false
        }).to_string();

        let rc = get_record(record_type, id, &options);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
    }

    #[test]
    fn test_retrieve_record_success() {
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, record) = _record();

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
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, _) = _record();

        let rc = delete_record(record_type, id);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
    }

    #[test]
    fn test_delete_record_success() {
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, record) = _record();

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
        let _setup = SetupLibraryWallet::init();

        let (record_type, id, record) = _record();

        let rc = update_record_value(record_type, id, record);
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::WalletRecordNotFound);
    }

    #[test]
    fn test_update_record_value_success() {
        let _setup = SetupLibraryWallet::init();

        let initial_record = "Record1";
        let changed_record = "Record2";
        let record_type = "Type";
        let id = "123";
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
