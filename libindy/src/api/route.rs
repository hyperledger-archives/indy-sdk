extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::route::RouteCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

#[no_mangle]
pub fn indy_auth_pack_message(command_handle: i32,
                     wallet_handle: i32,
                     message: *const c_char,
                     recv_keys: *const c_char,
                     my_vk: *const c_char,
                     cb: Option<extern fn(xcommand_handle: i32,
                                          err: ErrorCode,
                                          ames: *const c_char)>) -> ErrorCode {

    trace!("indy_auth_pack_message: >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
           wallet_handle, message, recv_keys, my_vk);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(my_vk, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_auth_pack_message: entities >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}, my_vk: {:?}",
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
            trace!("indy_auth_pack_message: cb command_handle: {:?}, err: {:?}, ames: {:?}",
                   command_handle, err, ames);
            let ames = CStringUtils::string_to_cstring(ames);
            cb(command_handle, err, ames.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_auth_pack_message: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub fn indy_anon_pack_message(command_handle: i32,
                     wallet_handle: i32,
                     message: *const c_char,
                     recv_keys: *const c_char,
                     cb: Option<extern fn(xcommand_handle: i32,
                                          err: ErrorCode,
                                          ames: *const c_char)>) -> ErrorCode {


    trace!("indy_anon_pack_message: >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}",
           wallet_handle, message, recv_keys);

    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(recv_keys, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_anon_pack_message: entities >>> wallet_handle: {:?}, message: {:?}, recv_keys: {:?}",
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
            trace!("indy_anon_pack_message: cb command_handle: {:?}, err: {:?}, ames: {:?}",
                   command_handle, err, ames);
            let verkey = CStringUtils::string_to_cstring(ames);
            cb(command_handle, err, verkey.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_anon_pack_message: <<< res: {:?}", res);

    res
}

//return key for authcrypt
#[no_mangle]
pub fn indy_unpack_message(command_handle: i32,
                  wallet_handle: i32,
                  ames: *const c_char,
                  my_vk: *const c_char,
                  cb: Option<extern fn(xcommand_handle: i32,
                                       err: ErrorCode,
                                       plaintext: *const c_char)>) -> ErrorCode {
    trace!("indy_unpack_message: >>> wallet_handle: {:?}, ames: {:?}, my_vk: {:?}",
           wallet_handle, ames, my_vk);

    check_useful_c_str!(ames, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(my_vk, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_unpack_message: entities >>> wallet_handle: {:?}, ames: {:?}, my_vk: {:?}",
           wallet_handle, ames, my_vk);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::UnpackMessage(
        ames,
        my_vk,
        wallet_handle,
        Box::new(move |result| {
            let (err, plaintext) = result_to_err_code_1!(result, String::new());
            trace!("indy_unpack_message: cb command_handle: {:?}, err: {:?}, plaintext: {:?}",
                   command_handle, err, plaintext);
            let plaintext = CStringUtils::string_to_cstring(plaintext);
            cb(command_handle, err, plaintext.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_unpack_message: <<< res: {:?}", res);

    res


}

#[no_mangle]
pub fn indy_add_route(command_handle: i32,
                 wallet_handle: i32,
                 did_with_key : *const c_char,
                 endpoint : *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode)>) -> ErrorCode {

    trace!("indy_add_route: >>> wallet_handle: {:?}, did_with_key: {:?}, endpoint: {:?}",
           wallet_handle, did_with_key, endpoint);

    check_useful_c_str!(did_with_key, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(endpoint, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_add_route: entities >>> wallet_handle: {:?}, did_with_key: {:?}, endpoint: {:?}",
           wallet_handle, did_with_key, endpoint);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::AddRoute(
        did_with_key,
        endpoint,
        wallet_handle,
        Box::new(move |result| {
            let err = result_to_err_code!(result);
            trace!("indy_add_route: cb command_handle: {:?}, err: {:?}", command_handle, err);
            cb(command_handle, err)
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_add_route: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub fn indy_lookup_route(command_handle: i32,
                         wallet_handle: i32,
                         did_with_key: *const c_char,
                         cb: Option<extern fn(xcommand_handle: i32,
                                              err: ErrorCode,
                                              endpoint: *const c_char)>) -> ErrorCode {

    trace!("indy_lookup_route: >>> wallet_handle: {:?}, did_with_key: {:?}",
           wallet_handle, did_with_key);

    check_useful_c_str!(did_with_key, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_lookup_route: entities >>> wallet_handle: {:?}, did_with_key: {:?}",
           wallet_handle, did_with_key);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::LookupRoute(
        did_with_key,
        wallet_handle,
        Box::new(move |result| {
            let (err, endpoint) = result_to_err_code_1!(result, String::new());
            trace!("indy_lookup_route: cb command_handle: {:?}, err: {:?}, plaintext: {:?}", command_handle, err, endpoint);
            let endpoint = CStringUtils::string_to_cstring(endpoint);
            cb(command_handle, err, endpoint.as_ptr())
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_lookup_route: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub fn indy_remove_route(command_handle: i32,
                         wallet_handle: i32,
                         did_with_key : *const c_char,
                    cb: Option<extern fn(xcommand_handle: i32,
                                         err: ErrorCode)>) -> ErrorCode {

    trace!("indy_remove_route: >>> wallet_handle: {:?}, did_with_key: {:?}",
           wallet_handle, did_with_key);

    check_useful_c_str!(did_with_key, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_remove_route: entities >>> wallet_handle: {:?}, did_with_key: {:?}",
           wallet_handle, did_with_key);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::RemoveRoute(
        did_with_key,
        wallet_handle,
        Box::new(move |result| {
            let err = result_to_err_code!(result);
            trace!("indy_remove_route: cb command_handle: {:?}, err: {:?}", command_handle, err);
            cb(command_handle, err)
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_remove_route: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub fn indy_update_route(command_handle: i32,
                         wallet_handle: i32,
                         did_with_key : *const c_char,
                         new_endpoint : *const c_char,
                         cb: Option<extern fn(xcommand_handle: i32,
                                              err: ErrorCode)>) -> ErrorCode {

    trace!("indy_update_route: >>> wallet_handle: {:?}, did_with_key: {:?}, new endpoint: {:?}",
           wallet_handle, did_with_key, new_endpoint);

    check_useful_c_str!(did_with_key, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(new_endpoint, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_update_route: entities >>> wallet_handle: {:?}, did_with_key: {:?}, new endpoint: {:?}",
           wallet_handle, did_with_key, new_endpoint);

    let result = CommandExecutor::instance()
    .send(Command::Route(RouteCommand::UpdateRoute(
        did_with_key,
        new_endpoint,
        wallet_handle,
        Box::new(move |result| {
            let err = result_to_err_code!(result);
            trace!("indy_update_route: cb command_handle: {:?}, err: {:?}", command_handle, err);
            cb(command_handle, err)
        })
    )));

    let res = result_to_err_code!(result);

    trace!("indy_update_route: <<< res: {:?}", res);

    res
}
