extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate named_type;
#[macro_use]
extern crate named_type_derive;

#[macro_use]
mod utils;

use utils::constants::WALLET_CREDENTIALS;
use utils::wallet::WalletUtils;
use utils::non_secrets::*;
use utils::test::TestUtils;
use utils::types::{WalletRecord, SearchRecords};

use std::collections::HashMap;

use indy::api::ErrorCode;

pub const FORBIDDEN_TYPE: &'static str = "Indy::Test";


mod high_cases {
    use super::*;

    mod add_record {
        use super::*;

        #[test]
        fn indy_add_wallet_record_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_plugged_wallet() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_plugged_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_duplicate() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None);
            assert_eq!(ErrorCode::WalletItemAlreadyExists, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_tags() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_same_types_different_ids() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID_2, VALUE, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_same_ids_different_types() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE_2, ID, VALUE, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::add_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID, VALUE, None);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let invalid_wallet_handle = wallet_handle + 1;

            let res = NonSecretsUtils::add_wallet_record(invalid_wallet_handle, TYPE, ID, VALUE, None);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_tags() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(r#"tag:1"#));
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_works_for_empty_params() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::add_wallet_record(wallet_handle, "", ID, VALUE, None);
            assert_eq!(ErrorCode::CommonInvalidParam3, res.unwrap_err());

            let res = NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, "", VALUE, None);
            assert_eq!(ErrorCode::CommonInvalidParam4, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod update_record_value {
        use super::*;

        #[test]
        fn indy_update_record_value_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "value", VALUE);

            NonSecretsUtils::update_wallet_record_value(wallet_handle, TYPE, ID, VALUE_2).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "value", VALUE_2);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_record_value_works_for_plugged_wallet() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_plugged_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "value", VALUE);

            NonSecretsUtils::update_wallet_record_value(wallet_handle, TYPE, ID, VALUE_2).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "value", VALUE_2);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_record_value_works_for_not_found_record() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::update_wallet_record_value(wallet_handle, TYPE, ID, VALUE);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_record_value_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;

            let res = NonSecretsUtils::update_wallet_record_value(invalid_wallet_handle, TYPE, ID, VALUE_2);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_record_value_works_for_empty_value() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::update_wallet_record_value(wallet_handle, TYPE, ID, "");
            assert_eq!(ErrorCode::CommonInvalidParam5, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_record_value_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::update_wallet_record_value(wallet_handle, FORBIDDEN_TYPE, ID, VALUE);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod update_record_tags {
        use super::*;

        #[test]
        fn indy_update_wallet_record_tags_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            NonSecretsUtils::update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            NonSecretsUtils::update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            NonSecretsUtils::update_wallet_record_tags(wallet_handle, TYPE, ID, "{}").unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_not_found_record() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::update_wallet_record_tags(wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_empty() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::update_wallet_record_tags(wallet_handle, TYPE, ID, "");
            assert_eq!(ErrorCode::CommonInvalidParam5, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_wallet_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;

            let res = NonSecretsUtils::update_wallet_record_tags(invalid_wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::update_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod add_record_tags {
        use super::*;

        #[test]
        fn indy_add_wallet_record_tags_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            let tags_json = r#"{"tagName1": "str1"}"#;
            NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", tags_json);

            let tags_json_2 = r#"{"tagName2": "str2"}"#;
            NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json_2).unwrap();

            let expected_tags = r#"{"tagName1": "str1", "tagName2": "str2"}"#;

            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice_add_same_tag() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_rewrite_tag() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS);

            let tags_json = r#"{"tagName1": "str2"}"#;
            NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, tags_json).unwrap();

            let expected_result = r#"{"tagName1": "str2", "~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_result);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_found_record() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::add_wallet_record_tags(wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = NonSecretsUtils::add_wallet_record_tags(invalid_wallet_handle, TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::add_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_record_tags {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_tags_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            let expected_tags_json = r#"{"~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags_json);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_delete_all() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1", "~tagName2", "~tagName3"]"#).unwrap();

            check_record_field(wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_record() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_tag() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS_EMPTY)).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["~tagName2"]"#).unwrap();

            let expected_tags = r#"{"~tagName3": "8"}"#;
            check_record_field(wallet_handle, TYPE, ID, "tags", expected_tags);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete_same_tag() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            NonSecretsUtils::delete_wallet_record_tags(wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = NonSecretsUtils::delete_wallet_record_tags(invalid_wallet_handle, TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::delete_wallet_record_tags(wallet_handle, FORBIDDEN_TYPE, ID, r#"["tagName1"]"#);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_record {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, "{}").unwrap();

            NonSecretsUtils::delete_wallet_record(wallet_handle, TYPE, ID).unwrap();

            let res = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_works_for_not_found_record() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::delete_wallet_record(wallet_handle, TYPE, ID);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            NonSecretsUtils::delete_wallet_record(wallet_handle, TYPE, ID).unwrap();

            let res = NonSecretsUtils::delete_wallet_record(wallet_handle, TYPE, ID);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;

            let res = NonSecretsUtils::delete_wallet_record(invalid_wallet_handle, TYPE, ID);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_works_for_empty_params() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::delete_wallet_record(wallet_handle, "", ID);
            assert_eq!(ErrorCode::CommonInvalidParam3, res.unwrap_err());

            let res = NonSecretsUtils::delete_wallet_record(wallet_handle, TYPE, "");
            assert_eq!(ErrorCode::CommonInvalidParam4, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::delete_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod get_record {
        use super::*;

        #[test]
        fn indy_get_wallet_record_works_for_default_options() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_plugged_wallet_default_options() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_plugged_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_only() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": false
            }).to_string();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: None, type_: None };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_value() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": true,
                "retrieveTags": false
            }).to_string();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_tags() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": true
            }).to_string();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: Some(HashMap::new()), type_: None };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_full() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": true,
                "retrieveValue": true,
                "retrieveTags": true
            }).to_string();

            let record = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, &options).unwrap();
            let record: WalletRecord = serde_json::from_str(&record).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: Some(HashMap::new()), type_: Some(TYPE.to_string()) };
            assert_eq!(expected_record, record);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_not_found() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            let res = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletItemNotFound, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_invalid_options() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = "not_json";
            let res = NonSecretsUtils::get_wallet_record(wallet_handle, TYPE, ID, options);
            assert_eq!(ErrorCode::CommonInvalidStructure, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_get_wallet_record_works_for_invalid_type() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = NonSecretsUtils::get_wallet_record(wallet_handle, FORBIDDEN_TYPE, ID, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod search {
        use super::*;

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
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_1(),
                                                           NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_3(),
                                                           NonSecretsUtils::record_4(),
                                                           NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_eq_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "tagName1": "str1"
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_1(),
                                                           NonSecretsUtils::record_3()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_neq_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "tagName1": {"$neq": "str1"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, &query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_4(),
                                                           NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_gt_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "~tagName3": {"$gt": "6"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_1()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_gte_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "~tagName3": {"$gte": "6"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_1(),
                                                           NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_lt_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "~tagName3": {"$lt": "5"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_lte_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "~tagName3": {"$lte": "5"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_4()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_like_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "~tagName2": {"$like": "%str3%"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_in_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "tagName1": {"$in": ["str1", "str2"]}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_1(),
                                                           NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_3()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_and_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "tagName1": "str1",
                    "tagName2": "str2"
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_3()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_or_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "$or": [
                        {"tagName1": "str2"},
                        {"tagName2": "str2"}
                    ]
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_3()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_not_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "$not": {"tagName1": "str1"}
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_2(),
                                                           NonSecretsUtils::record_4(),
                                                           NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            #[ignore] //TODO: doesn't work
            fn indy_wallet_search_for_mix_and_or_query() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "$or": [
                        {"tagName1": "str1"},
                        {"tagName2": "str1"}
                    ],
                    "$or": [
                        {"tagName1": "str2"},
                        {"tagName2": "str2"}
                    ]
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![NonSecretsUtils::record_3()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_no_records() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let query_json = r#"{
                    "tagName1": "no_records"
                }"#;

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();
                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();

                assert_eq!(0, search_records.total_count.unwrap());
                assert!(search_records.records.is_none());

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }
        }

        mod options {
            use super::*;

            #[test]
            fn indy_wallet_search_for_default_options() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                let records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: None },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: None },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: None },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: None },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: None }]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": true,
                    "retrieveTags": false
                }).to_string();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: None },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: None },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: None },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: None },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: None }]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value_tags() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": true,
                    "retrieveTags": true
                }).to_string();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: Some(NonSecretsUtils::tags_1()) },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: Some(NonSecretsUtils::tags_2()) },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: Some(NonSecretsUtils::tags_3()) },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: Some(NonSecretsUtils::tags_4()) },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: Some(NonSecretsUtils::tags_5()) }]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_retrieve_full_record() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": true,
                    "retrieveType": true,
                    "retrieveValue": true,
                    "retrieveTags": true
                }).to_string();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![NonSecretsUtils::record_1(),
                                                    NonSecretsUtils::record_2(),
                                                    NonSecretsUtils::record_3(),
                                                    NonSecretsUtils::record_4(),
                                                    NonSecretsUtils::record_5()]);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_retrieve_total_count_only() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let options = json!({
                    "retrieveRecords": false,
                    "retrieveTotalCount": true,
                    "retrieveType": false,
                    "retrieveValue": false,
                    "retrieveTags": false
                }).to_string();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
                assert_eq!(5, search_records.total_count.unwrap());
                assert_eq!(None, search_records.records);

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_wallet_search_for_retrieve_records_only() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let options = json!({
                    "retrieveRecords": true,
                    "retrieveTotalCount": false,
                    "retrieveType": false,
                    "retrieveValue": false,
                    "retrieveTags": false
                }).to_string();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, &options).unwrap();

                let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
                assert!(search_records.total_count.is_none());
                assert!(search_records.records.is_some());

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }
        }

        #[test]
        fn indy_wallet_search_for_fetch_twice() {
            NonSecretsUtils::populate_wallet_for_search();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_FULL).unwrap();

            let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 3).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(5, search_records.total_count.unwrap());
            assert_eq!(3, search_records.records.unwrap().len());

            let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 2).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(5, search_records.total_count.unwrap());
            assert_eq!(2, search_records.records.unwrap().len());

            NonSecretsUtils::close_wallet_search(search_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_no_records() {
            NonSecretsUtils::populate_wallet_for_search();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE_2, QUERY_EMPTY, OPTIONS_FULL).unwrap();

            let search_records = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(0, search_records.total_count.unwrap());
            assert!(search_records.records.is_none());

            NonSecretsUtils::close_wallet_search(search_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_invalid_wallet_handle() {
            NonSecretsUtils::populate_wallet_for_search();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = NonSecretsUtils::open_wallet_search(invalid_wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_invalid_search_handle() {
            NonSecretsUtils::populate_wallet_for_search();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            let invalid_search_handle = search_handle + 1;
            let res = NonSecretsUtils::fetch_wallet_search_next_records(wallet_handle, invalid_search_handle, 5);
            assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

            NonSecretsUtils::close_wallet_search(search_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_invalid_type() {
            NonSecretsUtils::populate_wallet_for_search();
            let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            let res = NonSecretsUtils::open_wallet_search(wallet_handle, FORBIDDEN_TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            WalletUtils::close_wallet(wallet_handle).unwrap();
        }

        mod close {
            use super::*;

            #[test]
            fn indy_close_wallet_search_works() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn indy_close_wallet_search_works_for_invalid_handle() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                let invalid_search_handle = search_handle + 1;
                let res = NonSecretsUtils::close_wallet_search(invalid_search_handle);
                assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();
                WalletUtils::close_wallet(wallet_handle).unwrap();
            }

            #[test]
            fn close_wallet_search_works_for_twice() {
                NonSecretsUtils::populate_wallet_for_search();
                let wallet_handle = WalletUtils::open_wallet(SEARCH_COMMON_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

                let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                NonSecretsUtils::close_wallet_search(search_handle).unwrap();

                let res = NonSecretsUtils::close_wallet_search(search_handle);
                assert_eq!(ErrorCode::WalletInvalidHandle, res.unwrap_err());

                WalletUtils::close_wallet(wallet_handle).unwrap();
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
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_default_wallet().unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID, VALUE, None).unwrap();
            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, ID_2, VALUE_2, None).unwrap();

            let search_handle = NonSecretsUtils::open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            NonSecretsUtils::add_wallet_record(wallet_handle, TYPE, "IDSPEC", VALUE, Some(TAGS_2)).unwrap();

            NonSecretsUtils::close_wallet_search(search_handle).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

fn check_record_field(wallet_handle: i32, type_: &str, id: &str, field: &str, expected_value: &str) {
    let record = NonSecretsUtils::get_wallet_record(wallet_handle, type_, id, OPTIONS_FULL).unwrap();
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

