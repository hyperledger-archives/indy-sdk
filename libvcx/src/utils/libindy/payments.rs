extern crate libc;
extern crate serde_json;

use utils::libindy::wallet::get_wallet_handle;
use utils::constants::{ SUBMIT_SCHEMA_RESPONSE };
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use utils::libindy::ledger::libindy_sign_and_submit_request;
use utils::error;
use error::payment::PaymentError;
use error::ToErrorCode;

use indy::payments::Payment;
use std::fmt;
use std::sync::{Once, ONCE_INIT};
use std::collections::HashMap;
use serde_json::Value;
use settings;

static NULL_PAYMENT: &str = "null";
static EMPTY_CONFIG: &str = "{}";
static FEES: &str = r#"{"0":1, "1":1, "101":2, "102":42, "103":1999998889, "104":0, "105":0, "107":0, "108":0, "109":0, "110":0, "111":0, "112":0, "113":0, "114":0, "115":0, "116":0, "117":0, "118":0, "119":0}"#;
static PARSED_TXN_PAYMENT_RESPONSE: &str = r#"[{"amount":4,"extra":null,"input":"["pov:null:1","pov:null:2"]"}]"#;

static PAYMENT_INIT: Once = ONCE_INIT;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo {
    balance: u64,
    addresses: Vec<AddressInfo>,
}

