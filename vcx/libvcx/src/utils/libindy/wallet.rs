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
    let pool_name = match settings::get_config_value(settings::CONFIG_POOL_NAME) {
        Ok(x) => x,
        Err(x) => settings::DEFAULT_POOL_NAME.to_string(),
    };

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
        Err(e) => Err(WalletError::from(e as u32)),
    }
}

pub fn import(path: &Path, backup_key: &str) -> Result<(), WalletError> {
    use settings;

    let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)?;
    let name = settings::get_config_value(settings::CONFIG_WALLET_NAME)?;
    let key = settings::get_config_value(settings::CONFIG_WALLET_KEY)?;
    let credentials = json!({"key": key, "storage":"{}"}).to_string();
    let import_config = json!({"key": backup_key, "path": path}).to_string();

    let config = format!(r#"{{"id":"{}"}}"#, name);

    match Wallet::import(&config, &credentials, &import_config) {
        Ok(_) => Ok(()),
        Err(e) => Err(WalletError::from(e as u32)),
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
        use utils::devsetup::tests::setup_wallet_env;
        use indy::wallet::Wallet;
        use std::{fs, env};
        settings::set_defaults();
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let filename_str = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME).unwrap();
        let backup_key = "backup_key";
        let mut dir = env::temp_dir();
        dir.push(filename_str);
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }
        let credential_config = json!({"key": wallet_key, "storage": "{}"}).to_string();

        let handle = setup_wallet_env(&wallet_name).unwrap();

        let xtype = CStringUtils::string_to_cstring("type1".to_string());
        let id = CStringUtils::string_to_cstring("id1".to_string());
        let value = CStringUtils::string_to_cstring("value1".to_string());
        let options = CStringUtils::string_to_cstring("{}".to_string());
        ::api::wallet::vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(::api::wallet::tests::indy_generic_no_msg_cb));

        export(handle, &dir, backup_key).unwrap();

        let config = format!(r#"{{"id":"{}"}}"#, wallet_name);
        Wallet::close(handle).unwrap();
        Wallet::delete(&config, &credential_config).unwrap();
        println!("credential_config: {}", credential_config);

        import(&dir, backup_key).unwrap();
        let handle = setup_wallet_env(&wallet_name).unwrap();

        ::api::wallet::vcx_wallet_get_record(handle, xtype.as_ptr(), id.as_ptr(), options.as_ptr(), Some(::api::wallet::tests::indy_generic_msg_cb));
        Wallet::close(handle).unwrap();
        Wallet::delete(&config, &credential_config).unwrap();
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }
}
