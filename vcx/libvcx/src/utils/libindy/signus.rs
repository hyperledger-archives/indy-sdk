extern crate libc;

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use indy::did::Did;

pub fn create_and_store_my_did(seed: Option<&str>) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() {
        return Ok((::utils::constants::DID.to_string(), ::utils::constants::VERKEY.to_string()));
    }

    let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
    Did::new(::utils::libindy::wallet::get_wallet_handle(), &my_did_json).map_err(map_rust_indy_sdk_error_code)
}

pub fn get_local_verkey(did: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(::utils::constants::VERKEY.to_string());
    }

    Did::get_ver_key_local(::utils::libindy::wallet::get_wallet_handle(), did).map_err(map_rust_indy_sdk_error_code)
}
