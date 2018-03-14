/* test isn't ready until > libindy 1.0.1 */
extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use utils::libindy::{ indy_function_eval, mock_libindy_rc};
use utils::libindy::return_types::{ Return_I32_BIN, Return_I32_OPTSTR_BIN };
use utils::libindy::error_codes::{ map_indy_error_code, map_string_error };
use utils::error;
use settings;

extern {
    fn indy_prep_msg(command_handle: i32,
                     wallet_handle: i32,
                     sender_vk: *const c_char,
                     recipient_vk: *const c_char,
                     msg_data: *const u8,
                     msg_len: u32,
                     cb: Option<extern fn(command_handle_: i32, err: i32, encrypted_msg: *const u8, encrypted_len: u32)>) -> i32;

    fn indy_prep_anonymous_msg(command_handle: i32,
                               recipient_vk: *const c_char,
                               msg_data: *const u8,
                               msg_len: u32,
                               cb: Option<extern fn(command_handle_: i32, err: i32, encrypted_msg: *const u8, encrypted_len: u32)>) -> i32;

    fn indy_parse_msg(command_handle: i32,
                      wallet_handle: i32,
                      recipient_vk: *const c_char,
                      encrypted_msg: *const u8,
                      encrypted_len: u32,
                      cb: Option<extern fn(command_handle_: i32, err: i32, sender_vk: *const c_char, msg_data: *const u8, msg_len: u32)>) -> i32;

    fn indy_sign(command_handle: i32,
                 wallet_handle: i32,
                 did: *const c_char,
                 message_raw: *const u8,
                 message_len: u32,
                 cb: Option<extern fn(xcommand_handle: i32, err: i32, signature_raw: *const u8, signature_len: u32)>) -> i32;
}

pub fn prep_msg(wallet_handle: i32, sender_vk: &str, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(Vec::from(msg).to_owned());
    }

    debug!("prep_msg svk: {} rvk: {}",sender_vk, recipient_vk);

    let rtn_obj = Return_I32_BIN::new()?;
    let sender_vk = CString::new(sender_vk).map_err(map_string_error)?;
    let recipient_vk = CString::new(recipient_vk).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prep_msg(rtn_obj.command_handle,
                          wallet_handle as i32,
                          sender_vk.as_ptr(),
                          recipient_vk.as_ptr(),
                          msg.as_ptr() as *const u8,
                          msg.len() as u32,
                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn prep_anonymous_msg(recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    debug!("prep_anonymous_msg rvk: {}",recipient_vk);

    let rtn_obj = Return_I32_BIN::new()?;
    let recipient_vk = CString::new(recipient_vk).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prep_anonymous_msg(rtn_obj.command_handle,
                                    recipient_vk.as_ptr(),
                                    msg.as_ptr() as *const u8,
                                    msg.len() as u32,
                                    Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn parse_msg(wallet_handle: i32, recipient_vk: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() { return Ok(Vec::from(msg).to_owned()) }

    debug!("parse_msg vk: {}",recipient_vk);

    let rtn_obj = Return_I32_OPTSTR_BIN::new()?;
    let recipient_vk = CString::new(recipient_vk).map_err(map_string_error)?;

    unsafe {
            indy_function_eval(
                indy_parse_msg(rtn_obj.command_handle,
                                   wallet_handle,
                                  recipient_vk.as_ptr(),
                                  msg.as_ptr() as *const u8,
                                  msg.len() as u32,
                                  Some(rtn_obj.get_callback()))
            ).map_err(map_indy_error_code)?;
        }

        let (verkey, msg) = rtn_obj.receive(TimeoutUtils::some_long())?;
        check_str(verkey)?;
        Ok(msg)
}


pub fn sign(wallet_handle: i32, their_did: &str, msg: &[u8]) -> Result<Vec<u8>, u32> {
    if settings::test_indy_mode_enabled() {return Ok(Vec::from(msg).to_owned())}

    debug!("sign msg did: {}", their_did);

    let rtn_obj = Return_I32_BIN::new()?;
    let their_did = CString::new(their_did).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_sign(rtn_obj.command_handle,
                          wallet_handle,
                         their_did.as_ptr(),
                         msg.as_ptr() as *const u8,
                         msg.len() as u32,
                                    Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(str) => Ok(str),
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use utils::libindy::wallet;
    use utils::libindy::signus::SignusUtils;
    use utils::constants::*;

    #[test]
    fn test_send_msg() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let my_wallet = wallet::init_wallet("test_send_msg_my_wallet").unwrap();
        let their_wallet = wallet::init_wallet("test_send_msg_their_wallet").unwrap();

        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
        let (their_did, their_vk) = SignusUtils::create_and_store_my_did(their_wallet, Some(MY2_SEED)).unwrap();

        SignusUtils::store_their_did_from_parts(my_wallet, their_did.as_ref(), their_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(their_wallet, my_did.as_ref(), my_vk.as_ref()).unwrap();

        let message = "this is a test message for encryption";
        let encrypted_message = prep_msg(my_wallet, my_vk.as_ref(), their_vk.as_ref(),message.as_bytes()).unwrap();
        let decrypted_message = parse_msg(their_wallet,their_vk.as_ref(),&encrypted_message[..]).unwrap();

        assert_eq!(message.as_bytes().to_vec(), decrypted_message);
        wallet::delete_wallet("test_send_msg_my_wallet").unwrap();
        wallet::delete_wallet("test_send_msg_their_wallet").unwrap();
    }
}

