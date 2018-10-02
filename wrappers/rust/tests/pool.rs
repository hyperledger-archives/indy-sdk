#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
pub mod utils;

use indy::wallet::Wallet;
use utils::constants::{DEFAULT_CREDENTIALS, DID_1};
use utils::setup::{Setup, SetupConfig};
use std::time::Duration;
use std::sync::mpsc::channel;
use indy::ErrorCode;
use indy::pool::Pool;
use utils::pool;

#[cfg(test)]
mod open_pool {
    use super::*;

    #[test]
    pub fn open_pool_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_config() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let config = Some(r#"{"timeout": 20}"#);

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, config).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_config() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let config = Some(r#"{"timeout": 20}"#);

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, config, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_config() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let config = Some(r#"{"timeout": 20}"#);

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, config, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let ec = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap_err();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();
        let ec = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_two_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 2,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_two_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 2,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_two_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 2,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_three_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 3,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_three_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 3,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_three_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 3,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_correct_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_correct_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_correct_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_works_for_corrupted_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_incorrect_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_corrupted_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_incorrect_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_corrupted_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });
        utils::pool::dump_incorrect_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, pool_handle| {
            sender.send((ec, pool_handle)).unwrap();
        };

        let ec = indy::pool::Pool::open_ledger_async(&setup.pool_name, None, cb);
        assert_eq!(ec, ErrorCode::Success);

        let (ec, pool_handle) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();
    }
}

#[cfg(test)]
mod close_pool {
    use super::*;

    #[test]
    pub fn close_pool_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn close_pool_timeout_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap();
    }

    #[test]
    pub fn close_pool_async_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let ec = indy::pool::Pool::close_async(pool_handle, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
    }

    #[test]
    pub fn close_pool_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
        let ec = indy::pool::Pool::close(pool_handle).unwrap_err();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);
    }

    #[test]
    pub fn close_pool_timeout_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap();
        let ec = indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap_err();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);
    }

    #[test]
    pub fn close_pool_async_works_for_twice() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };


        let ec = indy::pool::Pool::close_async(pool_handle, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };


        let ec = indy::pool::Pool::close_async(pool_handle, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::PoolLedgerInvalidPoolHandle);
    }

    #[test]
    pub fn close_pool_works_for_reopen_after_close() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();
        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn close_pool_timeout_works_for_reopen_after_close() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap();
        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn close_pool_async_works_for_reopen_after_close() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let ec = indy::pool::Pool::close_async(pool_handle, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();
        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn close_pool_works_for_pending_request() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(Some(DID_1), DID_1).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, s| {
            sender.send((ec, s)).unwrap();
        };

        let ec = indy::ledger::Ledger::submit_request_async(pool_handle, &get_nym_req, cb);
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close(pool_handle).unwrap();

        let (err, _) = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(err, ErrorCode::PoolLedgerTerminated);
    }

    #[test]
    pub fn close_pool_timeout_works_for_pending_request() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(Some(DID_1), DID_1).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, s| {
            sender.send((ec, s)).unwrap();
        };

        let ec = indy::ledger::Ledger::submit_request_async(pool_handle, &get_nym_req, cb);
        assert_eq!(ec, ErrorCode::Success);

        indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap();

        let (err, _) = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(err, ErrorCode::PoolLedgerTerminated);
    }

    #[test]
    pub fn close_pool_async_works_for_pending_request() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver_close) = channel();

        let cb_close = move |ec| {
            sender.send(ec).unwrap();
        };

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(Some(DID_1), DID_1).unwrap();

        let (sender, receiver) = channel();

        let cb = move |ec, s| {
            sender.send((ec, s)).unwrap();
        };

        let ec = indy::ledger::Ledger::submit_request_async(pool_handle, &get_nym_req, cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = indy::pool::Pool::close_async(pool_handle, cb_close);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver_close.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);

        let (err, _) = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(err, ErrorCode::PoolLedgerTerminated);
    }
}

#[cfg(test)]
mod test_pool_create_config {
    use super::*;

    use std::fs;
    use std::time::Duration;
    use std::sync::mpsc::channel;
    use utils::file::TempFile;
    use utils::pool::{PoolList, test_pool_name, test_genesis_config};

