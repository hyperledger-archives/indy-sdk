extern crate libc;

use self::libc::c_char;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use settings;
use utils::libindy::check_str;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32, Return_I32_STR_STR };
use utils::libindy::error_codes::{map_indy_error_code};

extern {
    fn indy_create_and_store_my_did(command_handle: i32,
                                    wallet_handle: i32,
                                    did_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32, did: *const c_char, verkey: *const c_char)>) -> i32;

    fn indy_store_their_did(command_handle: i32,
                            wallet_handle: i32,
                            identity_json: *const c_char,
                            cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;
}

pub struct SignusUtils {}

impl SignusUtils {

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), u32> {
        if settings::test_indy_mode_enabled() {
            return Ok(("8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_owned()));
        }
        let rtn_obj = Return_I32_STR_STR::new()?;
        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
        let my_did_json = CString::new(my_did_json).unwrap();
        unsafe {
            indy_function_eval(
                indy_create_and_store_my_did(rtn_obj.command_handle,
                                             wallet_handle,
                                             my_did_json.as_ptr(),
                                         Some(rtn_obj.get_callback()))
            ).map_err(map_indy_error_code)?;
        }

        let (opt_did, opt_ver) = rtn_obj.receive(TimeoutUtils::some_long())?;
        let did = check_str(opt_did)?;
        let verkey = check_str(opt_ver)?;
        Ok((did, verkey))
    }

    pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_verkey: &str) -> Result<(), u32> {
        if settings::test_indy_mode_enabled() { return Ok(()) }

        let rtn_obj = Return_I32::new()?;
        let their_identity_json = format!("{{\"did\":\"{}\",\
                                            \"verkey\":\"{}\"\
                                           }}",
                                          their_did, their_verkey);
        let their_identity_json = CString::new(their_identity_json).unwrap();

        unsafe {
            indy_function_eval(
                indy_store_their_did(rtn_obj.command_handle,
                                     wallet_handle,
                                     their_identity_json.as_ptr(),
                                     Some(rtn_obj.get_callback()))
            ).map_err(map_indy_error_code)?;
        }
        rtn_obj.receive(TimeoutUtils::some_long())
    }

}
