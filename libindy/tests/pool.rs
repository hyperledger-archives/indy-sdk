#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

extern crate serde;

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

#[macro_use]
mod utils;

use self::indy::ErrorCode;

use utils::{environment, ledger, pool};
use utils::constants::*;

mod high_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());

            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_name() {
            utils::setup();

            let pool_name = "";
            let res = pool::create_pool_ledger_config(pool_name, None);
            assert_code!(ErrorCode::CommonInvalidParam2, res);

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_config_json() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());

            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            utils::tear_down();
        }


        #[test]
        fn create_pool_ledger_config_works_for_specific_config() {
            utils::setup();

            let txn_file_path = environment::tmp_file_path("specific_filename.txn");
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, Some(txn_file_path.as_path()));
            let pool_config = pool::pool_config_json(txn_file_path.as_path());

            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_genesis_txns() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool("pool_create", Some(0), None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            let res = pool::create_pool_ledger_config("pool_create", Some(pool_config.as_str()));
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "pool_open";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            pool::open_pool_ledger(pool_name, None).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_config() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_config";
            let config = r#"{"timeout": 20}"#;

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            pool::open_pool_ledger(pool_name, Some(config)).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_twice() {
            utils::setup();

            let pool_name = "pool_open_twice";
            pool::create_and_open_pool_ledger(pool_name).unwrap();

            let res = pool::open_pool_ledger(pool_name, None);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_two_nodes() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_two_nodes";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, Some(2), None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            pool::open_pool_ledger(pool_name, None).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_three_nodes() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_three_nodes";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, Some(3), None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            pool::open_pool_ledger(pool_name, None).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        pub fn open_pool_ledger_works_for_cached_txns() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_cached_txns";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();
            pool::dump_correct_genesis_txns_to_cache(pool_name).unwrap();

            pool::open_pool_ledger(pool_name, None).unwrap();

            utils::tear_down();
        }

        #[test]
        pub fn open_pool_ledger_works_for_corrupted_cached_txns() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_corrupted_for_cached_txns";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();
            pool::dump_incorrect_genesis_txns_to_cache(pool_name).unwrap();

            pool::open_pool_ledger(pool_name, None).unwrap();

            utils::tear_down();
        }
    }

    mod refresh {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_refresh_pool_ledger_works() {
            let pool_handle = utils::setup_with_pool();

            pool::refresh(pool_handle).unwrap();

            utils::tear_down_with_pool(pool_handle);
        }
    }

    mod close {
        use super::*;
        extern crate futures;
        use self::futures::Future;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works() {
            let pool_handle = utils::setup_with_pool();

            pool::close(pool_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_twice() {
            let pool_handle = utils::setup_with_pool();

            pool::close(pool_handle).unwrap();
            let res= pool::close(pool_handle);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_reopen_after_close() {
            let pool_handle = utils::setup_with_pool();

            pool::close(pool_handle).unwrap();
            let pool_handle = pool::open_pool_ledger(POOL, None).unwrap();
            pool::close(pool_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_pending_request() {
            let pool_handle = utils::setup_with_pool();

            let get_nym_req = ledger::build_get_nym_request(Some(DID_MY1), DID_MY1).unwrap();

            let submit_fut = indy::ledger::submit_request(pool_handle, &get_nym_req);

            pool::close(pool_handle).unwrap();

            let res = submit_fut.wait();
            assert_code!(ErrorCode::PoolLedgerTerminated, res);

            /* Now any request to API can failed, if pool::close works incorrect in case of pending requests.
               For example try to delete the pool. */
            pool::delete(POOL).unwrap();

            utils::tear_down();
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn indy_delete_pool_ledger_config_works() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            pool::delete(POOL).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_delete_pool_ledger_config_works_for_opened() {
            utils::setup();

            let pool_handle = pool::create_and_open_pool_ledger(POOL).unwrap();

            let res = pool::delete(POOL);
            assert_code!(ErrorCode::CommonInvalidState, res);

            pool::close(pool_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_closed() {
            utils::setup();

            let pool_handle = pool::create_and_open_pool_ledger(POOL).unwrap();
            pool::close(pool_handle).unwrap();
            pool::delete(POOL).unwrap();

            utils::tear_down();
        }
    }

    mod set_protocol_version {
        use super::*;

        #[test]
        fn indy_set_protocol_version_works() {
            pool::set_protocol_version(1).unwrap();
        }
    }
}

mod medium_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works_for_invalid_config_json() {
            utils::setup();

            let config = r#"{}"#.to_string();

            let res = pool::create_pool_ledger_config(POOL, Some(config.as_str()));
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_invalid_genesis_txn_path() {
            utils::setup();

            let config = r#"{"genesis_txn": "path"}"#.to_string();

            let res = pool::create_pool_ledger_config(POOL, Some(config.as_str()));
            assert_code!(ErrorCode::CommonIOError, res);

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_twice() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());

            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();
            let res = pool::create_pool_ledger_config(POOL, Some(pool_config.as_str()));
            assert_code!(ErrorCode::PoolLedgerConfigAlreadyExistsError, res);

            utils::tear_down();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_lines_in_genesis_txn_file() {
            utils::setup();

            let txn_file_path = pool::create_genesis_txn_file_for_empty_lines(POOL, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config("pool_create", Some(pool_config.as_str())).unwrap();

            utils::tear_down();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_name() {
            utils::setup();

            let res = pool::open_pool_ledger(POOL, None);
            assert_code!(ErrorCode::PoolLedgerNotCreatedError, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_after_error() {
            utils::setup();

            let res = pool::open_pool_ledger(POOL, None);
            assert_code!(ErrorCode::PoolLedgerNotCreatedError, res);

            let pool_handle = pool::create_and_open_pool_ledger(POOL).unwrap();

            pool::close(pool_handle).unwrap();

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_nodes_file() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_invalid_nodes_file";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool_with_invalid_nodes(pool_name, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            let res = pool::open_pool_ledger(pool_name, Some(pool_config.as_str()));
            assert_code!(ErrorCode::CommonInvalidState, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_wrong_alias() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let pool_name = "open_pool_ledger_works_for_wrong_alias";
            let txn_file_path = pool::create_genesis_txn_file_for_test_pool_with_wrong_alias(pool_name, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            let res = pool::open_pool_ledger(pool_name, None);
            assert_code!(ErrorCode::CommonInvalidState, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_config() {
            utils::setup();
            let name = "pool_open_invalid_confi";
            let config = r#"{"timeout": "true"}"#;

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(name, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(name, Some(pool_config.as_str())).unwrap();

            let res = pool::open_pool_ledger(name, Some(config));
            assert_code!(ErrorCode::CommonInvalidStructure, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_incompatible_protocol_version() {
            utils::setup();

            pool::set_protocol_version(1).unwrap();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            let res = pool::open_pool_ledger(POOL, None);
            assert_code!(ErrorCode::PoolIncompatibleProtocolVersion, res);

            utils::tear_down();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_wrong_ips() {
            utils::setup();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            let txn_file_path = pool::create_genesis_txn_file_for_test_pool_with_wrong_ips(POOL, None);
            let pool_config = pool::pool_config_json(txn_file_path.as_path());
            pool::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            let res = pool::open_pool_ledger(POOL, None);
            assert_code!(ErrorCode::PoolLedgerTimeout, res);

            utils::tear_down();
        }
    }

    mod close {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_invalid_handle() {
            let pool_handle = utils::setup_with_pool();

            let res = pool::close(pool_handle + 1);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            utils::tear_down_with_pool(pool_handle);
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn indy_delete_pool_ledger_config_works_for_not_created() {
            utils::setup();

            let res = pool::delete(POOL);
            assert_code!(ErrorCode::CommonIOError, res);

            utils::tear_down();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_twice() {
            utils::setup();

            let pool_handle = pool::create_and_open_pool_ledger(POOL).unwrap();
            pool::close(pool_handle).unwrap();
            pool::delete(POOL).unwrap();
            let res = pool::delete(POOL);
            assert_code!(ErrorCode::CommonIOError, res);

            utils::tear_down();
        }
    }

    mod refresh {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_refresh_pool_ledger_works_for_invalid_handle() {
            utils::setup();

            let pool_handle = pool::create_and_open_pool_ledger(POOL).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = pool::refresh(invalid_pool_handle);
            assert_code!(ErrorCode::PoolLedgerInvalidPoolHandle, res);

            pool::close(pool_handle).unwrap();

            utils::tear_down();
        }
    }

    mod set_protocol_version {
        use super::*;

        #[test]
        fn indy_set_protocol_version_works_for_unsupported() {
            let res = pool::set_protocol_version(0);
            assert_code!(ErrorCode::PoolIncompatibleProtocolVersion, res);
        }
    }
}
