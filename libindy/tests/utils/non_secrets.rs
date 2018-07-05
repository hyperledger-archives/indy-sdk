extern crate serde_json;

use indy::api::ErrorCode;
use indy::api::non_secrets::*;

use utils::callback::CallbackUtils;
use utils::wallet::WalletUtils;
use utils::constants::WALLET_CREDENTIALS;
use utils::test::TestUtils;
use utils::types::WalletRecord;

use std::ffi::CString;
use std::ptr::null;
use std::sync::{Once, ONCE_INIT};
use std::collections::HashMap;

pub const SEARCH_COMMON_WALLET_CONFIG: &'static str = r#"{"id":"search_common"}"#;
pub const TYPE: &'static str = "TestType";
pub const TYPE_2: &'static str = "TestType2";
pub const ID: &'static str = "RecordId";
pub const ID_2: &'static str = "RecordId2";
pub const ID_3: &'static str = "RecordId3";
pub const ID_4: &'static str = "RecordId4";
pub const ID_5: &'static str = "RecordId5";
pub const VALUE: &'static str = "RecordValue";
pub const VALUE_2: &'static str = "RecordValue2";
pub const VALUE_3: &'static str = "RecordValue3";
pub const VALUE_4: &'static str = "RecordValue4";
pub const VALUE_5: &'static str = "RecordValue5";
pub const QUERY_EMPTY: &'static str = r#"{}"#;
pub const OPTIONS_EMPTY: &'static str = r#"{}"#;
pub const OPTIONS_ID_TYPE_VALUE: &'static str = r#"{"retrieveType":true, "retrieveValue":true, "retrieveTags":false}"#;
pub const OPTIONS_FULL: &'static str = r#"{"retrieveType":true, "retrieveValue":true, "retrieveTags":true, "retrieveTotalCount":true}"#;
pub const TAGS_EMPTY: &'static str = r#"{}"#;
pub const TAGS: &'static str = r#"{"tagName1":"str1","~tagName2":"5","~tagName3":"8"}"#;
pub const TAGS_2: &'static str = r#"{"tagName1":"str2","~tagName2":"pre_str3","~tagName3":"2"}"#;
pub const TAGS_3: &'static str = r#"{"tagName1":"str1","tagName2":"str2","tagName3":"str3"}"#;
pub const TAGS_4: &'static str = r#"{"tagName1":"somestr","~tagName2":"4","~tagName3":"5"}"#;
pub const TAGS_5: &'static str = r#"{"tagName1":"prefix_str2","~tagName2":"str3","~tagName3":"6"}"#;

pub struct NonSecretsUtils {}

