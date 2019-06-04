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
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::cache::*;

use self::indy::ErrorCode;

pub const FORBIDDEN_TYPE: &'static str = "Indy::Test";

mod high_cases {
    use super::*;

    mod schema_cache {
        use super::*;
        use utils::domain::anoncreds::schema::{Schema, SchemaV1};
        use utils::constants::*;
        use std::thread::sleep;

        #[test]
        fn indy_get_schema_empty_options() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_empty_options");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();

            let schema_json = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json).unwrap();

            let _schema: SchemaV1 = serde_json::from_str(&schema_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_empty_options", &wallet_config);
        }

        #[test]
        fn indy_get_schema_empty_options_for_unknown_id() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_empty_options_for_unknown_id");

            let options_json = json!({}).to_string();

            let res = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                &Schema::schema_id(DID, "other_schema", "1.0"),
                &options_json);

            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_empty_options_for_unknown_id", &wallet_config);
        }

        #[test]
        fn indy_get_schema_only_cache_no_cached_data() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_only_cache_no_cached_data");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({"noUpdate": true}).to_string();

            let res = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json);

            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_only_cache_no_cached_data", &wallet_config);
        }

        #[test]
        fn indy_get_schema_cache_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_cache_works");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let schema_json1 = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            ).unwrap();
            let _schema: SchemaV1 = serde_json::from_str(&schema_json1).unwrap();

            // now retrieve it from cache
            let options_json = json!({"noUpdate": true}).to_string();
            let schema_json2 = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            ).unwrap();
            let _schema: SchemaV1 = serde_json::from_str(&schema_json2).unwrap();

            assert_eq!(schema_json1, schema_json2);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_cache_works", &wallet_config);
        }

        #[test]
        fn indy_get_schema_no_store_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_no_store_works");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({"noStore": true}).to_string();
            let schema_json1 = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            ).unwrap();
            let _schema: SchemaV1 = serde_json::from_str(&schema_json1).unwrap();

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true}).to_string();
            let res = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_no_store_works", &wallet_config);
        }

        #[test]
        fn indy_get_schema_no_cache_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_no_cache_works");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let schema_json1 = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            ).unwrap();
            let _schema: SchemaV1 = serde_json::from_str(&schema_json1).unwrap();

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true, "noCache": true}).to_string();
            let res = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_no_cache_works", &wallet_config);
        }

        #[test]
        fn indy_get_schema_min_fresh_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_schema_min_fresh_works");

            let (schema_id, _, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let schema_json1 = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            ).unwrap();
            let _schema: SchemaV1 = serde_json::from_str(&schema_json1).unwrap();

            sleep(std::time::Duration::from_secs(2));

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true, "minFresh": 1}).to_string();
            let res = get_schema_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                schema_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_schema_min_fresh_works", &wallet_config);
        }

        #[test]
        fn indy_purge_schema_cache_no_options() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_schema_cache_no_options");

            purge_schema_cache(wallet_handle, "{}").unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_schema_cache_no_options", &wallet_config);
        }

        #[test]
        fn indy_purge_schema_cache_all_data() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_schema_cache_all_data");

            purge_schema_cache(wallet_handle, &json!({"minFresh": -1}).to_string()).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_schema_cache_all_data", &wallet_config);
        }

        #[test]
        fn indy_purge_schema_cache_older_than_1000_seconds() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_schema_cache_older_than_1000_seconds");

            purge_schema_cache(wallet_handle, &json!({"minFresh": 1000}).to_string()).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_schema_cache_older_than_1000_seconds", &wallet_config);
        }
    }

    mod cred_def_cache {
        use super::*;
        use utils::domain::anoncreds::credential_definition::{CredentialDefinition};
        use utils::constants::*;
        use std::thread::sleep;


        #[test]
        fn indy_get_cred_def_empty_options() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_empty_options");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();

            let cred_def_json = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json).unwrap();

            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json).unwrap();

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_empty_options", &wallet_config);
        }

        #[test]
        fn indy_get_cred_def_only_cache_no_cached_data() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_only_cache_no_cached_data");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({"noUpdate": true}).to_string();

            let res = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json);

            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_only_cache_no_cached_data", &wallet_config);
        }

        #[test]
        fn indy_get_cred_def_cache_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_cache_works");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let cred_def_json1 = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            ).unwrap();
            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json1).unwrap();

            // now retrieve it from cache
            let options_json = json!({"noUpdate": true}).to_string();
            let cred_def_json2 = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            ).unwrap();
            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json2).unwrap();

            assert_eq!(cred_def_json1, cred_def_json2);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_cache_works", &wallet_config);
        }

        #[test]
        fn indy_get_cred_def_no_store_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_no_store_works");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({"noStore": true}).to_string();
            let cred_def_json1 = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            ).unwrap();
            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json1).unwrap();

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true}).to_string();
            let res = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_no_store_works", &wallet_config);
        }

        #[test]
        fn indy_get_cred_def_no_cache_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_no_cache_works");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let cred_def_json1 = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            ).unwrap();
            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json1).unwrap();

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true, "noCache": true}).to_string();
            let res = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_no_cache_works", &wallet_config);
        }

        #[test]
        fn indy_get_cred_def_min_fresh_works() {
            let (wallet_handle, pool_handle, wallet_config) = utils::setup_with_wallet_and_pool("indy_get_cred_def_min_fresh_works");

            let (_, cred_def_id, _) = utils::ledger::post_entities();

            let options_json = json!({}).to_string();
            let cred_def_json1 = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            ).unwrap();
            let _cred_def: CredentialDefinition = serde_json::from_str(&cred_def_json1).unwrap();

            sleep(std::time::Duration::from_secs(2));

            // it should not be present inside of cache, because of noStore option in previous request.
            let options_json = json!({"noUpdate": true, "minFresh": 1}).to_string();
            let res = get_cred_def_cache(
                pool_handle,
                wallet_handle,
                DID_MY1,
                cred_def_id,
                &options_json
            );
            assert_code!(ErrorCode::LedgerNotFound, res);

            utils::tear_down_with_wallet_and_pool(wallet_handle, pool_handle, "indy_get_cred_def_min_fresh_works", &wallet_config);
        }

        #[test]
        fn indy_purge_cred_def_cache_no_options() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_cred_def_cache_no_options");

            purge_cred_def_cache(wallet_handle, "{}").unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_cred_def_cache_no_options", &wallet_config);
        }

        #[test]
        fn indy_purge_cred_def_cache_all_data() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_cred_def_cache_all_data");

            purge_cred_def_cache(wallet_handle, &json!({"minFresh": -1}).to_string()).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_cred_def_cache_all_data", &wallet_config);
        }

        #[test]
        fn indy_purge_cred_def_cache_older_than_1000_seconds() {
            let (wallet_handle, wallet_config) = utils::setup_with_wallet("indy_purge_cred_def_cache_older_than_1000_seconds");

            purge_cred_def_cache(wallet_handle, &json!({"minFresh": 1000}).to_string()).unwrap();

            utils::tear_down_with_wallet(wallet_handle, "indy_purge_cred_def_cache_older_than_1000_seconds", &wallet_config);
        }
    }
}