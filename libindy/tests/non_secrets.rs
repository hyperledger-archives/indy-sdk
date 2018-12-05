#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate indy_crypto;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::constants::WALLET_CREDENTIALS;
use utils::wallet;
use utils::non_secrets::*;
use utils::types::{WalletRecord, SearchRecords};

use std::collections::HashMap;

use self::indy::ErrorCode;

pub const FORBIDDEN_TYPE: &'static str = "Indy::Test";


mod high_cases {
    use super::*;

    mod add_record {
        use super::*;

        #[test]
        fn indy_add_wallet_record_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_plugged_wallet() {
            let wallet_handle = utils::setup_with_plugged_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_duplicate() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = add_wallet_record(wallet_handle, TYPE, ID, VALUE, None);
            assert_eq!(ErrorCode::WalletItemAlreadyExists, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_tags() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_same_types_different_ids() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(wallet_handle, TYPE, ID_2, VALUE, None).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_same_ids_different_types() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(wallet_handle, TYPE_2, ID, VALUE, None).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID, VALUE, None);
            assert_eq!(Err(ErrorCode::WalletAccessFailed), res);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record(wallet_handle + 1, TYPE, ID, VALUE, None);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_tags() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(r#"tag:1"#));
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_works_for_empty_params() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record(wallet_handle, "", ID, VALUE, None);
            assert_eq!(ErrorCode::CommonInvalidParam3, res.unwrap_err());

            let res = add_wallet_record(wallet_handle, TYPE, "", VALUE, None);
            assert_eq!(ErrorCode::CommonInvalidParam4, res.unwrap_err());

            let res = add_wallet_record(wallet_handle, TYPE, ID, "", None);
            assert_eq!(ErrorCode::CommonInvalidParam5, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod update_record_value {
        use super::*;

        #[test]
        fn indy_update_record_value_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "value", VALUE);

            update_wallet_record_value(wallet_handle, TYPE, ID, VALUE_2).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "value", VALUE_2);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_record_value_works_for_plugged_wallet() {
            let wallet_handle = utils::setup_with_plugged_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "value", VALUE);

            update_wallet_record_value(wallet_handle, TYPE, ID, VALUE_2).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "value", VALUE_2);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_record_value_works_for_not_found_record() {
            let wallet_handle = utils::setup_with_wallet();

            let res = update_wallet_record_value(wallet_handle, TYPE, ID, VALUE);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_record_value_works_for_invalid_wallet_handle() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_value(wallet_handle + 1, TYPE, ID, VALUE_2);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_record_value_works_for_empty_value() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_value(wallet_handle, TYPE, ID, "");
            assert_eq!(ErrorCode::CommonInvalidParam5, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_record_value_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = update_wallet_record_value(wallet_handle, FORBIDDEN_TYPE, ID, VALUE);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod update_record_tags {
        use super::*;

        #[test]
        fn indy_update_wallet_record_tags_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_twice() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            update_wallet_record_tags(wallet_handle, TYPE, ID, "{}").unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_not_found_record() {
            let wallet_handle = utils::setup_with_wallet();

            let res = update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_empty() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_tags(wallet_handle, TYPE, ID, "");
            assert_eq!(ErrorCode::CommonInvalidParam5, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_wallet_handle() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_tags(wallet_handle + 1, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = update_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod add_record_tags {
        use super::*;

        #[test]
        fn indy_add_wallet_record_tags_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            let tags_json = r#"{"tagName1": "str1"}"#;
            add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", tags_json);

            let tags_json_2 = r#"{"tagName2": "str2"}"#;
            add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json_2).unwrap();

            let expected_tags = r#"{"tagName1": "str1", "tagName2": "str2"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice_add_same_tag() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_rewrite_tag() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            let tags_json = r#"{"tagName1": "str2"}"#;
            add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json).unwrap();

            let expected_result = r#"{"tagName1": "str2", "~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_result);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_found_record() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = add_wallet_record_tags(wallet_handle + 1, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = add_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod delete_record_tags {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_tags_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            let expected_tags_json = r#"{"~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags_json);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_delete_all() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1", "~tagName2", "~tagName3"]"#).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_record() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_tag() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS_EMPTY)).unwrap();

            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["~tagName2"]"#).unwrap();

            let expected_tags = r#"{"~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete_same_tag() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
            delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record_tags(wallet_handle + 1, TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod delete_record {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_works() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            get_wallet_record(wallet_handle, TYPE, ID, "{}").unwrap();

            delete_wallet_record(wallet_handle, TYPE, ID).unwrap();

            let res = get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_not_found_record() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record(wallet_handle, TYPE, ID);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_twice() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            delete_wallet_record(wallet_handle, TYPE, ID).unwrap();

            let res = delete_wallet_record(wallet_handle, TYPE, ID);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_handle() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = delete_wallet_record(wallet_handle + 1, TYPE, ID);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_empty_params() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record(wallet_handle, "", ID);
            assert_eq!(ErrorCode::CommonInvalidParam3, res.unwrap_err());

            let res = delete_wallet_record(wallet_handle, TYPE, "");
            assert_eq!(ErrorCode::CommonInvalidParam4, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = delete_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod get_record {
        use super::*;

        fn check_record(record: &str, expected_record: WalletRecord){
            let record: WalletRecord = serde_json::from_str(&record).unwrap();
            assert_eq!(expected_record, record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_default_options() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_plugged_wallet_default_options() {
            let wallet_handle = utils::setup_with_plugged_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_only() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": false
            }).to_string();

            let record = get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: None, type_: None };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_value() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": true,
                "retrieveTags": false
            }).to_string();

            let record = get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_tags() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": true
            }).to_string();

            let record = get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: Some(HashMap::new()), type_: None };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_full() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": true,
                "retrieveValue": true,
                "retrieveTags": true
            }).to_string();

            let record = get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: Some(HashMap::new()), type_: Some(TYPE.to_string()) };
            check_record(&record, expected_record);

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_not_found() {
            let wallet_handle = utils::setup_with_wallet();

            let res = get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            wallet::close_wallet(wallet_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_get_wallet_record_works_for_invalid_options() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = get_wallet_record(wallet_handle, TYPE, ID, "not_json");
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }

        #[test]
        fn indy_get_wallet_record_works_for_invalid_type() {
            let wallet_handle = utils::setup_with_wallet();

            let res = get_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            utils::tear_down_with_wallet(wallet_handle);
        }
    }

    mod search {
        use super::*;

        fn setup() -> i32{
            populate_wallet_for_search();
            wallet::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap()
        }

        fn tear_down(wallet_handle: i32, search_handle: i32){
            close_wallet_search(search_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
        }

        // Prepare common wallet containing following records
        // {type: TestType, id: RecordId, value: RecordValue, tags:   {tagName1: str1,      tagName2: 4,        tagName3: 12 }}
        // {type: TestType, id: RecordId2, value: RecordValue2, tags: {tagName1: str2,      tagName2: pre_str3, tagName3: 2 }}
        // {type: TestType, id: RecordId3, value: RecordValue3, tags: {tagName1: str1,      tagName2: str2,     tagName3: str3 }}
        // {type: TestType, id: RecordId4, value: RecordValue4, tags: {tagName1: 2,         tagName2: 4,        tagName3: 5 }}
        // {type: TestType, id: RecordId5, value: RecordValue5, tags: {tagName1: p_str2_s,  tagName2: str3,     tagName3: 6 }}

        mod queries {
            use super::*;

            #[test]
            fn indy_wallet_search_for_empty_query() {
                let wallet_handle = setup();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_2(),
                                                           record_3(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_eq_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "tagName1": "str1"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_3()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_neq_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "tagName1": {"$neq": "str1"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, &query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_gt_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "~tagName3": {"$gt": "6"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_gte_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "~tagName3": {"$gte": "6"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_lt_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "~tagName3": {"$lt": "5"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_lte_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "~tagName3": {"$lte": "5"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_like_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "~tagName2": {"$like": "%str3%"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_in_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "tagName1": {"$in": ["str1", "str2"]}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_2(),
                                                           record_3()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_and_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "tagName1": "str1",
                    "tagName2": "str2"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_3()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_or_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "$or": [
                        {"tagName1": "str2"},
                        {"tagName2": "str2"}
                    ]
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_3()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_not_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "$not": {"tagName1": "str1"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_mix_and_or_query() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "$and": [
                        {
                            "$or": [
                                {"tagName1": "str1"},
                                {"tagName2": "str1"}
                            ]
                        },
                        {
                            "$or": [
                                {"tagName1": "str2"},
                                {"tagName2": "str2"}
                            ]
                        }
                    ]
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_3()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_no_records() {
                let wallet_handle = setup();

                let query_json = r#"{
                    "tagName1": "no_records"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();
                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();

                assert_eq!(0, search_records.total_count.unwrap());
                assert!(search_records.records.is_none());

                tear_down(wallet_handle, search_handle);
            }
        }

        mod options {
            use super::*;

            #[test]
            fn indy_wallet_search_for_default_options() {
                let wallet_handle = setup();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                let records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: None },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: None },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: None },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: None },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: None }]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value() {
                let wallet_handle = setup();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": true,
                    "retrieveTags": false
                }).to_string();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: None },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: None },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: None },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: None },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: None }]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value_tags() {
                let wallet_handle = setup();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": true,
                    "retrieveTags": true
                }).to_string();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: Some(tags_1()) },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: Some(tags_2()) },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: Some(tags_3()) },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: Some(tags_4()) },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: Some(tags_5()) }]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_retrieve_full_record() {
                let wallet_handle = setup();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": true,
                    "retrieveValue": true,
                    "retrieveTags": true
                }).to_string();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![record_1(),
                                                    record_2(),
                                                    record_3(),
                                                    record_4(),
                                                    record_5()]);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_retrieve_total_count_only() {
                let wallet_handle = setup();

                let options = json!({
                    "retrieveRecords": false,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": false,
                    "retrieveTags": false
                }).to_string();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
                assert_eq!(5, search_records.total_count.unwrap());
                assert_eq!(None, search_records.records);

                tear_down(wallet_handle, search_handle);
            }

            #[test]
            fn indy_wallet_search_for_retrieve_records_only() {
                let wallet_handle = setup();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": false,
                    "retrieveType": false,
                    "retrieveValue": false,
                    "retrieveTags": false
                }).to_string();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
                assert!(search_records.total_count.is_none());
                assert!(search_records.records.is_some());

                tear_down(wallet_handle, search_handle);
            }
        }

        #[test]
        fn indy_wallet_search_for_fetch_twice() {
            let wallet_handle = setup();

            let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_FULL).unwrap();

            let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 3).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(5, search_records.total_count.unwrap());
            assert_eq!(3, search_records.records.unwrap().len());

            let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 2).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(5, search_records.total_count.unwrap());
            assert_eq!(2, search_records.records.unwrap().len());

            tear_down(wallet_handle, search_handle);
        }

        #[test]
        fn indy_wallet_search_for_no_records() {
            let wallet_handle = setup();

            let search_handle = open_wallet_search(wallet_handle, TYPE_2, QUERY_EMPTY, OPTIONS_FULL).unwrap();

            let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(0, search_records.total_count.unwrap());
            assert!(search_records.records.is_none());

            tear_down(wallet_handle, search_handle);
        }

        #[test]
        fn indy_wallet_search_for_invalid_wallet_handle() {
            let wallet_handle = setup();

            let res = open_wallet_search(wallet_handle + 1, TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            wallet::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_invalid_search_handle() {
            let wallet_handle = setup();

            let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            let res = fetch_wallet_search_next_records(wallet_handle, search_handle + 1, 5);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            tear_down(wallet_handle, search_handle);
        }

        #[test]
        fn indy_wallet_search_for_invalid_type() {
            let wallet_handle = setup();

            let res = open_wallet_search(wallet_handle, FORBIDDEN_TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            wallet::close_wallet(wallet_handle).unwrap();
        }

        mod close {
            use super::*;

            #[test]
            fn indy_close_wallet_search_works() {
                let wallet_handle = setup();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                close_wallet_search(search_handle).unwrap();
                wallet::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_close_wallet_search_works_for_invalid_handle() {
                let wallet_handle = setup();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                let res = close_wallet_search(search_handle + 1);
                assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

                close_wallet_search(search_handle).unwrap();
                wallet::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn close_wallet_search_works_for_twice() {
                let wallet_handle = setup();

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                close_wallet_search(search_handle).unwrap();

                let res = close_wallet_search(search_handle);
                assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

                wallet::close_wallet(wallet_handle).unwrap();
            }
        }
    }
}


mod medium_cases {
    use super::*;

    mod rusqlite_transaction_fix {
        use super::*;

        #[test]
        pub fn transaction_works_during_opened_wallet_search() {
            let wallet_handle = utils::setup_with_wallet();

            add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(wallet_handle, TYPE, ID_2, VALUE_2, None).unwrap();

            let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            add_wallet_record(wallet_handle, TYPE, "IDSPEC", VALUE, Some(TAGS_2)).unwrap();

            close_wallet_search(search_handle).unwrap();
            utils::tear_down_with_wallet(wallet_handle);
        }
    }
}

fn check_record_field(wallet_handle: i32, type_: &str, id: &str, field: &str, expected_value: &str) {
    let record = get_wallet_record(wallet_handle, type_, id, OPTIONS_FULL).unwrap();
    let record = serde_json::from_str::<WalletRecord>(&record).unwrap();

    match field {
        "value" => assert_eq!(expected_value, record.value.unwrap()),
        "tags" => assert_eq!(serde_json::from_str::<HashMap<String, String>>(&expected_value).unwrap(),
                             record.tags.unwrap()),
        _ => panic!()
    };
}

fn check_search_records(search_records: &str, expected_records: Vec<WalletRecord>) {
    let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();

    let mut records = search_records.records.unwrap();
    records.sort_by_key(|record| record.id.to_string());

    assert_eq!(records.len(), expected_records.len());
    assert_eq!(records, expected_records);
}

