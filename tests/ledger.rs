#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
#[cfg(feature = "local_nodes_pool")]
use utils::test::TestUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::pool::PoolUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::wallet::WalletUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::ledger::LedgerUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::signus::SignusUtils;
#[cfg(feature = "local_nodes_pool")]
use utils::anoncreds::AnoncredsUtils;

use std::collections::{HashMap, HashSet};


// TODO: FIXME: create_my_did doesn't support CID creation, but this trustee has CID as DID. So it is rough workaround for this issue.
// See: https://github.com/hyperledger/indy-sdk/issues/25


mod high_cases {
    use super::*;

    #[test]
    #[cfg(feature = "local_nodes_pool")]
    fn sovrin_send_request_works_for_invalid_pool_handle() {
        TestUtils::cleanup_storage();
        let pool_name = "pool1";

        let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
        let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

        let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

        let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

        let invalid_pool_handle = pool_handle + 1;
        let res = PoolUtils::send_request(invalid_pool_handle, &get_nym_request);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

        TestUtils::cleanup_storage();
    }

    #[test]
    #[cfg(feature = "local_nodes_pool")]
    fn sovrin_sign_and_submit_request_works_for_invalid_pool_handle() {
        TestUtils::cleanup_storage();
        let pool_name = "pool2";

        let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
        let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

        let (trustee_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
        let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

        let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

        let invalid_pool_handle = pool_handle + 1;
        let res = LedgerUtils::sign_and_submit_request(invalid_pool_handle, wallet_handle, &trustee_did, &nym_request);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorCode::PoolLedgerInvalidPoolHandle);

        TestUtils::cleanup_storage();
    }

    #[test]
    #[cfg(feature = "local_nodes_pool")]
    fn sovrin_sign_and_submit_request_works_for_invalid_wallet_handle() {
        TestUtils::cleanup_storage();
        let pool_name = "pool3";

        let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
        let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

        let (trustee_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
        let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

        let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

        let invalid_wallet_handle = wallet_handle + 1;
        let res = LedgerUtils::sign_and_submit_request(pool_handle, invalid_wallet_handle, &trustee_did, &nym_request);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

        TestUtils::cleanup_storage();
    }

    #[test]
    #[cfg(feature = "local_nodes_pool")]
    fn sovrin_sign_and_submit_request_works_for_not_found_signer() {
        TestUtils::cleanup_storage();
        let pool_name = "pool4";

        let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
        let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

        let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

        let trustee_did = "some_trustee_did";
        let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

        let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);

        TestUtils::cleanup_storage();
    }

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_nym_request_works_without_signature() {
            TestUtils::cleanup_storage();
            let pool_name = "pool5";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&my_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_nym_request_works_for_wrong_signer_role() {
            TestUtils::cleanup_storage();

            let pool_name = "pool6";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\", \"cid\":true}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let (my_did2, my_verkey2, my_pk2) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My2\"}").unwrap();
            let nym_request = LedgerUtils::build_nym_request(&my_did.clone(), &my_did2.clone(), None, None, None).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_nym_request_works_for_unknown_signer_did() {
            TestUtils::cleanup_storage();

            let pool_name = "pool7";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee9\", \"cid\":true}").unwrap();
            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();
            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_get_nym_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "pool8";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My3\"}").unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_send_get_nym_request_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool9";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_nym_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool10";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();

            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            println!("nym_response {:?}", nym_response);

            let get_nym_request = LedgerUtils::build_get_nym_request(&my_did.clone(), &my_did.clone()).unwrap();

            let get_nym_response = PoolUtils::send_request(pool_handle, &get_nym_request).unwrap();
            println!("get_nym_response {:?}", get_nym_response);

