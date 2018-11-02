/* test isn't ready until > libindy 1.0.1 */
extern crate libc;

use futures::Future;

use utils::libindy::{ mock_libindy_rc};
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use settings;
use indy::crypto::Crypto;

pub fn prep_msg(wallet_handle: i32, sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(Vec::from(msg).to_owned());
    }

    Crypto::auth_crypt(wallet_handle, sender_vk, recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    Crypto::anon_crypt(recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn parse_msg(recipient_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), u32> {
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::VERKEY.to_string(), Vec::from(msg).to_owned())) }

    Crypto::auth_decrypt(::utils::libindy::wallet::get_wallet_handle(), recipient_vk, msg).map_err(map_rust_indy_sdk_error_code)
}

pub fn parse_anonymous_msg(wallet_handle: i32, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() { return Ok(Vec::from(msg).to_owned()) }

    Crypto::anon_decrypt(wallet_handle, recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn sign(wallet_handle: i32, my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    Crypto::sign(wallet_handle, my_vk, msg).wait().map_err(map_rust_indy_sdk_error_code)
}
