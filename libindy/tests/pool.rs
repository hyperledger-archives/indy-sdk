extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use indy::api::ErrorCode;

use utils::environment::EnvironmentUtils;
use utils::pool::PoolUtils;
use utils::test::TestUtils;


mod high_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool("pool_create", None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config("pool_create", Some(pool_config.as_str())).unwrap();

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

            let pool_name = "create_pool_ledger_config_works_for_config_json";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn create_pool_ledger_config_works_for_specific_config() {
            TestUtils::cleanup_storage();

            let pool_name = "create_pool_ledger_config_works_for_specific_config";
            let txn_file_path = EnvironmentUtils::tmp_file_path("specific_filename.txn");
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, Some(txn_file_path.as_path()));
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

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
            assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

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

            let pool_handle = PoolUtils::create_and_open_pool_ledger("indy_refresh_pool_ledger_works").unwrap();

            PoolUtils::refresh(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_close_pool_ledger_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_close_pool_ledger_works_twice";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            assert_eq!(PoolUtils::close(pool_handle).unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_close_pool_ledger_works_for_reopen_after_close() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_close_pool_ledger_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::open_pool_ledger(pool_name, None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete {
        use super::*;

        #[test]
        fn indy_delete_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_remove_pool_ledger_config_works";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();

            PoolUtils::delete(pool_name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn indy_delete_pool_ledger_config_works_for_opened() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_remove_pool_ledger_config_works_for_opened";
            PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();

            assert_eq!(PoolUtils::delete(pool_name).unwrap_err(), ErrorCode::CommonInvalidState);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_closed() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_delete_pool_ledger_config_works_for_closed";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::delete(pool_name).unwrap();

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

            let pool_name = "create_pool_ledger_config_works_for_invalid_config";
            let config = r#"{}"#.to_string();

            let res = PoolUtils::create_pool_ledger_config(pool_name, Some(config.as_str()));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_invalid_genesis_txn_path() {
            TestUtils::cleanup_storage();

            let pool_name = "create_pool_ledger_config_works_for_invalid_genesis_txn_path";
            let config = r#"{"genesis_txn": "path"}"#.to_string();

            let res = PoolUtils::create_pool_ledger_config(pool_name, Some(config.as_str()));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn create_pool_ledger_config_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "pool_create";
            let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
            let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());

            PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str())).unwrap();
            let res = PoolUtils::create_pool_ledger_config(pool_name, Some(pool_config.as_str()));

            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerConfigAlreadyExistsError);

            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_name() {
            TestUtils::cleanup_storage();
            let pool_name = "open_pool_ledger_works_for_invalid_name";

            let res = PoolUtils::open_pool_ledger(pool_name, None);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerTerminated);//TODO change it on IOError

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
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerTerminated);//TODO Replace on InvalidState Error

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
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerTerminated);//TODO Replace on InvalidState Error

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

            let pool_name = "indy_delete_pool_ledger_config_works_for_invalid_name";
            let res = PoolUtils::delete(pool_name);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_pool_ledger_config_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_delete_pool_ledger_config_works_for_twice";
            let pool_handle = PoolUtils::create_and_open_pool_ledger(pool_name).unwrap();
            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::delete(pool_name).unwrap();
            let res = PoolUtils::delete(pool_name);
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

            let pool_handle = PoolUtils::create_and_open_pool_ledger("indy_refresh_pool_ledger_works_for_invalid_handle").unwrap();

            let pool_handle = pool_handle + 1;
            let res = PoolUtils::refresh(pool_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

            TestUtils::cleanup_storage();
        }
    }
}