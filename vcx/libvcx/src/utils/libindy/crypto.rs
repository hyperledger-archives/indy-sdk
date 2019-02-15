/* test isn't ready until > libindy 1.0.1 */
extern crate libc;

use futures::Future;

use utils::libindy::{ mock_libindy_rc};
use utils::libindy::error_codes::map_rust_indy_sdk_error;
use settings;
use indy::crypto;

pub fn prep_msg(sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(Vec::from(msg).to_owned());
    }

    crypto::auth_crypt(::utils::libindy::wallet::get_wallet_handle(), sender_vk, recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    crypto::anon_crypt(recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn parse_msg(recipient_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), u32> {
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::VERKEY.to_string(), Vec::from(msg).to_owned())) }

    crypto::auth_decrypt(::utils::libindy::wallet::get_wallet_handle(), recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn parse_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() { return Ok(Vec::from(msg).to_owned()) }

    crypto::anon_decrypt(::utils::libindy::wallet::get_wallet_handle(), recipient_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn sign(my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    crypto::sign(::utils::libindy::wallet::get_wallet_handle(), my_vk, msg)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn verify(vk: &str, msg: &[u8], signature: &[u8]) -> Result<bool, u32> {
    if settings::test_indy_mode_enabled() { return Ok(true) }

    crypto::verify(vk, msg, signature)
	.wait()
	.map_err(map_rust_indy_sdk_error)
}
