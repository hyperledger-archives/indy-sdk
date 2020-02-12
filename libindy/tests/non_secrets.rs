#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as indy;
extern crate indyrs as api;

use crate::utils::constants::WALLET_CREDENTIALS;
use crate::utils::wallet;
use crate::utils::non_secrets::*;
use crate::utils::types::{WalletRecord, SearchRecords};

use std::collections::HashMap;

use self::indy::ErrorCode;

pub const FORBIDDEN_TYPE: &'static str = "Indy::Test";

use indy::WalletHandle;
use crate::utils::test::cleanup_wallet;
use crate::utils::Setup;

mod high_cases {
    use super::*;

    mod add_record {
        use super::*;

        #[test]
        fn indy_add_wallet_record_works() {
            let setup = Setup::wallet();
            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
        }

        #[test]
        fn indy_add_wallet_record_works_for_plugged_wallet() {
            let setup = Setup::plugged_wallet();
            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
        }

        #[test]
        fn indy_add_wallet_record_works_for_duplicate() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None);
            assert_code!(ErrorCode::WalletItemAlreadyExists, res);
        }
    }

    mod update_record_value {
        use super::*;

        #[test]
        fn indy_update_record_value_works() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "value", VALUE);

            update_wallet_record_value(setup.wallet_handle, TYPE, ID, VALUE_2).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "value", VALUE_2);
        }

        #[test]
        fn indy_update_record_value_works_for_plugged_wallet() {
            let setup = Setup::plugged_wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "value", VALUE);

            update_wallet_record_value(setup.wallet_handle, TYPE, ID, VALUE_2).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "value", VALUE_2);
        }

        #[test]
        fn indy_update_record_value_works_for_not_found_record() {
            let setup = Setup::wallet();

            let res = update_wallet_record_value(setup.wallet_handle, TYPE, ID, VALUE);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod update_record_tags {
        use super::*;

        #[test]
        fn indy_update_wallet_record_tags_works() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            update_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_twice() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            update_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);

            update_wallet_record_tags(setup.wallet_handle, TYPE, ID, "{}").unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_not_found_record() {
            let setup = Setup::wallet();

            let res = update_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod add_record_tags {
        use super::*;

        #[test]
        fn indy_add_wallet_record_tags_works() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            add_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);

            let tags_json = r#"{"tagName1": "str1"}"#;
            add_wallet_record_tags(setup.wallet_handle, TYPE, ID, tags_json).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", tags_json);

            let tags_json_2 = r#"{"tagName2": "str2"}"#;
            add_wallet_record_tags(setup.wallet_handle, TYPE, ID, tags_json_2).unwrap();

            let expected_tags = r#"{"tagName1": "str1", "tagName2": "str2"}"#;
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", expected_tags);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_found_record() {
            let setup = Setup::wallet();

            let res = add_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod delete_record_tags {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_tags_works() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);

            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();

            let expected_tags_json = r#"{"~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", expected_tags_json);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_delete_all() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1", "~tagName2", "~tagName3"]"#).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS_EMPTY);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_record() {
            let setup = Setup::wallet();

            let res = delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_not_found_tag() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS_EMPTY)).unwrap();

            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
        }
    }

    mod delete_record {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_works() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            get_wallet_record(setup.wallet_handle, TYPE, ID, "{}").unwrap();

            delete_wallet_record(setup.wallet_handle, TYPE, ID).unwrap();

            let res = get_wallet_record(setup.wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_not_found_record() {
            let setup = Setup::wallet();

            let res = delete_wallet_record(setup.wallet_handle, TYPE, ID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
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
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_plugged_wallet_default_options() {
            let setup = Setup::plugged_wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_only() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": false
            }).to_string();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: None, type_: None };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_value() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": true,
                "retrieveTags": false
            }).to_string();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: None, type_: None };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_id_tags() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": false,
                "retrieveValue": false,
                "retrieveTags": true
            }).to_string();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: None, tags: Some(HashMap::new()), type_: None };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_full() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let options = json!({
                "retrieveType": true,
                "retrieveValue": true,
                "retrieveTags": true
            }).to_string();

            let record = get_wallet_record(setup.wallet_handle, TYPE, ID, &options).unwrap();

            let expected_record = WalletRecord { id: ID.to_string(), value: Some(VALUE.to_string()), tags: Some(HashMap::new()), type_: Some(TYPE.to_string()) };
            check_record(&record, expected_record);
        }

        #[test]
        fn indy_get_wallet_record_works_for_not_found() {
            let setup = Setup::wallet();

            let res = get_wallet_record(setup.wallet_handle, TYPE, ID, OPTIONS_EMPTY);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }
    }

    mod search {
        use super::*;
        use indy::{WalletHandle, SearchHandle};

        fn setup(name: &str, wallet_config: &str) -> WalletHandle {
            init_non_secret_test_wallet(name, wallet_config);
            wallet::open_wallet(wallet_config, WALLET_CREDENTIALS).unwrap()
        }

        fn tear_down(wallet_handle: WalletHandle, search_handle: SearchHandle) {
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
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_empty_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_empty_query", SEARCH_WALLET_CONFIG);

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_2(),
                                                           record_3(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_empty_query");
            }

            #[test]
            fn indy_wallet_search_for_eq_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_eq_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_eq_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "tagName1": "str1"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_3()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_eq_query");
            }

            #[test]
            fn indy_wallet_search_for_neq_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_neq_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_neq_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "tagName1": {"$neq": "str1"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, &query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_neq_query");
            }

            #[test]
            fn indy_wallet_search_for_gt_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_gt_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_gt_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "~tagName3": {"$gt": "6"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_gt_query");
            }

            #[test]
            fn indy_wallet_search_for_gte_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_gte_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_gte_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "~tagName3": {"$gte": "6"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_gte_query");
            }

            #[test]
            fn indy_wallet_search_for_lt_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_lt_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_lt_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "~tagName3": {"$lt": "5"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_lt_query");
            }

            #[test]
            fn indy_wallet_search_for_lte_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_lte_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_lte_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{

                    "~tagName3": {"$lte": "5"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_lte_query");
            }

            #[test]
            fn indy_wallet_search_for_like_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_lte_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_lte_query", SEARCH_WALLET_CONFIG);

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
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_in_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_in_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "tagName1": {"$in": ["str1", "str2"]}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_1(),
                                                           record_2(),
                                                           record_3()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_in_query");
            }

            #[test]
            fn indy_wallet_search_for_and_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_and_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_and_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "tagName1": "str1",
                    "tagName2": "str2"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_3()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_and_query");
            }

            #[test]
            fn indy_wallet_search_for_or_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_or_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_or_query", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_or_query");
            }

            #[test]
            fn indy_wallet_search_for_not_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_not_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_not_query", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "$not": {"tagName1": "str1"}
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&search_records, vec![record_2(),
                                                           record_4(),
                                                           record_5()]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_not_query");
            }

            #[test]
            fn indy_wallet_search_for_mix_and_or_query() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_mix_and_or_query"}"#;
                let wallet_handle = setup("indy_wallet_search_for_mix_and_or_query", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_mix_and_or_query");
            }

            #[test]
            fn indy_wallet_search_for_no_records() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_no_records"}"#;
                let wallet_handle = setup("indy_wallet_search_for_no_records", SEARCH_WALLET_CONFIG);

                let query_json = r#"{
                    "tagName1": "no_records"
                }"#;

                let search_handle = open_wallet_search(wallet_handle, TYPE, query_json, OPTIONS_FULL).unwrap();

                let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();
                let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();

                assert_eq!(0, search_records.total_count.unwrap());
                assert!(search_records.records.is_none());

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_no_records");
            }
        }

        mod options {
            use super::*;

            #[test]
            fn indy_wallet_search_for_default_options() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_default_options"}"#;
                let wallet_handle = setup("indy_wallet_search_for_default_options", SEARCH_WALLET_CONFIG);

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                let records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

                check_search_records(&records, vec![
                    WalletRecord { id: ID.to_string(), type_: None, value: Some(VALUE.to_string()), tags: None },
                    WalletRecord { id: ID_2.to_string(), type_: None, value: Some(VALUE_2.to_string()), tags: None },
                    WalletRecord { id: ID_3.to_string(), type_: None, value: Some(VALUE_3.to_string()), tags: None },
                    WalletRecord { id: ID_4.to_string(), type_: None, value: Some(VALUE_4.to_string()), tags: None },
                    WalletRecord { id: ID_5.to_string(), type_: None, value: Some(VALUE_5.to_string()), tags: None }]);

                tear_down(wallet_handle, search_handle);
                cleanup_wallet("indy_wallet_search_for_default_options");
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_retrieve_id_value"}"#;
                let wallet_handle = setup("indy_wallet_search_for_retrieve_id_value", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_retrieve_id_value");
            }

            #[test]
            fn indy_wallet_search_for_retrieve_id_value_tags() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_retrieve_id_value_tags"}"#;
                let wallet_handle = setup("indy_wallet_search_for_retrieve_id_value_tags", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_retrieve_id_value_tags");
            }

            #[test]
            fn indy_wallet_search_for_retrieve_full_record() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_retrieve_full_record"}"#;
                let wallet_handle = setup("indy_wallet_search_for_retrieve_full_record", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_retrieve_full_record");
            }

            #[test]
            fn indy_wallet_search_for_retrieve_total_count_only() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_retrieve_total_count_only"}"#;
                let wallet_handle = setup("indy_wallet_search_for_retrieve_total_count_only", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_retrieve_total_count_only");
            }

            #[test]
            fn indy_wallet_search_for_retrieve_records_only() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_retrieve_records_only"}"#;
                let wallet_handle = setup("indy_wallet_search_for_retrieve_records_only", SEARCH_WALLET_CONFIG);

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
                cleanup_wallet("indy_wallet_search_for_retrieve_records_only");
            }
        }

        mod close {
            use super::*;

            #[test]
            fn indy_close_wallet_search_works() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_close_wallet_search_works"}"#;
                let wallet_handle = setup("indy_close_wallet_search_works", SEARCH_WALLET_CONFIG);

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                close_wallet_search(search_handle).unwrap();
                wallet::close_wallet(wallet_handle).unwrap();
                cleanup_wallet("indy_close_wallet_search_works");
            }

            #[test]
            fn close_wallet_search_works_for_twice() {
                const SEARCH_WALLET_CONFIG: &str = r#"{"id":"close_wallet_search_works_for_twice"}"#;
                let wallet_handle = setup("close_wallet_search_works_for_twice", SEARCH_WALLET_CONFIG);

                let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

                close_wallet_search(search_handle).unwrap();

                let res = close_wallet_search(search_handle);
                assert_code!(ErrorCode::WalletInvalidHandle, res);

                wallet::close_wallet(wallet_handle).unwrap();
                cleanup_wallet("close_wallet_search_works_for_twice");
            }
        }
    }
}

