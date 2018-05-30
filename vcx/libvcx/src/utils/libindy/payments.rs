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
static FEES: &str = r#"{"1":1, "101":2}"#;

static PAYMENT_INIT: Once = ONCE_INIT;

#[derive(Serialize)]
struct AddressInfo {
    address: String,
    utxo: Vec<UTXO>,
}

#[derive(Deserialize, Serialize, Debug)]
struct UTXO {
    input: String,
    amount: i32,
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
    if settings::test_indy_mode_enabled() { return Ok(r#"["pay:null:J81AxU9hVHYFtJc"]"#.to_string()); }

    match Payment::create_payment_address(get_wallet_handle() as i32, NULL_PAYMENT, EMPTY_CONFIG) {
        Ok(x) => Ok(x),
        Err(x) => Err(x as u32),
    }
}

fn get_address_info(address: &str) -> Result<AddressInfo, u32> {
    if settings::test_indy_mode_enabled() {
        let utxo: Vec<UTXO> = serde_json::from_str(r#"[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]"#).unwrap();
        return Ok(AddressInfo { address: address.to_string(), utxo, })
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
    Ok(AddressInfo { address: address.to_string(), utxo, })
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
            balance += utxo.amount as i64;
        }

        wallet_info.push(info);
    }

    let addresses = serde_json::to_string(&wallet_info).unwrap();
    Ok(format!("{{\"balance\":{}, {}}}", balance, addresses))
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
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
    }

    #[test]
    fn test_create_address() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
    }

    #[test]
    fn test_get_addresses() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
        let addresses = list_addresses().unwrap();
    }

    #[test]
    fn test_get_wallet_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert_eq!(balance, r#"{"balance":6, [{"address":"pay:null:9UFgyjuJxi1i1HD","utxo":[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]},{"address":"pay:null:zR3GN9lfbCVtHjp","utxo":[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]}]}"#);
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
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
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
}
