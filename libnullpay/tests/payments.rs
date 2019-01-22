#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate nullpay;
extern crate indyrs as indy;

#[macro_use]
extern crate lazy_static;
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
use utils::did;
use indy::ErrorCode;

use std::collections::HashMap;

static EMPTY_OBJECT: &str = "{}";
static PAYMENT_METHOD_NAME: &str = "null";
static POOL_NAME: &str = "pool_1";
static SUBMITTER_DID: &str = "Th7MpTaRZVRYnPiabds81Y";
static TRUSTEE_SEED: &str = "000000000000000000000000Trustee1";
static FEES: &str = r#"{"NYM":1, "SCHEMA":2}"#;
static EXTRA: &str = "extra_1";

mod high_cases {
    use super::*;

    mod create_payment_address {
        use super::*;

        #[test]
        pub fn create_payment_address_works() {
            test_utils::setup();

            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();

            let payment_address = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();
            assert!(payment_address.starts_with("pay:null:"));

            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod list_payments_addresses {
        use super::*;

        #[test]
        pub fn list_payment_addresses_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();

            let payment_address = payments::create_payment_address(wallet_handle, PAYMENT_METHOD_NAME, EMPTY_OBJECT).unwrap();

            let payment_addresses_list = payments::list_payment_addresses(wallet_handle).unwrap();

            let payment_addresses_list: Vec<String> = serde_json::from_str(payment_addresses_list.as_str()).unwrap();

            assert_eq!(payment_addresses_list.len(), 1);
            assert!(payment_addresses_list.contains(&payment_address));

            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod get_payment_sources {
        use super::*;

        #[test]
        pub fn get_payment_sources_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, (i + 1) as i32)).collect();

            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let (req_sources_1, payment_method) = payments::build_get_payment_sources_request(wallet_handle, SUBMITTER_DID, addresses.get(0).unwrap().as_str()).unwrap();
            let resp_sources_1 = ledger::submit_request(pool_handle, req_sources_1.as_str()).unwrap();
            let resp_sources_1 = payments::parse_get_payment_sources_response(payment_method.as_str(), resp_sources_1.as_str()).unwrap();

            let (req_sources_2, payment_method) = payments::build_get_payment_sources_request(wallet_handle, SUBMITTER_DID, addresses.get(1).unwrap().as_str()).unwrap();
            let resp_sources_2 = ledger::submit_request(pool_handle, req_sources_2.as_str()).unwrap();
            let resp_sources_2 = payments::parse_get_payment_sources_response(payment_method.as_str(), resp_sources_2.as_str()).unwrap();

            let sources_1: Vec<SourceInfo> = serde_json::from_str(resp_sources_1.as_str()).unwrap();
            assert_eq!(sources_1.len(), 1);
            let sources_info: &SourceInfo = sources_1.get(0).unwrap();
            assert_eq!(sources_info.amount, 1);
            assert!(sources_info.extra.is_none());

            let sources_2: Vec<SourceInfo> = serde_json::from_str(resp_sources_2.as_str()).unwrap();
            assert_eq!(sources_2.len(), 1);
            let sources_info: &SourceInfo = sources_2.get(0).unwrap();
            assert_eq!(sources_info.amount, 2);
            assert!(sources_info.extra.is_none());

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn get_payment_sources_works_for_no_sources() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let nonexistent_addr = "pay:null:no-addr";
            let res = payments_utils::get_sources_with_balance(vec![nonexistent_addr.to_string()], wallet_handle, pool_handle, SUBMITTER_DID);

            let res_vec = res.get(nonexistent_addr).unwrap();
            assert_eq!(res_vec.len(), 0);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod mint {
        use super::*;

        #[test]
        pub fn mint_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);
            let mint: Vec<Output> = addresses.clone().into_iter().enumerate().map(|(i, payment_address)| Output {
                recipient: payment_address,
                amount: ((i + 1) * 10) as i32
            }).collect();

            let outputs = serde_json::to_string(&mint).unwrap();

            let (req, _) = payments::build_mint_req(wallet_handle, SUBMITTER_DID, outputs.as_str(), None).unwrap();

            ledger::submit_request(pool_handle, req.as_str()).unwrap();

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let source_1 = sources.get(addresses.get(0).unwrap()).unwrap();
            assert_eq!(source_1.len(), 1);
            let source_info: &SourceInfo = source_1.get(0).unwrap();
            assert_eq!(source_info.amount, 10);
            assert!(source_info.extra.is_none());

            let source_2 = sources.get(addresses.get(1).unwrap()).unwrap();
            assert_eq!(source_2.len(), 1);
            let source_info: &SourceInfo = source_2.get(0).unwrap();
            assert_eq!(source_info.amount, 20);
            assert!(source_info.extra.is_none());

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn mint_works_for_extra() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);
            let mint: Vec<Output> = addresses.clone().into_iter().enumerate().map(|(i, payment_address)| Output {
                recipient: payment_address,
                amount: ((i + 1) * 10) as i32
            }).collect();

