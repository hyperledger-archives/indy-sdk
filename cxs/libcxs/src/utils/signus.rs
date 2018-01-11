extern crate libc;

use self::libc::c_char;
use std::sync::mpsc::channel;
use std::ffi::CString;
use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use settings;
use utils::error;

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

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), i32> {
        if settings::test_indy_mode_enabled() {
            return Ok(("8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_owned()));
        }

        let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
        let create_and_store_my_did_cb = Box::new(move |err, did, verkey| {
            create_and_store_my_did_sender.send((err, did, verkey)).unwrap();
        });
        let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));

        let my_did_json = CString::new(my_did_json).unwrap();

        unsafe {
            let err =
                indy_create_and_store_my_did(create_and_store_my_did_command_handle,
                                             wallet_handle,
                                             my_did_json.as_ptr(),
                                             create_and_store_my_did_callback);
            if err != 0 {
                return Err(err);
            }
        }

        let (err, my_did, my_verkey) = create_and_store_my_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != 0 {
            return Err(err);
        }
        if my_did.is_none() || my_verkey.is_none() {
            Err(error::UNKNOWN_LIBINDY_ERROR.code_num as i32)
        }
        else {
            Ok((my_did.unwrap(), my_verkey.unwrap()))
        }

    }

    pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_verkey: &str) -> Result<(), i32> {
        if settings::test_indy_mode_enabled() { return Ok(()) }

        let (store_their_did_sender, store_their_did_receiver) = channel();
        let store_their_did_cb = Box::new(move |err| { store_their_did_sender.send((err)).unwrap(); });
        let (store_their_did_command_handle, store_their_did_callback) = CallbackUtils::closure_to_store_their_did_cb(store_their_did_cb);

        let their_identity_json = format!("{{\"did\":\"{}\",\
                                            \"verkey\":\"{}\"\
                                           }}",
                                          their_did, their_verkey);

        let their_identity_json = CString::new(their_identity_json).unwrap();

        unsafe {
            let err =
                indy_store_their_did(store_their_did_command_handle,
                                     wallet_handle,
                                     their_identity_json.as_ptr(),
                                     store_their_did_callback);

            if err != 0 {
                return Err(err);
            }
            let err = store_their_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
            if err != 0 {
                return Err(err);
            }
            Ok(())
        }
    }

}