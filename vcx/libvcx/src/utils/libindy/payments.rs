extern crate libc;
extern crate serde_json;

use futures::Future;

use utils::libindy::wallet::get_wallet_handle;
use utils::constants::{SUBMIT_SCHEMA_RESPONSE, TRANSFER_TXN_TYPE};
use utils::libindy::error_codes::map_rust_indy_sdk_error;
#[allow(unused_imports)]
use utils::libindy::ledger::{libindy_submit_request, libindy_sign_and_submit_request, libindy_sign_request};
use utils::error;
use error::payment::PaymentError;
use error::ToErrorCode;

use indy::payments;
use std::fmt;
use std::collections::HashMap;
use serde_json::Value;
use settings;

static DEFAULT_FEES: &str = r#"{"0":0, "1":0, "101":2, "10001":0, "102":42, "103":0, "104":0, "105":0, "107":0, "108":0, "109":0, "110":0, "111":0, "112":0, "113":2, "114":2, "115":0, "116":0, "117":0, "118":0, "119":0}"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo {
    balance: u64,
    balance_str: String,
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
    source: Option<String>,
    #[serde(rename = "paymentAddress")]
    recipient: String,
    amount: u64,
    extra: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Output {
    source: Option<String>,
    recipient: String,
    amount: u64,
    extra: Option<String>,
}

impl fmt::Display for WalletInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match serde_json::to_string(&self) {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "null"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PaymentTxn {
    pub amount: u64,
    pub credit: bool,
    pub inputs: Vec<String>,
    pub outputs: Vec<Output>,
}

impl PaymentTxn {
    pub fn from_parts(inputs: &str, outputs: &str, amount: u64, credit: bool) -> Result<PaymentTxn, u32> {
        let inputs: Vec<String> = serde_json::from_str(&inputs)
            .map_err(|err| { error::INVALID_JSON.code_num })?;
        let outputs: Vec<Output> = serde_json::from_str(&outputs)
            .map_err(|err| { error::INVALID_JSON.code_num })?;
        Ok(PaymentTxn {
            amount,
            credit,
            inputs,
            outputs,
        })
    }
}

pub fn build_test_address(address: &str) -> String {
    format!("pay:{}:{}", ::settings::get_payment_method(), address)
}

pub fn create_address(seed: Option<String>) -> Result<String, u32> {
    trace!("create_address >>> seed: {:?}", seed);

    if settings::test_indy_mode_enabled() {
        return Ok(build_test_address("J81AxU9hVHYFtJc"));
    }

    let config = match seed {
        Some(x) => format!("{{\"seed\":\"{}\"}}", x),
        None => format!("{{}}"),
    };

    payments::create_payment_address(get_wallet_handle() as i32, settings::get_payment_method().as_str(), &config)
        .wait()
        .map_err(map_rust_indy_sdk_error)
}

pub fn get_address_info(address: &str) -> Result<AddressInfo, u32> {
    if settings::test_indy_mode_enabled() {
        let utxos = json!(
            [
                {
                    "source": build_test_address("1"),
                    "paymentAddress": build_test_address("zR3GN9lfbCVtHjp"),
                    "amount": 1,
                    "extra": "yqeiv5SisTeUGkw"
                },
                {
                    "source": build_test_address("2"),
                    "paymentAddress": build_test_address("zR3GN9lfbCVtHjp"),
                    "amount": 2,
                    "extra": "Lu1pdm7BuAN2WNi"
                }
            ]
        );

        let utxo: Vec<UTXO> = serde_json::from_value(utxos).unwrap();

        return Ok(AddressInfo { address: address.to_string(), balance: _address_balance(&utxo), utxo });
    }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    let (txn, _) = payments::build_get_payment_sources_request(get_wallet_handle() as i32, Some(&did), address)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    let response = libindy_sign_and_submit_request(&did, &txn)?;

    let response = payments::parse_get_payment_sources_response(settings::get_payment_method().as_str(), &response)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    trace!("indy_parse_get_utxo_response() --> {}", response);
    let utxo: Vec<UTXO> = serde_json::from_str(&response).or(Err(error::INVALID_JSON.code_num))?;

    Ok(AddressInfo { address: address.to_string(), balance: _address_balance(&utxo), utxo })
}

pub fn list_addresses() -> Result<Vec<String>, u32> {
    if settings::test_indy_mode_enabled() {
        let addresses = json!([
                build_test_address("9UFgyjuJxi1i1HD"),
                build_test_address("zR3GN9lfbCVtHjp")
        ]);
        return Ok(serde_json::from_value(addresses).unwrap());
    }

    let addresses = payments::list_payment_addresses(get_wallet_handle() as i32)
        .wait()
        .map_err(map_rust_indy_sdk_error)?;

    trace!("--> {}", addresses);
    Ok(serde_json::from_str(&addresses).or(Err(error::INVALID_JSON.code_num))?)
}

fn is_valid_address(address: &str, method: &str) -> bool {
    let prefix = format!("pay:{}", method);

    address.starts_with(&prefix)
}

pub fn get_wallet_token_info() -> Result<WalletInfo, u32> {
    trace!("get_wallet_token_info >>>");

    let addresses = list_addresses()?;

    let method = settings::get_config_value(settings::CONFIG_PAYMENT_METHOD)?;
    let mut balance = 0;
    let mut wallet_info = Vec::new();

    for address in addresses.iter() {
        if is_valid_address(&address, &method) {
            debug!("getting address info for {}", address);
            let mut info = get_address_info(&address)?;

            for utxo in info.utxo.iter() { balance += utxo.amount as u64; }

            wallet_info.push(info);
        } else {
            warn!("payment address {} is not compatible with payment type '{}'", address, method);
        }
    }

    let info = WalletInfo { balance, balance_str: format!("{}", balance), addresses: wallet_info };

    trace!("get_wallet_token_info <<< info: {:?}", info);

    Ok(info)
}

pub fn get_ledger_fees() -> Result<String, u32> {
    trace!("get_ledger_fees >>>");

    if settings::test_indy_mode_enabled() { return Ok(DEFAULT_FEES.to_string()); }

    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).or(Err(error::INVALID_CONFIGURATION.code_num))?;

