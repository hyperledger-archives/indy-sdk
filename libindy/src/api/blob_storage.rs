extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::blob_storage::BlobStorageCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;

#[no_mangle]
pub extern fn indy_open_blob_storage_reader(command_handle: i32,
                                            type_: *const c_char,
                                            config_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 handle: i32)>) -> ErrorCode {
    trace!("indy_open_blob_storage_reader: >>> type_: {:?}, config_json: {:?}", type_, config_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_open_blob_storage_reader: entities >>> type_: {:?}, config_json: {:?}", type_, config_json);

    let result = CommandExecutor::instance()
        .send(Command::BlobStorage(BlobStorageCommand::OpenReader(
            type_,
            config_json,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                trace!("indy_open_blob_storage_reader: handle: {:?}", handle);
                cb(command_handle, err, handle)
            }),
        )));

    let res = result_to_err_code!(result);

    trace!("indy_open_blob_storage_reader: <<< res: {:?}", res);

    res
}

#[no_mangle]
pub extern fn indy_open_blob_storage_writer(command_handle: i32,
                                            type_: *const c_char,
                                            config_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 handle: i32)>) -> ErrorCode {
    trace!("indy_open_blob_storage_writer: >>> type_: {:?}, config_json: {:?}", type_, config_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_open_blob_storage_writer: entities >>> type_: {:?}, config_json: {:?}", type_, config_json);

    let result = CommandExecutor::instance()
        .send(Command::BlobStorage(BlobStorageCommand::OpenWriter(
            type_,
            config_json,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                trace!("indy_open_blob_storage_writer: handle: {:?}", handle);
                cb(command_handle, err, handle)
            }),
        )));

    let res = result_to_err_code!(result);

    trace!("indy_open_blob_storage_writer: <<< res: {:?}", res);

    res
}
