extern crate libc;

use api::ErrorCode;
use commands::agent::RouteCommand;
use commands::{Command, CommandExecutor};
use errors::ToErrorCode;
use utils::ctypes;

use self::libc::c_char;

#[no_mangle]
pub fn indy_auth_pack_message(
    command_handle: i32,
    wallet_handle: i32,
    message: *const c_char,
    receiver_keys: *const c_char,
    sender_verkey: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
) -> ErrorCode {
    trace!("indy_auth_pack_message: >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
           wallet_handle, message, receiver_keys, sender_verkey);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(my_vk, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_auth_pack_message: entities >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
           wallet_handle, message, receiver_keys, sender_verkey);

    let result = CommandExecutor::instance().send(Command::Route(RouteCommand::AuthPackMessage(
        message,
        recveiver_keys,
        sender_verkey,
        wallet_handle,
        Box::new(move |result| {
            let (err, jwe) = result_to_err_code_1!(result, String::new());
            trace!(
                "indy_auth_pack_message: cb command_handle: {:?}, err: {:?}, jwe: {:?}",
                command_handle,
                err,
                jwe
            );
            let jwe = ctypes::string_to_cstring(jwe);
            cb(command_handle, err, jwe.as_ptr())
        }),
    )));

    let res = result_to_err_code!(result);

    trace!("indy_auth_pack_message: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub fn indy_anon_pack_message(
    command_handle: i32,
    message: *const c_char,
    recveiver_keys: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
) -> ErrorCode {
    trace!(
        "indy_anon_pack_message: >>> message: {:?}, recv_keys: {:?}",
        message,
        recveiver_keys
    );

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_anon_pack_message: entities >>> message: {:?}, recv_keys: {:?}",
        message,
        recveiver_keys
    );

    let result = CommandExecutor::instance().send(Command::Route(RouteCommand::AnonPackMessage(
        message,
        recveiver_keys,
        Box::new(move |result| {
            let (err, jwe) = result_to_err_code_1!(result, String::new());
            trace!(
                "indy_anon_pack_message: cb command_handle: {:?}, err: {:?}, jwe: {:?}",
                command_handle,
                err,
                jwe
            );
            let verkey = ctypes::string_to_cstring(jwe);
            cb(command_handle, err, verkey.as_ptr())
        }),
    )));

    let res = result_to_err_code!(result);

    trace!("indy_anon_pack_message: <<< res: {:?}", res);

    res
}

//update function to return key used
#[no_mangle]
pub fn indy_unpack_message(
    command_handle: i32,
    wallet_handle: i32,
    jwe: *const c_char,
    sender_verkey: *const c_char,
    cb: Option<
        extern "C" fn(
            xcommand_handle: i32,
            err: ErrorCode,
            plaintext: *const c_char,
            sender_vk: *const c_char,
        ),
    >,
) -> ErrorCode {
    trace!(
        "indy_unpack_message: >>> wallet_handle: {:?}, jwe: {:?}, my_vk: {:?}",
        wallet_handle,
        jwe,
        sender_verkey
    );

    check_useful_c_str!(jwe_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(my_vk, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_unpack_message: entities >>> wallet_handle: {:?}, jwe: {:?}, my_vk: {:?}",
        wallet_handle,
        jwe,
        sender_verkey
    );

    let result = CommandExecutor::instance().send(Command::Route(RouteCommand::UnpackMessage(
        jwe,
        sender_verkey,
        wallet_handle,
        Box::new(move |result| {
            let (err, plaintext, sender_vk) =
                result_to_err_code_2!(result, String::new(), String::new());
            trace!(
                "indy_unpack_message: cb command_handle: {:?}, err: {:?}, plaintext: {:?}",
                command_handle,
                err,
                plaintext
            );
            let plaintext = ctypes::string_to_cstring(plaintext);
            let sender_vk = ctypes::string_to_cstring(sender_vk);
            cb(command_handle, err, plaintext.as_ptr(), sender_vk.as_ptr())
        }),
    )));

    let res = result_to_err_code!(result);

    trace!("indy_unpack_message: <<< res: {:?}", res);

    res
}
