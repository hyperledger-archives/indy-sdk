use utils::version_constants;
use libc::c_char;
use utils::cstring::CStringUtils;
use utils::libindy::{wallet, pool};
use utils::error;
use settings;
use error::prelude::*;

#[no_mangle]
pub extern fn vcx_wallet_get_handle() -> i32 {
    wallet::get_wallet_handle()
}

#[no_mangle]
pub extern fn vcx_pool_get_handle() -> i32 {
    match pool::get_pool_handle() {
        Ok(x) => x,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern fn vcx_wallet_set_handle(handle: i32) -> i32 {
    wallet::set_wallet_handle(handle)
}

#[no_mangle]
pub extern fn vcx_pool_set_handle(handle: i32) -> i32 {
    if handle <= 0 { pool::change_pool_handle(None); }
    else { pool::change_pool_handle(Some(handle)); }

    handle
}

#[no_mangle]
pub extern fn vcx_init_post_indy(config: *const c_char) -> u32 {
    check_useful_c_str!(config,VcxErrorKind::InvalidOption);

    trace!("vcx_init_without_indy(config: {:?})", config);

    if config == "ENABLE_TEST_MODE" {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        settings::set_defaults();
    } else {
        match settings::process_config_string(&config) {
            Err(e) => {
                error!("Invalid configuration specified: {}", e);
                return e.into();
            }
            Ok(_) => (),
        }
    };

    if wallet::get_wallet_handle() <= 0 || pool::get_pool_handle().is_err() {
        error!("Library was initialized without wallet/pool");
        return error::INVALID_STATE.code_num;
    }

    ::utils::threadpool::init();

    settings::log_settings();

    trace!("libvcx version: {}{}", version_constants::VERSION, version_constants::REVISION);

    error::SUCCESS.code_num
}

extern {
    fn indy_pack_message(command_handle: i32,
                         wallet_handle: i32,
                         message: *const u8,
                         message_len: u32,
                         receiver_keys: *const c_char,
                         sender: *const c_char,
                         cb: Option<extern fn(xcommand_handle: i32, err: i32, jwe_data: *const u8, jwe_len: u32)>) -> i32;

    fn indy_unpack_message(command_handle: i32,
                           wallet_handle: i32,
                           jwe_data: *const u8,
                           jwe_len: u32,
                           cb: Option<extern fn(xcommand_handle: i32, err: i32, res_json_data: *const u8, res_json_len: u32 )>) -> i32;
}

#[no_mangle]
pub extern fn vcx_pack_message(command_handle: i32,
                               wallet_handle: i32, //ignored
                               message: *const u8,
                               message_len: u32,
                               receiver_keys: *const c_char,
                               sender: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: i32, jwe_data: *const u8, jwe_len: u32)>) -> i32 {
    unsafe {
        indy_pack_message(command_handle, wallet::get_wallet_handle(), message, message_len, receiver_keys, sender, cb)
    }
}

#[no_mangle]
pub extern fn vcx_unpack_message(command_handle: i32,
                                 wallet_handle: i32, //ignored
                                 jwe_data: *const u8,
                                 jwe_len: u32,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32, res_json_data : *const u8, res_json_len : u32)>) -> i32 {
    unsafe {
        indy_unpack_message(command_handle, wallet::get_wallet_handle(), jwe_data, jwe_len, cb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_settings() -> String {
        json!({
            settings::CONFIG_AGENCY_DID:           settings::get_config_value(settings::CONFIG_AGENCY_DID).unwrap(),
            settings::CONFIG_AGENCY_VERKEY:        settings::get_config_value(settings::CONFIG_AGENCY_VERKEY).unwrap(),
            settings::CONFIG_AGENCY_ENDPOINT:      settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT).unwrap(),
            settings::CONFIG_REMOTE_TO_SDK_DID:    settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID).unwrap(),
            settings::CONFIG_REMOTE_TO_SDK_VERKEY: settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY).unwrap(),
            settings::CONFIG_SDK_TO_REMOTE_DID:    settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_DID).unwrap(),
            settings::CONFIG_SDK_TO_REMOTE_VERKEY: settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).unwrap(),
            settings::CONFIG_INSTITUTION_NAME:     settings::get_config_value(settings::CONFIG_INSTITUTION_NAME).unwrap(),
            settings::CONFIG_INSTITUTION_DID:      settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            settings::CONFIG_INSTITUTION_LOGO_URL: settings::get_config_value(settings::CONFIG_INSTITUTION_LOGO_URL).unwrap(),
            settings::CONFIG_PAYMENT_METHOD:       settings::get_config_value(settings::CONFIG_PAYMENT_METHOD).unwrap()
        }).to_string()
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "agency")]
    #[test]
    fn test_init_post_indy() {
	use std::ffi::CString;

        init!("agency");
        let content = get_settings();
        settings::clear_config();
        // Store settings and handles
        let config = CString::new(content).unwrap().into_raw();
        let wallet_handle = vcx_wallet_get_handle();
        let pool_handle = vcx_pool_get_handle();
        assert!(wallet_handle > 0);
        assert!(pool_handle > 0);
        // Reset handles to 0
        assert_eq!(vcx_pool_set_handle(0), 0);
        assert_eq!(vcx_wallet_set_handle(0), 0);
        assert_eq!(vcx_wallet_get_handle(), 0);
        assert_eq!(vcx_pool_get_handle(), 0);
        // Test for errors when handles not set
        assert_ne!(error::SUCCESS.code_num, vcx_init_post_indy(config));
        vcx_wallet_set_handle(wallet_handle);
        assert_ne!(error::SUCCESS.code_num, vcx_init_post_indy(config));
        vcx_pool_set_handle(pool_handle);
        // NOTE: handles are set independently, test config with no wallet or pool
        assert_eq!(error::SUCCESS.code_num, vcx_init_post_indy(config));
        // test that wallet and pool are operational
        ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);
        assert!(::connection::tests::build_test_connection() > 0);
        teardown!("agency");
    }
}
