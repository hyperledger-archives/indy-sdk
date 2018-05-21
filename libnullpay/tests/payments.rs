#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate nullpay;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

use utils::plugin;
use utils::payments;
use utils::payments_utils;
use utils::wallet;
use utils::test_utils;
use utils::types::*;
use utils::ledger;
use utils::pool;

use std::collections::HashMap;

static EMPTY_OBJECT: &str = "{}";
static PAYMENT_METHOD_NAME: &str = "null";
static POOL_NAME: &str = "pool_1";
static SUBMITTER_DID: &str = "Th7MpTaRZVRYnPiabds81Y";
static FEES: &str = r#"{"1":1, "101":2}"#;

mod high_cases {
    use super::*;

    mod create_payment_address {
        use super::*;

        #[test]
        pub fn create_payment_address_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();

            let payment_address = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();
            assert!(payment_address.starts_with("pay:null:"));

            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }
    }

    mod list_payments_addresses {
        use super::*;

        #[test]
        pub fn list_payment_addresses_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();

            let payment_address = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            let payment_addresses_list = payments::list_payment_addresses(wallet_handle).unwrap();

            let payment_addresses_list: Vec<String> = serde_json::from_str(payment_addresses_list.as_str()).unwrap();

            assert_eq!(payment_addresses_list.len(), 1);
            assert!(payment_addresses_list.contains(&payment_address));

            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }
    }

    mod fees {
        use super::*;

        #[test]
        pub fn set_request_fees_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let fees_req = payments::build_set_txn_fees_req(wallet_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES).unwrap();
            let fees_resp = ledger::submit_request(pool_handle, fees_req.as_str()).unwrap();

            let fees_stored = payments_utils::get_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME);

            let fee_1 = fees_stored.get("1").unwrap();
            assert_eq!(fee_1, &1);
            let fee_2 = fees_stored.get("101").unwrap();
            assert_eq!(fee_2, &2);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }

        #[test]
        pub fn get_request_fees_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES);

            let req = payments::build_get_txn_fees_req(wallet_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME).unwrap();
            let resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();
            let resp = payments::parse_get_utxo_response(PAYMENT_METHOD_NAME, resp.as_str()).unwrap();
            let map = serde_json::from_str::<HashMap<String, i32>>(resp.as_str()).unwrap();

            let fee_1 = map.get("1").unwrap();
            assert_eq!(fee_1, &1);
            let fee_2 = map.get("101").unwrap();
            assert_eq!(fee_2, &2);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }
    }

    mod mint {
        use super::*;

        #[test]
        pub fn mint_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let payment_address_1 = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();
            let payment_address_2 = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            let mint = vec![UTXOOutput {
                payment_address: payment_address_1.clone(),
                amount: 10,
                extra: None,
            }, UTXOOutput {
                payment_address: payment_address_2.clone(),
                amount: 20,
                extra: None,
            }];

            let outputs = serde_json::to_string(&mint).unwrap();

            let (req, payment_method) = payments::build_mint_req(wallet_handle, SUBMITTER_DID, outputs.as_str()).unwrap();

            assert_eq!(payment_method, "null");

            let mint_resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();

            let utxos = payments_utils::get_utxos_with_balance(vec![payment_address_1.as_str(), payment_address_2.as_str()], wallet_handle, pool_handle,SUBMITTER_DID);

            let utxo_1 = utxos.get(&payment_address_1).unwrap();
            assert_eq!(utxo_1.len(), 1);
            let utxo_info: &UTXOInfo = utxo_1.get(0).unwrap();
            assert_eq!(utxo_info.amount, 10);

            let utxo_2 = utxos.get(&payment_address_2).unwrap();
            assert_eq!(utxo_2.len(), 1);
            let utxo_info: &UTXOInfo = utxo_2.get(0).unwrap();
            assert_eq!(utxo_info.amount, 20);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }
    }
}