    const VALID_TIMEOUT: Duration = Duration::from_millis(250);

    #[inline]
    pub fn assert_pool_exists(name: &str) {
        assert!(PoolList::new().pool_exists(name));
    }

    #[inline]
    pub fn assert_pool_not_exists(name: &str) {
        assert!(! PoolList::new().pool_exists(name));
    }

    /*
    Returns the file, otherwise the file would be deleted
    when it goes out of scope.rustc_lsan
    */
    fn invalid_temporary_genesis_config() -> (String, TempFile) {
        let file = TempFile::new(None).unwrap();
        fs::write(&file, b"Some nonsensical data").unwrap();
        let config = json!({"genesis_txn": file.as_ref()}).to_string();

        (config, file)
    }

    #[test]
    /* Create a valid config with custom genesis txn. */
    fn config_with_genesis_txn() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();
        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!((), result.unwrap());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Config options missing genesis_txn */
    fn config_missing_genesis_txn() {
        let name = test_pool_name();
        let result = Pool::create_ledger_config(&name, Some("{}"));

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* A path which doesn't exists results in error. */
    fn config_with_unknown_path_genesis_txn() {
        let name = test_pool_name();
        let config = json!({"genesis_txn": "/nonexist15794345"}).to_string();
        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Error with an incorrectly formed gensis txn. */
    fn config_with_bad_genesis_txn() {
        let name = test_pool_name();
        let (config, _file) = invalid_temporary_genesis_config();

        let result = Pool::create_ledger_config(&name, Some(&config));

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Must specify pool name when config is created. */
    fn config_with_empty_pool_name() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();
        let result = Pool::create_ledger_config("", Some(&config));

        assert_eq!(ErrorCode::CommonInvalidParam2, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Error when creating a pool that already exists */
    fn config_already_exists() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();

        let result = Pool::create_ledger_config(&name, Some(&config));
        assert_eq!((), result.unwrap());

        let result = Pool::create_ledger_config(&name, Some(&config));
        assert_eq!(ErrorCode::PoolLedgerConfigAlreadyExistsError, result.unwrap_err());

        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config async. */
    fn config_async() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();

        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some(&config),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::Success, result);

        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config async resulting in an early error: callback isn't called. */
    fn config_async_with_early_error() {
        let name = test_pool_name();
        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some("{}"),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result);
        let result = receiver.recv_timeout(VALID_TIMEOUT);
        assert!(result.is_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Create a config async resulting in a late error: callback is called. */
    fn config_async_with_late_error() {
        let name = test_pool_name();
        let (config, _file) = invalid_temporary_genesis_config();
        let (sender, receiver) = channel();
        let result = Pool::create_ledger_config_async(
            &name,
            Some(&config),
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::CommonInvalidStructure, result);

        assert_pool_not_exists(&name);
    }

    #[test]
    /* Create a config with timeout. */
    fn config_timeout() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some(&config),
            VALID_TIMEOUT
        );

        assert_eq!((), result.unwrap());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }

    #[test]
    /* Create a config timeout resulting in an error. */
    fn config_timeout_with_error() {
        let name = test_pool_name();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some("{}"),
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
        assert_pool_not_exists(&name);
    }

    #[test]
    /* Timeout occurs while creating config. Pool is still created. */
    fn config_timeout_timeouts() {
        let name = test_pool_name();
        let (config, _file) = test_genesis_config();
        let result = Pool::create_ledger_config_timeout(
            &name,
            Some(&config),
            Duration::from_micros(1)
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        assert_pool_exists(&name);
        Pool::delete(&name).unwrap();
    }
}

#[cfg(test)]
mod test_delete_config {
    use super::*;

    use std::sync::mpsc::channel;
    use utils::pool::{PoolList, test_pool_name, create_default_pool};

    const VALID_TIMEOUT: Duration = Duration::from_millis(250);
    const NON_EXISTENT_NAME: &str = "a_pool_name_which_does_not_exist";

    #[inline]
    pub fn assert_pool_exists(name: &str) {
        assert!(PoolList::new().pool_exists(name));
    }

    #[inline]
    pub fn assert_pool_not_exists(name: &str) {
        assert!(! PoolList::new().pool_exists(name));
    }

    #[test]
    /* Delete a pool_config. */
    fn delete_pool() {
        let pool_name = create_default_pool();
        assert_pool_exists(&pool_name);

        let result = Pool::delete(&pool_name);
        assert_eq!((), result.unwrap());

        assert_pool_not_exists(&pool_name);
    }

    #[test]
    /* Error deleting non existent pool_config. */
    fn delete_pool_not_exist() {
        let result = Pool::delete(NON_EXISTENT_NAME);
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }

    #[test]
    /* Error deleting an open pool_config. */
    fn delete_pool_open() {
        let pool_name = create_default_pool();
        let config = json!({
            "refresh_on_open": false,
            "auto_refresh_time": 0,
        }).to_string();

        Pool::set_protocol_version(2).unwrap();
        let pool_handle = Pool::open_ledger(&pool_name, Some(&config)).unwrap();

        let result = Pool::delete(&pool_name);
        assert_eq!(ErrorCode::CommonInvalidState, result.unwrap_err());
        assert_pool_exists(&pool_name);

        Pool::close(pool_handle).unwrap();
        Pool::delete(&pool_name).unwrap();
        Pool::set_protocol_version(1).unwrap();
    }

    #[test]
    /* Delete pool async. */
    fn delete_pool_async() {
        let pool_name = create_default_pool();
        let (sender, receiver) = channel();

        let result = Pool::delete_async(&pool_name, move |ec| sender.send(ec).unwrap());
        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::Success, result);

        assert_pool_not_exists(&pool_name);
    }


    #[test]
    /* Delete pool async resuting in a late error: callback is called. */
    fn delete_pool_async_late_error() {
        let (sender, receiver) = channel();

        let result = Pool::delete_async(
            NON_EXISTENT_NAME,
            move |ec| sender.send(ec).unwrap()
        );

        assert_eq!(ErrorCode::Success, result);

        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::CommonIOError, result);
    }

    #[test]
    /* Delete a pool with a timeout. */
    fn delete_pool_timeout() {
        let pool_name = create_default_pool();

        let result = Pool::delete_timeout(&pool_name, VALID_TIMEOUT);
        assert_eq!((), result.unwrap());

        assert_pool_not_exists(&pool_name)
    }

    #[test]
    /* Error deleting a pool with a timeout. */
    fn delete_pool_timeout_error() {
        let result = Pool::delete_timeout(
            NON_EXISTENT_NAME,
            VALID_TIMEOUT
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }

    #[test]
    /* Delete a pool with timeout that timeouts. */
    fn delete_pool_timeout_timeouts() {
        let pool_name = create_default_pool();

        let result = Pool::delete_timeout(
            &pool_name,
            Duration::from_micros(1)
        );

        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());

        assert_pool_not_exists(&pool_name)
    }
}

#[cfg(test)]
mod test_set_protocol_version {
    use super::*;

