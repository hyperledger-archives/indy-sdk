extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::blob_storage::BlobStorageCommand;
use utils::byte_array::vec_to_pointer;
use utils::cstring::CStringUtils;

use self::libc::c_char;

#[no_mangle]
pub extern fn indy_blob_storage_open_reader(command_handle: i32,
                                            type_: *const c_char,
                                            config_json: *const c_char,
                                            location: *const c_char,
                                            hash: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32, err: ErrorCode, handle: i32)>) -> ErrorCode {
    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(location, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(hash, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::BlobStorage(BlobStorageCommand::OpenReader(
            type_,
            config_json,
            location,
            hash,
            Box::new(move |result| {
                let (err, handle) = result_to_err_code_1!(result, 0);
                cb(command_handle, err, handle)
            })
        )));

    result_to_err_code!(result)
}

#[no_mangle]
pub extern fn indy_blob_storage_read(command_handle: i32,
                                     reader_handle: i32,
                                     size: u64,
                                     offset: u64,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: ErrorCode,
                                                          data_raw: *const u8,
                                                          data_len: u32)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::BlobStorage(BlobStorageCommand::Read(
            reader_handle,
            size,
            offset,
            Box::new(move |result| {
                let (err, data) = result_to_err_code_1!(result, Vec::new());
                let (data_raw, data_len) = vec_to_pointer(&data);
                cb(command_handle, err, data_raw, data_len)
            })
        )));

    result_to_err_code!(result)
}
