use indy_api_types::{
    errors::prelude::*, validation::Validatable, CommandHandle, ErrorCode, PoolHandle, WalletHandle,
};

use indy_utils::ctypes;
use libc::c_char;

use crate::{
    commands::CommandExecutor,
    domain::{
        anoncreds::{credential_definition::CredentialDefinitionId, schema::SchemaId},
        cache::{GetCacheOptions, PurgeOptions},
        crypto::did::DidValue,
    },
};

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
pub extern "C" fn indy_get_cred_def(
    command_handle: CommandHandle,
    pool_handle: PoolHandle,
    wallet_handle: WalletHandle,
    submitter_did: *const c_char,
    id: *const c_char,
    options_json: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, cred_def_json: *const c_char),
    >,
) -> ErrorCode {
    trace!(
        "indy_get_cred_def > pool_handle {:?} \
            wallet_handle {:?} submitter_did {:?} \
            id {:?} options_json {:?}",
        pool_handle,
        wallet_handle,
        submitter_did,
        id,
        options_json
    );

    check_useful_validatable_string!(submitter_did, ErrorCode::CommonInvalidParam4, DidValue);
    check_useful_validatable_string!(id, ErrorCode::CommonInvalidParam5, CredentialDefinitionId);

    check_useful_json!(
        options_json,
        ErrorCode::CommonInvalidParam6,
        GetCacheOptions
    );

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!(
        "indy_get_cred_def ? pool_handle {:?} \
            wallet_handle {:?} submitter_did {:?} \
            id {:?} options_json {:?}",
        pool_handle,
        wallet_handle,
        submitter_did,
        id,
        options_json
    );

    let (executor, controller) = {
        let locator = CommandExecutor::instance();
        let executor = locator.executor.clone();
        let controller = locator.cache_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .get_cred_def(pool_handle, wallet_handle, submitter_did, id, options_json)
            .await;

        let (err, cred_def) = prepare_result_1!(res, String::new());
        trace!("indy_get_cred_def ? err {:?} cred_def {:?}", err, cred_def);

        let cred_def = ctypes::string_to_cstring(cred_def);
        cb(command_handle, err, cred_def.as_ptr())
    });

    let res = ErrorCode::Success;
    trace!("indy_get_cred_def < {:?}", res);
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
pub extern "C" fn indy_get_schema(
    command_handle: CommandHandle,
    pool_handle: PoolHandle,
    wallet_handle: WalletHandle,
    submitter_did: *const c_char,
    id: *const c_char,
    options_json: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, schema_json: *const c_char),
    >,
) -> ErrorCode {
    trace!(
        "indy_get_schema > pool_handle {:?} wallet_handle {:?} \
            submitter_did {:?} id {:?} options_json {:?}",
        pool_handle,
        wallet_handle,
        submitter_did,
        id,
        options_json
    );

    check_useful_validatable_string!(submitter_did, ErrorCode::CommonInvalidParam4, DidValue);
    check_useful_validatable_string!(id, ErrorCode::CommonInvalidParam5, SchemaId);

    check_useful_json!(
        options_json,
        ErrorCode::CommonInvalidParam6,
        GetCacheOptions
    );

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!(
        "indy_get_schema ? pool_handle {:?} wallet_handle {:?} \
            submitter_did {:?} id {:?} options_json {:?}",
        pool_handle,
        wallet_handle,
        submitter_did,
        id,
        options_json
    );

    let (executor, controller) = {
        let locator = CommandExecutor::instance();
        let executor = locator.executor.clone();
        let controller = locator.cache_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .get_schema(pool_handle, wallet_handle, submitter_did, id, options_json)
            .await;

        let (err, schema) = prepare_result_1!(res, String::new());
        trace!("indy_get_cred_def ? err {:?} schema {:?}", err, schema);

        let schema = ctypes::string_to_cstring(schema);
        cb(command_handle, err, schema.as_ptr())
    });

    let res = ErrorCode::Success;
    trace!("indy_get_schema < {:?}", res);
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
pub extern "C" fn indy_purge_cred_def_cache(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    options_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_purge_cred_def_cache > wallet_handle {:?} \
            options_json {:?}",
        wallet_handle,
        options_json
    );

    check_useful_json!(options_json, ErrorCode::CommonInvalidParam3, PurgeOptions);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!(
        "indy_purge_cred_def_cache ? wallet_handle {:?} \
            options_json {:?}",
        wallet_handle,
        options_json
    );

    let (executor, controller) = {
        let locator = CommandExecutor::instance();
        let executor = locator.executor.clone();
        let controller = locator.cache_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .purge_cred_def_cache(wallet_handle, options_json)
            .await;

        let err = prepare_result!(res);
        trace!("indy_purge_cred_def_cache ? err {:?}", err);
        cb(command_handle, err)
    });

    let res = ErrorCode::Success;
    trace!("indy_purge_cred_def_cache < {:?}", res);
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
///    minFresh: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
///  }
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern "C" fn indy_purge_schema_cache(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    options_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_purge_schema_cache > wallet_handle {:?} \
            options_json {:?}",
        wallet_handle,
        options_json
    );

    check_useful_json!(options_json, ErrorCode::CommonInvalidParam3, PurgeOptions);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!(
        "indy_purge_schema_cache ? wallet_handle {:?} \
            options_json {:?}",
        wallet_handle,
        options_json
    );

    let (executor, controller) = {
        let locator = CommandExecutor::instance();
        let executor = locator.executor.clone();
        let controller = locator.cache_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .purge_schema_cache(wallet_handle, options_json)
            .await;

        let err = prepare_result!(res);
        trace!("indy_purge_schema_cache ? err {:?}", err);
        cb(command_handle, err)
    });

    let res = ErrorCode::Success;
    trace!("indy_purge_schema_cache < {:?}", res);
    res
}
