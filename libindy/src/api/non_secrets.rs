extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::non_secrets::NonSecretsCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;


/// Create a new non-secret record in the wallet
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record
/// tags_json: the record tags used for search and storing meta information as json:
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
pub extern fn indy_add_wallet_record(command_handle: i32,
                                     wallet_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     value: *const c_char,
                                     tags_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_add_wallet_record: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}, tags_json: {:?}", wallet_handle, type_, id, value, tags_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(value, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(tags_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_add_wallet_record: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}, tags_json: {:?}", wallet_handle, type_, id, value, tags_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::AddRecord(
                wallet_handle,
                type_,
                id,
                value,
                tags_json,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_add_wallet_record:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_add_wallet_record: <<< res: {:?}", res);

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
pub extern fn indy_update_wallet_record_value(command_handle: i32,
                                              wallet_handle: i32,
                                              type_: *const c_char,
                                              id: *const c_char,
                                              value: *const c_char,
                                              cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_update_wallet_record_value: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}", wallet_handle, type_, id, value);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(value, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_update_wallet_record_value: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, value: {:?}", wallet_handle, type_, id, value);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::UpdateRecordValue(
                wallet_handle,
                type_,
                id,
                value,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_update_wallet_record_value:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_update_wallet_record_value: <<< res: {:?}", res);

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
pub extern fn indy_update_wallet_record_tags(command_handle: i32,
                                             wallet_handle: i32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             tags_json: *const c_char,
                                             cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_update_wallet_record_tags: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tags_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_update_wallet_record_tags: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::UpdateRecordTags(
                wallet_handle,
                type_,
                id,
                tags_json,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_update_wallet_record_tags:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_update_wallet_record_tags: <<< res: {:?}", res);

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
pub extern fn indy_add_wallet_record_tags(command_handle: i32,
                                          wallet_handle: i32,
                                          type_: *const c_char,
                                          id: *const c_char,
                                          tags_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_add_wallet_record_tags: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tags_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_add_wallet_record_tags: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tags_json: {:?}", wallet_handle, type_, id, tags_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::AddRecordTags(
                wallet_handle,
                type_,
                id,
                tags_json,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_add_wallet_record_tags:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_add_wallet_record_tags: <<< res: {:?}", res);

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
pub extern fn indy_delete_wallet_record_tags(command_handle: i32,
                                             wallet_handle: i32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             tag_names_json: *const c_char,
                                             cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_delete_wallet_record_tags: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tag_names_json: {:?}", wallet_handle, type_, id, tag_names_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(tag_names_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_delete_wallet_record_tags: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, tag_names_json: {:?}", wallet_handle, type_, id, tag_names_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::DeleteRecordTags(
                wallet_handle,
                type_,
                id,
                tag_names_json,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_delete_wallet_record_tags:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_delete_wallet_record_tags: <<< res: {:?}", res);

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
pub extern fn indy_delete_wallet_record(command_handle: i32,
                                        wallet_handle: i32,
                                        type_: *const c_char,
                                        id: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_delete_wallet_record: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}", wallet_handle, type_, id);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_delete_wallet_record: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}", wallet_handle, type_, id);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::DeleteRecord(
                wallet_handle,
                type_,
                id,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_delete_wallet_record:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_delete_wallet_record: <<< res: {:?}", res);

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
pub  extern fn indy_get_wallet_record(command_handle: i32,
                                      wallet_handle: i32,
                                      type_: *const c_char,
                                      id: *const c_char,
                                      options_json: *const c_char,
                                      cb: Option<extern fn(command_handle_: i32, err: ErrorCode,
                                                           record_json: *const c_char)>) -> ErrorCode {
    trace!("indy_get_wallet_record: >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, options_json: {:?}", wallet_handle, type_, id, options_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_get_wallet_record: entities >>> wallet_handle: {:?}, type_: {:?}, id: {:?}, options_json: {:?}", wallet_handle, type_, id, options_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::GetRecord(
                wallet_handle,
                type_,
                id,
                options_json,
                Box::new(move |result| {
                    let (err, record_json) = result_to_err_code_1!(result, String::new());
                    trace!("indy_get_wallet_record: record_json: {:?}", record_json);
                    let record_json = CStringUtils::string_to_cstring(record_json);
                    cb(command_handle, err, record_json.as_ptr())
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_get_wallet_record: <<< res: {:?}", res);

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
pub  extern fn indy_open_wallet_search(command_handle: i32,
                                       wallet_handle: i32,
                                       type_: *const c_char,
                                       query_json: *const c_char,
                                       options_json: *const c_char,
                                       cb: Option<extern fn(command_handle_: i32, err: ErrorCode,
                                                            search_handle: i32)>) -> ErrorCode {
    trace!("indy_open_wallet_search: >>> wallet_handle: {:?}, type_: {:?}, query_json: {:?}, options_json: {:?}", wallet_handle, type_, query_json, options_json);

    check_useful_c_str!(type_, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(query_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(options_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_open_wallet_search: entities >>> wallet_handle: {:?}, type_: {:?}, query_json: {:?}, options_json: {:?}", wallet_handle, type_, query_json, options_json);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::OpenSearch(
                wallet_handle,
                type_,
                query_json,
                options_json,
                Box::new(move |result| {
                    let (err, handle) = result_to_err_code_1!(result, 0);
                    trace!("indy_open_wallet_search: handle: {:?}", handle);
                    cb(command_handle, err, handle)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_open_wallet_search: <<< res: {:?}", res);

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
pub  extern fn indy_fetch_wallet_search_next_records(command_handle: i32,
                                                     wallet_handle: i32,
                                                     wallet_search_handle: i32,
                                                     count: usize,
                                                     cb: Option<extern fn(command_handle_: i32, err: ErrorCode,
                                                                          records_json: *const c_char)>) -> ErrorCode {
    trace!("indy_fetch_wallet_search_next_records: >>> wallet_handle: {:?}, wallet_search_handle: {:?}, count: {:?}", wallet_handle, wallet_search_handle, count);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_fetch_wallet_search_next_records: entities >>> wallet_handle: {:?}, wallet_search_handle: {:?}, count: {:?}", wallet_handle, wallet_search_handle, count);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::FetchSearchNextRecords(
                wallet_handle,
                wallet_search_handle,
                count,
                Box::new(move |result| {
                    let (err, records_json) = result_to_err_code_1!(result, String::new());
                    trace!("indy_fetch_wallet_search_next_records: records_json: {:?}", records_json);
                    let records_json = CStringUtils::string_to_cstring(records_json);
                    cb(command_handle, err, records_json.as_ptr())
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_fetch_wallet_search_next_records: <<< res: {:?}", res);

    res
}

/// Close wallet search (make search handle invalid)
///
/// #Params
/// wallet_search_handle: wallet search handle
#[no_mangle]
pub  extern fn indy_close_wallet_search(command_handle: i32,
                                        wallet_search_handle: i32,
                                        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode {
    trace!("indy_close_wallet_search: >>> wallet_search_handle: {:?}", wallet_search_handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_close_wallet_search: entities >>> wallet_search_handle: {:?}", wallet_search_handle);

    let result = CommandExecutor::instance()
        .send(Command::NonSecrets(
            NonSecretsCommand::CloseSearch(
                wallet_search_handle,
                Box::new(move |result| {
                    let err = result_to_err_code!(result);
                    trace!("indy_close_wallet_search:");
                    cb(command_handle, err)
                })
            )));

    let res = result_to_err_code!(result);

    trace!("indy_close_wallet_search: <<< res: {:?}", res);

    res
}