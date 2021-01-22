use indy_api_types::{
    domain::wallet::Tags, errors::prelude::*, CommandHandle, ErrorCode, SearchHandle, WalletHandle,
    INVALID_SEARCH_HANDLE,
};

use indy_utils::ctypes;
use libc::c_char;
use serde_json;

use crate::commands::Locator;

/// Create a new non-secret record in the wallet
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record
/// tags_json: (optional) the record tags used for search and storing meta information as json:
///   {
///     "tagName1": <str>, // string tag (will be stored encrypted)
///     "tagName2": <str>, // string tag (will be stored encrypted)
///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
///   }
///   Note that null means no tags
///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
///   usage of this tag in complex search queries (comparison, predicates)
///   Encrypted tags can be searched only for exact matching
#[no_mangle]
pub extern "C" fn indy_add_wallet_record(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    value: *const c_char,
    tags_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_add_wallet_record > wallet_handle {:?} \
            type_ {:?} id {:?} value {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        value,
        tags_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(value, ErrorCode::CommonInvalidParam5);
    check_useful_opt_json!(tags_json, ErrorCode::CommonInvalidParam6, Tags);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!(
        "indy_add_wallet_record ? wallet_handle {:?} \
            type_ {:?} id {:?} value {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        value,
        tags_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .add_record(wallet_handle, type_, id, value, tags_json)
            .await;

        let err = prepare_result!(res);
        trace!("indy_add_wallet_record ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_add_wallet_record < {:?}", res);
    res
}

/// Update a non-secret wallet record value
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the new value of record
#[no_mangle]
pub extern "C" fn indy_update_wallet_record_value(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    value: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_update_wallet_record_value > wallet_handle {:?} \
            type_ {:?} id {:?} value {:?}",
        wallet_handle,
        type_,
        id,
        value
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(value, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!(
        "indy_update_wallet_record_value ? wallet_handle {:?} \
            type_ {:?} id {:?} value {:?}",
        wallet_handle,
        type_,
        id,
        value
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .update_record_value(wallet_handle, type_, id, value)
            .await;

        let err = prepare_result!(res);
        trace!("indy_update_wallet_record_value ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_update_wallet_record_value < {:?}", res);
    res
}

/// Update a non-secret wallet record tags
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": <str>, // string tag (will be stored encrypted)
///     "tagName2": <str>, // string tag (will be stored encrypted)
///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
///   }
///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
///   usage of this tag in complex search queries (comparison, predicates)
///   Encrypted tags can be searched only for exact matching
#[no_mangle]
pub extern "C" fn indy_update_wallet_record_tags(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    tags_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_update_wallet_record_tags > wallet_handle {:?} \
            type_ {:?} id {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        tags_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_json!(tags_json, ErrorCode::CommonInvalidParam5, Tags);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!(
        "indy_update_wallet_record_tags ? wallet_handle {:?} \
            type_ {:?} id {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        tags_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .update_record_tags(wallet_handle, type_, id, tags_json)
            .await;

        let err = prepare_result!(res);
        trace!("update_wallet_record_tags ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_update_wallet_record_tags < {:?}", res);
    res
}

/// Add new tags to the wallet record
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": <str>, // string tag (will be stored encrypted)
///     "tagName2": <str>, // string tag (will be stored encrypted)
///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
///   }
///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
///   usage of this tag in complex search queries (comparison, predicates)
///   Encrypted tags can be searched only for exact matching
///   Note if some from provided tags already assigned to the record than
///     corresponding tags values will be replaced
#[no_mangle]
pub extern "C" fn indy_add_wallet_record_tags(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    tags_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_add_wallet_record_tags > wallet_handle {:?} \
            type_ {:?} id {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        tags_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_json!(tags_json, ErrorCode::CommonInvalidParam5, Tags);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!(
        "indy_add_wallet_record_tags ? wallet_handle {:?} \
            type_ {:?} id {:?} tags_json {:?}",
        wallet_handle,
        type_,
        id,
        tags_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .add_record_tags(wallet_handle, type_, id, tags_json)
            .await;

        let err = prepare_result!(res);
        trace!("indy_add_wallet_record_tags ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_add_wallet_record_tags < {:?}", res);
    res
}

/// Delete tags from the wallet record
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tag_names_json: the list of tag names to remove from the record as json array:
///   ["tagName1", "tagName2", ...]
#[no_mangle]
pub extern "C" fn indy_delete_wallet_record_tags(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    tag_names_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!("indy_delete_wallet_record_tags > wallet_handle {:?} type_ {:?} id {:?} tag_names_json {:?}", wallet_handle, type_, id, tag_names_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag_names_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_delete_wallet_record_tags ? wallet_handle {:?} type_ {:?} id {:?} tag_names_json {:?}", wallet_handle, type_, id, tag_names_json);

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .delete_record_tags(wallet_handle, type_, id, tag_names_json)
            .await;

        let err = prepare_result!(res);
        trace!("indy_delete_wallet_record_tags ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_delete_wallet_record_tags < {:?}", res);
    res
}

/// Delete an existing wallet record in the wallet
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: record type
/// id: the id of record
#[no_mangle]
pub extern "C" fn indy_delete_wallet_record(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_delete_wallet_record > wallet_handle {:?} type_ {:?} id {:?}",
        wallet_handle,
        type_,
        id
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_delete_wallet_record ? wallet_handle {:?} type_ {:?} id {:?}",
        wallet_handle,
        type_,
        id
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller.delete_record(wallet_handle, type_, id).await;

        let err = prepare_result!(res);
        trace!(
            "indy_adindy_delete_wallet_recordd_wallet_record_tags ? err {:?}",
            err
        );

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_delete_wallet_record < {:?}", res);
    res
}

/// Get an wallet record by id
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, false by default) Retrieve record tags
///  }
/// #Returns
/// wallet record json:
/// {
///   id: "Some id",
///   type: "Some type", // present only if retrieveType set to true
///   value: "Some value", // present only if retrieveValue set to true
///   tags: <tags json>, // present only if retrieveTags set to true
/// }
#[no_mangle]
pub extern "C" fn indy_get_wallet_record(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    id: *const c_char,
    options_json: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, record_json: *const c_char),
    >,
) -> ErrorCode {
    trace!(
        "indy_get_wallet_record > wallet_handle {:?} type_ {:?} id {:?} options_json {:?}",
        wallet_handle,
        type_,
        id,
        options_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!(
        "indy_get_wallet_record ? wallet_handle {:?} type_ {:?} id {:?} options_json {:?}",
        wallet_handle,
        type_,
        id,
        options_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .get_record(wallet_handle, type_, id, options_json)
            .await;

        let (err, res) = prepare_result_1!(res, String::new());
        trace!("indy_get_wallet_record ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr());
    });

    let res = ErrorCode::Success;
    trace!("indy_get_wallet_record < {:?}", res);
    res
}

/// Search for wallet records.
///
/// Note instead of immediately returning of fetched records
/// this call returns wallet_search_handle that can be used later
/// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
///
/// #Params
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// query_json: MongoDB style query to wallet record tags:
///  {
///    "tagName": "tagValue",
///    $or: {
///      "tagName2": { $regex: 'pattern' },
///      "tagName3": { $gte: '123' },
///    },
///  }
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
///    retrieveTotalCount: (optional, false by default) Calculate total count,
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, false by default) Retrieve record tags,
///  }
/// #Returns
/// search_handle: Wallet search handle that can be used later
///   to fetch records by small batches (with indy_fetch_wallet_search_next_records)
#[no_mangle]
pub extern "C" fn indy_open_wallet_search(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    type_: *const c_char,
    query_json: *const c_char,
    options_json: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, search_handle: SearchHandle),
    >,
) -> ErrorCode {
    trace!(
        "indy_open_wallet_search > wallet_handle {:?} \
            type_ {:?} query_json {:?} options_json {:?}",
        wallet_handle,
        type_,
        query_json,
        options_json
    );

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(query_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!(
        "indy_open_wallet_search ? wallet_handle {:?} \
            type_ {:?} query_json {:?} options_json {:?}",
        wallet_handle,
        type_,
        query_json,
        options_json
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .open_search(wallet_handle, type_, query_json, options_json)
            .await;

        let (err, handle) = prepare_result_1!(res, INVALID_SEARCH_HANDLE);

        trace!(
            "indy_open_wallet_search ? err {:?} handle {:?}",
            err,
            handle
        );

        cb(command_handle, err, handle)
    });

    let res = ErrorCode::Success;
    trace!("indy_open_wallet_search < {:?}", res);
    res
}

/// Fetch next records for wallet search.
///
/// Not if there are no records this call returns WalletNoRecords error.
///
/// #Params
/// wallet_handle: wallet handle (created by open_wallet)
/// wallet_search_handle: wallet search handle (created by indy_open_wallet_search)
/// count: Count of records to fetch
///
/// #Returns
/// wallet records json:
/// {
///   totalCount: <str>, // present only if retrieveTotalCount set to true
///   records: [{ // present only if retrieveRecords set to true
///       id: "Some id",
///       type: "Some type", // present only if retrieveType set to true
///       value: "Some value", // present only if retrieveValue set to true
///       tags: <tags json>, // present only if retrieveTags set to true
///   }],
/// }
#[no_mangle]
pub extern "C" fn indy_fetch_wallet_search_next_records(
    command_handle: CommandHandle,
    wallet_handle: WalletHandle,
    wallet_search_handle: SearchHandle,
    count: usize,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, records_json: *const c_char),
    >,
) -> ErrorCode {
    trace!(
        "indy_fetch_wallet_search_next_records > \
            wallet_handle {:?} wallet_search_handle {:?} count {:?}",
        wallet_handle,
        wallet_search_handle,
        count
    );

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_fetch_wallet_search_next_records ? wallet_handle {:?} \
            wallet_search_handle {:?} count {:?}",
        wallet_handle,
        wallet_search_handle,
        count
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller
            .fetch_search_next_records(wallet_handle, wallet_search_handle, count)
            .await;

        let (err, res) = prepare_result_1!(res, String::new());

        trace!(
            "indy_fetch_wallet_search_next_records ? err {:?} res {:?}",
            err,
            res
        );

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr());
    });

    let res = ErrorCode::Success;
    trace!("indy_fetch_wallet_search_next_records < {:?}", res);
    res
}

/// Close wallet search (make search handle invalid)
///
/// #Params
/// wallet_search_handle: wallet search handle
#[no_mangle]
pub extern "C" fn indy_close_wallet_search(
    command_handle: CommandHandle,
    wallet_search_handle: SearchHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    trace!(
        "indy_close_wallet_search > wallet_search_handle {:?}",
        wallet_search_handle
    );

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!(
        "indy_close_wallet_search ? wallet_search_handle {:?}",
        wallet_search_handle
    );

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.non_secret_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller.close_search(wallet_search_handle).await;

        let err = prepare_result!(res);

        trace!("indy_close_wallet_search ? err {:?}", err);

        cb(command_handle, err);
    });

    let res = ErrorCode::Success;
    trace!("indy_close_wallet_search < {:?}", res);
    res
}