impl WalletInfo {
    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddressInfo {
    pub address: String,
    pub balance: u64,
    utxo: Vec<UTXO>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UTXO {
    txo: Option<String>,
    #[serde(rename = "paymentAddress")]
    payment_address: String,
    amount: u64,
    extra: Option<String>,
}

impl fmt::Display for WalletInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match serde_json::to_string(&self){
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "null"),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PaymentTxn {
    pub amount: u64,
    pub inputs: Vec<String>,
    pub outputs: Vec<UTXO>,
}

impl PaymentTxn {
    pub fn from_parts(inputs: &str, outputs: &str, amount: u64) -> Result<PaymentTxn, u32> {
        let inputs: Vec<String> = serde_json::from_str(&inputs)
            .map_err(|err| {error::INVALID_JSON.code_num})?;

        let outputs: Vec<UTXO> = serde_json::from_str(&outputs)
            .map_err(|err| {error::INVALID_JSON.code_num})?;

        Ok(PaymentTxn {
            amount,
            inputs,
            outputs,
        })
    }
}

pub fn create_address() -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(r#"pay:null:J81AxU9hVHYFtJc"#.to_string()); }

    Payment::create_payment_address(get_wallet_handle() as i32, NULL_PAYMENT, EMPTY_CONFIG)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn get_address_info(address: &str) -> Result<AddressInfo, u32> {
    if settings::test_indy_mode_enabled() {
        let utxo: Vec<UTXO> = serde_json::from_str(r#"[{"txo":"pov:null:1","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":1,"extra":"yqeiv5SisTeUGkw"},{"txo":"pov:null:2","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]"#).unwrap();
        return Ok(AddressInfo { address: address.to_string(), balance: _address_balance(&utxo), utxo})
    }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let (txn, _) = Payment::build_get_utxo_request(get_wallet_handle() as i32, &did, address)
        .map_err(map_rust_indy_sdk_error_code)?;

    let response = libindy_sign_and_submit_request(&did, &txn)?;

    let response = Payment::parse_get_utxo_response(NULL_PAYMENT, &response)
        .map_err(map_rust_indy_sdk_error_code)?;

    trace!("indy_parse_get_utxo_response() --> {}", response);
    let utxo: Vec<UTXO> = match serde_json::from_str(&response) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    Ok(AddressInfo { address: address.to_string(), balance: _address_balance(&utxo), utxo })
}

pub fn list_addresses() -> Result<Vec<String>, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(serde_json::from_str(r#"["pay:null:9UFgyjuJxi1i1HD","pay:null:zR3GN9lfbCVtHjp"]"#).unwrap());
    }

    let addresses = Payment::list_payment_addresses(get_wallet_handle() as i32)
        .map_err(map_rust_indy_sdk_error_code)?;

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

pub fn get_wallet_token_info() -> Result<WalletInfo, u32> {
    let addresses = list_addresses()?;

    let mut balance = 0;
    let mut wallet_info = Vec::new();

    for address in addresses.iter() {
        let mut info = get_address_info(&address)?;

        for utxo in info.utxo.iter() { balance += utxo.amount as u64; }

        wallet_info.push(info);
    }

    Ok(WalletInfo { balance, addresses: wallet_info })
}

pub fn get_ledger_fees() -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(FEES.to_string()); }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let response = match Payment::build_get_txn_fees_req(get_wallet_handle() as i32, &did, NULL_PAYMENT) {
        Ok(txn) => libindy_sign_and_submit_request(&did, &txn)?,
        Err(x) => return Err(map_rust_indy_sdk_error_code(x)),
    };

    Payment::parse_get_txn_fees_response(NULL_PAYMENT, &response)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn pay_for_txn(req: &str, txn_type: &str) -> Result<(PaymentTxn, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"paymentAddress":"pay:null:xkIsxem0YNtHrRO"}]"#,1).unwrap(), SUBMIT_SCHEMA_RESPONSE.to_string())); }

    let txn_price = get_txn_price(txn_type)?;

    let (refund, inputs) = inputs(txn_price).map_err(|e| e.to_error_code())?;

    let output = outputs(refund, None, None).map_err(|e| e.to_error_code())?;

    let (fee_response, txn_response) = _submit_fees_request(req, &inputs, &output)?;

    let payment = PaymentTxn::from_parts(&inputs, &output, txn_price)?;
    Ok((payment, txn_response))
}

fn _submit_fees_request(req: &str, inputs: &str, outputs: &str) -> Result<(String, String), u32> {
    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let (response, payment_method) = match Payment::add_request_fees(get_wallet_handle(),
                                                                     &did,
                                                                     req,
                                                                     &inputs,
                                                                     &outputs) {
        Ok((req, payment_method)) => (libindy_sign_and_submit_request(&did, &req)?, payment_method),
        Err(x) => return Err(map_rust_indy_sdk_error_code(x)),
    };

    let parsed_response = Payment::parse_response_with_fees(&payment_method, &response)
        .map_err(map_rust_indy_sdk_error_code)?;

    Ok((parsed_response, response))
}

pub fn pay_a_payee(price: u64, address: &str) -> Result<(PaymentTxn, String), PaymentError> {
    if settings::test_indy_mode_enabled() { return Ok((PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"paymentAddress":"pay:null:xkIsxem0YNtHrRO"}]"#,1).unwrap(), SUBMIT_SCHEMA_RESPONSE.to_string())); }

    let (remainder, input) = inputs(price)?;
    let output = outputs(remainder, Some(address.to_string()), Some(price))?;

    let payment = PaymentTxn::from_parts(&input, &output, price).map_err(|e|PaymentError::CommonError(e))?;
    let my_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
    match Payment::build_payment_req(get_wallet_handle(), &my_did, &input, &output) {
        Ok((request, payment_method)) => {
            let result = libindy_sign_and_submit_request(&my_did, &request).map_err(|ec| PaymentError::CommonError(ec))?;
            Ok((payment, result))
        },
        Err(ec) => {
            error!("error: {:?}", ec);
            Err(PaymentError::CommonError(ec as u32))
        },
    }
}

fn get_txn_price(txn_type: &str) -> Result<u64, u32> {
    let ledger_fees = get_ledger_fees()?;

    let fees: HashMap<String, u64> = serde_json::from_str(&ledger_fees) .or(Err(error::INVALID_JSON.code_num))?;

    Ok(fees.get(txn_type).ok_or(error::UNKNOWN_TXN_TYPE.code_num)?.clone())
}

fn _address_balance(address: &Vec<UTXO>) -> u64 {
    address.iter().fold(0, |balance, utxo| balance + utxo.amount)
}

pub fn inputs(cost: u64) -> Result<(u64, String), PaymentError> {
    let mut inputs: Vec<String> = Vec::new();
    let mut balance = 0;
    let wallet_info: WalletInfo = get_wallet_token_info().map_err(|ec| PaymentError::CommonError(ec))?;

    if wallet_info.balance < cost {
        warn!("not enough tokens in wallet to pay");
        return Err(PaymentError::InsufficientFunds());
    }

    // Todo: explore 'smarter' ways of selecting utxos ie bitcoin algorithms etc
    'outer: for address in wallet_info.addresses.iter() {
        'inner: for utxo in address.utxo.iter() {
            if balance < cost {
                inputs.push(utxo.txo.clone().unwrap().to_string());
                balance += utxo.amount;
            } else { break 'outer }
        }
    }

    let remainder = balance - cost;

    Ok((remainder, serde_json::to_string(&inputs).or(Err(PaymentError::InvalidWalletJson()))?))
}

pub fn outputs(remainder: u64, payee_address: Option<String>, payee_amount: Option<u64>) -> Result<String, PaymentError> {
    // In the future we might provide a way for users to specify multiple output address for their remainder tokens
    // As of now, we only handle one output address which we create

    let mut outputs = Vec::new();
    if remainder > 0 {
        outputs.push(json!({ "paymentAddress": create_address().map_err(|ec| PaymentError::CommonError(ec))?, "amount": remainder, "extra": null }));
    }

    if let Some(address) = payee_address {
        outputs.push(json!({
            "paymentAddress": address,
            "amount": payee_amount,
            "extra": null
        }));
    }

    Ok(serde_json::to_string(&outputs).unwrap())
}

// This is used for testing purposes only!!!
pub fn mint_tokens(number_of_addresses: Option<u32>, tokens_per_address: Option<u32>) -> Result<(), u32> {
    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let number_of_addresses = number_of_addresses.unwrap_or(3);
    let tokens_per_address = tokens_per_address.unwrap_or(15);
    let mut addresses = Vec::new();

    for n in 0..number_of_addresses {addresses.push(create_address().unwrap())}

    let mint: Vec<Value> = addresses.clone().into_iter().enumerate().map(|(i, payment_address)|
        json!( { "paymentAddress": payment_address, "amount": tokens_per_address, "extra": null } )
    ).collect();
    let outputs = serde_json::to_string(&mint).unwrap();

    let (req, _) = Payment::build_mint_req(get_wallet_handle() as i32, &did, &outputs).unwrap();

    ::utils::libindy::ledger::libindy_submit_request(&req).unwrap();

    Ok(())
}

// This is used for testing purposes only!!!
pub fn set_ledger_fees(fees: Option<String>) -> Result<(), u32> {
    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
    let fees = fees.unwrap_or(FEES.to_string());

    match Payment::build_set_txn_fees_req(get_wallet_handle() as i32, &did, NULL_PAYMENT, &fees) {
        Ok(txn) => match libindy_sign_and_submit_request(&did, &txn) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        },
        Err(x) => Err(x as u32),
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;
    use settings;

    pub fn token_setup(number_of_addresses: Option<u32>, tokens_per_address: Option<u32>) {
        init_payments().unwrap();
        set_ledger_fees(None).unwrap();
        mint_tokens(number_of_addresses, tokens_per_address).unwrap();
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
        let balance = get_wallet_token_info().unwrap().to_string();
        assert_eq!(balance, r#"{"balance":6,"addresses":[{"address":"pay:null:9UFgyjuJxi1i1HD","balance":3,"utxo":[{"txo":"pov:null:1","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":1,"extra":"yqeiv5SisTeUGkw"},{"txo":"pov:null:2","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]},{"address":"pay:null:zR3GN9lfbCVtHjp","balance":3,"utxo":[{"txo":"pov:null:1","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":1,"extra":"yqeiv5SisTeUGkw"},{"txo":"pov:null:2","paymentAddress":"pay:null:zR3GN9lfbCVtHjp","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]}]}"#);
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_get_wallet_token_info_real() {
        let name = "test_get_wallet_info_real";
        ::utils::devsetup::tests::setup_ledger_env(name);
        create_address().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        let wallet_info = get_wallet_token_info().unwrap();
        assert_eq!(wallet_info.balance, 45);
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

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_get_ledger_fees_real() {
        let name = "test_get_ledger_fees_real";
        ::utils::devsetup::tests::setup_ledger_env(name);
        set_ledger_fees(None).unwrap();
        let fees = get_ledger_fees().unwrap();
        println!("{}", fees);
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":1"#));
        ::utils::devsetup::tests::cleanup_dev_env(name);
    }

    #[test]
    fn test_address_balance() {
        let addresses = vec![
            UTXO { txo: Some("pov::null:2".to_string()), payment_address: "pay:null:J81AxU9hVHYFtJc".to_string(), amount: 2, extra: Some("abcde".to_string()) },
            UTXO { txo: Some("pov::null:3".to_string()), payment_address: "pay:null:J81AxU9hVHYFtJc".to_string(), amount: 3, extra: Some("bcdef".to_string()) }
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
        assert_eq!(inputs(7).err(), Some(PaymentError::InsufficientFunds()));
    }

    #[test]
    fn test_gen_outputs_for_txn_fees() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let mut cost = 5;
        let mut expected_output = r#"[{"amount":1,"extra":null,"paymentAddress":"pay:null:J81AxU9hVHYFtJc"}]"#;
        let (remainder, _) = inputs(cost).unwrap();
        assert_eq!(&outputs(remainder, None, None).unwrap(), expected_output);

        // No remainder so don't create an address in outputs
        cost = 6;
        expected_output = r#"[]"#;
        let (remainder, _) = inputs(cost).unwrap();
        assert_eq!(remainder, 0);
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
        let (payment, response) = pay_for_txn(&create_schema_req, "101").unwrap();
        assert_eq!(response, SUBMIT_SCHEMA_RESPONSE.to_string());
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_pay_for_txn_real() {
        let name = "test_pay_for_txn_real";

        ::utils::devsetup::tests::setup_ledger_env(name);

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema();
        let create_schema_req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);
        let start_wallet = get_wallet_token_info().unwrap();

        let (payment, response) = pay_for_txn(&create_schema_req, "101").unwrap();

        let end_wallet = get_wallet_token_info().unwrap();

        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert_eq!(payment.amount, 2);
        assert_eq!(payment.outputs.len(), 1);
        assert_eq!(start_wallet.balance - 2, end_wallet.balance);
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_pay_for_txn_fails_with_insufficient_tokens_in_wallet() {
        let name = "test_pay_for_txn_real";
        ::utils::devsetup::tests::setup_ledger_env(name);

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema();
        let create_schema_req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);
        let start_wallet = get_wallet_token_info().unwrap();

        let rc= pay_for_txn(&create_schema_req, "103");

        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert_eq!(rc, Err(error::INSUFFICIENT_TOKEN_AMOUNT.code_num));
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_build_payment_request() {
        use utils::devsetup::tests;
        fn get_my_balance() -> u64 {
            let info:WalletInfo = get_wallet_token_info().unwrap();
            info.balance
        }

        let name = "test_build_payment_request";
        tests::setup_ledger_env(name);

        let price = get_my_balance();
        let address = "pay:null:4jtvRvSl6OTDEMqrUBsqAfCFWeTOF86H";
        let result_from_paying = pay_a_payee(price, address);
        assert!(result_from_paying.is_ok());
        assert_eq!(get_my_balance(), 0);
        mint_tokens(None, None).unwrap();
        assert_eq!(get_my_balance(), 45);

        let price = get_my_balance() - 5;
        let address = "pay:null:4jtvRvSl6OTDEMqrUBsqAfCFWeTOF86H";
        let result_from_paying = pay_a_payee(price, address);
        assert!(result_from_paying.is_ok());
        assert_eq!(get_my_balance(), 5);

        let price = get_my_balance() + 5;
        let address = "pay:null:4jtvRvSl6OTDEMqrUBsqAfCFWeTOF86H";
        let result_from_paying = pay_a_payee(price, address);
        assert_eq!(result_from_paying.err(), Some(PaymentError::InsufficientFunds()));
        assert_eq!(get_my_balance(), 5);


        tests::cleanup_dev_env(name);
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_submit_fees_with_insufficient_tokens_on_ledger() {
        let name = "test_submit_fees_with_insufficient_tokens_on_ledger";

        ::utils::devsetup::tests::setup_ledger_env(name);

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema();
        let req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);
        let (remainder, inputs) = inputs(40).unwrap();
        let output = outputs(remainder, None, None).unwrap();
        let start_wallet = get_wallet_token_info().unwrap();

        _submit_fees_request(&req, &inputs, &output).unwrap();

        let end_wallet = get_wallet_token_info().unwrap();
        assert_eq!(start_wallet.balance - 40, end_wallet.balance);

        let rc = _submit_fees_request(&req, &inputs, &output);

        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert_eq!(rc, Err(error::INSUFFICIENT_TOKEN_AMOUNT.code_num));
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_pay_for_txn_with_empty_outputs_success() {
        let name = "test_pay_for_txn_with_empty_outputs_success";
        ::utils::devsetup::tests::setup_ledger_env(name);

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema();
        let req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);

        let (remainder, inputs) = inputs(45).unwrap();
        assert_eq!(remainder, 0);

        let output = outputs(remainder, None, None).unwrap();
        assert_eq!(output, "[]");

        let start_wallet = get_wallet_token_info().unwrap();
        let rc = _submit_fees_request(&req, &inputs, &output);
        let end_wallet = get_wallet_token_info().unwrap();

        assert!(rc.is_ok());
        assert_eq!(end_wallet.balance, 0);

        ::utils::devsetup::tests::cleanup_dev_env(name);
    }

    #[test]
    fn test_wallet_info_to_string() {
        let wallet_info = WalletInfo {
            balance: 12345,
            addresses: Vec::new(),
        };
        assert_eq!(wallet_info.to_string(), r#"{"balance":12345,"addresses":[]}"#.to_string());
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_custom_mint_tokens() {
        let name = "test_custom_mint_tokens";
        ::utils::devsetup::tests::setup_ledger_env(name);
        token_setup(Some(4), Some(1430000));

        let start_wallet = get_wallet_token_info().unwrap();
        ::utils::devsetup::tests::cleanup_dev_env(name);
        assert_eq!(start_wallet.balance, 5720045);
    }
}
