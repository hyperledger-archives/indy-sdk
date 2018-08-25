use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref FEES: Mutex<HashMap<String, u64>> = Default::default();
}

const NYM: &'static str = "1";
const ATTRIB: &'static str = "100";
const SCHEMA: &'static str = "101";
const CRED_DEF: &'static str = "102";

pub fn set_fees(txn_name: String, txn_fee: u64) {
    let mut fees = FEES.lock().unwrap();
    fees.insert(_txn_name_to_code(&txn_name), txn_fee);
}

pub fn get_fee(txn_name: String) -> Option<u64> {
    let fees = FEES.lock().unwrap();
    fees.get(&_txn_name_to_code(&txn_name)).map(|res| res.clone())
}

pub fn get_all_fees() -> HashMap<String, u64> {
    let fees = FEES.lock().unwrap();
    let fees: HashMap<String, u64> = fees.clone();
    fees
}

#[allow(dead_code)]
pub fn clear_fees() {
    let mut fees = FEES.lock().unwrap();
    fees.clear();
}

fn _txn_name_to_code(txn: &str) -> String {
    match txn {
        "NYM" => NYM.to_string(),
        "ATTRIB" => ATTRIB.to_string(),
        "SCHEMA" => SCHEMA.to_string(),
        "CRED_DEF" => CRED_DEF.to_string(),
        val @ _ => val.to_string()
    }
}

fn _txn_code_to_name(txn: &str) -> String {
    match txn {
        NYM => "NYM".to_string(),
        ATTRIB => "ATTRIB".to_string(),
        SCHEMA => "SCHEMA".to_string(),
        CRED_DEF => "CRED_DEF".to_string(),
        val @ _ => val.to_string()
    }
}