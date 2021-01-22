use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode, IndyHandle};
use indy_utils::ctypes;
use libc::c_char;

use crate::commands::Locator;

#[no_mangle]
pub extern "C" fn indy_open_blob_storage_reader(
    command_handle: CommandHandle,
    type_: *const c_char,
    config_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, handle: IndyHandle)>,
) -> ErrorCode {
    trace!(
        "indy_open_blob_storage_reader > type_ {:?} config_json {:?}",
        type_,
        config_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!(
        "indy_open_blob_storage_reader ? type_ {:?} config_json {:?}",
        type_,
        config_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.blob_storage_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller.open_reader(type_, config_json).await;
        let (err, handle) = prepare_result_1!(res, 0);

        trace!(
            "indy_open_blob_storage_reader ? err {:?} handle {:?}",
            err,
            handle
        );

        cb(command_handle, err, handle)
    });

    let res = ErrorCode::Success;
    trace!("indy_open_blob_storage_reader < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_open_blob_storage_writer(
    command_handle: CommandHandle,
    type_: *const c_char,
    config_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, handle: IndyHandle)>,
) -> ErrorCode {
    trace!(
        "indy_open_blob_storage_writer > type_ {:?} config_json {:?}",
        type_,
        config_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(config_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!(
        "indy_open_blob_storage_writer ? type_ {:?} config_json {:?}",
        type_,
        config_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.blob_storage_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller.open_writer(type_, config_json).await;
        let (err, handle) = prepare_result_1!(res, 0);

        trace!(
            "indy_open_blob_storage_writer ? err {:?} handle {:?}",
            err,
            handle
        );

        cb(command_handle, err, handle)
    });

    let res = ErrorCode::Success;
    trace!("indy_open_blob_storage_writer < {:?}", res);
    res
}
