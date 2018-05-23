use std::collections::HashMap;
use std::sync::Mutex;
use utils::utxo::to_utxo;
use utils::utxo::from_utxo;

lazy_static! {
    static ref UTXOS: Mutex<HashMap<String, Vec<String>>> = Default::default();
    static ref BALANCES: Mutex<HashMap<String, i32>> = Default::default();
}

pub fn get_utxos_by_payment_address(payment_address: &str) -> Vec<String> {
    let utxos = UTXOS.lock().unwrap();
    match utxos.get(payment_address) {
        Some(v) => v.clone(),
        None => Vec::new()
    }
}

pub fn get_balanse_of_utxo(utxo: &String) -> Option<i32> {
    let balances = BALANCES.lock().unwrap();
    balances.get(utxo).map(|a| a.clone())
}

pub fn add_utxo(payment_address: &str, seq_no: i32, balance: i32) -> Option<String> {
    to_utxo(payment_address, seq_no).map(|utxo| {
        let mut balances = BALANCES.lock().unwrap();
        let mut utxos = UTXOS.lock().unwrap();
        balances.insert(utxo.clone(), balance);
        let vec = utxos.remove(payment_address);
        let vec = match vec {
            Some(v) => {
                let mut v = Vec::from(v);
                v.push(utxo.clone());
                v
            }
            None => vec![utxo.clone()]
        };
        utxos.insert(payment_address.to_string(), vec);
        utxo
    })
}

pub fn remove_utxo(utxo: &str) {
    let res = from_utxo(utxo);
    match res {
        Some((_,payment_address)) => {
            let mut balances = BALANCES.lock().unwrap();
            let mut utxos = UTXOS.lock().unwrap();
            balances.remove(utxo);
            match utxos.remove(&payment_address)
                .map(|vs|
                    vs.into_iter()
                        .filter(|v| v != utxo)
                        .collect::<Vec<String>>()
                ) {
                Some(ref v) if !v.is_empty() => {utxos.insert(payment_address, v.to_vec());},
                _ => ()
            };
        },
        None => ()
    };
}