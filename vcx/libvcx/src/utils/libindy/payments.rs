extern crate libc;
extern crate serde_json;

use utils::libindy::wallet::get_wallet_handle;
use utils::libindy::ledger::libindy_sign_and_submit_request;
use utils::error;
use indy::payments::Payment;
use std::sync::{Once, ONCE_INIT};
use serde_json::Value;
use settings;

static NULL_PAYMENT: &str = "null";
static EMPTY_CONFIG: &str = "{}";
static FEES: &str = r#"{"1":1, "101":2, "102":44}"#;

static PAYMENT_INIT: Once = ONCE_INIT;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo {
    balance: u32,
    addresses: Vec<AddressInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressInfo {
    address: String,
    utxo: Vec<UTXO>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UTXO {
    input: String,
    amount: u32,
    extra: String,
}

/// libnullpay
#[cfg(feature = "nullpay")]
extern { fn nullpay_init() -> i32; }

#[cfg(feature = "no_payments")]
unsafe fn nullpay_init() -> i32 { 0 }

pub fn init_payments() -> Result<(), u32> {
    let mut rc = 0;

    PAYMENT_INIT.call_once(|| {
        unsafe { rc = nullpay_init(); }
    });

    if rc != 0 {
        Err(rc as u32)
    } else {
        Ok(())
    }
}

pub fn create_address() -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(r#"pay:null:J81AxU9hVHYFtJc"#.to_string()); }

    match Payment::create_payment_address(get_wallet_handle() as i32, NULL_PAYMENT, EMPTY_CONFIG) {
        Ok(x) => Ok(x),
        Err(x) => Err(x as u32),
    }
}

fn get_address_info(address: &str) -> Result<AddressInfo, u32> {
    if settings::test_indy_mode_enabled() {
        let utxo: Vec<UTXO> = serde_json::from_str(r#"[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]"#).unwrap();
        return Ok(AddressInfo { address: address.to_string(), utxo})
    }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let (txn, _) = match Payment::build_get_utxo_request(get_wallet_handle() as i32, &did, address) {
        Ok(x) => x,
        Err(x) => return Err(x as u32),
    };

    let response = libindy_sign_and_submit_request(&did, &txn)?;

    let response = match Payment::parse_get_utxo_response(NULL_PAYMENT, &response) {
        Ok(x) => x,
        Err(x) => return Err(x as u32),
    };

    trace!("indy_parse_get_utxo_response() --> {}", response);
    let utxo: Vec<UTXO> = match serde_json::from_str(&response) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    Ok(AddressInfo { address: address.to_string(), utxo })
}

pub fn list_addresses() -> Result<Vec<String>, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(serde_json::from_str(r#"["pay:null:9UFgyjuJxi1i1HD","pay:null:zR3GN9lfbCVtHjp"]"#).unwrap());
    }

    let addresses = match Payment::list_payment_addresses(get_wallet_handle() as i32) {
        Ok(x) => x,
        Err(x) => return Err(x as u32),
    };

    trace!("--> {}", addresses);
    let addresses: Value = match serde_json::from_str(&addresses) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };
    match addresses.as_array() {
        None => Err(error::INVALID_JSON.code_num),
        Some(x) => Ok(x.into_iter().map(|address|address.as_str().unwrap().to_string()).collect()),
    }
}

pub fn get_wallet_token_info() -> Result<String, u32> {
    let addresses = list_addresses()?;

    let mut balance = 0;
    let mut wallet_info = Vec::new();

    for address in addresses.iter() {
        let mut info = get_address_info(&address)?;

        for utxo in info.utxo.iter() {
            balance += utxo.amount as u32;
        }

        wallet_info.push(info);
    }

    let wallet_info = WalletInfo {balance, addresses: wallet_info};
    Ok(serde_json::to_string(&wallet_info).unwrap())
}

pub fn get_ledger_fees() -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(FEES.to_string()); }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let response = match Payment::build_get_txn_fees_req(get_wallet_handle() as i32, &did, NULL_PAYMENT) {
        Ok(txn) => libindy_sign_and_submit_request(&did, &txn)?,
        Err(x) => return Err(x as u32),
    };

    match Payment::parse_get_txn_fees_response(NULL_PAYMENT, &response) {
        Ok(x) => Ok(x),
        Err(x) => Err(x as u32),
    }
}

pub fn pay_for_txn(req: &str, txn_type: &str) -> Result<String, u32> {
    // Find cost for txn_type
    // gen inputs and outputs
    // indy_add_request_fees
    // sign_and_submit
    // parse response to make sure its valid
    // return response
    Ok("{}".to_string())
}

fn get_txn_cost(txn_type: &str) -> Result<u32, u32> {
    let ledger_fees = get_ledger_fees()?;

    let fees: ::std::collections::HashMap<String, u32> = serde_json::from_str(&ledger_fees).unwrap();

    Ok(fees.get(txn_type).ok_or(error::UNKNOWN_TXN_TYPE.code_num)?.clone())
}

fn _address_balance(address: &Vec<UTXO>) -> u32 {
    address.iter().fold(0, |balance, utxo| balance + utxo.amount) as u32
}

