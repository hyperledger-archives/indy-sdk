/* test isn't ready until > libindy 1.0.1 */
extern crate libc;

use utils::libindy::{ mock_libindy_rc};
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use settings;
use indy::crypto::Crypto;

pub fn prep_msg(sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(Vec::from(msg).to_owned());
    }

    Crypto::auth_crypt(::utils::libindy::wallet::get_wallet_handle(), sender_vk, recipient_vk, msg)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    Crypto::anon_crypt(recipient_vk, msg).map_err(map_rust_indy_sdk_error_code)
}

pub fn parse_msg(recipient_vk: &str, msg: &[u8]) -> Result<(String, Vec<u8>), u32> {
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::VERKEY.to_string(), Vec::from(msg).to_owned())) }

    Crypto::auth_decrypt(::utils::libindy::wallet::get_wallet_handle(), recipient_vk, msg).map_err(map_rust_indy_sdk_error_code)
}

pub fn parse_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() { return Ok(Vec::from(msg).to_owned()) }

    Crypto::anon_decrypt(::utils::libindy::wallet::get_wallet_handle(), recipient_vk, msg).map_err(map_rust_indy_sdk_error_code)
}

pub fn sign(my_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    Crypto::sign(::utils::libindy::wallet::get_wallet_handle(), my_vk, msg).map_err(map_rust_indy_sdk_error_code)
}

pub fn verify(vk: &str, msg: &str, signature: &[u8]) -> Result<bool, u32> {
    Crypto::verify(vk, msg.as_bytes(), signature).map_err(map_rust_indy_sdk_error_code)
}

#[cfg(test)]
mod tests {
    extern crate json;
    extern crate base64;
    use super::*;

    #[test]
    fn test_sign() {
        init!("ledger");
        let name = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME).unwrap();
        let logo = settings::get_config_value(settings::CONFIG_INSTITUTION_LOGO_URL).unwrap();
        let vrky = settings::get_config_value(settings::CONFIG_INSTITUTION_VERKEY).unwrap();
        let json = json!({"url": logo, "name": name});
        let signature_array = sign(&vrky, json.to_string().as_bytes()).unwrap();
        let signature = base64::encode(&signature_array);
        assert_eq!(verify(&vrky, &json.to_string(), &signature_array).unwrap(), true);
    }
}
