extern crate libc;

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use indy::did::Did;

pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() {
        return Ok((::utils::constants::DID.to_string(), ::utils::constants::VERKEY.to_string()));
    }

    let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
    Did::new(wallet_handle, &my_did_json).map_err(map_rust_indy_sdk_error_code)
}
