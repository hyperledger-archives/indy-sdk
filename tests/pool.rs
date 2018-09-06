#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_indy_sdk_wrapper as indy;
#[macro_use]
mod utils;

use indy::wallet::Wallet;
use utils::constants::{DEFAULT_CREDENTIALS, DID_MY1, DID_MY2};
use utils::setup::{Setup, SetupConfig};
use std::time::Duration;
use std::sync::mpsc::channel;
use indy::ErrorCode;

mod open_pool {
    use super::*;

    #[test]
    pub fn open_pool_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let config = Some(r#"{"timeout": 20}"#);

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, config).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_config() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let config = Some(r#"{"timeout": 20}"#);

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, config, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_config() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 2,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_two_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 2,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_two_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 2,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 3,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_three_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 3,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_three_nodes() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 3,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });
        utils::pool::dump_correct_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });
        utils::pool::dump_correct_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });
        utils::pool::dump_incorrect_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_timeout_works_for_corrupted_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });
        utils::pool::dump_incorrect_genesis_txns_to_cache(&setup.pool_name).unwrap();

        let pool_handle = indy::pool::Pool::open_ledger_timeout(&setup.pool_name, None, Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn open_pool_async_works_for_corrupted_cached_txns() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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


mod close_pool {
    use super::*;

    #[test]
    pub fn close_pool_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();
    }

    #[test]
    pub fn close_pool_timeout_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        indy::pool::Pool::close_timeout(pool_handle, Duration::from_secs(5)).unwrap();
    }

    #[test]
    pub fn close_pool_async_works() {
        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(DID_MY1, DID_MY1).unwrap();

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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(DID_MY1, DID_MY1).unwrap();

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
            num_trustees: 0,
            num_addresses: 0,
            connect_to_pool: false,
            number_of_nodes: 4,
            mint_tokens: None,
            num_users: 0,
            fees: None,
        });

        let (sender, receiver_close) = channel();

        let cb_close = move |ec| {
            sender.send(ec).unwrap();
        };

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();

        let get_nym_req = indy::ledger::Ledger::build_get_nym_request(DID_MY1, DID_MY1).unwrap();

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