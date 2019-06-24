use api::{ErrorCode, CommandHandle, WalletHandle, PoolHandle};
use commands::{Command, CommandExecutor};
use commands::cache::CacheCommand;
use errors::prelude::*;
use utils::ctypes;

use
libc::c_char;


/// Gets credential definition json data for specified credential definition id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
///
/// EXPERIMENTAL
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_handle: pool handle (created by open_pool_ledger).
/// wallet_handle: wallet handle (created by open_wallet).
/// submitter_did: DID of the submitter stored in secured Wallet.
/// id: identifier of credential definition.
/// options_json:
///  {
///    forceUpdate: (optional, false by default) Force update of record in cache from the ledger,
///  }
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_get_cred_def(command_handle: CommandHandle,
                                pool_handle: PoolHandle,
                                wallet_handle: WalletHandle,
                                submitter_did: *const c_char,
                                id: *const c_char,
                                options_json: *const c_char,
                                cb: Option<extern fn(command_handle_: CommandHandle,
                                                     err: ErrorCode,
                                                     cred_def_json: *const c_char)>) -> ErrorCode {
    trace!("indy_get_cred_def: >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
           pool_handle, wallet_handle, submitter_did, id, options_json);

    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_get_cred_def: entities >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
           pool_handle, wallet_handle, submitter_did, id, options_json);

    let result = CommandExecutor::instance()
        .send(Command::Cache(CacheCommand::GetCredDef(
            pool_handle,
            wallet_handle,
            submitter_did,
            id,
            options_json,
            Box::new(move |result| {
                let (err, cred_def_json) = prepare_result_1!(result, String::new());
                trace!("indy_get_cred_def: cred_def_json: {:?}", cred_def_json);
                let cred_def_json = ctypes::string_to_cstring(cred_def_json);
                cb(command_handle, err, cred_def_json.as_ptr())
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_get_schema: <<< res: {:?}", res);

    res
}

/// Gets schema json data for specified schema id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
///
/// EXPERIMENTAL
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_handle: pool handle (created by open_pool_ledger).
/// wallet_handle: wallet handle (created by open_wallet).
/// submitter_did: DID of the submitter stored in secured Wallet.
/// id: identifier of schema.
/// options_json:
///  {
///    noCache: (bool, optional, false by default) Skip usage of cache,
///    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
///    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
///    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
///  }
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_get_schema(command_handle: CommandHandle,
                              pool_handle: PoolHandle,
                              wallet_handle: WalletHandle,
                              submitter_did: *const c_char,
                              id: *const c_char,
                              options_json: *const c_char,
                              cb: Option<extern fn(command_handle_: CommandHandle,
                                                   err: ErrorCode,
                                                   schema_json: *const c_char)>) -> ErrorCode {
    trace!("indy_get_schema: >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
           pool_handle, wallet_handle, submitter_did, id, options_json);

    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_get_schema: entities >>> pool_handle: {:?}, wallet_handle: {:?}, submitter_did: {:?}, id: {:?}, options_json: {:?}",
           pool_handle, wallet_handle, submitter_did, id, options_json);

    let result = CommandExecutor::instance()
        .send(Command::Cache(CacheCommand::GetSchema(
            pool_handle,
            wallet_handle,
            submitter_did,
            id,
            options_json,
            Box::new(move |result| {
                let (err, schema_json) = prepare_result_1!(result, String::new());
                trace!("indy_get_schema: schema_json: {:?}", schema_json);
                let schema_json = ctypes::string_to_cstring(schema_json);
                cb(command_handle, err, schema_json.as_ptr())
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_get_schema: <<< res: {:?}", res);

    res
}

/// Purge credential definition cache.
///
/// EXPERIMENTAL
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// wallet_handle: wallet handle (created by open_wallet).
/// options_json:
///  {
///    minFresh: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
///  }
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_purge_cred_def_cache(command_handle: CommandHandle,
                                        wallet_handle: WalletHandle,
                                        options_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: CommandHandle,
                                                             err: ErrorCode)>) -> ErrorCode {
    trace!("indy_purge_cred_def_cache: >>> wallet_handle: {:?}, options_json: {:?}",
           wallet_handle, options_json);

    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_purge_cred_def_cache: entities >>> wallet_handle: {:?}, options_json: {:?}",
           wallet_handle, options_json);

    let result = CommandExecutor::instance()
        .send(Command::Cache(CacheCommand::PurgeCredDefCache(
            wallet_handle,
            options_json,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_purge_cred_def_cache:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_purge_cred_def_cache: <<< res: {:?}", res);

    res
}

/// Purge schema cache.
///
/// EXPERIMENTAL
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// wallet_handle: wallet handle (created by open_wallet).
/// options_json:
///  {
///    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
///  }
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_purge_schema_cache(command_handle: CommandHandle,
                                      wallet_handle: WalletHandle,
                                      options_json: *const c_char,
                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                           err: ErrorCode)>) -> ErrorCode {
    trace!("indy_purge_schema_cache: >>> wallet_handle: {:?}, options_json: {:?}",
           wallet_handle, options_json);

    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_purge_schema_cache: entities >>> wallet_handle: {:?}, options_json: {:?}",
           wallet_handle, options_json);

    let result = CommandExecutor::instance()
        .send(Command::Cache(CacheCommand::PurgeSchemaCache(
            wallet_handle,
            options_json,
            Box::new(move |result| {
                let err = prepare_result!(result);
                trace!("indy_purge_schema_cache:");
                cb(command_handle, err)
            })
        )));

    let res = prepare_result!(result);

    trace!("indy_purge_schema_cache: <<< res: {:?}", res);

    res
}
