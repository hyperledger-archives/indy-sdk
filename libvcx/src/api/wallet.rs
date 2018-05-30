extern crate libc;

use self::libc::c_char;
use std::ptr;
use std::thread;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use serde_json;
use utils::libindy::payments::{get_wallet_token_info, create_address};

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

    thread::spawn(move|| {
        match get_wallet_token_info() {
            Ok(x) => {
                info!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(x), "null");

                cb(command_handle, x, ptr::null_mut());
            },
        }
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
                                                cb: Option<extern fn(xcommand_handle: u32, err:u32, address: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    info!("vcx_wallet_create_payment_address(command_handle: {})",
          command_handle);

    thread::spawn(move|| {
        match create_address() {
            Ok(x) => {
                info!("vcx_wallet_create_payment_address_cb(command_handle: {}, rc: {}, address: {})",
                    command_handle, error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_wallet_create_payment_address_cb(command_handle: {}, rc: {}, address: {})",
                    command_handle, error_string(x), "null");

                cb(command_handle, x, ptr::null_mut());
            },
        }
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
    let tags_json:serde_json::Value = match serde_json::from_str(&tags_json) {
        Err(_) => return error::INVALID_JSON.code_num,
        Ok(v) => v,
    };
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num );
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
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num );
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
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num);
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
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num);
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
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num);
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
///

#[no_mangle]
pub extern fn vcx_wallet_get_record(command_handle: u32,
                                       type_: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32,
                                       record_json: *const c_char)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_JSON.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    thread::spawn(move || {
        let msg = CStringUtils::string_to_cstring(r#"{"id":"RecordId","type": "TestType","value": "RecordValue","tags":null}"#.to_string());
        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
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
///

#[no_mangle]
pub extern fn vcx_wallet_delete_record(command_handle: u32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    check_useful_c_str!(type_, error::INVALID_OPTION.code_num);
    check_useful_c_str!(id, error::INVALID_OPTION.code_num);
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num);
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
                                     tokens: u64,
                                     recipient: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, receipt: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(recipient, error::INVALID_OPTION.code_num);

    info!("vcx_wallet_send_tokens(command_handle: {}, payment_handle: {}, tokens: {}, recipient: {})",
          command_handle, payment_handle, tokens, recipient);

    thread::spawn(move|| {
        let msg = format!("{{\"paid\":\"true\"}}");

        info!("vcx_wallet_send_tokens_cb(command_handle: {}, rc: {}, receipt: {})",
              command_handle, error_string(0), msg);

        let msg = CStringUtils::string_to_cstring(msg);
        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
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
    thread::spawn(move || {
        cb(command_handle, error::SUCCESS.code_num, DEFAULT_SEARCH_HANDLE as i32)
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
    thread::spawn(move || {
        use utils::constants::DEFAULT_SEARCH_RECORD;
        let msg = CStringUtils::string_to_cstring(DEFAULT_SEARCH_RECORD.to_string());
        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
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

    thread::spawn(move|| {
        info!("vcx_wallet_close_search(command_handle: {}, rc: {})",
              command_handle, error_string(0));
        cb(command_handle, error::SUCCESS.code_num);
    });
    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use settings;

    extern "C" fn generic_cb(command_handle: u32, err: u32, msg: *const c_char) {
        assert_eq!(err, 0);
        check_useful_c_str!(msg, ());
        println!("successfully called callback - {}", msg);
    }

    #[test]
    fn test_get_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_wallet_get_token_info(0, 0, Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_send_tokens() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_wallet_send_tokens(0, 0, 50, CString::new("address").unwrap().into_raw(), Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_create_address() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert_eq!(vcx_wallet_create_payment_address(0, Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}
