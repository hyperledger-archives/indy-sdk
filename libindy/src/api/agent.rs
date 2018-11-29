extern crate libc;

use api::ErrorCode;
use commands::agent::AgentCommand;
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
    sender: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
) -> ErrorCode {
    trace!("indy_auth_pack_message: >>> wallet_handle: {:?}, message: {:?}, receiver_keys: {:?}, sender: {:?}",
           wallet_handle, message, receiver_keys, sender);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(receiver_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(sender, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_auth_pack_message: entities >>> wallet_handle: {:?}, message: {:?}, receiver_keys: {:?}, sender: {:?}",
           wallet_handle, message, receiver_keys, sender);

    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::AuthPackMessage(
        message,
        receiver_keys,
        sender,
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
    receiver_keys: *const c_char,
    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
) -> ErrorCode {
    trace!(
        "indy_anon_pack_message: >>> message: {:?}, receiver_keys: {:?}",
        message,
        receiver_keys
    );

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(receiver_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_anon_pack_message: entities >>> message: {:?}, receiver_keys: {:?}",
        message,
        receiver_keys
    );

    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::AnonPackMessage(
        message,
        receiver_keys,
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
    sender: *const c_char,
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
        "indy_unpack_message: >>> wallet_handle: {:?}, jwe: {:?}, sender: {:?}",
        wallet_handle,
        jwe,
        sender
    );

    check_useful_c_str!(jwe, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(sender, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_unpack_message: entities >>> wallet_handle: {:?}, jwe: {:?}, sender: {:?}",
        wallet_handle,
        jwe,
        sender
    );

    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::UnpackMessage(
        jwe,
        sender,
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
