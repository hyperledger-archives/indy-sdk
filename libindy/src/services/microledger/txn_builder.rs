extern crate serde;
extern crate serde_json;

use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::Value as JValue;

use errors::common::CommonError;
use services::ledger::LedgerService;
use services::microledger::constants::*;
use services::microledger::auth::Auth;

lazy_static! {
    pub static ref ML_PROTOCOL_VERSION: AtomicUsize = AtomicUsize::new(1);
    pub static ref ML_TXN_VERSION: AtomicUsize = AtomicUsize::new(1);
}

pub struct MLProtocolVersion {}

impl MLProtocolVersion {
    pub fn set(version: usize) {
        ML_PROTOCOL_VERSION.store(version, Ordering::Relaxed);
    }

    pub fn get() -> usize {
        ML_PROTOCOL_VERSION.load(Ordering::Relaxed)
    }
}

pub struct MLTxnVersion {}

impl MLTxnVersion {
    pub fn set(version: usize) {
        ML_TXN_VERSION.store(version, Ordering::Relaxed);
    }

    pub fn get() -> usize {
        ML_TXN_VERSION.load(Ordering::Relaxed)
    }
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Txn<T: serde::Serialize> {
    protocol_version: usize,
    txn_version: usize,
    operation: T
}

impl<T: serde::Serialize> Txn<T> {
    pub fn new(protocol_version: usize, txn_version: usize, operation: T) -> Txn<T> {
        Txn {
            protocol_version,
            txn_version,
            operation
        }
    }
}

pub struct TxnBuilder {}

impl TxnBuilder {
    pub fn build_nym_txn(dest: &str, verkey: Option<&str>) -> Result<String, CommonError> {
        let operation = LedgerService::build_nym_operation(dest, verkey, None, None)?;
        TxnBuilder::build_txn(operation).map_err(|err|
            CommonError::InvalidState(format!("NYM txn operation is invalid {:?}.", err)))
    }

    pub fn build_key_txn(verkey: &str, authorisations: &Vec<&str>) -> Result<String, CommonError> {
        let operation = TxnBuilder::build_key_operation(verkey, authorisations)?;
        TxnBuilder::build_txn(operation).map_err(|err|
            CommonError::InvalidState(format!("KEY txn operation is invalid {:?}.", err)))
    }

    pub fn build_endpoint_txn(verkey: &str, address: &str) -> Result<String, CommonError> {
        let operation = TxnBuilder::build_endpoint_operation(verkey, address)?;
        TxnBuilder::build_txn(operation).map_err(|err|
            CommonError::InvalidState(format!("ENDPOINT txn operation is invalid {:?}.", err)))
    }

    pub fn build_endpoint_rem_txn(verkey: &str, address: &str) -> Result<String, CommonError> {
        let operation = TxnBuilder::build_endpoint_rem_operation(verkey, address)?;
        TxnBuilder::build_txn(operation).map_err(|err|
            CommonError::InvalidState(format!("ENDPOINT_REM txn operation is invalid {:?}.", err)))
    }

    pub fn add_signature_to_txn(txn: &str, signature: &str) -> Result<String, CommonError> {
        let mut j_txn: JValue = serde_json::from_str(txn).map_err(|err|
            CommonError::InvalidState(format!("txn is not json {:?}.", err)))?;
        let m = j_txn.as_object_mut().unwrap();
        m.insert(SIGNATURE.to_string(), JValue::from(signature.to_string()));
        let signed_jval: JValue = JValue::from(m.to_owned());
        serde_json::to_string(&signed_jval).map_err(|err|
            CommonError::InvalidState(format!("Cannot jsonify signed txn {:?}.", err)))
    }

    fn build_key_operation(verkey: &str, authorisations: &Vec<&str>) -> Result<JValue, CommonError> {
        let mut authz: Vec<JValue> = Vec::new();

        for auth in authorisations.to_vec() {
            if Auth::is_valid_auth(auth) {
                authz.push(JValue::String(auth.to_string()))
            } else {
                return Err(CommonError::InvalidStructure(format!("Invalid authorization: {}", &auth)))
            }
        }
        let mut operation: JValue = JValue::Object(serde_json::map::Map::new());
        operation["type"] = JValue::String(KEY_TXN.to_string());
        operation[VERKEY] = JValue::String(verkey.to_string());
        operation[AUTHORIZATIONS] = JValue::Array(authz);
        Ok(operation)
    }

    fn build_endpoint_operation(verkey: &str, address: &str) -> Result<JValue, CommonError> {
        // TODO: Validate if endpoint is a valid URL (HTTP(S)/TCP/???)
        let mut operation: JValue = JValue::Object(serde_json::map::Map::new());
        operation["type"] = JValue::String(ENDPOINT_TXN.to_string());
        operation[VERKEY] = JValue::String(verkey.to_string());
        operation[ADDRESS] = JValue::String(address.to_string());
        Ok(operation)
    }

