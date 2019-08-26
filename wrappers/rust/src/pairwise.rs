use {ErrorCode, IndyError};

use futures::Future;

use std::ffi::CString;
use std::ptr::null;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::pairwise;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseBoolCB};
use {WalletHandle, CommandHandle};

pub fn is_pairwise_exists(wallet_handle: WalletHandle, their_did: &str) -> Box<dyn Future<Item=bool, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

    let err = _is_pairwise_exists(command_handle, wallet_handle, their_did, cb);

    ResultHandler::bool(command_handle, err, receiver)
}

fn _is_pairwise_exists(command_handle: CommandHandle, wallet_handle: WalletHandle, their_did: &str, cb: Option<ResponseBoolCB>) -> ErrorCode {
    let their_did = c_str!(their_did);

    ErrorCode::from(unsafe {
        pairwise::indy_is_pairwise_exists(command_handle, wallet_handle, their_did.as_ptr(), cb)
    })
}

pub fn create_pairwise(wallet_handle: WalletHandle, their_did: &str, my_did: &str, metadata: Option<&str>) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _create_pairwise(command_handle, wallet_handle, their_did, my_did, metadata, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _create_pairwise(command_handle: CommandHandle, wallet_handle: WalletHandle, their_did: &str, my_did: &str, metadata: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let their_did = c_str!(their_did);
    let my_did = c_str!(my_did);
    let metadata_str = opt_c_str!(metadata);

    ErrorCode::from(unsafe {
        pairwise::indy_create_pairwise(command_handle, wallet_handle, their_did.as_ptr(), my_did.as_ptr(), opt_c_ptr!(metadata, metadata_str), cb)
    })
}

pub fn list_pairwise(wallet_handle: WalletHandle) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _list_pairwise(command_handle, wallet_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _list_pairwise(command_handle: CommandHandle, wallet_handle: WalletHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        pairwise::indy_list_pairwise(command_handle, wallet_handle, cb)
    })
}

pub fn get_pairwise(wallet_handle: WalletHandle, their_did: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_pairwise(command_handle, wallet_handle, their_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_pairwise(command_handle: CommandHandle, wallet_handle: WalletHandle, their_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let their_did = c_str!(their_did);

    ErrorCode::from(unsafe {
        pairwise::indy_get_pairwise(command_handle, wallet_handle, their_did.as_ptr(), cb)
    })
}

pub fn set_pairwise_metadata(wallet_handle: WalletHandle, their_did: &str, metadata: Option<&str>) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _set_pairwise_metadata(command_handle, wallet_handle, their_did, metadata, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _set_pairwise_metadata(command_handle: CommandHandle, wallet_handle: WalletHandle, their_did: &str, metadata: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let their_did = c_str!(their_did);
    let metadata_str = opt_c_str!(metadata);

    ErrorCode::from(unsafe {
        pairwise::indy_set_pairwise_metadata(command_handle, wallet_handle, their_did.as_ptr(), opt_c_ptr!(metadata, metadata_str), cb)
    })
}