            let get_nym_response: Reply<GetNymReplyResult> = serde_json::from_str(&get_nym_response).unwrap();
            assert!(get_nym_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_attrib_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "pool11";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &attrib_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "pool12";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let res = PoolUtils::send_request(pool_handle, &attrib_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_get_attrib_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "pool13";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My2\"}").unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "endpoint").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_get_attrib_request_works_for_unknown_attribute() {
            TestUtils::cleanup_storage();

            let pool_name = "pool14";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "some_attribute").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_attrib_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool15";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let attrib_request = LedgerUtils::build_attrib_request(&my_did.clone(),
                                                                   &my_did.clone(),
                                                                   None,
                                                                   Some("{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"),
                                                                   None).unwrap();

            let attrib_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &attrib_request).unwrap();
            println!("attrib_response {}", attrib_response);

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&my_did.clone(), &my_did.clone(), "endpoint").unwrap();

            let get_attrib_response = PoolUtils::send_request(pool_handle, &get_attrib_request).unwrap();
            println!("get_attrib_response {}", get_attrib_response);

            let get_attrib_response: Reply<GetAttribReplyResult> = serde_json::from_str(&get_attrib_response).unwrap();
            assert!(get_attrib_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod schema_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_schema_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "pool16";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"keys\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let res = PoolUtils::send_request(pool_handle, &schema_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_schema_request_works_for_unknown_did() {
            TestUtils::cleanup_storage();

            let pool_name = "pool17";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My3\"}").unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"keys\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let res = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_get_schema_request_works_for_unknown_name() {
            TestUtils::cleanup_storage();

            let pool_name = "pool18";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let get_schema_data = "{\"name\":\"schema_name\",\"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did.clone(), get_schema_data).unwrap();
            ;

            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();
            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            assert!(get_schema_response.result.data.is_none());

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_schema_requests_works() {
            TestUtils::cleanup_storage();

            let pool_name = "pool19";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            println!("nym_response {}", nym_response.clone());

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"keys\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();

            let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();
            println!("schema_response {}", schema_response);

            let get_schema_data = "{\"name\":\"gvt2\",\
                                    \"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();

            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();
            println!("get_schema_response {}", get_schema_response.clone());

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            assert!(get_schema_response.result.data.is_some());

            TestUtils::cleanup_storage();
        }
    }

    mod node_request {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_send_node_request_works_without_signature() {
            TestUtils::cleanup_storage();

            let pool_name = "pool20";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Steward1\", \"cid\":true}").unwrap();

            let node_data = "{\"node_ip\":\"192.168.53.148\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"192.168.53.148\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"]}";
            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res = PoolUtils::send_request(pool_handle, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_send_node_request_works_for_wrong_role() {
            TestUtils::cleanup_storage();
            let pool_name = "pool21";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let node_data = "{\"node_ip\":\"192.168.53.148\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"192.168.53.148\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"]}";
            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res: Result<String, ErrorCode> = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_submit_node_request_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool22";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Steward1\", \"cid\":true}").unwrap();

            let node_data = "{\"node_ip\":\"192.168.53.148\",\
                              \"node_port\":9710, \
                              \"client_ip\":\"192.168.53.148\",\
                              \"client_port\":9709, \
                              \"alias\":\"Node5\", \
                              \"services\": [\"VALIDATOR\"]}";
            let node_request = LedgerUtils::build_node_request(&my_did.clone(), &my_did.clone(), node_data).unwrap();

            let res: Result<String, ErrorCode> = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &node_request);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::LedgerInvalidTransaction);

            TestUtils::cleanup_storage();
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_claim_def_requests_works() {
            TestUtils::cleanup_storage();
            let pool_name = "pool23";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, trustee_verkey, trustee_pk) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, my_verkey, my_pk) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey.clone()), None, None).unwrap();
            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            let schema_data = "{\"name\":\"gvt2\",\
                                \"version\":\"2.0\",\
                                \"keys\": [\"name\", \"male\"]}";
            let schema_request = LedgerUtils::build_schema_request(&my_did.clone(), schema_data).unwrap();
            let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &schema_request).unwrap();

            let get_schema_data = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            let get_schema_request = LedgerUtils::build_get_schema_request(&my_did.clone(), &my_did, get_schema_data).unwrap();
            let get_schema_response = PoolUtils::send_request(pool_handle, &get_schema_request).unwrap();

            let get_schema_response: Reply<GetSchemaReplyResult> = serde_json::from_str(&get_schema_response).unwrap();
            let schema_result_data = get_schema_response.result.data.clone().unwrap();

            let schema = Schema {
                name: schema_result_data.name,
                keys: schema_result_data.keys,
                version: schema_result_data.version,
                seq_no: get_schema_response.result.seq_no.unwrap()
            };

            let (claim_def_json, claim_def_uuid) = AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                                                                  &serde_json::to_string(&schema).unwrap()).unwrap();
            println!("claim_def_json {:}", claim_def_json);

            let claim_def: ClaimDefinition = serde_json::from_str(&claim_def_json).unwrap();
            let claim_def_data = ClaimDefinitionData {
                primary: claim_def.public_key,
                revocation: claim_def.public_key_revocation
            };
            let claim_def_data_json = serde_json::to_string(&claim_def_data).unwrap();

            let claim_def_request = LedgerUtils::build_claim_def_txn(&my_did.clone(), schema.seq_no, &claim_def.signature_type, &claim_def_data_json).unwrap();

            let claim_def_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &my_did, &claim_def_request).unwrap();
            println!("claim_def_response {}", claim_def_response);

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(&my_did.clone(), schema.seq_no, &claim_def.signature_type, &schema_result_data.origin).unwrap();

            let get_claim_def_response = PoolUtils::send_request(pool_handle, &get_claim_def_request).unwrap();
            println!("get_claim_def_response {}", get_claim_def_response);
            let get_claim_def_response: Reply<GetClaimDefReplyResult> = serde_json::from_str(&get_claim_def_response).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}

