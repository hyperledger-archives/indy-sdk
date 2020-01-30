use utils::types::{SourceInfo, Output};
use utils::payments;

use serde_json;
use utils::ledger;
use std::collections::HashMap;

use indy::WalletHandle;
use indy::PoolHandle;

pub fn create_addresses(cfgs: Vec<&str>, wallet_handle: WalletHandle, payment_method: &str) -> Vec<String> {
    cfgs.into_iter().map(|cfg| {
        payments::create_payment_address(wallet_handle, payment_method, cfg).unwrap()
    }).collect()
}

pub fn mint_sources(addresses: Vec<(String, i32)>, extra: Option<&str>, wallet_handle: WalletHandle, pool_handle: PoolHandle, submitter_did: &str) {
    let mint: Vec<Output> = addresses.into_iter().map(|(recipient, amount)| {
        Output {
            recipient,
            amount,
        }
    }).collect();

    let outputs = serde_json::to_string(&mint).unwrap();

    let (req, _) = payments::build_mint_req(wallet_handle, submitter_did, outputs.as_str(), extra).unwrap();
    ledger::submit_request(pool_handle, req.as_str()).unwrap();
}

pub fn get_sources_with_balance(payment_addresses: Vec<String>, wallet_handle: WalletHandle, pool_handle: PoolHandle, submitter_did: &str) -> HashMap<String, Vec<SourceInfo>> {
    payment_addresses.into_iter().map(|addr| {
        let (req_sources, payment_method) = payments::build_get_payment_sources_request(wallet_handle, submitter_did, addr.as_str()).unwrap();
        let resp_sources = ledger::submit_request(pool_handle, req_sources.as_str()).unwrap();
        let resp_sources = payments::parse_get_payment_sources_response(payment_method.as_str(), resp_sources.as_str()).unwrap();

        (addr.to_string(), serde_json::from_str::<Vec<SourceInfo>>(resp_sources.as_str()).unwrap())
    }).collect()
}

pub fn set_request_fees(wallet_handle: WalletHandle, pool_handle: PoolHandle, submitter_did: &str, payment_method: &str, fees: &str) {
    let req = payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, fees).unwrap();
    ledger::submit_request(pool_handle, req.as_str()).unwrap();
}

pub fn get_request_fees(wallet_handle: WalletHandle, pool_handle: PoolHandle, submitter_did: &str, payment_method: &str) -> HashMap<String, i32> {
    let req = payments::build_get_txn_fees_req(wallet_handle, submitter_did, payment_method).unwrap();
    let resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();
    let resp = payments::parse_get_payment_sources_response(payment_method, resp.as_str()).unwrap();
    serde_json::from_str::<HashMap<String, i32>>(resp.as_str()).unwrap()
}