            let outputs = serde_json::to_string(&mint).unwrap();

            let (req, _) = payments::build_mint_req(wallet_handle, SUBMITTER_DID, outputs.as_str(), Some(EXTRA)).unwrap();

            ledger::submit_request(pool_handle, req.as_str()).unwrap();

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let source_1 = sources.get(addresses.get(0).unwrap()).unwrap();
            assert_eq!(source_1.len(), 1);
            let source_info: &SourceInfo = source_1.get(0).unwrap();
            assert_eq!(source_info.amount, 10);
            assert_eq!(source_info.extra.clone().unwrap(), EXTRA.to_string());

            let source_2 = sources.get(addresses.get(1).unwrap()).unwrap();
            assert_eq!(source_2.len(), 1);
            let source_info: &SourceInfo = source_2.get(0).unwrap();
            assert_eq!(source_info.amount, 20);
            assert_eq!(source_info.extra.clone().unwrap(), EXTRA.to_string());

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod add_request_fees {
        use super::*;

        #[test]
        pub fn add_request_fees_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Prepare new nym request
            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();

            //2. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //3. Mint sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, my_did.as_str());

            //4. Get created Sources
            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, my_did.as_str());

            //5. Set transaction fees
            payments_utils::set_request_fees(wallet_handle, pool_handle, my_did.as_str(), PAYMENT_METHOD_NAME, FEES);

            //6. Create inputs and outputs
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //7. Add fees to request, send it and parse response
            let (nym_req_with_fees, _) = payments::add_request_fees(wallet_handle, my_did.as_str(), nym_req.as_str(), inputs.as_str(), outputs.as_str(), None).unwrap();

            let nym_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did.as_str(), nym_req_with_fees.as_str()).unwrap();