#[cfg(not(feature = "only_high_cases"))]
mod medium_cases {
    use super::*;
    use crate::api::INVALID_WALLET_HANDLE;

    mod add_record {
        use super::*;

        #[test]
        fn indy_add_wallet_record_works_for_tags() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);
        }

        #[test]
        fn indy_add_wallet_record_works_same_types_different_ids() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(setup.wallet_handle, TYPE, ID_2, VALUE, None).unwrap();
        }

        #[test]
        fn indy_add_wallet_record_works_same_ids_different_types() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(setup.wallet_handle, TYPE_2, ID, VALUE, None).unwrap();
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = add_wallet_record(setup.wallet_handle, FORBIDDEN_TYPE, ID, VALUE, None);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_handle() {
            Setup::empty();

            let res = add_wallet_record(INVALID_WALLET_HANDLE, TYPE, ID, VALUE, None);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_add_wallet_record_works_for_invalid_tags() {
            let setup = Setup::wallet();

            let res = add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(r#"tag:1"#));
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_add_wallet_record_works_for_empty_params() {
            let setup = Setup::wallet();

            let res = add_wallet_record(setup.wallet_handle, "", ID, VALUE, None);
            assert_code!(ErrorCode::CommonInvalidParam3, res);

            let res = add_wallet_record(setup.wallet_handle, TYPE, "", VALUE, None);
            assert_code!(ErrorCode::CommonInvalidParam4, res);

            let res = add_wallet_record(setup.wallet_handle, TYPE, ID, "", None);
            assert_code!(ErrorCode::CommonInvalidParam5, res);
        }
    }

    mod update_record_value {
        use super::*;

        #[test]
        fn indy_update_record_value_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = update_wallet_record_value(INVALID_WALLET_HANDLE, TYPE, ID, VALUE_2);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_update_record_value_works_for_empty_value() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_value(setup.wallet_handle, TYPE, ID, "");
            assert_code!(ErrorCode::CommonInvalidParam5, res);
        }

        #[test]
        fn indy_update_record_value_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = update_wallet_record_value(setup.wallet_handle, FORBIDDEN_TYPE, ID, VALUE);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod update_record_tags {
        use super::*;

        #[test]
        fn indy_update_wallet_record_tags_works_for_empty() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = update_wallet_record_tags(setup.wallet_handle, TYPE, ID, "");
            assert_code!(ErrorCode::CommonInvalidParam5, res);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_wallet_handle() {
            Setup::empty();

            let res = update_wallet_record_tags(INVALID_WALLET_HANDLE, TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_update_wallet_record_tags_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = update_wallet_record_tags(setup.wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod add_record_tags {
        use super::*;

        #[test]
        fn indy_add_wallet_record_tags_works_for_twice_add_same_tag() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);

            add_wallet_record_tags(setup.wallet_handle, TYPE, ID, TAGS).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_rewrite_tag() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", TAGS);

            let tags_json = r#"{"tagName1": "str2"}"#;
            add_wallet_record_tags(setup.wallet_handle, TYPE, ID, tags_json).unwrap();

            let expected_result = r#"{"tagName1": "str2", "~tagName2": "5", "~tagName3": "8"}"#;
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", expected_result);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_handle() {
            Setup::empty();

            let res = add_wallet_record_tags(INVALID_WALLET_HANDLE, TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_add_wallet_record_tags_works_for_not_invalid_type() {
            let setup = Setup::wallet();

            let res = add_wallet_record_tags(setup.wallet_handle, FORBIDDEN_TYPE, ID, TAGS);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod delete_record_tags {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["~tagName2"]"#).unwrap();

            let expected_tags = r#"{"~tagName3": "8"}"#;
            check_record_field(setup.wallet_handle, TYPE, ID, "tags", expected_tags);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_twice_delete_same_tag() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, Some(TAGS)).unwrap();

            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
            delete_wallet_record_tags(setup.wallet_handle, TYPE, ID, r#"["tagName1"]"#).unwrap();
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_handle() {
            Setup::empty();

            let res = delete_wallet_record_tags(INVALID_WALLET_HANDLE, TYPE, ID, r#"["tagName1"]"#);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_delete_wallet_record_tags_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = delete_wallet_record_tags(setup.wallet_handle, FORBIDDEN_TYPE, ID, r#"["tagName1"]"#);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod delete_record {
        use super::*;

        #[test]
        fn indy_delete_wallet_record_works_for_twice() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            get_wallet_record(setup.wallet_handle, TYPE, ID, OPTIONS_EMPTY).unwrap();

            delete_wallet_record(setup.wallet_handle, TYPE, ID).unwrap();

            let res = delete_wallet_record(setup.wallet_handle, TYPE, ID);
            assert_code!(ErrorCode::WalletItemNotFound, res);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_handle() {
            Setup::empty();

            let res = delete_wallet_record(INVALID_WALLET_HANDLE, TYPE, ID);
            assert_code!(ErrorCode::WalletInvalidHandle, res);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_empty_params() {
            let setup = Setup::wallet();

            let res = delete_wallet_record(setup.wallet_handle, "", ID);
            assert_code!(ErrorCode::CommonInvalidParam3, res);

            let res = delete_wallet_record(setup.wallet_handle, TYPE, "");
            assert_code!(ErrorCode::CommonInvalidParam4, res);
        }

        #[test]
        fn indy_delete_wallet_record_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = delete_wallet_record(setup.wallet_handle, FORBIDDEN_TYPE, ID);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod get_record {
        use super::*;

        #[test]
        fn indy_get_wallet_record_works_for_invalid_options() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();

            let res = get_wallet_record(setup.wallet_handle, TYPE, ID, "not_json");
            assert_code!(ErrorCode::CommonInvalidStructure, res);
        }

        #[test]
        fn indy_get_wallet_record_works_for_invalid_type() {
            let setup = Setup::wallet();

            let res = get_wallet_record(setup.wallet_handle, FORBIDDEN_TYPE, ID, OPTIONS_EMPTY);
            assert_code!(ErrorCode::WalletAccessFailed, res);
        }
    }

    mod search {
        use super::*;
        use indy::{WalletHandle, SearchHandle};

        fn setup(name: &str, wallet_config: &str) -> WalletHandle {
            init_non_secret_test_wallet(name, wallet_config);
            wallet::open_wallet(wallet_config, WALLET_CREDENTIALS).unwrap()
        }

        fn tear_down(wallet_handle: WalletHandle, search_handle: SearchHandle) {
            close_wallet_search(search_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
        }

        #[test]
        fn indy_wallet_search_for_fetch_twice() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_fetch_twice"}"#;
            let wallet_handle = setup("indy_wallet_search_for_fetch_twice", SEARCH_WALLET_CONFIG);

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
            cleanup_wallet("indy_wallet_search_for_fetch_twice");
        }

        #[test]
        fn indy_wallet_search_for_no_records() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_no_records"}"#;
            let wallet_handle = setup("indy_wallet_search_for_no_records", SEARCH_WALLET_CONFIG);

            let search_handle = open_wallet_search(wallet_handle, TYPE_2, QUERY_EMPTY, OPTIONS_FULL).unwrap();

            let search_records = fetch_wallet_search_next_records(wallet_handle, search_handle, 5).unwrap();

            let search_records: SearchRecords = serde_json::from_str(&search_records).unwrap();
            assert_eq!(0, search_records.total_count.unwrap());
            assert!(search_records.records.is_none());

            tear_down(wallet_handle, search_handle);
        }

        #[test]
        fn indy_wallet_search_for_invalid_wallet_handle() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_invalid_wallet_handle"}"#;
            let wallet_handle = setup("indy_wallet_search_for_invalid_wallet_handle", SEARCH_WALLET_CONFIG);

            let res = open_wallet_search(INVALID_WALLET_HANDLE, TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            wallet::close_wallet(wallet_handle).unwrap();
            cleanup_wallet("indy_wallet_search_for_invalid_wallet_handle");
        }

        #[test]
        fn indy_wallet_search_for_invalid_search_handle() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_invalid_search_handle"}"#;
            let wallet_handle = setup("indy_wallet_search_for_invalid_search_handle", SEARCH_WALLET_CONFIG);

            let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            let res = fetch_wallet_search_next_records(wallet_handle, search_handle + 1, 5);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            tear_down(wallet_handle, search_handle);
            cleanup_wallet("indy_wallet_search_for_invalid_search_handle");
        }

        #[test]
        fn indy_wallet_search_for_invalid_type() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_wallet_search_for_invalid_type"}"#;
            let wallet_handle = setup("indy_wallet_search_for_invalid_type", SEARCH_WALLET_CONFIG);

            let res = open_wallet_search(wallet_handle, FORBIDDEN_TYPE, QUERY_EMPTY, OPTIONS_EMPTY);
            assert_code!(ErrorCode::WalletAccessFailed, res);

            wallet::close_wallet(wallet_handle).unwrap();
            cleanup_wallet("indy_wallet_search_for_invalid_type");
        }

        #[test]
        fn indy_close_wallet_search_works_for_invalid_handle() {
            const SEARCH_WALLET_CONFIG: &str = r#"{"id":"indy_close_wallet_search_works_for_invalid_handle"}"#;
            let wallet_handle = setup("indy_close_wallet_search_works_for_invalid_handle", SEARCH_WALLET_CONFIG);

            let search_handle = open_wallet_search(wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            let res = close_wallet_search(search_handle + 1);
            assert_code!(ErrorCode::WalletInvalidHandle, res);

            close_wallet_search(search_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            cleanup_wallet("indy_close_wallet_search_works_for_invalid_handle");
        }
    }

    mod rusqlite_transaction_fix {
        use super::*;

        #[test]
        pub fn transaction_works_during_opened_wallet_search() {
            let setup = Setup::wallet();

            add_wallet_record(setup.wallet_handle, TYPE, ID, VALUE, None).unwrap();
            add_wallet_record(setup.wallet_handle, TYPE, ID_2, VALUE_2, None).unwrap();

            let search_handle = open_wallet_search(setup.wallet_handle, TYPE, QUERY_EMPTY, OPTIONS_EMPTY).unwrap();

            add_wallet_record(setup.wallet_handle, TYPE, "IDSPEC", VALUE, Some(TAGS_2)).unwrap();

            close_wallet_search(search_handle).unwrap();
        }
    }
}

fn check_record_field(wallet_handle: WalletHandle, type_: &str, id: &str, field: &str, expected_value: &str) {
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