pub fn inputs(cost: u32) -> Result<(u32, String), u32> {
    let mut inputs: Vec<String> = Vec::new();
    let mut balance = 0;

    let wallet_info: WalletInfo = serde_json::from_str(&get_wallet_token_info()?)
        .or(Err(error::INVALID_JSON.code_num))?;

    if wallet_info.balance < cost {
        warn!("not enough tokens in wallet to pay");
        return Err(error::INSUFFICIENT_TOKEN_AMOUNT.code_num);
    }

    // Todo: explore 'smarter' ways of selecting utxos ie bitcoin algorithms etc
    'outer: for address in wallet_info.addresses.iter() {
        'inner: for utxo in address.utxo.iter() {
            if balance < cost {
                inputs.push(utxo.input.to_string());
                balance += utxo.amount;
            } else { break 'outer }
        }
    }

    let remainder = balance - cost;

    Ok((remainder, serde_json::to_string(&inputs).or(Err(error::INVALID_JSON.code_num))?))
}

pub fn outputs(amount: u32) -> Result<String, u32> {
    // In the future we might provide a way for users to specify multiple output address
    // As of now, we only handle one output address which we create
    Ok(json!([
        {
            "paymentAddress": create_address()?,
            "amount": amount,
            "extra": null
        }
    ]).to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use settings;

    pub fn set_ledger_fees() -> Result<(), u32> {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        match Payment::build_set_txn_fees_req(get_wallet_handle() as i32, &did, NULL_PAYMENT, FEES) {
            Ok(txn) => match libindy_sign_and_submit_request(&did, &txn) {
                Ok(_) => Ok(()),
                Err(x) => Err(x),
            },
            Err(x) => Err(x as u32),
        }
    }

    #[test]
    fn test_init_payments() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        init_payments().unwrap();
    }

    #[test]
    fn test_create_address() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        init_payments().unwrap();
        create_address().unwrap();
    }

    #[test]
    fn test_get_addresses() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        init_payments().unwrap();
        create_address().unwrap();
        let addresses = list_addresses().unwrap();
    }

    #[test]
    fn test_get_wallet_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        init_payments().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert_eq!(balance, r#"{"balance":6,"addresses":[{"address":"pay:null:9UFgyjuJxi1i1HD","utxo":[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]},{"address":"pay:null:zR3GN9lfbCVtHjp","utxo":[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]}]}"#);
    }

    #[cfg(feature = "nullpay")]
    #[test]
    fn test_get_wallet_token_info_real() {
        let name = "test_get_wallet_info_real";
        ::utils::devsetup::setup_dev_env(name);
        init_payments().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert!(balance.contains(r#""balance":0"#));
        ::utils::devsetup::cleanup_dev_env(name);
    }

    #[test]
    fn test_get_ledger_fees() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let fees = get_ledger_fees().unwrap();
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":1"#));
    }

    #[cfg(feature = "nullpay")]
    #[test]
    fn test_get_ledger_fees_real() {
        let name = "test_get_ledger_fees_real";
        ::utils::devsetup::setup_dev_env(name);
        init_payments().unwrap();
        set_ledger_fees().unwrap();
        let fees = get_ledger_fees().unwrap();
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":1"#));
        ::utils::devsetup::cleanup_dev_env(name);
    }

    #[test]
    fn test_address_balance() {
        let addresses = vec![
            UTXO { input: "pov::null:2".to_string(), amount: 2, extra: "abcde".to_string() },
            UTXO { input: "pov::null:3".to_string(), amount: 3, extra: "bcdef".to_string() }
        ];

        assert_eq!(_address_balance(&addresses), 5);
    }

    #[test]
    fn test_inputs() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        // Success - Exact amount
        assert_eq!(inputs(6).unwrap(), (0, r#"["pov:null:1","pov:null:2","pov:null:1","pov:null:2"]"#.to_string()));

        // Success - utxo with remainder tokens
        assert_eq!(inputs(5).unwrap(), (1, r#"["pov:null:1","pov:null:2","pov:null:1","pov:null:2"]"#.to_string()));

        // Success - requesting amount that partial address (1 of 2 utxos) can satisfy
        assert_eq!(inputs(1).unwrap(), (0, r#"["pov:null:1"]"#.to_string()));

        // Err - request more than wallet contains
        assert_eq!(inputs(7), Err(error::INSUFFICIENT_TOKEN_AMOUNT.code_num));
    }

    #[test]
    fn test_select_outputs() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let cost = 6;
        let expected_output = r#"[{"amount":0,"extra":null,"paymentAddress":"pay:null:J81AxU9hVHYFtJc"}]"#;
        let (remainder, _) = inputs(cost).unwrap();
        assert_eq!(&outputs(remainder).unwrap(), expected_output);
    }

    #[test]
    fn test_get_txn_cost() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        assert_eq!(get_txn_cost("101").unwrap(), 2);
        assert_eq!(get_txn_cost("102").unwrap(), 44);
        assert_eq!(get_txn_cost("Unknow txn type"), Err(error::UNKNOWN_TXN_TYPE.code_num));
    }

    #[test]
    fn test_pay_for_txn() {
        // Schema
        let create_schema_req = ::utils::constants::SCHEMA_CREATE_JSON.to_string();
        let price_req = pay_for_txn(&create_schema_req, "101").unwrap();
    }
}