impl NonSecretsUtils {
    pub fn add_wallet_record(wallet_handle: i32, type_: &str, id: &str, value: &str, tags_json: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let value = CString::new(value).unwrap();
        let tags_json_str = tags_json.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_add_wallet_record(command_handle,
                                   wallet_handle,
                                   type_.as_ptr(),
                                   id.as_ptr(),
                                   value.as_ptr(),
                                   if tags_json.is_some() { tags_json_str.as_ptr() } else { null() },
                                   cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn update_wallet_record_value(wallet_handle: i32, type_: &str, id: &str, value: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let value = CString::new(value).unwrap();

        let err =
            indy_update_wallet_record_value(command_handle,
                                            wallet_handle,
                                            type_.as_ptr(),
                                            id.as_ptr(),
                                            value.as_ptr(),
                                            cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn update_wallet_record_tags(wallet_handle: i32, type_: &str, id: &str, tags_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let tags_json = CString::new(tags_json).unwrap();

        let err =
            indy_update_wallet_record_tags(command_handle,
                                           wallet_handle,
                                           type_.as_ptr(),
                                           id.as_ptr(),
                                           tags_json.as_ptr(),
                                           cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn add_wallet_record_tags(wallet_handle: i32, type_: &str, id: &str, tags_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let tags_json = CString::new(tags_json).unwrap();

        let err =
            indy_add_wallet_record_tags(command_handle,
                                        wallet_handle,
                                        type_.as_ptr(),
                                        id.as_ptr(),
                                        tags_json.as_ptr(),
                                        cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn delete_wallet_record_tags(wallet_handle: i32, type_: &str, id: &str, tag_names_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let tag_names_json = CString::new(tag_names_json).unwrap();

        let err =
            indy_delete_wallet_record_tags(command_handle,
                                           wallet_handle,
                                           type_.as_ptr(),
                                           id.as_ptr(),
                                           tag_names_json.as_ptr(),
                                           cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn delete_wallet_record(wallet_handle: i32, type_: &str, id: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();

        let err =
            indy_delete_wallet_record(command_handle,
                                      wallet_handle,
                                      type_.as_ptr(),
                                      id.as_ptr(),
                                      cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn get_wallet_record(wallet_handle: i32, type_: &str, id: &str, options_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let type_ = CString::new(type_).unwrap();
        let id = CString::new(id).unwrap();
        let options_json = CString::new(options_json).unwrap();

        let err =
            indy_get_wallet_record(command_handle,
                                   wallet_handle,
                                   type_.as_ptr(),
                                   id.as_ptr(),
                                   options_json.as_ptr(),
                                   cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn open_wallet_search(wallet_handle: i32, type_: &str, query_json: &str, options_json: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

        let type_ = CString::new(type_).unwrap();
        let query_json = CString::new(query_json).unwrap();
        let options_json = CString::new(options_json).unwrap();

        let err =
            indy_open_wallet_search(command_handle,
                                    wallet_handle,
                                    type_.as_ptr(),
                                    query_json.as_ptr(),
                                    options_json.as_ptr(),
                                    cb);

        super::results::result_to_int(err, receiver)
    }

    pub fn fetch_wallet_search_next_records(wallet_handle: i32, wallet_search_handle: i32, count: usize) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let err =
            indy_fetch_wallet_search_next_records(command_handle,
                                                  wallet_handle,
                                                  wallet_search_handle,
                                                  count,
                                                  cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn close_wallet_search(wallet_search_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let err =
            indy_close_wallet_search(command_handle,
                                     wallet_search_handle,
                                     cb);

        super::results::result_to_empty(err, receiver)
    }

    pub fn tags_1() -> HashMap<String, String> {
        serde_json::from_str(TAGS).unwrap()
    }

    pub fn tags_2() -> HashMap<String, String> {
        serde_json::from_str(TAGS_2).unwrap()
    }

    pub fn tags_3() -> HashMap<String, String> {
        serde_json::from_str(TAGS_3).unwrap()
    }

    pub fn tags_4() -> HashMap<String, String> {
        serde_json::from_str(TAGS_4).unwrap()
    }

    pub fn tags_5() -> HashMap<String, String> {
        serde_json::from_str(TAGS_5).unwrap()
    }

    pub fn record_1() -> WalletRecord {
        WalletRecord { id: ID.to_string(), type_: Some(TYPE.to_string()), value: Some(VALUE.to_string()), tags: Some(NonSecretsUtils::tags_1()) }
    }

    pub fn record_2() -> WalletRecord {
        WalletRecord { id: ID_2.to_string(), type_: Some(TYPE.to_string()), value: Some(VALUE_2.to_string()), tags: Some(NonSecretsUtils::tags_2()) }
    }

    pub fn record_3() -> WalletRecord {
        WalletRecord { id: ID_3.to_string(), type_: Some(TYPE.to_string()), value: Some(VALUE_3.to_string()), tags: Some(NonSecretsUtils::tags_3()) }
    }

    pub fn record_4() -> WalletRecord {
        WalletRecord { id: ID_4.to_string(), type_: Some(TYPE.to_string()), value: Some(VALUE_4.to_string()), tags: Some(NonSecretsUtils::tags_4()) }
    }

    pub fn record_5() -> WalletRecord {
        WalletRecord { id: ID_5.to_string(), type_: Some(TYPE.to_string()), value: Some(VALUE_5.to_string()), tags: Some(NonSecretsUtils::tags_5()) }
    }

    pub fn populate_wallet_for_search() {
        lazy_static! {
                    static ref COMMON_WALLET_INIT: Once = ONCE_INIT;

                }

        COMMON_WALLET_INIT.call_once(|| {
            TestUtils::cleanup_storage();

            //1. Create and Open wallet
            WalletUtils::create_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let record_1 = NonSecretsUtils::record_1();
            NonSecretsUtils::add_wallet_record(wallet_handle,
                                               TYPE,
                                               &record_1.id,
                                               &record_1.value.clone().unwrap(),
                                               Some(TAGS)).unwrap();

            let record_2 = NonSecretsUtils::record_2();
            NonSecretsUtils::add_wallet_record(wallet_handle,
                                               TYPE,
                                               &record_2.id,
                                               &record_2.value.clone().unwrap(),
                                               Some(TAGS_2)).unwrap();

            let record_3 = NonSecretsUtils::record_3();
            NonSecretsUtils::add_wallet_record(wallet_handle,
                                               TYPE,
                                               &record_3.id,
                                               &record_3.value.clone().unwrap(),
                                               Some(TAGS_3)).unwrap();

            let record_4 = NonSecretsUtils::record_4();
            NonSecretsUtils::add_wallet_record(wallet_handle,
                                               TYPE,
                                               &record_4.id,
                                               &record_4.value.clone().unwrap(),
                                               Some(TAGS_4)).unwrap();

            let record_5 = NonSecretsUtils::record_5();
            NonSecretsUtils::add_wallet_record(wallet_handle,
                                               TYPE,
                                               &record_5.id,
                                               &record_5.value.clone().unwrap(),
                                               Some(TAGS_5)).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
        });
    }
}