use {ErrorCode, IndyHandle, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::did;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseStringStringCB};

use utils::callbacks::{ClosureHandler, ResultHandler};

/// Creates keys (signing and encryption keys) for a new
/// DID (owned by the caller of the library).
/// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
/// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
/// and encrypt transactions.
///
/// # Arguments
/// * `wallet_handle` - wallet handler (created by Wallet::open).
/// * `did_json` - Identity information as json.
///
///  # Examples
/// `did_json`
/// {
///     "did": string, (optional;
///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
///             if not provided and cid is true then the full verkey will be used as a new DID;
///             if provided, then keys will be replaced - key rotation use case)
///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
///                                Can be UTF-8, base64 or hex string.
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
///     "cid": bool, (optional; if not set then false is used;)
/// }
///
/// # Returns
///   * `did` - DID generated and stored in the wallet
///   * `verkey` - The DIDs verification key
pub fn create_and_store_my_did(wallet_handle: IndyHandle, did_json: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _create_and_store_my_did(command_handle, wallet_handle, did_json, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _create_and_store_my_did(command_handle: IndyHandle, wallet_handle: IndyHandle, did_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let did_json = c_str!(did_json);

    ErrorCode::from(unsafe { did::indy_create_and_store_my_did(command_handle, wallet_handle, did_json.as_ptr(), cb) })
}

/// Generated temporary keys (signing and encryption keys) for an existing
/// DID (owned by the caller of the library).
///
/// # Arguments
/// * `wallet_handle` - wallet handler (created by Wallet::open).
/// * `tgt_did` - DID to replace keys.
/// * `identity_json` - Identity information as json.
/// # Example
/// * `identity_json`-
/// {
///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
///                                Can be UTF-8, base64 or hex string.
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
/// }
///
/// # Returns
/// * `verkey` - The DIDs verification key
pub fn replace_keys_start(wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _replace_keys_start(command_handle, wallet_handle, tgt_did, identity_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _replace_keys_start(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let tgt_did = c_str!(tgt_did);
    let identity_json = c_str!(identity_json);

    ErrorCode::from(unsafe { did::indy_replace_keys_start(command_handle, wallet_handle, tgt_did.as_ptr(), identity_json.as_ptr(), cb) })
}

/// Apply temporary keys as main for an existing DID (owned by the caller of the library).
///
/// # Arguments
/// * `wallet_handle` - wallet handler (created by Wallet::open).
/// * `tgt_did` - DID stored in the wallet
pub fn replace_keys_apply(wallet_handle: IndyHandle, tgt_did: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _replace_keys_apply(command_handle, wallet_handle, tgt_did, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _replace_keys_apply(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let tgt_did = c_str!(tgt_did);

    ErrorCode::from(unsafe { did::indy_replace_keys_apply(command_handle, wallet_handle, tgt_did.as_ptr(), cb) })
}

/// Saves their DID for a pairwise connection in a secured Wallet,
/// so that it can be used to verify transaction.
///
/// # Arguments
/// * `wallet_handle` - wallet handler (created by Wallet::open).
/// * `identity_json` - Identity information as json.
/// # Example:
/// * `identity_json`
///     {
///        "did": string, (required)
///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
///     }
pub fn store_their_did(wallet_handle: IndyHandle, identity_json: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _store_their_did(command_handle, wallet_handle, identity_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _store_their_did(command_handle: IndyHandle, wallet_handle: IndyHandle, identity_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let identity_json = c_str!(identity_json);

    ErrorCode::from(unsafe { did::indy_store_their_did(command_handle, wallet_handle, identity_json.as_ptr(), cb) })
}

/// Returns ver key (key id) for the given DID.
///
/// "get_ver_key" call follow the idea that we resolve information about their DID from
/// the ledger with cache in the local wallet. The "indy_Wallet::open" call has freshness parameter
/// that is used for checking the freshness of cached pool value.
///
/// Note if you don't want to resolve their DID info from the ledger you can use
/// "get_ver_key" call instead that will look only to the local wallet and skip
/// freshness checking.
///
/// Note that "new" makes similar wallet record as "Key::create_key".
/// As result we can use returned ver key in all generic crypto and messaging functions.
///
/// # Arguments
/// * `pool_handle` - Pool handle (created by Pool::open).
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - The DID to resolve key.
///
/// # Returns
/// * `key` - The DIDs ver key (key id).
pub fn key_for_did(pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _key_for_did(command_handle, pool_handle, wallet_handle, did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _key_for_did(command_handle: IndyHandle, pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let did = c_str!(did);

    ErrorCode::from(unsafe { did::indy_key_for_did(command_handle, pool_handle, wallet_handle, did.as_ptr(), cb) })
}

/// Returns ver key (key id) for the given DID.
///
/// "get_ver_key_did" call looks data stored in the local wallet only and skips freshness
/// checking.
///
/// Note if you want to get fresh data from the ledger you can use "get_ver_key" call
/// instead.
///
/// Note that "new" makes similar wallet record as "Key::create_key".
/// As result we can use returned ver key in all generic crypto and messaging functions.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - The DID to resolve key.
///
/// # Returns
/// * `key` - The DIDs ver key (key id).
pub fn key_for_local_did(wallet_handle: IndyHandle, did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _key_for_local_did(command_handle, wallet_handle, did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _key_for_local_did(command_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let did = c_str!(did);

    ErrorCode::from(unsafe { did::indy_key_for_local_did(command_handle, wallet_handle, did.as_ptr(), cb) })
}

/// Set/replaces endpoint information for the given DID.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - The DID to resolve endpoint.
/// * `address` -  The DIDs endpoint address.
/// * `transport_key` - The DIDs transport key (ver key, key id).
pub fn set_endpoint_for_did(wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _set_endpoint_for_did(command_handle, wallet_handle, did, address, transport_key, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _set_endpoint_for_did(command_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let did = c_str!(did);
    let address = c_str!(address);
    let transport_key = c_str!(transport_key);

    ErrorCode::from(unsafe { did::indy_set_endpoint_for_did(command_handle, wallet_handle, did.as_ptr(), address.as_ptr(), transport_key.as_ptr(), cb) })
}

/// Returns endpoint information for the given DID.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - The DID to resolve endpoint.
///
/// # Returns
/// * `endpoint` - The DIDs endpoint.
/// * `transport_vk` - The DIDs transport key (ver key, key id).
pub fn get_endpoint_for_did(wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str) -> Box<Future<Item=(String, Option<String>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

    let err = _get_endpoint_for_did(command_handle, wallet_handle, pool_handle, did, cb);

    ResultHandler::str_optstr(command_handle, err, receiver)
}

fn _get_endpoint_for_did(command_handle: IndyHandle, wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let did = c_str!(did);

    ErrorCode::from(unsafe { did::indy_get_endpoint_for_did(command_handle, wallet_handle, pool_handle, did.as_ptr(), cb) })
}

/// Saves/replaces the meta information for the giving DID in the wallet.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - the DID to store metadata.
/// * `metadata`  - the meta information that will be store with the DID.
pub fn set_did_metadata(wallet_handle: IndyHandle, tgt_did: &str, metadata: &str) -> Box<Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _set_did_metadata(command_handle, wallet_handle, tgt_did, metadata, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _set_did_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, metadata: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let tgt_did = c_str!(tgt_did);
    let metadata = c_str!(metadata);

    ErrorCode::from(unsafe { did::indy_set_did_metadata(command_handle, wallet_handle, tgt_did.as_ptr(), metadata.as_ptr(), cb) })
}

/// Retrieves the meta information for the giving DID in the wallet.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did`  - The DID to retrieve metadata.
///
/// #Returns
/// * `metadata`  - The meta information stored with the DID; Can be null if no metadata was saved for this DID.
pub fn get_did_metadata(wallet_handle: IndyHandle, tgt_did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_did_metadata(command_handle, wallet_handle, tgt_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_did_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let tgt_did = c_str!(tgt_did);

    ErrorCode::from(unsafe { did::indy_get_did_metadata(command_handle, wallet_handle, tgt_did.as_ptr(), cb) })
}

/// Retrieves the information about the giving DID in the wallet.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
/// * `did` - The DID to retrieve information.
///
/// # Returns
///  * `did_with_meta` -  {
///     "did": string - DID stored in the wallet,
///     "verkey": string - The DIDs transport key (ver key, key id),
///     "metadata": string - The meta information stored with the DID
///   }
pub fn get_my_did_with_metadata(wallet_handle: IndyHandle, my_did: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_my_did_with_metadata(command_handle, wallet_handle, my_did, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_my_did_with_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, my_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let my_did = c_str!(my_did);

    ErrorCode::from(unsafe { did::indy_get_my_did_with_meta(command_handle, wallet_handle, my_did.as_ptr(), cb) })
}

/// Retrieves the information about all DIDs stored in the wallet.
///
/// # Arguments
/// * `wallet_handle` - Wallet handle (created by Wallet::open).
///
/// # Returns
///  * `dids` - [{
///     "did": string - DID stored in the wallet,
///     "verkey": string - The DIDs transport key (ver key, key id).,
///     "tempVerkey": string - Temporary DIDs transport key (will be active after key rotation).
///     "metadata": string - The meta information stored with the DID
///   }]
pub fn list_my_dids_with_metadata(wallet_handle: IndyHandle) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _list_my_dids_with_metadata(command_handle, wallet_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _list_my_dids_with_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe { did::indy_list_my_dids_with_meta(command_handle, wallet_handle, cb) })
}

/// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
///
/// # Arguments
/// * `tgt_did` - DID.
/// * `full_verkey` - The DIDs verification key,
///
/// #Returns
///  * `verkey` - The DIDs verification key in either abbreviated or full form
pub fn abbreviate_verkey(tgt_did: &str, verkey: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _abbreviate_verkey(command_handle, tgt_did, verkey, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _abbreviate_verkey(command_handle: IndyHandle, tgt_did: &str, verkey: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let tgt_did = c_str!(tgt_did);
    let verkey = c_str!(verkey);

    ErrorCode::from(unsafe { did::indy_abbreviate_verkey(command_handle, tgt_did.as_ptr(), verkey.as_ptr(), cb) })
}
