extern crate libc;
extern crate serde_json;

use utils::libindy::wallet::get_wallet_handle;
use utils::constants::{ SUBMIT_SCHEMA_RESPONSE };
use utils::libindy::ledger::libindy_sign_and_submit_request;
use utils::error;
use indy::payments::Payment;
use std::sync::{Once, ONCE_INIT};
use std::collections::HashMap;
use serde_json::Value;
use settings;

static NULL_PAYMENT: &str = "null";
static EMPTY_CONFIG: &str = "{}";
static FEES: &str = r#"{"1":1, "101":2, "102":42, "9999":1999998889}"#;
static PARSED_TXN_PAYMENT_RESPONSE: &str = r#"[{"amount":4,"extra":null,"input":"["pov:null:1","pov:null:2"]"}]"#;

static PAYMENT_INIT: Once = ONCE_INIT;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo {
    balance: u64,
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
    amount: u64,
    extra: Option<String>,
}

//#[derive(Serialize, Deserialize, Debug)]
//struct OUTPUT {
//    payment_address: String,
//    amount: u64,
//    extra: Option<String>,
//}

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

        for utxo in info.utxo.iter() { balance += utxo.amount as u64; }

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

pub fn pay_for_txn(req: &str, txn_type: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((PARSED_TXN_PAYMENT_RESPONSE.to_string(), SUBMIT_SCHEMA_RESPONSE.to_string())); }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let txn_price = get_txn_price(txn_type)?;

    let (refund, inputs) = inputs(txn_price)?;

    let output = outputs(refund, None, None)?;

    let (response, payment_method) = match Payment::add_request_fees(get_wallet_handle(),
                                                   &did,
                                                   req,
                                                   &inputs,
                                                   &output) {
        Ok((req, payment_method)) => (libindy_sign_and_submit_request(&did, &req)?, payment_method),
        Err(x) => return Err(x as u32),
    };

    // Todo: Handle libindy error
    let parsed_response = Payment::parse_response_with_fees(NULL_PAYMENT, &response).map_err(|err| err as u32)?;
    Ok((parsed_response, response))
}

fn get_txn_price(txn_type: &str) -> Result<u64, u32> {
    let ledger_fees = get_ledger_fees()?;

    let fees: HashMap<String, u64> = serde_json::from_str(&ledger_fees) .or(Err(error::INVALID_JSON.code_num))?;

    Ok(fees.get(txn_type).ok_or(error::UNKNOWN_TXN_TYPE.code_num)?.clone())
}

fn _address_balance(address: &Vec<UTXO>) -> u32 {
    address.iter().fold(0, |balance, utxo| balance + utxo.amount) as u32
}

