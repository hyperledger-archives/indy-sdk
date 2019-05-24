use utils::types::{Output, ReceiptInfo, SourceInfo, ReceiptVerificationInfo, ShortReceiptInfo};
use utils::source::{from_source, to_source};

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

lazy_static! {
    static ref TXNS: Mutex<HashMap<i32, (Vec<String>, Vec<Output>, Option<String>)>> = Default::default();
}

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = AtomicUsize::new(1);
}

fn _next_seq_no() -> i32 {
    (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}

pub fn add_txn(inputs: Vec<String>, outputs: Vec<Output>, extra: Option<&str>) -> i32 {
    let mut txns = TXNS.lock().unwrap();
    let next_seq_no = _next_seq_no();
    txns.insert(next_seq_no, (inputs, outputs, extra.map(String::from)));
    next_seq_no
}

pub fn get_txn(seq_no: i32) -> Option<(Vec<String>, Vec<Output>, Option<String>)> {
    let txns = TXNS.lock().unwrap();
    txns.get(&seq_no).map(|&(ref a, ref b, ref c)| (a.clone(), b.clone(), c.clone()))
}

pub fn get_receipt_verification_info(source: String) -> Option<ReceiptVerificationInfo> {
    let (seq_no, _) = match from_source(source.as_str()) {
        Some(e) => e,
        None => return None
    };

    match get_txn(seq_no).map(|(sources, outputs, extra)|
        {
            let receipts: Vec<ShortReceiptInfo> =
                outputs.iter().map(|output|
                    ShortReceiptInfo {
                        receipt: to_source(&output.recipient, seq_no).unwrap(), // TODO: FIXME
                        recipient: output.recipient.clone(),
                        amount: output.amount.clone(),
                    }
                ).collect();

            ReceiptVerificationInfo {
                sources,
                receipts,
                extra,
            }
        }) {
        Some(o) => Some(o),
        _ => None
    }
}

pub fn get_receipt_info(source: String) -> Option<ReceiptInfo> {
    let (seq_no, recipient) = match from_source(source.as_str()) {
        Some(e) => e,
        None => return None
    };

    match get_txn(seq_no).map(|(_, outputs, extra)| {
        outputs.into_iter().find(|out| out.recipient == recipient).map(|out| {
            ReceiptInfo {
                receipt: source,
                recipient,
                amount: out.amount,
                extra,
            }
        })
    }) {
        Some(Some(o)) => Some(o),
        _ => None
    }
}

pub fn get_source_info(source: String) -> Option<SourceInfo> {
    let (seq_no, recipient) = match from_source(source.as_str()) {
        Some(e) => e,
        None => return None
    };

    match get_txn(seq_no).map(|(_, outputs, extra)| {
        outputs.into_iter().find(|out| out.recipient == recipient).map(|out| {
            SourceInfo {
                source,
                payment_address: recipient,
                amount: out.amount,
                extra,
            }
        })
    }) {
        Some(Some(o)) => Some(o),
        _ => None
    }
}
