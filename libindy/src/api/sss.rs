// TODO: Add 3 commands, add and create shards of a verkey, get shards of a verkey, recover secret using given shards
extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::sss::SSSCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;


#[no_mangle]
pub  extern fn indy_shard_msg_with_secret_and_store_shards(command_handle: i32,
                                            wallet_handle: i32,
                                            m: u8,
                                            n: u8,
                                            msg: *const c_char,
                                            verkey: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 vk: *const c_char)>) -> ErrorCode {
    check_usize_c_int!(m, ErrorCode::CommonInvalidParam2);
    check_usize_c_int!(n, ErrorCode::CommonInvalidParam3);
    check_useful_opt_c_str!(msg, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::SSS(SSSCommand::ShardMsgWithSecretAndStoreShards(
            wallet_handle,
            m,
            n,
            msg,
            verkey,
            Box::new(move |result| {
                let (err, vk) = result_to_err_code_1!(result, String::new());
                let vk = CStringUtils::string_to_cstring(vk);
                cb(command_handle, err, vk.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_get_shards_of_verkey(command_handle: i32,
                                                           wallet_handle: i32,
                                                           verkey: *const c_char,
                                                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                                shards_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::SSS(SSSCommand::GetShardsOfVerkey(
            wallet_handle,
            verkey,
            Box::new(move |result| {
                let (err, shards_json) = result_to_err_code_1!(result, String::new());
                let shards_json = CStringUtils::string_to_cstring(shards_json);
                cb(command_handle, err, shards_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

#[no_mangle]
pub  extern fn indy_get_shard_of_verkey(command_handle: i32,
                                         wallet_handle: i32,
                                         verkey: *const c_char,
                                         shard_number: u8,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              shard: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam2);
    check_usize_c_int!(shard_number, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::SSS(SSSCommand::GetShardOfVerkey(
            wallet_handle,
            verkey,
            shard_number,
            Box::new(move |result| {
                let (err, shard) = result_to_err_code_1!(result, String::new());
                let shard = CStringUtils::string_to_cstring(shard);
                cb(command_handle, err, shard.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_recover_secret_from_shards(command_handle: i32,
                                         shards_json: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                              secret: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(shards_json, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::SSS(SSSCommand::RecoverSecretFromShards(
            shards_json,
            Box::new(move |result| {
                let (err, secret) = result_to_err_code_1!(result, String::new());
                let secret = CStringUtils::string_to_cstring(secret);
                cb(command_handle, err, secret.as_ptr())
            })
        )));

    result_to_err_code!(result)
}
