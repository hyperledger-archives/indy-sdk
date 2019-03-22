use futures::Future;
use indy::did;

use settings;
use utils::libindy::error_codes::map_rust_indy_sdk_error;
use utils::libindy::wallet::get_wallet_handle;
use error::prelude::*;

pub fn create_and_store_my_did(seed: Option<&str>) -> VcxResult<(String, String)> {
    if settings::test_indy_mode_enabled() {
        return Ok((::utils::constants::DID.to_string(), ::utils::constants::VERKEY.to_string()));
    }

    let my_did_json = seed.map_or(json!({}), |seed| json!({"seed": seed}));

    did::create_and_store_my_did(get_wallet_handle(), &my_did_json.to_string())
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn get_local_verkey(did: &str) -> VcxResult<String> {
    if settings::test_indy_mode_enabled() {
        return Ok(::utils::constants::VERKEY.to_string());
    }

    did::key_for_local_did(get_wallet_handle(), did)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}