mod medium_cases {
    use super::*;

    mod nym_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_nym_requests_works_for_only_required_fields() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"1\",\
                    \"dest\":\"{}\"\
                }}", identifier, dest);

            let nym_request = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), None, None, None).unwrap();

            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_send_nym_request_works_for_only_required_fields() {
            TestUtils::cleanup_storage();
            let pool_name = "pool24";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();
            let (my_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), None, None, None).unwrap();

            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_nym_requests_works_with_option_fields() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            let verkey = "Anfh2rjAcxkE249DcdsaQl";
            let role = "STEWARD";
            let alias = "some_alias";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"1\",\
                    \"dest\":\"{}\",\
                    \"verkey\":\"{}\",\
                    \"alias\":\"{}\",\
                    \"role\":\"2\"\
                }}", identifier, dest, verkey, alias);

            let nym_request = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), Some(verkey), Some(alias), Some(role)).unwrap();

            assert!(nym_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_send_nym_request_works_with_option_fields() {
            TestUtils::cleanup_storage();

            let pool_name = "pool25";

            let pool_handle = PoolUtils::create_and_open_pool_ledger_config(pool_name).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, "wallet1", "default").unwrap();

            let (trustee_did, _, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}").unwrap();

            let (my_did, my_verkey, _) = SignusUtils::create_my_did(wallet_handle, "{\"seed\":\"00000000000000000000000000000My1\"}").unwrap();

            let role = "STEWARD";
            let alias = "some_alias";
            let nym_request = LedgerUtils::build_nym_request(&trustee_did.clone(), &my_did.clone(), Some(&my_verkey), Some(alias), Some(role)).unwrap();

            let nym_response = LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_request).unwrap();
            println!("nym_response {:?}", nym_response);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_nym_requests_works_for_wrong_role() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            let role = "WRONG_ROLE";

            let res = LedgerUtils::build_nym_request(&identifier.clone(), &dest.clone(), None, None, Some(role));
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_get_nym_requests_works() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"105\",\
                    \"dest\":\"{}\"\
                }}", identifier, dest);

            let get_nym_request = LedgerUtils::build_get_nym_request(&identifier.clone(), &dest.clone()).unwrap();

            assert!(get_nym_request.contains(&expected_result));
        }
    }

    mod attrib_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_attrib_requests_works_for_raw_data() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";
            let raw = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"100\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{{\\\"endpoint\\\":{{\\\"ha\\\":\\\"127.0.0.1:5555\\\"}}}}\"\
                }}", identifier, dest);

            let attrib_request = LedgerUtils::build_attrib_request(&identifier, &dest, None, Some(raw), None).unwrap();

            assert!(attrib_request.contains(&expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_attrib_requests_works_for_missed_attribute() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";

            let res = LedgerUtils::build_attrib_request(&identifier, &dest, None, None, None);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_get_attrib_requests_works() {
            let identifier = "Th7MpTaRZVRYnPiabds81Y";
            let dest = "Th7MpTaRZVRYnPiabds81Y";
            let raw = "endpoint";

            let expected_result = format!(
                "\"identifier\":\"{}\",\
                \"operation\":{{\
                    \"type\":\"104\",\
                    \"dest\":\"{}\",\
                    \"raw\":\"{}\"\
                }}", identifier, dest, raw);

            let get_attrib_request = LedgerUtils::build_get_attrib_request(&identifier, &dest, raw).unwrap();

            assert!(get_attrib_request.contains(&expected_result));
        }
    }

    mod schemas_requests {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_schema_requests_works_for_missed_field_in_data_json() {
            let identifier = "some_identifier";
            let data = r#"{"name":"name"}"#;

            let res = LedgerUtils::build_schema_request(identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_schema_requests_works_for_invalid_data_json_format() {
            let identifier = "some_identifier";
            let data = r#"{"name":"name", "keys":"name"}"#;

            let res = LedgerUtils::build_schema_request(identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_schema_requests_works_for_correct_data_json() {
            let identifier = "some_identifier";
            let data = r#"{"name":"name", "version":"1.0", "keys":["name","male"]}"#;

            let expected_result = "\"operation\":{\"type\":\"101\",\"data\":\"{\\\"name\\\":\\\"name\\\", \\\"version\\\":\\\"1.0\\\", \\\"keys\\\":[\\\"name\\\",\\\"male\\\"]";

            let schema_request = LedgerUtils::build_schema_request(identifier, data).unwrap();

            assert!(schema_request.contains(expected_result));
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_get_schema_requests_works_for_invalid_data_json() {
            let identifier = "some_identifier";
            let data = r#"{"name":"name"}"#;

            let res = LedgerUtils::build_get_schema_request(identifier, identifier, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        #[cfg(feature = "local_nodes_pool")]
        fn sovrin_build_get_schema_requests_works_for_correct_data_json() {
            let identifier = "some_identifier";
            let data = r#"{"name":"name","version":"1.0"}"#;

            let expected_result = r#""identifier":"some_identifier","operation":{"type":"107","dest":"some_identifier","data":{"name":"name","version":"1.0"}}"#;

            let get_schema_request = LedgerUtils::build_get_schema_request(identifier, identifier, data).unwrap();
            assert!(get_schema_request.contains(expected_result));
        }
    }

    mod node_requests {
        use super::*;

        #[test]
        fn sovrin_build_node_request_works_for_correct_data_json() {
            let identifier = "some_identifier";
            let dest = "some_dest";
            let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"]}"#;

            let expected_result = r#""identifier":"some_identifier","operation":{"type":"0","dest":"some_dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"]}}"#;

            let node_request = LedgerUtils::build_node_request(identifier, dest, data).unwrap();
            assert!(node_request.contains(expected_result));
        }

        #[test]
        fn sovrin_build_node_request_works_for_missed_field_in_data_json() {
            let identifier = "some_identifier";
            let dest = "some_dest";
            let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1}"#;

            let res = LedgerUtils::build_node_request(identifier, dest, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn sovrin_build_node_request_works_for_wrong_service() {
            let identifier = "some_identifier";
            let dest = "some_dest";
            let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["SERVICE"]}"#;

            let res = LedgerUtils::build_node_request(identifier, dest, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod claim_def_requests {
        use super::*;

        #[test]
        fn sovrin_build_claim_def_request_works_for_invalid_data_json() {
            TestUtils::cleanup_storage();

            let identifier = "some_identifier";
            let signature_type = "CL";
            let schema_seq_no = 1;
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"}}}"#;

            let res = LedgerUtils::build_claim_def_txn(identifier, schema_seq_no, signature_type, data);
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn sovrin_build_claim_def_request_works_for_correct_data_json() {
            let identifier = "some_identifier";
            let signature_type = "CL";
            let schema_seq_no = 1;
            let data = r#"{"primary":{"n":"1","s":"2","rms":"3","r":{"name":"1"},"rctxt":"1","z":"1"}}"#;

            let expected_result = r#""identifier":"some_identifier","operation":{"ref":1,"data":"{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"},\"rctxt\":\"1\",\"z\":\"1\"}}","type":"102","signature_type":"CL""#;

            let claim_def_request = LedgerUtils::build_claim_def_txn(identifier, schema_seq_no, signature_type, data).unwrap();
            assert!(claim_def_request.contains(expected_result));
        }

        #[test]
        fn sovrin_build_get_claim_def_request_works() {
            let identifier = "some_identifier";
            let _ref = 1;
            let signature_type = "signature_type";
            let origin = "some_origin";

            let expected_result = r#""identifier":"some_identifier","operation":{"type":"108","ref":1,"signature_type":"signature_type","origin":"some_origin"}"#;

            let get_claim_def_request = LedgerUtils::build_get_claim_def_txn(identifier, _ref, signature_type, origin).unwrap();
            assert!(get_claim_def_request.contains(expected_result));
        }
    }
}


#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    op: String,
    reason: String,
    req_id: u64,
    identifier: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct Reply<T> {
    op: String,
    result: T,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct GetNymReplyResult {
    identifier: String,
    req_id: u64,
    #[serde(rename = "type")]
    _type: String,
    data: Option<String>,
    dest: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct GetNymResultData {
    identifier: String,
    dest: String,
    role: Option<String>,
    verkey: Option<String>
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct GetAttribReplyResult {
    identifier: String,
    req_id: u64,
    #[serde(rename = "type")]
    _type: String,
    data: Option<String>,
    dest: String,
    raw: String,
    seq_no: Option<i32>
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct GetSchemaReplyResult {
    identifier: String,
    req_id: u64,
    seq_no: Option<i32>,
    //For tests/ In normal case seq_no exists
    #[serde(rename = "type")]
    _type: String,
    data: Option<GetSchemaResultData>,
    dest: Option<String>
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GetSchemaResultData {
    pub keys: HashSet<String>,
    pub name: String,
    pub origin: String,
    pub version: String
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct GetClaimDefReplyResult {
    identifier: String,
    #[serde(rename = "reqId")]
    req_id: u64,
    #[serde(rename = "seqNo")]
    seq_no: i32,
    #[serde(rename = "type")]
    _type: String,
    data: ClaimDefinitionData,
    origin: String,
    signature_type: String,
    #[serde(rename = "ref")]
    _ref: i32
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub keys: HashSet<String>,
    pub seq_no: i32
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct ClaimDefinition {
    pub public_key: PublicKey,
    pub public_key_revocation: Option<String>,
    pub schema_seq_no: i32,
    pub signature_type: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PublicKey {
    pub n: String,
    pub s: String,
    pub rms: String,
    pub r: HashMap<String, String>,
    pub rctxt: String,
    pub z: String
}

#[derive(Deserialize, Debug, Serialize, Eq, PartialEq)]
pub struct ClaimDefinitionData {
    pub primary: PublicKey,
    pub revocation: Option<String>
}