    fn build_endpoint_rem_operation(verkey: &str, address: &str) -> Result<JValue, CommonError> {
        // TODO: Validate if endpoint is a valid URL (HTTP(S)/TCP/???)
        let mut operation: JValue = JValue::Object(serde_json::map::Map::new());
        operation["type"] = JValue::String(ENDPOINT_REM_TXN.to_string());
        operation[VERKEY] = JValue::String(verkey.to_string());
        operation[ADDRESS] = JValue::String(address.to_string());
        Ok(operation)
    }

    fn build_txn(operation: JValue) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Txn::new(MLProtocolVersion::get(),
                                        MLTxnVersion::get(), operation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_protocol_version_set_get() {
        assert_eq!(MLProtocolVersion::get(), 1);
        MLProtocolVersion::set(2);
        assert_eq!(MLProtocolVersion::get(), 2);
        MLProtocolVersion::set(1);
        assert_eq!(MLProtocolVersion::get(), 1);
    }

    #[test]
    fn test_ml_txn_version_set_get() {
        assert_eq!(MLTxnVersion::get(), 1);
        MLTxnVersion::set(2);
        assert_eq!(MLTxnVersion::get(), 2);
        MLTxnVersion::set(1);
        assert_eq!(MLTxnVersion::get(), 1);
    }

    #[test]
    fn test_build_nym_txn() {
        let dest = "75KUW8tPUQNBS4W7ibFeY8";
        let expected_result_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;

        let nym_txn_1 = TxnBuilder::build_nym_txn(dest, None).unwrap();
        assert_eq!(nym_txn_1, expected_result_1);

        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let expected_result_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;

        let nym_txn_2 = TxnBuilder::build_nym_txn(dest, Some(verkey)).unwrap();
        assert_eq!(nym_txn_2, expected_result_2);
    }

    #[test]
    fn test_build_key_txn() {
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let authorisations: Vec<&str> = Vec::new();
        let expected_result_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":[],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;

        let key_txn_1 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_1, expected_result_1);

        let authorisations: Vec<&str> = vec![AUTHZ_ALL];
        let expected_result_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;

        let key_txn_2 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_2, expected_result_2);

        let authorisations: Vec<&str> = vec![AUTHZ_ALL, AUTHZ_ADD_KEY, AUTHZ_REM_KEY];
        let expected_result_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all","add_key","rem_key"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;

        let key_txn_3 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_3, expected_result_3);

