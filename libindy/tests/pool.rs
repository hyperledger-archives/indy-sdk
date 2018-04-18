extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use indy::api::ErrorCode;

use utils::environment::EnvironmentUtils;
use utils::callback::CallbackUtils;
use utils::constants::*;
use utils::ledger::LedgerUtils;
use utils::pool::PoolUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use std::ffi::CString;

mod high_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_name() {
            TestUtils::cleanup_storage();

            let pool_name = "";
            let res = PoolUtils::create_pool_ledger_config(pool_name, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidParam2);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_config_json() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn create_pool_ledger_config_works_for_specific_config() {
            TestUtils::cleanup_storage();

            let txn_file_path = EnvironmentUtils::tmp_file_path("specific_filename.txn");
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(POOL, None, Some(txn_file_path.as_path()));
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_genesis_txns() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool("pool_create", Some(0), None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            assert_eq!(ErrorCode::CommonInvalidStructure, PoolUtils::create_pool_ledger_config("pool_create", Some(pool_config.as_str())).unwrap_err());

            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works() {
            TestUtils::cleanup_storage();

            let pool_name = "pool_open";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            PoolUtils::open_pool_ledger(pool_name, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")] //TODO Not implemented yet
        fn open_pool_ledger_works_for_config() {
            TestUtils::cleanup_storage();

            let pool_name = "open_pool_ledger_works_for_config";
            let config = r#"{"refresh_on_open": true}"#;

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            PoolUtils::open_pool_ledger(pool_name, Some(config)).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "pool_open_twice";
            PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            let res = PoolUtils::open_pool_ledger(pool_name, None);
            assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, res.unwrap_err());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_two_nodes() {
            TestUtils::cleanup_storage();

            let pool_name = "open_pool_ledger_works_for_two_nodes";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, Some(2), None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            PoolUtils::open_pool_ledger(pool_name, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_three_nodes() {
            TestUtils::cleanup_storage();

            let pool_name = "open_pool_ledger_works_for_three_nodes";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, Some(3), None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            PoolUtils::open_pool_ledger(pool_name, None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod refresh {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_refresh_pool_ledger_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            PoolUtils::refresh(pool_handle).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            assert_eq!(PoolUtils::close(pool_handle).unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_reopen_after_close() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            let pool_handle = PoolUtils::open_pool_ledger(POOL, None).unwrap();
            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_pending_request() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let get_nym_req = LedgerUtils::build_get_nym_request(DID_MY1, DID_MY1).unwrap();

            let get_nym_req = CString::new(get_nym_req).unwrap();

            let (submit_receiver, submit_cmd_handle, submit_cb) = CallbackUtils::_closure_to_cb_ec_string();

            assert_eq!(api::ledger::indy_submit_request(submit_cmd_handle, pool_handle, get_nym_req.as_ptr(), submit_cb),
                       ErrorCode::Success);

            PoolUtils::close(pool_handle).unwrap();

            let (err, _) = submit_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
            assert_eq!(err, ErrorCode::PoolLedgerTerminated);

            /* Now any request to API can failed, if PoolUtils::close works incorrect in case of pending requests.
               For example try to delete the pool. */
            PoolUtils::delete(POOL).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn indy_delete_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();

            PoolUtils::delete(POOL).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_delete_pool_ledger_config_works_for_opened() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            assert_eq!(PoolUtils::delete(POOL).unwrap_err(), ErrorCode::CommonInvalidState);

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_closed() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::delete(POOL).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works_for_invalid_config_json() {
            TestUtils::cleanup_storage();

            let config = r#"{}"#.to_string();

            let res = PoolUtils::create_pool_ledger_config(POOL, Some(config.as_str()));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_invalid_genesis_txn_path() {
            TestUtils::cleanup_storage();

            let config = r#"{"genesis_txn": "path"}"#.to_string();

            let res = PoolUtils::create_pool_ledger_config(POOL, Some(config.as_str()));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_twice() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(POOL, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str())).unwrap();
            let res = PoolUtils::create_pool_ledger_config(POOL, Some(pool_config.as_str()));

            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerConfigAlreadyExistsError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_empty_lines_in_genesis_txn_file() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_empty_lines(POOL, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config("pool_create", Some(pool_config.as_str())).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_name() {
            TestUtils::cleanup_storage();

            let res = PoolUtils::open_pool_ledger(POOL, None);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerNotCreatedError);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_after_error() {
            TestUtils::cleanup_storage();

            let res = PoolUtils::open_pool_ledger(POOL, None);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerNotCreatedError);

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_nodes_file() {
            TestUtils::cleanup_storage();

            let pool_name = "open_pool_ledger_works_for_invalid_nodes_file";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool_with_invalid_nodes(pool_name, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            let res = PoolUtils::open_pool_ledger(pool_name, Some(pool_config.as_str()));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);//TODO Replace on InvalidState Error

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_wrong_alias() {
            TestUtils::cleanup_storage();

            let pool_name = "open_pool_ledger_works_for_wrong_alias";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool_with_wrong_alias(pool_name, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            let res = PoolUtils::open_pool_ledger(pool_name, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidState);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore]
        #[cfg(feature = "local_nodes_pool")] //TODO Not implemented yet
        fn open_pool_ledger_works_for_invalid_config() {
            TestUtils::cleanup_storage();
            let name = "pool_open";
            let config = r#"{"refresh_on_open": "true"}"#;

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(name, Some(pool_config.as_str())).unwrap();

            let res = PoolUtils::open_pool_ledger(name, Some(config));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod close {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_close_pool_ledger_works_for_invalid_handle";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            let pool_handle = pool_handle + 1;
            let res = PoolUtils::close(pool_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn indy_delete_pool_ledger_config_works_for_not_created() {
            TestUtils::cleanup_storage();

            let res = PoolUtils::delete(POOL);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();
            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::delete(POOL).unwrap();
            let res = PoolUtils::delete(POOL);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }
    }

    mod refresh {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_refresh_pool_ledger_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

            let invalid_pool_handle = pool_handle + 1;
            let res = PoolUtils::refresh(invalid_pool_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}