            let nym_response_parsed = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, nym_response.as_str()).unwrap();

            let created_receipts: Vec<ReceiptInfo> = serde_json::from_str(nym_response_parsed.as_str()).unwrap();

            assert_eq!(created_receipts.len(), 1);
            let new_source = created_receipts.get(0).unwrap();
            assert_eq!(new_source.amount, 19);

            let sources_map = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);
            let source_1 = sources_map.get(addresses.get(0).unwrap()).unwrap();
            assert!(source_1.is_empty());
            let source_2 = sources_map.get(addresses.get(1).unwrap()).unwrap();
            assert_eq!(source_2.len(), 2);
            let amounts: Vec<i32> = source_2.into_iter().map(|info| info.amount).collect();
            assert!(amounts.contains(&30));
            assert!(amounts.contains(&19));

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn add_request_fees_works_for_extra() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Prepare new nym request
            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();

            //2. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //3. Mint sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, my_did.as_str());

            //4. Get created Sources
            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, my_did.as_str());

            //5. Set transaction fees
            payments_utils::set_request_fees(wallet_handle, pool_handle, my_did.as_str(), PAYMENT_METHOD_NAME, FEES);

            //6. Create inputs and outputs
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //7. Add fees to request, send it and parse response
            let (nym_req_with_fees, _) = payments::add_request_fees(wallet_handle, my_did.as_str(), nym_req.as_str(), inputs.as_str(), outputs.as_str(), Some(EXTRA)).unwrap();

            let nym_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did.as_str(), nym_req_with_fees.as_str()).unwrap();

            let nym_response_parsed = payments::parse_response_with_fees(PAYMENT_METHOD_NAME, nym_response.as_str()).unwrap();

            let created_receipts: Vec<ReceiptInfo> = serde_json::from_str(nym_response_parsed.as_str()).unwrap();

            assert_eq!(created_receipts.len(), 1);
            let new_source = created_receipts.get(0).unwrap();
            assert_eq!(new_source.amount, 19);
            assert_eq!(new_source.extra.clone().unwrap(), EXTRA.to_string());

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn add_request_works_for_insufficient_funds() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, i as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, my_did.as_str());

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, r#"{"1": 10, "101": 10}"#);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let (nym_req_with_fees, payment_method) = payments::add_request_fees(wallet_handle, SUBMITTER_DID, nym_req.as_str(), inputs.as_str(), outputs.as_str(), None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did.as_str(), nym_req_with_fees.as_str()).unwrap();
            let nym_resp_parsed_err = payments::parse_response_with_fees(payment_method.as_str(), nym_resp.as_str()).unwrap_err();

            assert_eq!(nym_resp_parsed_err.error_code, ErrorCode::PaymentInsufficientFundsError);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn add_request_works_for_spent_source() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let (my_did_2, my_vk_2) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();
            let nym_req_2 = ledger::build_nym_request(&trustee_did, &my_did_2, &my_vk_2, "aaa", "TRUSTEE").unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i+3)*10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, my_did.as_str());

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, r#"{"1": 10, "101": 10}"#);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let (nym_req_with_fees, payment_method) = payments::add_request_fees(wallet_handle, SUBMITTER_DID, nym_req.as_str(), inputs.as_str(), outputs.as_str(), None).unwrap();
            let nym_resp = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did.as_str(), nym_req_with_fees.as_str()).unwrap();
            payments::parse_response_with_fees(payment_method.as_str(), nym_resp.as_str()).unwrap();

            let (nym_req_with_fees_2, payment_method) = payments::add_request_fees(wallet_handle, SUBMITTER_DID, nym_req_2.as_str(), inputs.as_str(), outputs.as_str(), None).unwrap();
            let nym_resp_2 = ledger::sign_and_submit_request(pool_handle, wallet_handle, trustee_did.as_str(), nym_req_with_fees_2.as_str()).unwrap();
            let ec = payments::parse_response_with_fees(payment_method.as_str(), nym_resp_2.as_str()).unwrap_err();
            assert_eq!(ec.error_code, ErrorCode::PaymentSourceDoesNotExistError);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn add_request_fees_works_for_source_not_correspond_to_wallet() {
            test_utils::setup();
            plugin::init_plugin();

            let wallet_handle_1 = wallet::create_and_open_wallet().unwrap();
            let wallet_handle_2 = wallet::create_and_open_wallet().unwrap();

            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Prepare new nym request
            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle_1, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle_1, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();

            //2. Create addresses 1
            let addresses_1 = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle_1, PAYMENT_METHOD_NAME);

            //4. Mint sources
            let mint: Vec<(String, i32)> = addresses_1.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle_1, pool_handle, my_did.as_str());

            //5. Get created sources
            let sources = payments_utils::get_sources_with_balance(addresses_1.clone(), wallet_handle_1, pool_handle, my_did.as_str());

            //6. Set transaction fees
            payments_utils::set_request_fees(wallet_handle_1, pool_handle, my_did.as_str(), PAYMENT_METHOD_NAME, FEES);

            //7. Create inputs and outputs
            let addr_1 = addresses_1.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses_1.get(1).unwrap().to_string(),
                amount: 19,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //8. Add fees to request by using other wallet
            let res = payments::add_request_fees(wallet_handle_2, my_did.as_str(), nym_req.as_str(), inputs.as_str(), outputs.as_str(), None);
            assert_eq!(res.unwrap_err().error_code, ErrorCode::CommonInvalidState);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle_1).unwrap();
            wallet::close_wallet(wallet_handle_2).unwrap();
            test_utils::tear_down();
        }
    }

    mod payment {
        use super::*;

        #[test]
        pub fn payment_request_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //2. Mint sources and get created sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            //3. Prepare inputs and outputs for payment txn
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //4. Build and send payment txn
            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            let payment_resp_parsed = payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap();

            //5. Check response sources
            let sources: Vec<ReceiptInfo> = serde_json::from_str(payment_resp_parsed.as_str()).unwrap();
            assert_eq!(sources.len(), 2);

            let sources: HashMap<i32, String> = sources.into_iter().map(|info| (info.amount, info.recipient)).collect();
            let payment_source = sources.get(&19).unwrap();
            let change_source = sources.get(&1).unwrap();

            let payment_source_end = payment_source.split("_").last().unwrap();
            assert!(addresses.get(1).unwrap().to_string().ends_with(payment_source_end));

            let change_source_end = change_source.split("_").last().unwrap();
            assert!(addresses.get(0).unwrap().to_string().ends_with(change_source_end));

            //6. Check all sources that are present on the ledger for payment addresses
            let sources_map = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);
            let source_1 = sources_map.get(addresses.get(0).unwrap()).unwrap();
            assert_eq!(source_1.len(), 1);
            let amounts: Vec<i32> = source_1.into_iter().map(|info| info.amount).collect();
            assert!(amounts.contains(&1));

            let source_2 = sources_map.get(addresses.get(1).unwrap()).unwrap();
            assert_eq!(source_2.len(), 2);
            let amounts: Vec<i32> = source_2.into_iter().map(|info| info.amount).collect();
            assert!(amounts.contains(&19));
            assert!(amounts.contains(&30));

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn payment_request_works_for_extra() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //2. Mint sources and get created sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            //3. Prepare inputs and outputs for payment txn
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //4. Build and send payment txn
            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), Some(EXTRA)).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            let payment_resp_parsed = payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap();

            //5. Check response sources
            let sources: Vec<ReceiptInfo> = serde_json::from_str(payment_resp_parsed.as_str()).unwrap();
            assert_eq!(sources.len(), 2);

            assert!(sources.iter().all(|info| info.extra.is_some()));

            let sources: HashMap<i32, String> = sources.into_iter().map(|info| (info.amount, info.recipient)).collect();
            let payment_source = sources.get(&19).unwrap();
            let change_source = sources.get(&1).unwrap();

            let payment_source_end = payment_source.split("_").last().unwrap();
            assert!(addresses.get(1).unwrap().to_string().ends_with(payment_source_end));

            let change_source_end = change_source.split("_").last().unwrap();
            assert!(addresses.get(0).unwrap().to_string().ends_with(change_source_end));

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn payments_work_for_insufficient_funds() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 119,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            let payment_err = payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap_err();
            assert_eq!(payment_err.error_code, ErrorCode::PaymentInsufficientFundsError);

            let sources_after = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);
            assert_eq!(sources, sources_after);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn payments_work_for_spent_source() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap();

            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            let ec = payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap_err();

            assert_eq!(ec.error_code, ErrorCode::PaymentSourceDoesNotExistError);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn payment_request_works_for_source_not_correspond_to_wallet() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let wallet_handle_2 = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //2. Mint sources and get created sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            //3. Prepare inputs and outputs for payment txn
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //4. Build and send payment txn
            let res = payments::build_payment_req(wallet_handle_2, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None);
            assert_eq!(res.unwrap_err().error_code, ErrorCode::CommonInvalidState);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            wallet::close_wallet(wallet_handle_2).unwrap();
            test_utils::tear_down();
        }
    }

    mod fees {
        use super::*;

        #[test]
        pub fn set_request_fees_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let fees_req = payments::build_set_txn_fees_req(wallet_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES).unwrap();
            ledger::submit_request(pool_handle, fees_req.as_str()).unwrap();

            let fees_stored = payments_utils::get_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME);

            let fee_1 = fees_stored.get("1").unwrap();
            assert_eq!(fee_1, &1);
            let fee_2 = fees_stored.get("101").unwrap();
            assert_eq!(fee_2, &2);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn get_request_fees_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES);

            let req = payments::build_get_txn_fees_req(wallet_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME).unwrap();
            let resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();
            let resp = payments::parse_get_txn_fees_response(PAYMENT_METHOD_NAME, resp.as_str()).unwrap();
            let map = serde_json::from_str::<HashMap<String, i32>>(resp.as_str()).unwrap();

            let fee_1 = map.get("1").unwrap();
            assert_eq!(fee_1, &1);
            let fee_2 = map.get("101").unwrap();
            assert_eq!(fee_2, &2);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod verify_payment {
        use super::*;

        #[test]
        pub fn verify_payment_works() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            //1. Create addresses
            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);

            //2. Mint sources and get created sources
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, ((i + 2) * 10) as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            //3. Prepare inputs and outputs for payment txn
            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let recipient_1 = addresses.get(1).unwrap().to_string();
            let recipient_2 = addresses.get(0).unwrap().to_string();
            let outputs = vec![Output {
                recipient: recipient_1.clone(),
                amount: 19,
            }, Output {
                recipient: recipient_2.clone(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            //4. Build and send payment txn
            let (payment_req, payment_method) = payments::build_payment_req(wallet_handle, SUBMITTER_DID, inputs.as_str(), outputs.as_str(), None).unwrap();
            let payment_resp = ledger::submit_request(pool_handle, payment_req.as_str()).unwrap();
            let payment_resp_parsed = payments::parse_payment_response(payment_method.as_str(), payment_resp.as_str()).unwrap();

            //5. Check response sources
            let sources: Vec<ReceiptInfo> = serde_json::from_str(payment_resp_parsed.as_str()).unwrap();
            assert_eq!(sources.len(), 2);

            //6. Verify receipts
            let receipt_1 = sources[0].receipt.clone();
            let receipt_2 = sources[1].receipt.clone();

            let expected_info = ReceiptVerificationInfo {
                sources: sources_1,
                receipts: vec![ShortReceiptInfo {
                    receipt: receipt_1.clone(),
                    recipient: recipient_1.clone(),
                    amount: 19,
                }, ShortReceiptInfo {
                    receipt: receipt_2.clone(),
                    recipient: recipient_2.clone(),
                    amount: 1,
                }],
                extra: None,
            };

            // Verify receipt 1
            let (verify_txn_json, payment_method) = payments::build_verify_payment_req(wallet_handle, SUBMITTER_DID, receipt_1.as_str()).unwrap();
            let verify_txn_resp = ledger::submit_request(pool_handle, verify_txn_json.as_str()).unwrap();
            let verification_info = payments::parse_verify_payment_response(payment_method.as_str(), verify_txn_resp.as_str()).unwrap();

            let info: ReceiptVerificationInfo = serde_json::from_str(verification_info.as_str()).unwrap();
            assert_eq!(info, expected_info);

            // Verify receipt 2
            let (verify_txn_json, payment_method) = payments::build_verify_payment_req(wallet_handle, SUBMITTER_DID, receipt_2.as_str()).unwrap();
            let verify_txn_resp = ledger::submit_request(pool_handle, verify_txn_json.as_str()).unwrap();
            let verification_info = payments::parse_verify_payment_response(payment_method.as_str(), verify_txn_resp.as_str()).unwrap();

            let info: ReceiptVerificationInfo = serde_json::from_str(verification_info.as_str()).unwrap();
            assert_eq!(info, expected_info);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }
}

mod medium_cases {
    use super::*;

    mod add_request_fees {
        use super::*;

        #[test]
        pub fn add_request_fees_works_for_request_without_operation() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, i as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let nym_req_err = payments::add_request_fees(wallet_handle, SUBMITTER_DID, EMPTY_OBJECT, inputs.as_str(), outputs.as_str(), None).unwrap_err();
            assert_eq!(nym_req_err.error_code, ErrorCode::CommonInvalidStructure);

            //IMPORTANT: check that source cache stays the same
            let sources_after = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);
            assert_eq!(sources, sources_after);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }

        #[test]
        pub fn add_request_fees_works_for_request_without_req_id() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let addresses = payments_utils::create_addresses(vec!["{}", "{}"], wallet_handle, PAYMENT_METHOD_NAME);
            let mint: Vec<(String, i32)> = addresses.clone().into_iter().enumerate().map(|(i, addr)| (addr, i as i32)).collect();
            payments_utils::mint_sources(mint, None, wallet_handle, pool_handle, SUBMITTER_DID);

            payments_utils::set_request_fees(wallet_handle, pool_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, FEES);

            let sources = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);

            let addr_1 = addresses.get(0).unwrap();
            let sources_1: Vec<String> = sources.get(addr_1.as_str()).unwrap().into_iter().map(|info| info.source.clone()).collect();
            let inputs = serde_json::to_string(&sources_1).unwrap();

            let outputs = vec![Output {
                recipient: addresses.get(1).unwrap().to_string(),
                amount: 19,
            }, Output {
                recipient: addresses.get(0).unwrap().to_string(),
                amount: 1,
            }];
            let outputs = serde_json::to_string(&outputs).unwrap();

            let nym_req_err = payments::add_request_fees(wallet_handle, SUBMITTER_DID, r#"{"reqId": 111}"#, inputs.as_str(), outputs.as_str(), None).unwrap_err();
            assert_eq!(nym_req_err.error_code, ErrorCode::CommonInvalidStructure);

            //IMPORTANT: check that source cache stays the same
            let sources_after = payments_utils::get_sources_with_balance(addresses.clone(), wallet_handle, pool_handle, SUBMITTER_DID);
            assert_eq!(sources, sources_after);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod fees {
        use super::*;

        #[test]
        pub fn build_set_txn_fees_works_for_invalid_json() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let err = payments::build_set_txn_fees_req(wallet_handle, SUBMITTER_DID, PAYMENT_METHOD_NAME, "{1}").unwrap_err();
            assert_eq!(err.error_code, ErrorCode::CommonInvalidStructure);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod verify_payment {
        use super::*;

        #[test]
        pub fn verify_payment_works_for_not_found_receipt() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            // Verify receipt
            let receipt = "pay:null:0_PqVjwJC42sxCTJp";
            let (verify_txn_json, payment_method) = payments::build_verify_payment_req(wallet_handle, SUBMITTER_DID, receipt).unwrap();
            let verify_txn_resp = ledger::submit_request(pool_handle, verify_txn_json.as_str()).unwrap();
            let res = payments::parse_verify_payment_response(payment_method.as_str(), verify_txn_resp.as_str());
            assert_eq!(res.unwrap_err().error_code, ErrorCode::PaymentSourceDoesNotExistError);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }

    mod parse_payment_response {
        use super::*;

        #[test]
        pub fn parse_response_with_fees_works_for_response_without_fees() {
            test_utils::setup();
            plugin::init_plugin();
            let wallet_handle = wallet::create_and_open_wallet().unwrap();
            let pool_handle = pool::create_and_open_pool_ledger(POOL_NAME).unwrap();

            let (trustee_did, _) = did::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
            let (my_did, my_vk) = did::create_and_store_my_did(wallet_handle, None).unwrap();
            let nym_req = ledger::build_nym_request(&trustee_did, &my_did, &my_vk, "aaa", "TRUSTEE").unwrap();

            let response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym_req).unwrap();
            let parsed_response = payments::parse_payment_response(PAYMENT_METHOD_NAME, response.as_str()).unwrap();
            assert_eq!("{}", parsed_response);

            pool::close(pool_handle).unwrap();
            wallet::close_wallet(wallet_handle).unwrap();
            test_utils::tear_down();
        }
    }
}