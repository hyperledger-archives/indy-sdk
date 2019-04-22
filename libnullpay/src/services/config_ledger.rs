use std::collections::HashMap;
use std::sync::Mutex;

const NYM: &'static str = "1";
const ATTRIB: &'static str = "100";
const SCHEMA: &'static str = "101";
const CRED_DEF: &'static str = "102";
const REV_REG_DEF: &'static str = "113";
const REV_REG_DELTA: &'static str = "114";

lazy_static! {
    static ref FEES: Mutex<HashMap<String, u64>> = Default::default();

    static ref DEFAULT_FEES: HashMap<String, u64> = {
        let mut map = HashMap::new();
        map.insert(NYM.to_string(), 10);
        map.insert(ATTRIB.to_string(), 5);
        map.insert(SCHEMA.to_string(), 200);
        map.insert(CRED_DEF.to_string(), 100);
        map.insert(REV_REG_DEF.to_string(), 20);
        map.insert(REV_REG_DELTA.to_string(), 20);
        map
    };
}

pub fn set_fees(enable_fees: String) -> () {
    if enable_fees == "1" {
        let mut map = FEES.lock().unwrap();
        *map = DEFAULT_FEES.clone();
    }
}

pub fn get_fee(txn_name: &str) -> Option<u64> {
    let fees = FEES.lock().unwrap();
    fees.get(txn_name).map(|res| res.clone())
}

pub fn get_all_fees() -> HashMap<String, u64> {
    let fees = FEES.lock().unwrap();
    let fees: HashMap<String, u64> = fees.clone();
    fees
}