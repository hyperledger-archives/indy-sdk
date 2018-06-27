use utils::types::{UTXOOutput, UTXOInfo};
use utils::utxo::from_utxo;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;

lazy_static! {
    static ref TXNS: Mutex<HashMap<i32, (Vec<String>, Vec<UTXOOutput>)>> = Default::default();
}

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

fn _next_seq_no() -> i32 {
    (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}

pub fn add_txn(inputs: Vec<String>, outputs: Vec<UTXOOutput>) -> i32 {
    let mut txns = TXNS.lock().unwrap();
    let next_seq_no = _next_seq_no();
    txns.insert(next_seq_no, (inputs, outputs));
    next_seq_no
}

pub fn get_txn(seq_no: i32) -> Option<(Vec<String>, Vec<UTXOOutput>)> {
    let txns = TXNS.lock().unwrap();
    txns.get(&seq_no).map(|&(ref a, ref b)| (a.clone(), b.clone()))
}

pub fn get_utxo_info(utxo: String) -> Option<UTXOInfo> {
    let (seq_no, payment_address) = match from_utxo(utxo.as_str()) {
        Some(e) => e,
        None => return None
    };

    match get_txn(seq_no).map(|(_, outputs)| {
        outputs.into_iter().find(|out| out.payment_address == payment_address).map(|out| {
            UTXOInfo {
                txo: utxo,
                payment_address,
                amount: out.amount,
                extra: out.extra,
            }
        })
    }) {
        Some(Some(o)) => Some(o),
        _ => None
    }
}