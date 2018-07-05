extern crate libc;

use self::libc::c_char;
use settings;
use std::ptr;
use std::thread;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use error::ToErrorCode;
use utils::libindy::payments::{pay_a_payee, get_wallet_token_info, create_address};
use utils::libindy::wallet::{export, import, get_wallet_handle};
use std::path::Path;

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

    thread::spawn(move|| {
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
/// Error will be a libindy error code
///

#[no_mangle]
pub extern fn vcx_wallet_add_record(command_handle: i32,
                                    type_: *const c_char,
                                    id: *const c_char,
                                    value: *const c_char,
                                    tags_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> u32 {
    if settings::test_indy_mode_enabled() {
        check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
        println!("vcx_wallet_add_record cb");
        cb(command_handle, error::SUCCESS.code_num as i32);
        return error::SUCCESS.code_num
    }
    unsafe {
        indy_add_wallet_record(command_handle as i32, get_wallet_handle(), type_, id, value, tags_json, cb) as u32
    }
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
/// Error will be a libindy error code
///

#[no_mangle]
pub extern fn vcx_wallet_update_record_value(command_handle: i32,
                                             type_: *const c_char,
                                             id: *const c_char,
                                             value: *const c_char,
                                             cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> u32 {
    if settings::test_indy_mode_enabled() {
        check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
        cb(command_handle, error::SUCCESS.code_num as i32);
        return error::SUCCESS.code_num
    }
    unsafe {
        indy_update_wallet_record_value(command_handle, get_wallet_handle(), type_, id, value, cb) as u32
    }
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
/// Error will be a libindy error code
///

#[no_mangle]
pub extern fn vcx_wallet_get_record(command_handle: i32,
                                       type_: *const c_char,
                                       id: *const c_char,
                                       options_json: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                       record_json: *const c_char)>) -> u32 {
    if settings::test_indy_mode_enabled() {
        check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
        let msg = r#"{"id":"123","type":"record type","value":"record value","tags":null}"#.to_string();
        cb(command_handle, error::SUCCESS.code_num as i32, CStringUtils::string_to_cstring(msg).as_ptr());
        return error::SUCCESS.code_num
    }
    unsafe {
        indy_get_wallet_record(command_handle, get_wallet_handle(), type_, id, options_json, cb) as u32
    }
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
pub extern fn vcx_wallet_delete_record(command_handle: i32,
                                            type_: *const c_char,
                                            id: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> u32 {
    if settings::test_indy_mode_enabled() {
        check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
        cb(command_handle, error::SUCCESS.code_num as i32);
        return error::SUCCESS.code_num
    }
    unsafe {
        indy_delete_wallet_record(command_handle, get_wallet_handle(), type_, id, cb) as u32
    }
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
                cb(command_handle, e.to_error_code(), ptr::null());
            },
        }
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

#[no_mangle]
pub extern fn vcx_wallet_export(command_handle: u32,
                                path: *const c_char,
                                backup_key: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32,
                                                     err: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(path,  error::INVALID_OPTION.code_num);
    check_useful_c_str!(backup_key, error::INVALID_OPTION.code_num);
    thread::spawn(move || {
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
        }
    });
    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_wallet_import(command_handle: u32,
                                path: *const c_char,
                                backup_key: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32,
                                                     err: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(path,  error::INVALID_OPTION.code_num);
    check_useful_c_str!(backup_key, error::INVALID_OPTION.code_num);
    thread::spawn(move || {
        let path = Path::new(&path);
        info!("vcx_wallet_import(command_handle: {}, path: {:?}, backup_key: ****)", command_handle, path);
        match import(&path, &backup_key) {
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
        }
    });
    error::SUCCESS.code_num
}

#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use settings;
    use utils::libindy::wallet::{ init_wallet, delete_wallet };

    extern "C" fn generic_cb(command_handle: u32, err: u32, msg: *const c_char) {
        assert_eq!(err, 0);
        check_useful_c_str!(msg, ());
        println!("successfully called callback - {}", msg);
    }

    pub extern "C" fn indy_generic_no_msg_cb(command_handle: i32, err: i32) {
        assert_eq!(err, 0);
        println!("successfully called indy_generic_no_msg_cb");
    }

    pub extern "C" fn indy_generic_msg_cb(command_handle: i32, err: i32, msg: *const c_char) {
        assert_eq!(err, 0);
        check_useful_c_str!(msg, ());
        println!("successfully called indy_generic_msg_cb - {}", msg);

    }

    extern "C" fn duplicate_record_cb(command_handle: i32, err: i32) {
        assert_ne!(err as u32, error::DUPLICATE_WALLET_RECORD.code_num);
        println!("successfully called duplicate_record_cb");

    }

    extern "C" fn record_not_found_msg_cb(command_handle: i32, err: i32, msg: *const c_char) {
        assert_ne!(err as u32, error::WALLET_RECORD_NOT_FOUND.code_num);
        println!("successfully called record_not_found_msg_cb");

    }

    extern "C" fn record_not_found_cb(command_handle: i32, err: i32) {
        assert_ne!(err as u32, error::WALLET_RECORD_NOT_FOUND.code_num);
        println!("successfully called record_not_found_cb");

    }

    #[test]
    fn test_get_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert_eq!(vcx_wallet_get_token_info(0, 0, Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_send_tokens() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
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

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_send_payment() {
        use utils::devsetup::tests;
        use utils::libindy::payments::{mint_tokens, get_wallet_token_info};
        let name = "test_send_payment";
        tests::setup_local_env(name);
        mint_tokens(Some(1), Some(1000)).unwrap();
        let balance = get_wallet_token_info().unwrap().get_balance();
        let recipient = CStringUtils::string_to_cstring("pay:null:iXvVdM4mjCUZFrnnFU2F0VoJrkzQEoLy".to_string());
        let tokens = 100;
        let cb = generic_cb;
        let err = vcx_wallet_send_tokens(0, 0, tokens, recipient.as_ptr(), Some(cb));
        assert_eq!(err, 0);
        thread::sleep(Duration::from_secs(2));
        let new_balance = get_wallet_token_info().unwrap().get_balance();
        assert_eq!(balance - tokens, new_balance);
        tests::cleanup_dev_env(name);
    }

    #[test]
    fn test_add_record() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_add_record";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());

        init_wallet(wallet_n).unwrap();

        // Valid add
        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        // Failure because of duplicate
        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(duplicate_record_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();

    }

    #[test]
    fn test_add_record_with_tag() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_add_record_with_tag";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let tags = CStringUtils::string_to_cstring(r#"{"tagName1":"tag1","tagName2":"tag2"}"#.to_string());

        init_wallet(wallet_n).unwrap();
        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), tags.as_ptr(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_get_record_fails_with_no_value() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_get_fails_with_no_value";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        init_wallet(wallet_n).unwrap();
        assert_eq!(vcx_wallet_get_record(0, xtype.as_ptr(), id.as_ptr(), options.as_ptr(), Some(record_not_found_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_get_record_value_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_get_value_success";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        init_wallet(wallet_n).unwrap();

        // Valid add
        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        assert_eq!(vcx_wallet_get_record(0, xtype.as_ptr(), id.as_ptr(), options.as_ptr(), Some(indy_generic_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();

    }


    #[test]
    fn test_delete_record() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_delete_record";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        init_wallet(wallet_n).unwrap();

        // Add record
        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        // Successful deletion
        assert_eq!(vcx_wallet_delete_record(0, xtype.as_ptr(), id.as_ptr(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        // Fails with no record
        assert_eq!(vcx_wallet_delete_record(0, xtype.as_ptr(), id.as_ptr(), Some(record_not_found_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_update_record_value() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_n = "test_update_record_value";
        let xtype = CStringUtils::string_to_cstring("record_type".to_string());
        let id = CStringUtils::string_to_cstring("123".to_string());
        let value = CStringUtils::string_to_cstring("Record Value".to_string());
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let options = CStringUtils::string_to_cstring(options);

        init_wallet(wallet_n).unwrap();
        // Assert no record to update
        assert_eq!(vcx_wallet_update_record_value(0, xtype.as_ptr(), id.as_ptr(), options.as_ptr(), Some(record_not_found_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        assert_eq!(vcx_wallet_add_record(0, xtype.as_ptr(), id.as_ptr(), value.as_ptr(), ptr::null(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

        // Assert update workds
        assert_eq!(vcx_wallet_update_record_value(0, xtype.as_ptr(), id.as_ptr(), options.as_ptr(), Some(indy_generic_no_msg_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_wallet_import_export() {
        use utils::devsetup::tests::setup_wallet_env;
        use indy::wallet::Wallet;
        use std::env;
        use std::fs;
        use std::path::Path;
        use utils::libindy::return_types_u32;
        use std::time::Duration;
        use settings;

        settings::set_defaults();
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let filename_str = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME).unwrap();
        let backup_key = "backup_key";
        let mut dir = env::temp_dir();
        dir.push(filename_str);
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }
        let credential_config = json!({"key": wallet_key, "storage": "{}"}).to_string();
        let handle = setup_wallet_env(&wallet_name).unwrap();
        let dir_c_str = CString::new(dir.to_str().unwrap()).unwrap();
        let backup_key_c_str = CString::new(backup_key).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_export(cb.command_handle,
                          dir_c_str.as_ptr(),
                          backup_key_c_str.as_ptr(),
                          Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(5))).unwrap();

        Wallet::close(handle).unwrap();
        Wallet::delete(&wallet_name, Some(&credential_config)).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_import(cb.command_handle,
                                     dir_c_str.as_ptr(),
                                     backup_key_c_str.as_ptr(),
                                     Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(5))).unwrap();

        let handle = setup_wallet_env(&wallet_name).unwrap();
        Wallet::close(handle).unwrap();
        Wallet::delete(&wallet_name, Some(&credential_config)).unwrap();
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }
}