    let response = match payments::build_get_txn_fees_req(get_wallet_handle() as i32, Some(&did), settings::get_payment_method().as_str()).wait() {
        Ok(txn) => libindy_sign_and_submit_request(&did, &txn)?,
        Err(x) => return Err(map_rust_indy_sdk_error(x)),
    };

    let res = payments::parse_get_txn_fees_response(settings::get_payment_method().as_str(), &response)
        .wait()
        .map_err(map_rust_indy_sdk_error);
    res
}

pub fn pay_for_txn(req: &str, txn_type: &str) -> Result<(Option<PaymentTxn>, String), u32> {
    debug!("pay_for_txn(req: {}, txn_type: {})", req, txn_type);
    if settings::test_indy_mode_enabled() {
        let inputs = format!(r#"["{}"]"#, build_test_address("9UFgyjuJxi1i1HD"));

        let outputs = format!(r#"[
            {{
                "amount": 1,
                "extra": null,
                "recipient": "{}"
            }}
        ]"#, build_test_address("xkIsxem0YNtHrRO"));

        return Ok((Some(PaymentTxn::from_parts(&inputs, &outputs, 1, false).unwrap()), SUBMIT_SCHEMA_RESPONSE.to_string()));
    }

    let txn_price = get_txn_price(txn_type)?;
    if txn_price == 0 {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).or(Err(error::INVALID_CONFIGURATION.code_num))?;
        let txn_response = libindy_sign_and_submit_request(&did, req)?;
        Ok((None, txn_response))
    } else {
        let (refund, inputs, refund_address) = inputs(txn_price).map_err(|e| e.to_error_code())?;

        let output = outputs(refund, &refund_address, None, None).map_err(|e| e.to_error_code())?;

        let (fee_response, txn_response) = _submit_fees_request(req, &inputs, &output)?;

        let payment = PaymentTxn::from_parts(&inputs, &output, txn_price, false)?;
        Ok((Some(payment), txn_response))
    }
}