    use indy::ledger::Ledger;
    use serde_json;
    use std::time::Duration;
    use std::sync::mpsc::channel;

    const VALID_VERSIONS: [usize; 2] = [1, 2];
    const VALID_TIMEOUT: Duration = Duration::from_millis(250);

    fn assert_protocol_version_set(version: usize) {
        let did = "5UBVMdSADMjGzuJMQwJ6yyzYV1krTcKRp6EqRAz8tiDP";
        let request = Ledger::build_get_nym_request(Some(did), did).unwrap();
        let request: serde_json::Value = serde_json::from_str(&request).unwrap();
        assert_eq!(json!(version), *request.get("protocolVersion").unwrap());
    }

    #[test]
    /* Set all available protocol versions. */
    fn set_all_valid_versions() {
        for &version in VALID_VERSIONS.into_iter() {
            let result = Pool::set_protocol_version(version);
            assert_eq!((), result.unwrap());
            assert_protocol_version_set(version);
        }

        Pool::set_protocol_version(1).unwrap();
    }

    #[test]
    /* Error setting invalid protocol version. */
    fn set_invalid_versions() {
        let result = Pool::set_protocol_version(0);
        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result.unwrap_err());
        assert_protocol_version_set(1);

        let next_protocol_version = *VALID_VERSIONS.last().unwrap() + 1;
        let result = Pool::set_protocol_version(next_protocol_version);
        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result.unwrap_err());
        assert_protocol_version_set(1);
    }

    #[test]
    /* Set protocol version async. */
    fn set_protocol_version_async() {
        let (sender, receiver) = channel();
        Pool::set_protocol_version_async(2, move |ec| sender.send(ec).unwrap());
        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert_eq!(ErrorCode::Success, result);
        assert_protocol_version_set(2);

        Pool::set_protocol_version(1).unwrap();
    }

    #[test]
    /* Error setting protocol version async. */
    fn set_invalid_version_async() {
        let (sender, receiver) = channel();
        Pool::set_protocol_version_async(0, move |ec| sender.send(ec).unwrap());
        let result = receiver.recv_timeout(VALID_TIMEOUT).unwrap();

        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result);
        assert_protocol_version_set(1);
    }

    #[test]
    /* Set protocol version with timeout. */
    fn set_protocol_version_timeout() {
        let result = Pool::set_protocol_version_timeout(2, VALID_TIMEOUT);
        assert_eq!((), result.unwrap());
        assert_protocol_version_set(2);

        Pool::set_protocol_version(1).unwrap();
    }

    #[test]
    /* Error setting protocol version with timeout. */
    fn set_invalid_version_timeout() {
        let result = Pool::set_protocol_version_timeout(0, VALID_TIMEOUT);
        assert_eq!(ErrorCode::PoolIncompatibleProtocolVersion, result.unwrap_err());
        assert_protocol_version_set(1);
    }

    #[test]
    /* Setting protcol version with timeout timeouts. */
    fn set_protocol_version_timeout_timeouts() {
        let result = Pool::set_protocol_version_timeout(0, Duration::from_micros(1));
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
        assert_protocol_version_set(1);
    }
}

