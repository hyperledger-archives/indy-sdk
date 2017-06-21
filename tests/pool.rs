extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use sovrin::api::ErrorCode;

use utils::pool::PoolUtils;
use utils::test::TestUtils;


mod high_cases {
    use super::*;

    mod create {
        use super::*;

        #[test]
        fn create_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            PoolUtils::create_pool_ledger_config("pool_create", None).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works() {
            TestUtils::cleanup_storage();
            let name = "pool_open";

            PoolUtils::create_pool_ledger_config(name, None).unwrap();

            PoolUtils::open_pool_ledger(name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_twice() {
            TestUtils::cleanup_storage();
            let pool_name = "pool_open_twice";

            PoolUtils::create_pool_ledger_config(pool_name, None).unwrap();

            PoolUtils::open_pool_ledger(pool_name).unwrap();
            let res = PoolUtils::open_pool_ledger(pool_name);
            assert_match!(Err(ErrorCode::PoolLedgerInvalidPoolHandle), res);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_two_nodes() {
            TestUtils::cleanup_storage();
            let pool_name = "open_pool_ledger_works_for_two_nodes";

            let nodes = format!("{}\n{}\n",
                                "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}");


            PoolUtils::create_pool_ledger_config(pool_name, Some(nodes)).unwrap();

            PoolUtils::open_pool_ledger(pool_name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_three_nodes() {
            TestUtils::cleanup_storage();
            let pool_name = "open_pool_ledger_works_for_three_nodes";

            let nodes = format!("{}\n{}\n{}\n",
                                "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.2\",\"client_port\":9706,\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}");


            PoolUtils::create_pool_ledger_config(pool_name, Some(nodes)).unwrap();

            PoolUtils::open_pool_ledger(pool_name).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_close_pool_ledger_works() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_close_pool_ledger_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            PoolUtils::close(pool_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_close_pool_ledger_works_for_reopen_after_close() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_close_pool_ledger_works";
            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            PoolUtils::close(pool_handle).unwrap();
            PoolUtils::open_pool_ledger(pool_name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_delete_pool_ledger_config_works() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_remove_pool_ledger_config_works";
            PoolUtils::create_pool_ledger_config(pool_name, None).unwrap();

            PoolUtils::delete(pool_name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_delete_pool_ledger_config_works_for_opened() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_remove_pool_ledger_config_works";
            PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();

            assert_eq!(PoolUtils::delete(pool_name).unwrap_err(), ErrorCode::CommonInvalidState);

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod open {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_invalid_nodes_file() {
            TestUtils::cleanup_storage();
            let pool_name = "open_pool_ledger_works_for_invalid_nodes_file";

            let nodes = format!("{}\n{}\n{}\n{}\n",
                                "{\"data\":{\"client_port\":9702,\"client_ip\":\"10.0.0.2\",\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                                "{\"data\":{\"client_port\":9704,\"client_ip\":\"10.0.0.2\",\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                                "{\"data\":{\"client_port\":9706,\"client_ip\":\"10.0.0.2\",\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                                "{\"data\":{\"client_port\":9708,\"client_ip\":\"10.0.0.2\",\"node_ip\":\"10.0.0.2\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");

            PoolUtils::create_pool_ledger_config(pool_name, Some(nodes)).unwrap();

            let res = PoolUtils::open_pool_ledger(pool_name);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerTerminated);//TODO Replace on InvalidState Error

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn open_pool_ledger_works_for_wrong_alias() {
            TestUtils::cleanup_storage();
            let pool_name = "open_pool_ledger_works_for_wrong_alias";

            let nodes = format!("{}\n{}\n{}\n{}\n",
                                "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.2\",\"client_port\":9706,\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                                "{\"data\":{\"alias\":\"ALIAS_NODE\",\"client_ip\":\"10.0.0.2\",\"client_port\":9708,\"node_ip\":\"10.0.0.2\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");

            PoolUtils::create_pool_ledger_config(pool_name, Some(nodes)).unwrap();

            let res = PoolUtils::open_pool_ledger(pool_name);
            assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerTerminated);//TODO Replace on InvalidState Error

            TestUtils::cleanup_storage();
        }
    }
}