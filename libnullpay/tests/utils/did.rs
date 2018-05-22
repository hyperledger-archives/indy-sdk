use nullpay::ErrorCode;
use std::ffi::CString;
use std::os::raw::c_char;

pub const TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";

pub fn create_store_and_publish_my_did_from_trustee(wallet_handle: i32, pool_handle: i32) -> Result<(String, String), ErrorCode> {
    let (trustee_did, _) = create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED))?;
    let (my_did, my_vk) = create_and_store_my_did(wallet_handle, None)?;
    let nym = super::ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "", "TRUSTEE")?;
    super::ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym)?; //TODO check response type
    Ok((my_did, my_vk))
}

pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), ErrorCode> {
    let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string_string();

    let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
    let my_did_json = CString::new(my_did_json).unwrap();

    let err = unsafe {
        indy_create_and_store_my_did(command_handle, wallet_handle, my_did_json.as_ptr(), cb)
    };

    super::results::result_to_string_string(err, receiver)
}

extern {
    #[no_mangle]
    fn indy_create_and_store_my_did(command_handle: i32,
                                    wallet_handle: i32,
                                    did_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32,
                                                         err: ErrorCode,
                                                         did: *const c_char,
                                                         verkey: *const c_char)>) -> ErrorCode;
}