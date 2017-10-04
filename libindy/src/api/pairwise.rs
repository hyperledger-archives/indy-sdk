extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::pairwise::PairwiseCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;


/// Check if pairwise is exists.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encrypted DID
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// exists: true - if pairwise is exists, false - otherwise
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_is_pairwise_exists(command_handle: i32,
                                       wallet_handle: i32,
                                       their_did: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, exists: bool)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::PairwiseExists(
            wallet_handle,
            their_did,
            Box::new(move |result| {
                let (err, exists) = result_to_err_code_1!(result, false);
                cb(command_handle, err, exists)
            })
        )));

    result_to_err_code!(result)
}


/// Creates pairwise.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encrypted DID
/// my_did: encrypted DID
/// metadata Optional: extra information for pairwise
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_create_pairwise(command_handle: i32,
                                    wallet_handle: i32,
                                    their_did: *const c_char,
                                    my_did: *const c_char,
                                    metadata: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(metadata, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::CreatePairwise(
            wallet_handle,
            their_did,
            my_did,
            metadata,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Get list of saved pairwise.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// list_pairwise: list of saved pairwise
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_list_pairwise(command_handle: i32,
                                  wallet_handle: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, list_pairwise: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::ListPairwise(
            wallet_handle,
            Box::new(move |result| {
                let (err, list_pairwise) = result_to_err_code_1!(result, String::new());
                let list_pairwise = CStringUtils::string_to_cstring(list_pairwise);
                cb(command_handle, err, list_pairwise.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Gets pairwise information for specific their_did.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encoded Did
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// pairwise_info_json: did info associated with their did
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_get_pairwise(command_handle: i32,
                                 wallet_handle: i32,
                                 their_did: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, pairwise_info_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::GetPairwise(
            wallet_handle,
            their_did,
            Box::new(move |result| {
                let (err, pairwise_info_json) = result_to_err_code_1!(result, String::new());
                let pairwise_info_json = CStringUtils::string_to_cstring(pairwise_info_json);
                cb(command_handle, err, pairwise_info_json.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Save some data in the Wallet for pairwise associated with Did.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encoded Did
/// metadata: some extra information for pairwise
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub  extern fn indy_set_pairwise_metadata(command_handle: i32,
                                          wallet_handle: i32,
                                          their_did: *const c_char,
                                          metadata: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(metadata, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::SetPairwiseMetadata(
            wallet_handle,
            their_did,
            metadata,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}