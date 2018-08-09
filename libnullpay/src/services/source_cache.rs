use std::collections::HashMap;
use std::sync::Mutex;
use utils::source::to_source;
use utils::source::from_source;

lazy_static! {
    static ref SOURCES: Mutex<HashMap<String, Vec<String>>> = Default::default();
    static ref BALANCES: Mutex<HashMap<String, u64>> = Default::default();
}

pub fn get_sources_by_payment_address(payment_address: &str) -> Vec<String> {
    let sources = SOURCES.lock().unwrap();
    match sources.get(payment_address) {
        Some(v) => v.clone(),
        None => Vec::new()
    }
}

pub fn get_balance_of_source(source: &String) -> Option<u64> {
    let balances = BALANCES.lock().unwrap();
    balances.get(source).map(|a| a.clone())
}

pub fn add_source(payment_address: &str, seq_no: i32, balance: u64) -> Option<String> {
    to_source(payment_address, seq_no).map(|source| {
        let mut balances = BALANCES.lock().unwrap();
        let mut sources = SOURCES.lock().unwrap();
        balances.insert(source.clone(), balance);
        let vec = sources.remove(payment_address);
        let vec = match vec {
            Some(v) => {
                let mut v = Vec::from(v);
                v.push(source.clone());
                v
            }
            None => vec![source.clone()]
        };
        sources.insert(payment_address.to_string(), vec);
        source
    })
}

pub fn remove_source(source: &str) {
    let res = from_source(source);
    match res {
        Some((_,payment_address)) => {
            let mut balances = BALANCES.lock().unwrap();
            let mut sources = SOURCES.lock().unwrap();
            balances.remove(source);
            match sources.remove(&payment_address)
                .map(|vs|
                    vs.into_iter()
                        .filter(|v| v != source)
                        .collect::<Vec<String>>()
                ) {
                Some(ref v) if !v.is_empty() => {sources.insert(payment_address, v.to_vec());},
                _ => ()
            };
        },
        None => ()
    };
}