extern crate libc;

use api::ErrorCode;
use commands::{Command, CommandExecutor};
use commands::agent::AgentCommand;
use errors::ToErrorCode;
use utils::byte_array::vec_to_pointer;
use utils::cstring::CStringUtils;

use self::libc::c_char;
use std::ptr;

#[no_mangle]
pub extern fn indy_prep_msg(command_handle: i32,
                            wallet_handle: i32,
                            sender_vk: *const c_char,
                            recipient_vk: *const c_char,
                            msg_data: *const u8,
                            msg_len: u32,
                            cb: Option<extern fn(command_handle_: i32,
                                                 err: ErrorCode,
                                                 encrypted_msg: *const u8,
                                                 encrypted_len: u32)>) -> ErrorCode {
    check_useful_c_str!(sender_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(msg_data, msg_len, ErrorCode::CommonInvalidParam5, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    let result = CommandExecutor::instance()
        .send(Command::Agent(AgentCommand::PrepMsg(
            wallet_handle,
            sender_vk,
            recipient_vk,
            msg_data,
            Box::new(move |result| {
                let (err, encrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_prep_anonymous_msg(command_handle: i32,
                                      recipient_vk: *const c_char,
                                      msg_data: *const u8,
                                      msg_len: u32,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           encrypted_msg: *const u8,
                                                           encrypted_len: u32)>) -> ErrorCode {
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam2);
    check_useful_c_byte_array!(msg_data, msg_len, ErrorCode::CommonInvalidParam3, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Agent(AgentCommand::PrepAnonymousMsg(
            recipient_vk,
            msg_data,
            Box::new(move |result| {
                let (err, encrypted_msg) = result_to_err_code_1!(result, Vec::new());
                let (encrypted_msg_raw, encrypted_msg_len) = vec_to_pointer(&encrypted_msg);
                cb(command_handle, err, encrypted_msg_raw, encrypted_msg_len)
            })
        )));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_parse_msg(command_handle: i32,
                             wallet_handle: i32,
                             recipient_vk: *const c_char,
                             encrypted_msg: *const u8,
                             encrypted_len: u32,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode,
                                                  sender_vk: *const c_char,
                                                  msg_data: *const u8,
                                                  msg_len: u32)>) -> ErrorCode {
    check_useful_c_str!(recipient_vk, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(encrypted_msg, encrypted_len, ErrorCode::CommonInvalidParam4, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Agent(AgentCommand::ParseMsg(
            wallet_handle,
            recipient_vk,
            encrypted_msg,
            Box::new(move |result| {
                let (err, sender_vk, msg) = result_to_err_code_2!(result, None, Vec::new());
                let (msg_data, msg_len) = vec_to_pointer(&msg);
                let sender_vk = sender_vk.map(CStringUtils::string_to_cstring);
                cb(command_handle, err,
                   sender_vk.as_ref().map(|vk| vk.as_ptr()).unwrap_or(ptr::null()),
                   msg_data, msg_len)
            })
        )));

    result_to_err_code!(result)
}