        let authorisations: Vec<&str> = vec![AUTHZ_ALL, "some invalid auth"];
        let key_txn_3 = TxnBuilder::build_key_txn(verkey, &authorisations);
        assert!(key_txn_3.is_err())
    }

    #[test]
    fn test_build_endpoint_txn() {
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let address_1 = "https://agent.example.com";
        let address_2 = "https://agent1.example.com:9080";
        let address_3 = "tcp://123.88.912.091:9876";

        let expected_result_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let ep_txn_1 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        assert_eq!(ep_txn_1, expected_result_1);

        let expected_result_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let ep_txn_2 = TxnBuilder::build_endpoint_txn(verkey, address_2).unwrap();
        assert_eq!(ep_txn_2, expected_result_2);

        let expected_result_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"tcp://123.88.912.091:9876","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let ep_txn_3 = TxnBuilder::build_endpoint_txn(verkey, address_3).unwrap();
        assert_eq!(ep_txn_3, expected_result_3);
    }

    #[test]
    fn test_build_endpoint_rem_txn() {
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let address_1 = "https://agent.example.com";
        let address_2 = "https://agent1.example.com:9080";
        let ep_txn_1 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        let ep_txn_2 = TxnBuilder::build_endpoint_txn(verkey, address_2).unwrap();

        let ep_rem_txn_1 = TxnBuilder::build_endpoint_rem_txn(verkey, address_2).unwrap();
        let expected_result_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"4","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        assert_eq!(ep_rem_txn_1, expected_result_1);

        let ep_rem_txn_2 = TxnBuilder::build_endpoint_rem_txn(verkey, address_1).unwrap();
        let expected_result_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"4","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        assert_eq!(ep_rem_txn_2, expected_result_2);
    }

    #[test]
    fn test_ml_protocol_version_with_txn() {
        let dest = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let expected_nym_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let nym_txn_1 = TxnBuilder::build_nym_txn(dest, None).unwrap();
        assert_eq!(nym_txn_1, expected_nym_txn);

        let authorisations: Vec<&str> = Vec::new();
        let expected_key_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":[],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let key_txn_1 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_1, expected_key_txn);

        let address_1 = "https://agent.example.com";
        let expected_ep_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let ep_txn_1 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        assert_eq!(ep_txn_1, expected_ep_txn);

        MLProtocolVersion::set(2);

        let nym_txn_2 = TxnBuilder::build_nym_txn(dest, None).unwrap();
        assert_eq!(nym_txn_2, expected_nym_txn.replace("\"protocolVersion\":1", "\"protocolVersion\":2"));

        let key_txn_2 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_2, expected_key_txn.replace("\"protocolVersion\":1", "\"protocolVersion\":2"));

        let ep_txn_2 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        assert_eq!(ep_txn_2, expected_ep_txn.replace("\"protocolVersion\":1", "\"protocolVersion\":2"));

        MLProtocolVersion::set(1);
    }

    #[test]
    fn test_ml_txn_version_with_txn() {
        let dest = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let expected_nym_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let nym_txn_1 = TxnBuilder::build_nym_txn(dest, None).unwrap();
        assert_eq!(nym_txn_1, expected_nym_txn);

        let authorisations: Vec<&str> = Vec::new();
        let expected_key_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":[],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let key_txn_1 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_1, expected_key_txn);

        let address_1 = "https://agent.example.com";
        let expected_ep_txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let ep_txn_1 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        assert_eq!(ep_txn_1, expected_ep_txn);

        MLTxnVersion::set(2);

        let nym_txn_2 = TxnBuilder::build_nym_txn(dest, None).unwrap();
        assert_eq!(nym_txn_2, expected_nym_txn.replace("\"txnVersion\":1", "\"txnVersion\":2"));

        let key_txn_2 = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(key_txn_2, expected_key_txn.replace("\"txnVersion\":1", "\"txnVersion\":2"));

        let ep_txn_2 = TxnBuilder::build_endpoint_txn(verkey, address_1).unwrap();
        assert_eq!(ep_txn_2, expected_ep_txn.replace("\"txnVersion\":1", "\"txnVersion\":2"));

        MLTxnVersion::set(1);
    }

    #[test]
    fn test_add_signature_to_txn() {
        let verkey = "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC";
        let authorisations: Vec<&str> = vec![AUTHZ_ALL];
        let key_txn = TxnBuilder::build_key_txn(verkey, &authorisations).unwrap();
        let signature = "4Be93xNcmaoHzUVK89Qz4aeQg9zMiC2PooegFWEY5aQEfzZo9uNgdjJJDQPj3K5Jj4gE5mERBetqLUBUu6G5cyX2";
        let signed_txn1 = TxnBuilder::add_signature_to_txn(&key_txn, signature).unwrap();
        let expected_key_txn = r#"{"operation":{"authorizations":["all"],"type":"2","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"},"protocolVersion":1,"signature":"4Be93xNcmaoHzUVK89Qz4aeQg9zMiC2PooegFWEY5aQEfzZo9uNgdjJJDQPj3K5Jj4gE5mERBetqLUBUu6G5cyX2","txnVersion":1}"#;
        assert_eq!(signed_txn1, expected_key_txn);

        let address = "https://agent.example.com";
        let ep_txn_1 = TxnBuilder::build_endpoint_txn(verkey, address).unwrap();
        let signature = "5PKvi7TNTaWDiGL1piHQpnnaFbQfRUArBGSKJ6GRBgYV1djWmv3Eff4vhtqJ5Lx3BRqWtgvnWAyNiSHQwcESTuKY";
        let signed_txn2 = TxnBuilder::add_signature_to_txn(&ep_txn_1, signature).unwrap();
        let expected_endpoint_txn = r#"{"operation":{"address":"https://agent.example.com","type":"3","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"},"protocolVersion":1,"signature":"5PKvi7TNTaWDiGL1piHQpnnaFbQfRUArBGSKJ6GRBgYV1djWmv3Eff4vhtqJ5Lx3BRqWtgvnWAyNiSHQwcESTuKY","txnVersion":1}"#;
        assert_eq!(signed_txn2, expected_endpoint_txn);

        let ep_txn_2 = TxnBuilder::build_endpoint_rem_txn(verkey, address).unwrap();
        let signature = "2VGxYYGZKEfTfg6s6JAYatQVyrpmHiHTrB8UcxQkQPAt89ffhTgU3PDqpWrsFd9pAedMoQwB5jZc5mj88tuLZ8mY";
        let signed_txn3 = TxnBuilder::add_signature_to_txn(&ep_txn_2, signature).unwrap();
        let expected_endpoint_rem_txn = r#"{"operation":{"address":"https://agent.example.com","type":"4","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"},"protocolVersion":1,"signature":"2VGxYYGZKEfTfg6s6JAYatQVyrpmHiHTrB8UcxQkQPAt89ffhTgU3PDqpWrsFd9pAedMoQwB5jZc5mj88tuLZ8mY","txnVersion":1}"#;
        assert_eq!(signed_txn3, expected_endpoint_rem_txn);
    }
}