fn _submit_fees_request(req: &str, inputs: &str, outputs: &str) -> Result<(String, String), u32> {
    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).or(Err(error::INVALID_CONFIGURATION.code_num))?;

    let req = libindy_sign_request(&did, req)?;

    let (response, payment_method) = match payments::add_request_fees(get_wallet_handle(),
                                                                      Some(&did),
                                                                      &req,
                                                                      &inputs,
                                                                      &outputs,
                                                                      None).wait() {
        Ok((req, payment_method)) => {
            (libindy_submit_request(&req)?, payment_method)
        }
        Err(x) => return Err(map_rust_indy_sdk_error(x)),
    };

    let parsed_response = match payments::parse_response_with_fees(&payment_method, &response).wait() {
        Ok(x) => x,
        Err(x) => return Err(error::INVALID_LEDGER_RESPONSE.code_num),
    };

    Ok((parsed_response, response))
}

pub fn pay_a_payee(price: u64, address: &str) -> Result<(PaymentTxn, String), PaymentError> {
    trace!("pay_a_payee >>> price: {}, address {}", price, address);
    debug!("sending {} tokens to address {}", price, address);

    let ledger_cost = get_txn_price(TRANSFER_TXN_TYPE).map_err(|e| PaymentError::CommonError(e))?;
    let (remainder, input, refund_address) = inputs(price + ledger_cost)?;
    let output = outputs(remainder, &refund_address, Some(address.to_string()), Some(price))?;

    let payment = PaymentTxn::from_parts(&input, &output, price, false).map_err(|e| PaymentError::CommonError(e))?;
    let my_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).or(Err(PaymentError::CommonError(error::INVALID_CONFIGURATION.code_num)))?;

    if settings::test_indy_mode_enabled() {
        let inputs = format!(r#"["{}"]"#, build_test_address("9UFgyjuJxi1i1HD"));

        let outputs = format!(r#"[
            {{
                "amount": 1,
                "extra": null,
                "recipient": "{}"
            }}
        ]"#, build_test_address("xkIsxem0YNtHrRO"));

        return Ok((PaymentTxn::from_parts(&inputs, &outputs, 1, false).unwrap(), SUBMIT_SCHEMA_RESPONSE.to_string()));
    }

    match payments::build_payment_req(get_wallet_handle(), Some(&my_did), &input, &output, None).wait() {
        Ok((request, payment_method)) => {
            let result = libindy_submit_request(&request).map_err(|ec| PaymentError::CommonError(ec))?;
            Ok((payment, result))
        }
        Err(ec) => {
            error!("error: {:?}", ec.message);
            Err(PaymentError::CommonError(ec.error_code as u32))
        }
    }
}

fn get_txn_price(txn_type: &str) -> Result<u64, u32> {
    let ledger_fees = get_ledger_fees()?;

    let fees: HashMap<String, u64> = serde_json::from_str(&ledger_fees).or(Err(error::INVALID_JSON.code_num))?;

    match fees.get(txn_type) {
        Some(x) => Ok(*x),
        None => Ok(0),
    }
}

fn _address_balance(address: &Vec<UTXO>) -> u64 {
    address.iter().fold(0, |balance, utxo| balance + utxo.amount)
}

pub fn inputs(cost: u64) -> Result<(u64, String, String), PaymentError> {
    let mut inputs: Vec<String> = Vec::new();
    let mut balance = 0;
    let wallet_info: WalletInfo = get_wallet_token_info().map_err(|ec| PaymentError::CommonError(ec))?;
    let mut refund_address = String::new();

    if wallet_info.balance < cost {
        warn!("not enough tokens in wallet to pay: balance: {}, cost: {}", wallet_info.balance, cost);
        return Err(PaymentError::InsufficientFunds());
    }

    // Todo: explore 'smarter' ways of selecting utxos ie bitcoin algorithms etc
    'outer: for address in wallet_info.addresses.iter() {
        refund_address = address.address.clone();
        'inner: for utxo in address.utxo.iter() {
            if balance < cost {
                inputs.push(utxo.source.clone().ok_or(PaymentError::InsufficientFunds())?.to_string());
                balance += utxo.amount;
            } else { break 'outer }
        }
    }

    let remainder = balance - cost;
    let inputs = serde_json::to_string(&inputs).or(Err(PaymentError::InvalidWalletJson()))?;

    Ok((remainder, inputs, refund_address))
}

