extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::route::RouteCommand;
use utils::cstring::CStringUtils;
use utils::byte_array::vec_to_pointer;

use self::libc::c_char;

pub fn indy_pack_msg_auth(command_handle: i32,
                     wallet_handle: i32,
                     message: *const c_char,
                     recv_keys: *const c_char,
                     my_vk: *const c_char,
                     cb: Option<extern fn(xcommand_handle: i32,
                                          err: ErrorCode,
                                          ames: *const c_char)>) -> ErrorCode {

    trace!("indy_pack_msg_auth: >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
           wallet_handle, message, recv_keys, my_vk);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(my_vk, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_pack_msg_auth: entities >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
           wallet_handle, message, recv_keys, my_vk);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::PackMessage(
        message,
        recv_keys,
        my_vk,
        true, //sets auth to true to use authcrypt
        wallet_handle,
        Box::new(move |result| {
            let (err, ames) = result_to_err_code_1!(result, String::new());
            trace!("indy_pack_msg_auth: ames_json: {:?}", ames);
            let ames = CStringUtils::string_to_cstring(ames);
            cb(command_handle, err, ames.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_pack_msg_auth: <<< res: {:?}", res);

    res
}

pub fn pack_msg_anon(command_handle: i32,
                     wallet_handle: i32,
                     message: *const c_char,
                     recv_keys: *const c_char,
                     cb: Option<extern fn(xcommand_handle: i32,
                                          err: ErrorCode,
                                          ames: *const c_char)>) {


    trace!("indy_pack_msg_anon: >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}",
           wallet_handle, message, recv_keys);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_pack_msg_anon: entities >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}",
           wallet_handle, message, recv_keys);


    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::PackMessage(
        message,
        recv_keys,
        "".to_string(), //passes empty verkey to align with service api
        false, //sets auth to false to use anoncrypt
        wallet_handle,
        Box::new(move |result| {
            let (err, ames) = result_to_err_code_1!(result, String::new());
            trace!("indy_pack_msg_auth: ames_json: {:?}", ames);
            let verkey = CStringUtils::string_to_cstring(ames);
            cb(command_handle, err, verkey.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_pack_msg_auth: <<< res: {:?}", res);

    res
}

pub fn unpack_msg(json_jwm: *const c_char,
                  my_vk: *const c_char,
                  wallet_handle: i32,
                  cb: Option<extern fn(xcommand_handle: i32,
                                       err: ErrorCode,
                                       plaintext: *const c_char)>) -> ErrorCode {

}

pub fn add_route(did_with_key_frag : *const c_char,
                 endpoint : *const c_char,
                 wallet_handle:i32,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode)>) -> ErrorCode {

}

pub fn lookup_route(did_with_key_frag : *const c_char,
                 wallet_handle:i32,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      endpoint: *const c_char)>) -> ErrorCode {

}

pub fn remove_route(did_with_key_frag : *const c_char,
                    endpoint : *const c_char,
                    wallet_handle:i32,
                    cb: Option<extern fn(xcommand_handle: i32,
                                         err: ErrorCode)>) -> ErrorCode {

}

pub fn update_route(did_with_key_frag : &str,
                    new_endpoint : &str,
                    wallet_handle : i32,
                    cb: Option<extern fn(xcommand_handle: i32,
                                         err: ErrorCode)>) -> ErrorCode {

}
