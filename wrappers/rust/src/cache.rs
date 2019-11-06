use futures::Future;

use {ErrorCode, IndyError};

use std::ffi::CString;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::cache;
use ffi::{ResponseEmptyCB, ResponseStringCB};
use {WalletHandle, CommandHandle, PoolHandle};

/// Get schema json data for specified schema id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
///
/// EXPERIMENTAL
///
/// # Arguments
/// * `pool_handle` - pool handle (created by open_pool_ledger).
/// * `wallet_handle` - wallet handle (created by open_wallet).
/// * `submitter_did` - DID of the submitter stored in secured Wallet.
/// * `id` - identifier of schema.
/// * `options_json` -
///  {
///    noCache: (bool, optional, false by default) Skip usage of cache,
///    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
///    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
///    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
///  }
/// # Returns
/// Schema json.
/// {
///     id: identifier of schema
///     attrNames: array of attribute name strings
///     name: Schema's name string
///     version: Schema's version string
///     ver: Version of the Schema json
/// }
pub fn get_schema(pool_handle: PoolHandle,
                  wallet_handle: WalletHandle,
                  submitter_did: &str,
                  id: &str,
                  options_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_schema(command_handle, pool_handle, wallet_handle, submitter_did, id, options_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

pub fn _get_schema(command_handle: CommandHandle,
                   pool_handle: PoolHandle,
                   wallet_handle: WalletHandle,
                   submitter_did: &str,
                   id: &str,
                   options_json: &str,
                   cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let id = c_str!(id);
    let options_json = c_str!(options_json);

    ErrorCode::from(
        unsafe {
            cache::indy_get_schema(command_handle, pool_handle, wallet_handle, submitter_did.as_ptr(), id.as_ptr(), options_json.as_ptr(), cb)
        }
    )
}

/// Get credential definition json data for specified credential definition id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
///
/// EXPERIMENTAL
///
/// # Arguments
/// * `pool_handle` - pool handle (created by open_pool_ledger).
/// * `wallet_handle` - wallet handle (created by open_wallet).
/// * `submitter_did` - DID of the submitter stored in secured Wallet.
/// * `id` - identifier of credential definition.
/// * `options_json` -
///  {
///    noCache: (bool, optional, false by default) Skip usage of cache,
///    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
///    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
///    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
///  }
/// # Returns
/// Credential Definition json.
/// {
///     id: string - identifier of credential definition
///     schemaId: string - identifier of stored in ledger schema
///     type: string - type of the credential definition. CL is the only supported type now.
///     tag: string - allows to distinct between credential definitions for the same issuer and schema
///     value: Dictionary with Credential Definition's data: {
///         primary: primary credential public key,
///         Optional<revocation>: revocation credential public key
///     },
///     ver: Version of the Credential Definition json
/// }
pub fn get_cred_def(pool_handle: PoolHandle,
                    wallet_handle: WalletHandle,
                    submitter_did: &str,
                    id: &str,
                    options_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_cred_def(command_handle, pool_handle, wallet_handle, submitter_did, id, options_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

pub fn _get_cred_def(command_handle: CommandHandle,
                     pool_handle: PoolHandle,
                     wallet_handle: WalletHandle,
                     submitter_did: &str,
                     id: &str,
                     options_json: &str,
                     cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did = c_str!(submitter_did);
    let id = c_str!(id);
    let options_json = c_str!(options_json);

    ErrorCode::from(
        unsafe {
            cache::indy_get_cred_def(command_handle, pool_handle, wallet_handle, submitter_did.as_ptr(), id.as_ptr(), options_json.as_ptr(), cb)
        }
    )
}

/// Purge schema cache.
///
/// EXPERIMENTAL
///
/// # Arguments
/// * `wallet_handle` - wallet handle (created by open_wallet).
/// * `id` - identifier of schema.
/// * `options_json` -
///  {
///    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
///  }
pub fn purge_schema_cache(wallet_handle: WalletHandle, options_json: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _purge_schema_cache(command_handle, wallet_handle, options_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _purge_schema_cache(command_handle: CommandHandle, wallet_handle: WalletHandle, options_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let options_json = c_str!(options_json);

    ErrorCode::from(unsafe { cache::indy_purge_schema_cache(command_handle, wallet_handle, options_json.as_ptr(), cb) })
}

/// Purge credential definition cache.
///
/// EXPERIMENTAL
///
/// # Arguments
/// * `wallet_handle` - wallet handle (created by open_wallet).
/// * `id` - identifier of credential definition.
/// * `options_json` -
///  {
///    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
///  }
pub fn purge_cred_def_cache(wallet_handle: WalletHandle, options_json: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _purge_cred_def_cache(command_handle, wallet_handle, options_json, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _purge_cred_def_cache(command_handle: CommandHandle, wallet_handle: WalletHandle, options_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let options_json = c_str!(options_json);

    ErrorCode::from(unsafe { cache::indy_purge_cred_def_cache(command_handle, wallet_handle, options_json.as_ptr(), cb) })
}