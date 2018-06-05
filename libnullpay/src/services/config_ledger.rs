use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref FEES: Mutex<HashMap<String, i32>> = Default::default();
}

pub fn set_fees(txn_name: String, txn_fee: i32) {
    let mut fees = FEES.lock().unwrap();
    fees.insert(txn_name, txn_fee);
}

pub fn get_fee(txn_name: String) -> Option<i32> {
    let fees = FEES.lock().unwrap();
    fees.get(&txn_name).map(|res| res.clone())
}

pub fn get_all_fees() -> HashMap<String, i32> {
    let fees = FEES.lock().unwrap();
    fees.clone()
}

pub fn clear_fees() {
    let mut fees = FEES.lock().unwrap();
    fees.clear();
}