pub fn outputs(remainder: u64, refund_address: &str, payee_address: Option<String>, payee_amount: Option<u64>) -> Result<String, PaymentError> {
    // In the future we might provide a way for users to specify multiple output address for their remainder tokens
    // As of now, we only handle one output address which we create

    let mut outputs = Vec::new();
    if remainder > 0 {
        outputs.push(json!({ "recipient": refund_address, "amount": remainder }));
    }

    if let Some(address) = payee_address {
        outputs.push(json!({
            "recipient": address,
            "amount": payee_amount,
        }));
    }

    Ok(serde_json::to_string(&outputs).or(Err(PaymentError::InvalidWalletJson()))?)
}

// This is used for testing purposes only!!!
pub fn mint_tokens_and_set_fees(number_of_addresses: Option<u32>, tokens_per_address: Option<u64>, fees: Option<String>, seed: Option<String>) -> Result<(), u32> {
    trace!("mint_tokens_and_set_fees >>> number_of_addresses: {:?}, tokens_per_address: {:?}, fees: {:?}, seed: {:?}",
           number_of_addresses, tokens_per_address, fees, seed);

    let did_1 = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let fees = if fees.is_some() {
        fees.as_ref().map(String::as_str)
    } else {
        None
    };

    let (did_2, _) = add_new_trustee_did()?;
    let (did_3, _) = add_new_trustee_did()?;
    let (did_4, _) = add_new_trustee_did()?;

    let number_of_addresses = number_of_addresses.unwrap_or(1);

    if number_of_addresses > 0 {
        let tokens_per_address: u64 = tokens_per_address.unwrap_or(50000000000);
        let mut addresses = Vec::new();

        for n in 0..number_of_addresses {
            addresses.push(create_address(seed.clone()).unwrap())
        }

        let mint: Vec<Value> = addresses.clone().into_iter().enumerate().map(|(i, payment_address)|
            json!( { "recipient": payment_address, "amount": tokens_per_address } )
        ).collect();
        let outputs = serde_json::to_string(&mint).unwrap();
        let (req, _) = payments::build_mint_req(get_wallet_handle() as i32, Some(&did_1), &outputs, None).wait().unwrap();

        let sign1 = ::utils::libindy::ledger::multisign_request(&did_1, &req).unwrap();
        let sign2 = ::utils::libindy::ledger::multisign_request(&did_2, &sign1).unwrap();
        let sign3 = ::utils::libindy::ledger::multisign_request(&did_3, &sign2).unwrap();
        let sign4 = ::utils::libindy::ledger::multisign_request(&did_4, &sign3).unwrap();

        match ::utils::libindy::ledger::libindy_submit_request(&sign4) {
            Ok(x) => (),
            Err(x) => println!("failure minting tokens: {}", x),
        };
    }

    if fees.is_some() {
        let txn = payments::build_set_txn_fees_req(get_wallet_handle() as i32, Some(&did_1), settings::get_payment_method().as_str(), fees.unwrap())
            .wait()
            .map_err(map_rust_indy_sdk_error)?;

        let sign1 = ::utils::libindy::ledger::multisign_request(&did_1, &txn).unwrap();
        let sign2 = ::utils::libindy::ledger::multisign_request(&did_2, &sign1).unwrap();
        let sign3 = ::utils::libindy::ledger::multisign_request(&did_3, &sign2).unwrap();
        let sign4 = ::utils::libindy::ledger::multisign_request(&did_4, &sign3).unwrap();

        ::utils::libindy::ledger::libindy_submit_request(&sign4).unwrap();
    }

    Ok(())
}

