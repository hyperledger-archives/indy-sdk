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
/// their_did: encrypting DID
/// my_did: encrypting DID
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// exists: true - if pairwise is exists, false - otherwise
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
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
/// their_did: encrypting DID
/// my_did: encrypting DID
/// cb: Callback that takes command result as parameter.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_create_pairwise(command_handle: i32,
                                    wallet_handle: i32,
                                    their_did: *const c_char,
                                    my_did: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(my_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::CreatePairwise(
            wallet_handle,
            their_did,
            my_did,
            Box::new(move |result| {
                let err = result_to_err_code!(result);
                cb(command_handle, err)
            })
        )));

    result_to_err_code!(result)
}

/// Get list of saved pairs.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// pairwise_list: list of saved pairs
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_list_pairwise(command_handle: i32,
                                  wallet_handle: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, pairwise_list: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::ListPairwise(
            wallet_handle,
            Box::new(move |result| {
                let (err, pairwise_list) = result_to_err_code_1!(result, String::new());
                let pairwise_list = CStringUtils::string_to_cstring(pairwise_list);
                cb(command_handle, err, pairwise_list.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Gets my did for specific their did.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encoded Did
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// my_did_json: did info associated with their did
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_pairwise_get_my_did(command_handle: i32,
                                        wallet_handle: i32,
                                        their_did: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, my_did: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::PairwiseGetMyDid(
            wallet_handle,
            their_did,
            Box::new(move |result| {
                let (err, my_did) = result_to_err_code_1!(result, String::new());
                let my_did = CStringUtils::string_to_cstring(my_did);
                cb(command_handle, err, my_did.as_ptr())
            })
        )));

    result_to_err_code!(result)
}

/// Save some data in the Wallet for a given DID .
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encoded Did
/// cb: Callback that takes command result as parameter.
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
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

/// Get some metadata from the Wallet for a given DID.
///
/// #Params
/// wallet_handle: wallet handler (created by open_wallet).
/// command_handle: command handle to map callback to user context.
/// their_did: encoded Did
/// cb: Callback that takes command result as parameter.
///
///
/// #Returns
/// metadata
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
#[no_mangle]
pub  extern fn indy_get_pairwise_metadata(command_handle: i32,
                                          wallet_handle: i32,
                                          their_did: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, metadata: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(their_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Pairwise(PairwiseCommand::GetPairwiseMetadata(
            wallet_handle,
            their_did,
            Box::new(move |result| {
                let (err, metadata) = result_to_err_code_1!(result, String::new());
                let metadata = CStringUtils::string_to_cstring(metadata);
                cb(command_handle, err, metadata.as_ptr())
            })
        )));

    result_to_err_code!(result)
}