pub fn inputs(cost: u64) -> Result<(u64, String), u32> {
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

pub fn outputs(remainder: u64, payee_address: Option<String>, payee_amount: Option<u64>) -> Result<String, u32> {
    // In the future we might provide a way for users to specify multiple output address for their remainder tokens
    // As of now, we only handle one output address which we create
    //Todo: create a struct for outputs?

    let mut outputs = Vec::new();

    outputs.push(json!({ "paymentAddress": create_address()?, "amount": remainder, "extra": null }));

    if let Some(address) = payee_address {
        outputs.push(json!({
            "paymentAddress": address,
            "amount": payee_amount,
            "extra": null
        }));
    }

    Ok(serde_json::to_string(&outputs).unwrap())
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

    pub fn mint_tokens() -> Result<(), u32> {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let addresses = vec![create_address().unwrap(), create_address().unwrap(), create_address().unwrap()];

        let mint: Vec<Value> = addresses.clone().into_iter().enumerate().map(|(i, payment_address)|
            json!( { "paymentAddress": payment_address, "amount": 15, "extra": null } )
        ).collect();
        let outputs = serde_json::to_string(&mint).unwrap();

        let (req, _) = Payment::build_mint_req(get_wallet_handle() as i32, &did, &outputs).unwrap();

        ::utils::libindy::ledger::libindy_submit_request(&req).unwrap();

        Ok(())
    }


    pub fn token_setup() {
        init_payments().unwrap();
        set_ledger_fees().unwrap();
        mint_tokens().unwrap();
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
        ::utils::devsetup::tests::setup_dev_env(name);
        init_payments().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert!(balance.contains(r#""balance":0"#));
        ::utils::devsetup::tests::cleanup_dev_env(name);
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
        ::utils::devsetup::tests::setup_dev_env(name);
        init_payments().unwrap();
        set_ledger_fees().unwrap();
        let fees = get_ledger_fees().unwrap();
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":1"#));
        ::utils::devsetup::tests::cleanup_dev_env(name);
    }

    #[test]
    fn test_address_balance() {
        let addresses = vec![
            UTXO { input: "pov::null:2".to_string(), amount: 2, extra: Some("abcde".to_string()) },
            UTXO { input: "pov::null:3".to_string(), amount: 3, extra: Some("bcdef".to_string()) }
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
    fn test_gen_outputs_for_txn_fees() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let cost = 6;
        let expected_output = r#"[{"amount":0,"extra":null,"paymentAddress":"pay:null:J81AxU9hVHYFtJc"}]"#;
        let (remainder, _) = inputs(cost).unwrap();
        assert_eq!(&outputs(remainder, None, None).unwrap(), expected_output);
    }

    #[test]
    fn test_gen_outputs_for_transfer_of_tokens() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let expected_output = r#"[{"amount":4,"extra":null,"paymentAddress":"pay:null:J81AxU9hVHYFtJc"},{"amount":11,"extra":null,"paymentAddress":"pay:null:payee_address"}]"#;
        let payee_amount = 11;
        let payee_address = r#"pay:null:payee_address"#.to_string();
        assert_eq!(&outputs(4, Some(payee_address), Some(payee_amount)).unwrap(), expected_output);
    }

    #[test]
    fn test_get_txn_cost() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        assert_eq!(get_txn_price("101").unwrap(), 2);
        assert_eq!(get_txn_price("102").unwrap(), 42);
        assert_eq!(get_txn_price("Unknown txn type"), Err(error::UNKNOWN_TXN_TYPE.code_num));
    }

    #[test]
    fn test_pay_for_txn() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        // Schema
        let create_schema_req = ::utils::constants::SCHEMA_CREATE_JSON.to_string();
        let (parsed_response, response) = pay_for_txn(&create_schema_req, "101").unwrap();
        assert_eq!(parsed_response, PARSED_TXN_PAYMENT_RESPONSE.to_string());
        assert_eq!(response, SUBMIT_SCHEMA_RESPONSE.to_string());
    }

    #[cfg(feature = "nullpay")]
    #[test]
    fn test_pay_for_txn_real() {
        let name = "test_pay_for_txn_real";
        ::utils::devsetup::tests::setup_dev_env(name);
        token_setup();

        let create_schema_req = ::utils::constants::SCHEMA_REQ.to_string();
        let start_wallet: WalletInfo = serde_json::from_str(&get_wallet_token_info().unwrap()).unwrap();

        let (price_response, response) = pay_for_txn(&create_schema_req, "101").unwrap();

        let end_wallet: WalletInfo = serde_json::from_str(&get_wallet_token_info().unwrap()).unwrap();

        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert!(price_response.contains(r#""amount":13"#));
        let output_address: Vec<Value> = serde_json::from_str(&price_response).unwrap();
        assert_eq!(output_address.len(), 1);
        assert_eq!(start_wallet.balance - 2, end_wallet.balance);
    }

    #[cfg(feature = "nullpay")]
    #[test]
    fn test_pay_for_txn_fails_with_insufficient_tokens() {
        let name = "test_pay_for_txn_real";
        ::utils::devsetup::tests::setup_dev_env(name);
        token_setup();

        let create_schema_req = ::utils::constants::SCHEMA_REQ.to_string();
        let start_wallet: WalletInfo = serde_json::from_str(&get_wallet_token_info().unwrap()).unwrap();

        let rc= pay_for_txn(&create_schema_req, "9999");

        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert_eq!(rc, Err(error::INSUFFICIENT_TOKEN_AMOUNT.code_num));
    }
}