#[cfg(test)]
/*
The sync Pool::list is already tested by the create tests.
There aren't tests for failure because I'm not sure how it would fail.
*/
mod test_pool_list {
    use super::*;

    use std::sync::mpsc::channel;
    use utils::pool::{PoolList, create_default_pool};

    const VALID_TIMEOUT: Duration = Duration::from_millis(250);

    #[test]
    fn list_pool_async() {
        let name = create_default_pool();
        let (sender, receiver) = channel();

        Pool::list_async(move |ec, result| sender.send(result).unwrap());

        let pool_json = receiver.recv_timeout(VALID_TIMEOUT).unwrap();
        assert!(PoolList::from_json(&pool_json).pool_exists(&name));

        Pool::delete(&name).unwrap();
    }

    #[test]
    /* List pools with timeout. */
    fn list_pool_timeout() {
        let name = create_default_pool();

        let pool_json = Pool::list_timeout(VALID_TIMEOUT).unwrap();
        assert!(PoolList::from_json(&pool_json).pool_exists(&name));

        Pool::delete(&name).unwrap();
    }

    #[test]
    /* List pools with timeout, timeouts. */
    fn list_pool_timeout_timeouts() {
        let result = Pool::list_timeout(Duration::from_micros(1));
        assert_eq!(ErrorCode::CommonIOError, result.unwrap_err());
    }
}

#[cfg(test)]
mod test_refresh_works {
    use super::*;

    #[test]
    pub fn refresh_pool_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        indy::pool::Pool::refresh(setup.pool_handle.unwrap()).unwrap();
    }

    #[test]
    pub fn refresh_pool_timeout_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        indy::pool::Pool::refresh_timeout(setup.pool_handle.unwrap(), Duration::from_secs(5)).unwrap();
    }

    #[test]
    pub fn refresh_pool_async_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let (sender, receiver) = channel();

        let cb = move |ec| {
            sender.send(ec).unwrap();
        };

        let ec = indy::pool::Pool::refresh_async(setup.pool_handle.unwrap(), cb);
        assert_eq!(ec, ErrorCode::Success);

        let ec = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success);
    }
}
