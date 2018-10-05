extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use error::ToErrorCode;
use utils::libindy::payments::{pay_a_payee, get_wallet_token_info, create_address};
use utils::libindy::wallet::{export, import, get_wallet_handle};
use utils::libindy::wallet;
use std::path::Path;
use utils::threadpool::spawn;
use std::thread;

extern {
    pub fn indy_add_wallet_record(command_handle: i32,
                                  wallet_handle: i32,
                                  type_: *const c_char,
                                  id: *const c_char,
                                  value: *const c_char,
                                  tags_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;

    pub fn indy_get_wallet_record(command_handle: i32,
                                  wallet_handle: i32,
                                  type_: *const c_char,
                                  id: *const c_char,
                                  options_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32, err: i32, record_json: *const c_char)>) -> i32;

    pub fn indy_update_wallet_record_value(command_handle: i32,
                                           wallet_handle: i32,
                                           type_: *const c_char,
                                           id: *const c_char,
                                           value: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;

    pub fn indy_delete_wallet_record(command_handle: i32,
                                     wallet_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;
}

/// Get the total balance from all addresses contained in the configured wallet
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// payment_handle: for future use
///
/// cb: Callback that provides wallet balance
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_wallet_get_token_info(command_handle: u32,
                                     payment_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err:u32, *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    info!("vcx_wallet_get_token_info(command_handle: {}, payment_handle: {})",
          command_handle, payment_handle);

    spawn(move|| {
        match get_wallet_token_info() {
            Ok(x) => {
                info!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(0), x.to_string());

                let msg = CStringUtils::string_to_cstring(x.to_string());
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(x), "null");

        		let msg = CStringUtils::string_to_cstring("".to_string());
                cb(command_handle, x, msg.as_ptr());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Add a payment address to the wallet
///
/// #params
///
/// cb: Callback that provides payment address info
///
/// #Returns
/// Error code as u32

#[no_mangle]
pub extern fn vcx_wallet_create_payment_address(command_handle: u32,
                                                seed: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: u32, err:u32, address: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    let seed = if !seed.is_null() {
        check_useful_opt_c_str!(seed, error::INVALID_OPTION.code_num);
        seed
    } else {
        None
    };

    info!("vcx_wallet_create_payment_address(command_handle: {})",
          command_handle);

    spawn(move|| {
        match create_address(seed) {
            Ok(x) => {
                info!("vcx_wallet_create_payment_address_cb(command_handle: {}, rc: {}, address: {})",
                    command_handle, error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_wallet_create_payment_address_cb(command_handle: {}, rc: {}, address: {})",
                    command_handle, error_string(x), "null");

        		let msg = CStringUtils::string_to_cstring("".to_string());
                cb(command_handle, x, msg.as_ptr());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Adds a record to the wallet
/// Assumes there is an open wallet.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// value: value of the record with the associated id.
///
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": <str>, // string tag (will be stored encrypted)
///     "tagName2": <int>, // int tag (will be stored encrypted)
///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
///     "~tagName4": <int>, // int tag (will be stored un-encrypted)
///   }
///  The tags_json must be valid json, and if no tags are to be associated with the
/// record, then the empty '{}' json must be passed.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
///

#[no_mangle]
pub extern fn vcx_wallet_add_record(command_handle: u32,
                                    type_: *const c_char,
                                    id: *const c_char,
                                    value: *const c_char,
                                    tags_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(value, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tags_json, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);


    info!("vcx_wallet_add_record(command_handle: {}, type_: {}, id: {}, value: {}, tags_json: {})",
          command_handle, type_, id, value, tags_json);

    spawn(move|| {
        match wallet::add_record(&type_, &id, &value, &tags_json) {
            Ok(x) => {
                info!("vcx_wallet_add_record(command_handle: {}, rc: {})",
                      command_handle, error_string(0));

                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(x) => {
                info!("vcx_wallet_add_record(command_handle: {}, rc: {})",
                      command_handle, error_string(x));

                cb(command_handle, x);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Updates the value of a record already in the wallet.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// value: New value of the record with the associated id.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
///

#[no_mangle]
pub extern fn vcx_wallet_update_record_value(command_handle: u32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             value: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(value, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_wallet_update_record_value(command_handle: {}, type_: {}, id: {}, value: {})",
          command_handle, type_, id, value);

    spawn(move|| {
        match wallet::update_record_value(&type_, &id, &value) {
            Ok(x) => {
                info!("vcx_wallet_update_record_value(command_handle: {}, rc: {})",
                      command_handle, error_string(0));

                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(x) => {
                info!("vcx_wallet_update_record_value(command_handle: {}, rc: {})",
                      command_handle, error_string(x));

                cb(command_handle, x);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Updates the value of a record already in the wallet.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// tags: New tags for the record with the associated id and type.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
///

#[no_mangle]
pub extern fn vcx_wallet_update_record_tags(command_handle: u32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             tags: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tags, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    spawn(move|| {
        cb(command_handle, error::SUCCESS.code_num);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Adds tags to a record.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// tags: Tags for the record with the associated id and type.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
///

#[no_mangle]
pub extern fn vcx_wallet_add_record_tags(command_handle: u32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            tags: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tags, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    spawn(move|| {
        cb(command_handle, error::SUCCESS.code_num);
        Ok(())
    });

    error::SUCCESS.code_num
}

/// Deletes tags from a record.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// tags: Tags to remove from the record with the associated id and type.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
///

#[no_mangle]
pub extern fn vcx_wallet_delete_record_tags(command_handle: u32,
                                         type_: *const c_char,
                                         id: *const c_char,
                                         tags: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tags, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    spawn(move|| {
        cb(command_handle, error::SUCCESS.code_num);
        Ok(())
    });

    error::SUCCESS.code_num
}

/// Deletes an existing record.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
/// Error will be a libindy error code
///

#[no_mangle]
pub extern fn vcx_wallet_get_record(command_handle: u32,
                                    type_: *const c_char,
                                    id: *const c_char,
                                    options_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, record_json: *const c_char)>) -> u32 {

    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(options_json, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_wallet_get_record(command_handle: {}, type_: {}, id: {}, options: {})",
          command_handle, type_, id, options_json);

    spawn(move|| {
        match wallet::get_record(&type_, &id, &options_json) {
            Ok(x) => {
                info!("vcx_wallet_get_record(command_handle: {}, rc: {}, record_json: {})",
                      command_handle, error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);

                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                info!("vcx_wallet_get_record(command_handle: {}, rc: {}, record_json: {})",
                      command_handle, error_string(x), "null");

                let msg = CStringUtils::string_to_cstring("".to_string());
                cb(command_handle, x, msg.as_ptr());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Deletes an existing record.
/// Assumes there is an open wallet and that a type and id pair already exists.
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// id: the id ("key") of the record.
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
/// Error will be a libindy error code
///

#[no_mangle]
pub extern fn vcx_wallet_delete_record(command_handle: u32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_wallet_delete_record(command_handle: {}, type_: {}, id: {})",
          command_handle, type_, id);

    spawn(move|| {
        match wallet::delete_record(&type_, &id) {
            Ok(x) => {
                info!("vcx_wallet_delete_record(command_handle: {}, rc: {})",
                      command_handle, error_string(0));

                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(x) => {
                info!("vcx_wallet_delete_record(command_handle: {}, rc: {})",
                      command_handle, error_string(x));

                cb(command_handle, x);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Send tokens to a specific address
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// payment_handle: for future use (currently uses any address in the wallet)
///
/// tokens: number of tokens to send
///
/// recipient: address of recipient
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_wallet_send_tokens(command_handle: u32,
                                     payment_handle: u32,
                                     tokens: *const c_char,
                                     recipient: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, receipt: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(recipient, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tokens, error::INVALID_OPTION.code_num);

    let tokens: u64 = match tokens.parse::<u64>() {
        Ok(x) => x,
        Err(_) => return error::INVALID_OPTION.code_num,
    };
    info!("vcx_wallet_send_tokens(command_handle: {}, payment_handle: {}, tokens: {}, recipient: {})",
          command_handle, payment_handle, tokens, recipient);

    spawn(move|| {
        match pay_a_payee(tokens, &recipient) {
            Ok((payment, msg)) => {
                info!("vcx_wallet_send_tokens_cb(command_handle: {}, rc: {}, receipt: {})",
                      command_handle, error_string(0), msg);
                let msg = CStringUtils::string_to_cstring(msg);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(e) => {
                let msg = "Failed to send tokens".to_string();
                info!("vcx_wallet_send_tokens_cb(command_handle: {}, rc: {}, reciept: {})", command_handle, e.to_error_code(), msg);
                let msg = CStringUtils::string_to_cstring("".to_string());
                cb(command_handle, e.to_error_code(), msg.as_ptr());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Opens a storage search handle
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// type_: type of record. (e.g. 'data', 'string', 'foobar', 'image')
///
/// query_json: MongoDB style query to wallet record tags:
///  {
///    "tagName": "tagValue",
///    $or: {
///      "tagName2": { $regex: 'pattern' },
///      "tagName3": { $gte: 123 },
///    },
///  }
/// options_json:
///  {
///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
///    retrieveTotalCount: (optional, false by default) Calculate total count,
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags,
///  }
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub  extern fn vcx_wallet_open_search(command_handle: i32,
                                       type_: *const c_char,
                                       query_json: *const c_char,
                                       options_json: *const c_char,
                                       cb: Option<extern fn(command_handle_: i32, err: u32,
                                                            search_handle: i32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    use utils::constants::DEFAULT_SEARCH_HANDLE;
    spawn(move|| {
        cb(command_handle, error::SUCCESS.code_num, DEFAULT_SEARCH_HANDLE as i32);
        Ok(())
    });

    error::SUCCESS.code_num
}
/// Fetch next records for wallet search.
///
/// Not if there are no records this call returns WalletNoRecords error.
///
/// #Params
/// wallet_handle: wallet handle (created by open_wallet)
/// wallet_search_handle: wallet wallet handle (created by indy_open_wallet_search)
/// count: Count of records to fetch
///
/// #Returns
/// wallet records json:
/// {
///   totalCount: <int>, // present only if retrieveTotalCount set to true
///   records: [{ // present only if retrieveRecords set to true
///       id: "Some id",
///       type: "Some type", // present only if retrieveType set to true
///       value: "Some value", // present only if retrieveValue set to true
///       tags: <tags json>, // present only if retrieveTags set to true
///   }],
/// }

#[no_mangle]
pub  extern fn vcx_wallet_search_next_records(command_handle: i32,
                                                     wallet_search_handle: i32,
                                                     count: usize,
                                                     cb: Option<extern fn(command_handle_: i32, err: u32,
                                                                          records_json: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    spawn(move|| {
        use utils::constants::DEFAULT_SEARCH_RECORD;
        let msg = CStringUtils::string_to_cstring(DEFAULT_SEARCH_RECORD.to_string());
        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
        Ok(())
    });

    error::SUCCESS.code_num
}

/// Close a search
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// search_handle: for future use
///
/// cb: Callback that provides wallet balance
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_wallet_close_search(command_handle: u32,
                                        search_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err:u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    info!("vcx_wallet_close_search(command_handle: {}, search_handle: {})",
          command_handle, search_handle);

    spawn(move|| {
        info!("vcx_wallet_close_search(command_handle: {}, rc: {})",
              command_handle, error_string(0));
        cb(command_handle, error::SUCCESS.code_num);
        Ok(())
    });

    error::SUCCESS.code_num
}

/// Exports opened wallet
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params:
/// command_handle: Handle for User's Reference only.
/// path: Path to export wallet to User's File System.
/// backup_key: String representing the User's Key for securing (encrypting) the exported Wallet.
/// cb: Callback that provides the success/failure of the api call.
/// #Returns
/// Error code - success indicates that the api call was successfully created and execution
/// is scheduled to begin in a separate thread.

#[no_mangle]
pub extern fn vcx_wallet_export(command_handle: u32,
                                path: *const c_char,
                                backup_key: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32,
                                                     err: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(path,  error::INVALID_OPTION.code_num);
    check_useful_c_str!(backup_key, error::INVALID_OPTION.code_num);
    spawn(move|| {
        let path = Path::new(&path);
        info!("vcx_wallet_export(command_handle: {}, path: {:?}, backup_key: ****)", command_handle, path);
        match export(get_wallet_handle(), &path, &backup_key) {
            Ok(_) => {
                let return_code = error::SUCCESS.code_num;
                info!("vcx_wallet_export(command_handle: {}, rc: {})", command_handle, return_code);
                cb(command_handle, return_code);
            }
            Err(e) => {
                let return_code = e.to_error_code();
                warn!("vcx_wallet_export(command_handle: {}, rc: {})", command_handle, return_code);
                cb(command_handle, return_code);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Creates a new secure wallet and then imports its content
/// according to fields provided in import_config
/// Cannot be used if wallet is already opened (Especially if vcx_init has already been used).
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// config: "{"wallet_name":"","wallet_key":"","exported_wallet_path":"","backup_key":""}"
/// exported_wallet_path: Path of the file that contains exported wallet content
/// backup_key: Key used when creating the backup of the wallet (For encryption/decrption)
/// cb: Callback that provides the success/failure of the api call.
/// #Returns
/// Error code - success indicates that the api call was successfully created and execution
/// is scheduled to begin in a separate thread.
#[no_mangle]
pub extern fn vcx_wallet_import(command_handle: u32,
                                config: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32,
                                                     err: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(config,  error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        info!("vcx_wallet_import(command_handle: {}, config: ****)", command_handle);
        match import(&config) {
            Ok(_) => {
                let return_code = error::SUCCESS.code_num;
                info!("vcx_wallet_import(command_handle: {}, rc: {})", command_handle, return_code);
                cb(command_handle, return_code);
            }
            Err(e) => {
                let return_code = e.to_error_code();
                warn!("vcx_wallet_import(command_handle: {}, rc: {})", command_handle, return_code);
                cb(command_handle, return_code);
            }
        };
    });

    error::SUCCESS.code_num
}

// Functionality in Libindy for validating an address in NOT there yet
/// Validates a Payment address
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// payment_address: value to be validated as a payment address
///
/// cb: Callback that any errors
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub  extern fn vcx_wallet_validate_payment_address(command_handle: i32,
                                                   payment_address: *const c_char,
                                                   cb: Option<extern fn(command_handle_: i32, err: u32)>) -> u32 {
    check_useful_c_str!(payment_address,  error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    spawn(move|| {
        cb(command_handle, error::SUCCESS.code_num);
        Ok(())
    });

    error::SUCCESS.code_num
}

#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use std::ptr;
    use std::ffi::CString;
    use std::time::Duration;
    use utils::libindy::{ return_types_u32, wallet::delete_wallet};

    #[test]
    fn test_get_token_info() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_get_token_info(cb.command_handle,
                                             0,
                                             Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_send_tokens() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_send_tokens(cb.command_handle,
                                          0,
                                          CString::new("1").unwrap().into_raw(),
                                          CString::new("address").unwrap().into_raw(),
                                          Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_create_address() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_create_payment_address(cb.command_handle,
                                                     ptr::null_mut(),
                                                     Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_send_payment() {
        init!("ledger");
        let recipient = CStringUtils::string_to_cstring(::utils::constants::PAYMENT_ADDRESS.to_string());
        println!("sending payment to {:?}", recipient);
        let balance = ::utils::libindy::payments::get_wallet_token_info().unwrap().get_balance();
        let tokens = 5;
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_send_tokens(cb.command_handle,
                                         0,
                                         CString::new(format!("{}", tokens)).unwrap().into_raw(),
                                         recipient.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
        let new_balance = ::utils::libindy::payments::get_wallet_token_info().unwrap().get_balance();
        assert_eq!(balance - tokens, new_balance);
    }

    #[test]
    fn test_add_record() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring("{}".to_string());

        // Valid add
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // Failure because of duplicate
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);

        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(error::DUPLICATE_WALLET_RECORD.code_num));
    }

    #[test]
    fn test_add_record_with_tag() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring(r#"{"tagName1":"tag1","tagName2":"tag2"}"#.to_string());

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_get_record_fails_with_no_value() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_get_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         options.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_get_record_value_success() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring("{}".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        // Valid add
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_wallet_get_record(cb.command_handle,
                                         xtype.as_ptr(),
                                         id.as_ptr(),
                                         options.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_delete_record() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring("{}".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        // Add record
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle, xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // Successful deletion
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_delete_record(cb.command_handle,
                                            xtype.as_ptr(),
                                            id.as_ptr(),
                                            Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // Fails with no record
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_delete_record(cb.command_handle,
                                            xtype.as_ptr(),
                                            id.as_ptr(),
                                            Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(),
                   Some(error::WALLET_RECORD_NOT_FOUND.code_num));
    }

    #[test]
    fn test_update_record_value() {
        init!("false");
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring("{}".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        // Assert no record to update
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_update_record_value(cb.command_handle,
                                                  xtype.as_ptr(),
                                                  id.as_ptr(),
                                                  options.as_ptr(),
                                                  Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(),
                   Some(error::WALLET_RECORD_NOT_FOUND.code_num));

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_add_record(cb.command_handle, xtype.as_ptr(),
                                         id.as_ptr(),
                                         value.as_ptr(),
                                         tags.as_ptr(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        // Assert update works
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_update_record_value(cb.command_handle,
                                                  xtype.as_ptr(),
                                                  id.as_ptr(),
                                                  options.as_ptr(),
                                                  Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_wallet_import_export() {
        use utils::devsetup::tests::setup_wallet_env;
        use std::env;
        use std::fs;
        use std::path::Path;
        use utils::libindy::return_types_u32;
        use std::time::Duration;
        use settings;

        settings::set_defaults();
        teardown!("false");
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let filename_str = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let backup_key = "backup_key";
        let mut dir = env::temp_dir();
        dir.push(filename_str);
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }
        let handle = setup_wallet_env(&wallet_name).unwrap();
        let dir_c_str = CString::new(dir.to_str().unwrap()).unwrap();
        let backup_key_c_str = CString::new(backup_key).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_export(cb.command_handle,
                          dir_c_str.as_ptr(),
                          backup_key_c_str.as_ptr(),
                          Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(50))).unwrap();

        delete_wallet(&wallet_name).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        let exported_path = dir.to_str().unwrap();
        let import_config = json!({
            settings::CONFIG_WALLET_NAME: wallet_name,
            settings::CONFIG_WALLET_KEY: wallet_key,
            settings::CONFIG_EXPORTED_WALLET_PATH: exported_path,
            settings::CONFIG_WALLET_BACKUP_KEY: backup_key,
        }).to_string();
        let import_config_c = CString::new(import_config).unwrap();
        assert_eq!(vcx_wallet_import(cb.command_handle,
                                     import_config_c.as_ptr(),
                                     Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(50))).unwrap();

        let handle = setup_wallet_env(&wallet_name).unwrap();

        delete_wallet(&wallet_name).unwrap();
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }
}