fn add_new_trustee_did() -> Result<(String, String), u32> {
    use indy::ledger;

    let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let (did, verkey) = ::utils::libindy::signus::create_and_store_my_did(None).unwrap();
    let req_nym = ledger::build_nym_request(&institution_did, &did, Some(&verkey), None, Some("TRUSTEE"))
        .wait()
        .map_err(map_rust_indy_sdk_error)?;
    ::utils::libindy::ledger::libindy_sign_and_submit_request(&institution_did, &req_nym)?;
    Ok((did, verkey))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn token_setup(number_of_addresses: Option<u32>, tokens_per_address: Option<u64>) {
        mint_tokens_and_set_fees(number_of_addresses, tokens_per_address, Some(DEFAULT_FEES.to_string()), None).unwrap();
    }

    fn get_my_balance() -> u64 {
        let info: WalletInfo = get_wallet_token_info().unwrap();
        info.balance
    }

    #[test]
    fn test_create_address() {
        init!("true");
        create_address(None).unwrap();
    }

    #[test]
    fn test_get_addresses() {
        init!("true");
        create_address(None).unwrap();
        let addresses = list_addresses().unwrap();
    }

    #[test]
    fn test_get_wallet_token_info() {
        init!("true");
        create_address(None).unwrap();
        let balance = get_wallet_token_info().unwrap().to_string();

        let expected_balance = json!({
            "balance":6,
            "balance_str":"6",
            "addresses":[
                {
                    "address": build_test_address("9UFgyjuJxi1i1HD"),
                    "balance":3,
                    "utxo":[
                        {
                            "source": build_test_address("1"),
                            "paymentAddress":build_test_address("zR3GN9lfbCVtHjp"),
                            "amount":1,
                            "extra":"yqeiv5SisTeUGkw"
                        },
                        {
                            "source":build_test_address("2"),
                            "paymentAddress":build_test_address("zR3GN9lfbCVtHjp"),
                            "amount":2,
                            "extra":"Lu1pdm7BuAN2WNi"
                        }
                    ]
                },
                {
                    "address":build_test_address("zR3GN9lfbCVtHjp"),
                    "balance":3,
                    "utxo":[
                        {
                            "source":build_test_address("1"),
                            "paymentAddress":build_test_address("zR3GN9lfbCVtHjp"),
                            "amount":1,
                            "extra":"yqeiv5SisTeUGkw"
                        },
                        {
                            "source":build_test_address("2"),
                            "paymentAddress":build_test_address("zR3GN9lfbCVtHjp"),
                            "amount":2,
                            "extra":"Lu1pdm7BuAN2WNi"
                        }
                    ]
                }
            ]
        });

        let balance: serde_json::Value = serde_json::from_str(&balance).unwrap();

        assert_eq!(balance, expected_balance);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_wallet_token_info_real() {
        init!("ledger");
        let wallet_info = get_wallet_token_info().unwrap();
        assert_eq!(wallet_info.balance, 50000000000);
    }

    #[test]
    fn test_get_ledger_fees() {
        init!("true");
        let fees = get_ledger_fees().unwrap();
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":0"#));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_ledger_fees_real() {
        init!("ledger");
        let fees = get_ledger_fees().unwrap();
        assert!(fees.contains(r#""101":2"#));
        assert!(fees.contains(r#""1":0"#));
    }

    #[test]
    fn test_address_balance() {
        let addresses = vec![
            UTXO { source: Some(build_test_address("2")), recipient: build_test_address("J81AxU9hVHYFtJc"), amount: 2, extra: Some("abcde".to_string()) },
            UTXO { source: Some(build_test_address("3")), recipient: build_test_address("J81AxU9hVHYFtJc"), amount: 3, extra: Some("bcdef".to_string()) }
        ];

        assert_eq!(_address_balance(&addresses), 5);
    }

    #[test]
    fn test_inputs() {
        init!("true");

        let pay_addr_1 = build_test_address("1");
        let pay_addr_2 = build_test_address("2");

        // Success - Exact amount
        let expected_inputs = format!(r#"["{}","{}","{}","{}"]"#, pay_addr_1, pay_addr_2, pay_addr_1, pay_addr_2);
        assert_eq!(inputs(6).unwrap(), (0, expected_inputs, build_test_address("zR3GN9lfbCVtHjp")));

        // Success - utxo with remainder tokens
        let expected_inputs = format!(r#"["{}","{}","{}","{}"]"#, pay_addr_1, pay_addr_2, pay_addr_1, pay_addr_2);
        assert_eq!(inputs(5).unwrap(), (1, expected_inputs, build_test_address("zR3GN9lfbCVtHjp")));

        // Success - requesting amount that partial address (1 of 2 utxos) can satisfy
        let expected_inputs = format!(r#"["{}"]"#, pay_addr_1);
        assert_eq!(inputs(1).unwrap(), (0, expected_inputs, build_test_address("9UFgyjuJxi1i1HD")));

        // Err - request more than wallet contains
        assert_eq!(inputs(7).err(), Some(PaymentError::InsufficientFunds()));
    }

    #[test]
    fn test_gen_outputs_for_txn_fees() {
        init!("true");

        let mut cost = 5;
        let (remainder, _, refund_address) = inputs(cost).unwrap();
        let mut expected_output = format!(r#"[{{"amount":1,"recipient":"{}"}}]"#, refund_address);
        assert_eq!(outputs(remainder, &refund_address, None, None).unwrap(), expected_output);

        // No remainder so don't create an address in outputs
        cost = 6;
        expected_output = r#"[]"#.to_string();
        let (remainder, _, refund_address) = inputs(cost).unwrap();
        assert_eq!(remainder, 0);
        assert_eq!(outputs(remainder, &refund_address, None, None).unwrap(), expected_output);
    }

    #[test]
    fn test_gen_outputs_for_transfer_of_tokens() {
        init!("true");

        let payee_amount = 11;
        let payee_address = build_test_address("payee_address");
        let refund_address = build_test_address("refund_address");
        let expected_output = format!(r#"[{{"amount":4,"recipient":"{}"}},{{"amount":11,"recipient":"{}"}}]"#, refund_address, payee_address);
        assert_eq!(outputs(4, refund_address.as_str(), Some(payee_address), Some(payee_amount)).unwrap(), expected_output);
    }

    #[test]
    fn test_get_txn_cost() {
        init!("true");
        assert_eq!(get_txn_price("101").unwrap(), 2);
        assert_eq!(get_txn_price("102").unwrap(), 42);
        assert_eq!(get_txn_price("Unknown txn type").unwrap(), 0);
    }

    #[test]
    fn test_pay_for_txn() {
        init!("true");

        // Schema
        let create_schema_req = ::utils::constants::SCHEMA_CREATE_JSON.to_string();
        let (payment, response) = pay_for_txn(&create_schema_req, "101").unwrap();
        assert_eq!(response, SUBMIT_SCHEMA_RESPONSE.to_string());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_pay_for_txn_real() {
        init!("ledger");
        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let create_schema_req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);
        let start_wallet = get_wallet_token_info().unwrap();

        let (payment, response) = pay_for_txn(&create_schema_req, "101").unwrap();

        let end_wallet = get_wallet_token_info().unwrap();

        let payment = payment.unwrap();
        assert_eq!(payment.amount, 2);
        assert_eq!(payment.outputs.len(), 1);
        assert_eq!(start_wallet.balance - 2, end_wallet.balance);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_pay_for_txn_fails_with_insufficient_tokens_in_wallet() {
        init!("ledger");
        mint_tokens_and_set_fees(Some(0), Some(0), Some(r#"{"101":50000000001}"#.to_string()), None).unwrap();

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let create_schema_req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);

        let rc = pay_for_txn(&create_schema_req, "101");

        assert!(rc.is_err());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_build_payment_request() {
        init!("ledger");

        let payment_address = build_test_address("2ZrAm5Jc3sP4NAXMQbaWzDxEa12xxJW3VgWjbbPtMPQCoznJyS");
        let payment_address = payment_address.as_str();

        let price = get_my_balance();
        let result_from_paying = pay_a_payee(price, payment_address);
        assert!(result_from_paying.is_ok());
        assert_eq!(get_my_balance(), 0);
        mint_tokens_and_set_fees(None, None, None, None).unwrap();
        assert_eq!(get_my_balance(), 50000000000);

        let price = get_my_balance() - 5;
        let result_from_paying = pay_a_payee(price, payment_address);
        assert!(result_from_paying.is_ok());
        assert_eq!(get_my_balance(), 5);

        let price = get_my_balance() + 5;
        let result_from_paying = pay_a_payee(price, payment_address);
        assert_eq!(result_from_paying.err(), Some(PaymentError::InsufficientFunds()));
        assert_eq!(get_my_balance(), 5);
    }

    // this test if failing to to both changes in error codes being produced
    // by master libindy and how wallets are deleted.
    #[cfg(feature = "pool_tests")]
    #[test]
    #[ignore]
    fn test_build_payment_request_bogus_payment_method() {
        init!("ledger");
        let payment_address = "pay:bogus:123";
        let result_from_paying = pay_a_payee(0, payment_address);

        assert!(result_from_paying.is_err());
        assert_eq!(result_from_paying.err(), Some(PaymentError::CommonError(error::UNKNOWN_LIBINDY_ERROR.code_num)));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_fees_transferring_tokens() {
        init!("ledger");

        let payment_address = build_test_address("2ZrAm5Jc3sP4NAXMQbaWzDxEa12xxJW3VgWjbbPtMPQCoznJyS");
        let payment_address = payment_address.as_str();

        let initial_wallet_balance = 100000000000;
        let transfer_fee = 5;
        let ledger_fees = json!({"10001": transfer_fee}).to_string();
        mint_tokens_and_set_fees(None, None, Some(ledger_fees), None).unwrap();
        assert_eq!(get_my_balance(), initial_wallet_balance);
        assert_eq!(get_txn_price(TRANSFER_TXN_TYPE).unwrap(), transfer_fee);

        // Transfer everything besides 50. Remaining balance will be 50 - ledger fees
        let balance_after_transfer = 50;
        let price = get_my_balance() - balance_after_transfer;
        let result_from_paying = pay_a_payee(price, payment_address);
        assert!(result_from_paying.is_ok());
        assert_eq!(get_my_balance(), balance_after_transfer - transfer_fee);

        // Has tokens but not enough for ledger fee
        let not_enough_for_ledger_fee = transfer_fee - 1;
        let price = get_my_balance() - not_enough_for_ledger_fee;
        assert!(price > 0);
        let result_from_paying = pay_a_payee(price, payment_address);
        assert_eq!(result_from_paying.err(), Some(PaymentError::CommonError(error::INSUFFICIENT_TOKEN_AMOUNT.code_num)));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_submit_fees_with_insufficient_tokens_on_ledger() {
        init!("ledger");

        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);
        let (remainder, inputs, refund_address) = inputs(2).unwrap();
        let output = outputs(remainder, &refund_address, None, None).unwrap();
        let start_wallet = get_wallet_token_info().unwrap();

        _submit_fees_request(&req, &inputs, &output).unwrap();

        let end_wallet = get_wallet_token_info().unwrap();
        assert_eq!(start_wallet.balance - 2, end_wallet.balance);

        let rc = _submit_fees_request(&req, &inputs, &output);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_pay_for_txn_with_empty_outputs_success() {
        init!("ledger");
        let (_, schema_json) = ::utils::libindy::anoncreds::tests::create_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let req = ::utils::libindy::anoncreds::tests::create_schema_req(&schema_json);

        let cost = 45;
        let start_wallet = get_wallet_token_info().unwrap();
        let remaining_balance = start_wallet.balance - cost;
        let (remainder, inputs, refund_address) = inputs(cost).unwrap();
        assert_eq!(remainder, remaining_balance);

        let output = outputs(remainder, &refund_address, None, None).unwrap();
        assert_eq!(output, format!(r#"[{{"amount":{},"recipient":"{}"}}]"#, remaining_balance, refund_address));

        let rc = _submit_fees_request(&req, &inputs, &output).unwrap();
        let end_wallet = get_wallet_token_info().unwrap();

        assert_eq!(end_wallet.balance, remaining_balance);
    }

    #[test]
    fn test_wallet_info_to_string() {
        let wallet_info = WalletInfo {
            balance: 12345,
            addresses: Vec::new(),
            balance_str: "12345".to_string(),
        };
        assert_eq!(wallet_info.to_string(), r#"{"balance":12345,"balance_str":"12345","addresses":[]}"#.to_string());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_custom_mint_tokens() {
        init!("ledger");
        //50000000000 comes from setup_ledger_env
        token_setup(Some(4), Some(1430000));

        let start_wallet = get_wallet_token_info().unwrap();
        assert_eq!(start_wallet.balance, 50005720000);
    }

    #[ignore] // Test only works when fees are null
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_empty_fees() {
        init!("ledger");
        let fees = get_ledger_fees().unwrap();
        println!("fees: {}", fees);
        ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_zero_fees() {
        init!("ledger");
        mint_tokens_and_set_fees(Some(0), Some(0), Some("{\"101\":0, \"102\":0}".to_string()), None).unwrap();
        let fees = get_ledger_fees().unwrap();
        println!("fees: {}", fees);
        ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_two_init() {
        init!("ledger");
        init!("ledger");
    }
}
