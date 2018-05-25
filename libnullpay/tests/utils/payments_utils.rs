use utils::types::{UTXOInfo, UTXOOutput};
use utils::payments;

use serde_json;
use utils::ledger;
use std::collections::HashMap;

pub fn create_addresses(cfgs: Vec<&str>, wallet_handle: i32, payment_method: &str) -> Vec<String> {
    cfgs.into_iter().map(|cfg| {
        payments::create_payment_address(wallet_handle, payment_method, cfg).unwrap()
    }).collect()
}

pub fn mint_tokens(addresses: Vec<(String, i32, Option<&str>)>, wallet_handle: i32, pool_handle: i32, submitter_did: &str){
    let mint: Vec<UTXOOutput> = addresses.into_iter().map(|(payment_address, amount, extra)| {
        UTXOOutput {
            payment_address,
            amount,
            extra: extra.map(|s| s.to_string()),
        }
    }).collect();

    let outputs = serde_json::to_string(&mint).unwrap();

    let (req, _) = payments::build_mint_req(wallet_handle, submitter_did, outputs.as_str()).unwrap();
    ledger::submit_request(pool_handle, req.as_str()).unwrap();
}

pub fn get_utxos_with_balance(payment_addresses: Vec<String>, wallet_handle: i32, pool_handle: i32, submitter_did: &str) -> HashMap<String, Vec<UTXOInfo>> {
    payment_addresses.into_iter().map(|addr| {
        let (req_utxo, payment_method) = payments::build_get_utxo_request(wallet_handle, submitter_did, addr.as_str()).unwrap();
        let resp_utxo = ledger::submit_request(pool_handle, req_utxo.as_str()).unwrap();
        let resp_utxo = payments::parse_get_utxo_response(payment_method.as_str(), resp_utxo.as_str()).unwrap();

        (addr.to_string(), serde_json::from_str::<Vec<UTXOInfo>>(resp_utxo.as_str()).unwrap())
    }).collect()
}

pub fn set_request_fees(wallet_handle: i32, pool_handle: i32, submitter_did: &str, payment_method: &str, fees: &str) {
    let req = payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, fees).unwrap();
    ledger::submit_request(pool_handle, req.as_str()).unwrap();
}

pub fn get_request_fees(wallet_handle: i32, pool_handle: i32, submitter_did: &str, payment_method: &str) -> HashMap<String, i32> {
    let req = payments::build_get_txn_fees_req(wallet_handle, submitter_did, payment_method).unwrap();
    let resp = ledger::submit_request(pool_handle, req.as_str()).unwrap();
    let resp = payments::parse_get_utxo_response(payment_method, resp.as_str()).unwrap();
    serde_json::from_str::<HashMap<String, i32>>(resp.as_str()).unwrap()
}