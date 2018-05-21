#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate nullpay;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate nullpay;

#[macro_use]
mod utils;

use utils::plugin;
use utils::payments;
use utils::wallet;
use utils::test_utils;
use utils::types::*;
use utils::ledger;
use utils::pool;

static EMPTY_OBJECT: &str = "{}";
static PAYMENT_METHOD_NAME: &str = "null";
static POOL_NAME: &str = "POOL";
static SUBMITTER_DID: &str = "Th7MpTaRZVRYnPiabds81Y";

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

    mod mint {
        use super::*;
        use utils::types::UTXOOutput;

        #[test]
        pub fn mint_works() {
            test_utils::cleanup_storage();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet(POOL_NAME, None).unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let payment_address_1 = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();
            let payment_address_2 = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            let mint = vec![UTXOOutput {
                payment_address: payment_address_1,
                amount: 10,
                extra: None,
            }, UTXOOutput {
                payment_address: payment_address_2,
                amount: 20,
                extra: None,
            }];

            let outputs = serde_json::to_string(&mint).unwrap();

            let (req, payment_method) = payments::build_mint_req(wallet_handle, SUBMITTER_DID, outputs.as_str()).unwrap();

            assert_eq!(payment_method, "null");

            let mint_resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();

            let (req_utxo_1, payment_method) = payments::build_get_utxo_request(wallet_handle, SUBMITTER_DID, payment_address_1.as_str()).unwrap();
            let resp_utxo_1 = ledger::submit_request(pool_handle, req_utxo_1.as_str()).unwrap();
            let resp_utxo_1 = payments::parse_get_utxo_response(payment_method.as_str(), resp_utxo_1.as_str()).unwrap();

            let utxo_1: Vec<UTXOInfo> = serde_json::from_str(resp_utxo_1.as_str()).unwrap();

            assert_eq!(utxo_1.len(), 1);
            let utxo_info = utxo_1.get(1).unwrap();
            assert_eq!()

            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::cleanup_storage();
        }
    }
}