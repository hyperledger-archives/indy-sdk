use std::collections::HashMap;
use std::str;

use serde_json;
use serde_json::Value as JValue;

use errors::common::CommonError;
use services::wallet::storage::WalletStorage;
use services::microledger::helpers::parse_options;
use services::microledger::helpers::get_storage_path_from_options;
use services::microledger::helpers::get_ledger_storage;
use services::microledger::helpers::{create_storage_options, gen_enc_key};
use services::microledger::view::View;
use serde_json::Map;
use services::microledger::constants::{KEY_TXN, ENDPOINT_TXN, VERKEY, AUTHORIZATIONS};
use services::wallet::language::{Operator, TagName, TargetValue};
use services::wallet::storage::Tag;
use services::microledger::auth::Auth;
use services::wallet::wallet::EncryptedValue;

const TYP: [u8; 3] = [1, 2, 3];

pub struct DidDoc {
    pub did: String,
    storage: Box<WalletStorage>,
}

impl View for DidDoc where Self: Sized {
    // initialize
    fn new(name: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let parsed_options = parse_options(options)?;
        // Create a new storage or load an existing storage
        let storage_path = get_storage_path_from_options(&parsed_options);
        let storage = get_ledger_storage(name, storage_path,
                                         &DidDoc::get_metadata()).map_err(|err|
            CommonError::InvalidStructure(format!("Error while getting storage for ledger: {:?}.", err)))?;
        Ok(DidDoc {
            did: name.to_string(),
            storage
        })
    }

    fn apply_txn(&mut self, txn: &str) -> Result<(), CommonError> {
        let j: JValue = serde_json::from_str(txn).map_err(|err|
            CommonError::InvalidStructure(format!("Unable to parse txn {:?}.", err)))?;
        let op_val = j.get("operation").clone();
        match op_val {
            Some(op) => {
                match op.is_object() {
                    true => {
                        let t_val = op.get("type").clone();
                        match t_val {
                            Some(t) => {
                                match t.as_str() {
                                    Some(typ) => match typ {
                                        KEY_TXN => {
                                            self.add_key_from_txn(&op)
                                        }
                                        ENDPOINT_TXN => {
                                            self.add_endpoint_from_txn(&op)
                                        }
                                        _ => Err(CommonError::InvalidState(format!("Unknown txn type {}", typ)))
                                    }
                                    None => Err(CommonError::InvalidStructure(String::from("type is not string")))
                                }
                            }
                            None => Err(CommonError::InvalidStructure(String::from("Did not find type in txn")))
                        }
                    }
                    false => return Err(CommonError::InvalidStructure(String::from("operation is not string")))
                }
            }
            None => Err(CommonError::InvalidStructure(String::from("Did not find operation in txn")))
        }
    }
}

impl DidDoc {
    pub fn create_options(storage_path: Option<&str>) -> HashMap<String, String> {
        create_storage_options(storage_path, vec!["did_doc_path"])
    }

    // TODO: Temporary, fix it
    fn get_metadata() -> Vec<u8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }

    fn get_key_and_authz_from_txn(txn: &JValue) -> Result<(String, Vec<String>), CommonError> {
        match (txn.get(VERKEY), txn.get(AUTHORIZATIONS)) {
            (Some(vk), Some(ath)) => match (vk.as_str(), ath.as_array()) {
                (Some(v), Some(a)) => {
                    let mut auths: Vec<String> = vec![];
                    for ath in a {
                        let s = ath.as_str();
                        match s {
                            Some(s) => {
                                if !Auth::is_valid_auth(s) {
                                    return Err(CommonError::InvalidStructure(format!("Invalid auth {}", s)))
                                }
                                auths.push(s.to_string());
                            },
                            None => return Err(CommonError::InvalidStructure(String::from("Cannot convert authorisation to string")))
                        }
                    }
                    Ok((v.to_string(), auths))
                }
                _ => Err(CommonError::InvalidStructure(String::from("Verkey and authorisation are of incorrect type")))
            }
            _ => Err(CommonError::InvalidStructure(String::from("Both verkey and authorisation are needed")))
        }
    }

    fn get_search_options() -> String {
        let mut map = HashMap::new();

        map.insert("retrieveRecords", true);
        map.insert("retrieveTotalCount", true);
        map.insert("retrieveValue", true);
        map.insert("retrieveTags", true);
        map.insert("retrieveType", true);

        serde_json::to_string(&map).unwrap()
    }

    fn new_key_entry(key: String, authorisations: Vec<String>) -> Result<String, CommonError> {
        let v = json!({
            AUTHORIZATIONS: authorisations
        });
        println!(">>>>>>>>2 {:?}", &v);
        serde_json::to_string(&v).map_err(|err|
            CommonError::InvalidStructure(format!("Failed to jsonify: {:?}.", err)))
    }

    pub fn add_key_from_txn(&mut self, txn: &JValue) -> Result<(), CommonError> {
        let (verkey, auths) = DidDoc::get_key_and_authz_from_txn(txn)?;
        let query = Operator::Eq(
            TagName::PlainTagName(VERKEY.as_bytes().to_vec()),
            TargetValue::Unencrypted(verkey.clone())
        );
        let mut storage_iterator = self.storage.search(&TYP, &query,
                                                       Some(&DidDoc::get_search_options())).map_err(|err|
            CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", err)))?;
        match storage_iterator.next() {
            // TODO: Check if more than 1 entry for key exists
            Ok(v) => match v {
                Some(r) => {
                    match r.value {
                        Some(ev) => {
                            let mut val: JValue = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                            val[AUTHORIZATIONS] = JValue::from(auths);
                            let data: String = serde_json::from_value(val).map_err(|err|
                                CommonError::InvalidStructure(format!("Error jsonifying : {:?}.", err)))?;
                            let enc_data = EncryptedValue {data: data.as_bytes().to_vec(), key: ev.key.clone()};
                            self.storage.update(&TYP, &r.id, &enc_data).map_err(|err|
                                CommonError::InvalidStructure(format!("Error while updating to DID doc storage: {:?}.", err)))
                        },
                        None => Err(CommonError::InvalidStructure(format!("No value found in record")))
                    }
                }
                None => {
                    let id = verkey.as_bytes();
                    let tags: [Tag; 1] = [Tag::PlainText(id.to_vec(), verkey.clone()), ];
                    let key_entry = DidDoc::new_key_entry(verkey.clone(), auths)?;
                    println!(">>>>>>3 {}", &key_entry);
                    let key_entry_bytes = key_entry.as_bytes().to_vec();
                    let enc_key = gen_enc_key(key_entry_bytes.len());
                    let enc_data = EncryptedValue::new(key_entry_bytes, enc_key);
                    self.storage.add(&TYP, &id, &enc_data, &tags).map_err(|err|
                        CommonError::InvalidStructure(format!("Error while adding to DID doc storage: {:?}.", err)))
                }
            }
            Err(e) => Err(CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", e)))
        }
    }

    pub fn add_endpoint_from_txn(&mut self, txn: &JValue) -> Result<(), CommonError> {
        Ok(())
    }

    pub fn as_json(&self) -> Result<String, CommonError> {
        let mut res: HashMap<String, JValue> = HashMap::new();

        let mut storage_iterator = self.storage.get_all().map_err(|err|
            CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", err)))?;
        loop {
            match storage_iterator.next() {
                Ok(v) => {
                    match v {
                        Some(r) => {
                            match r.value {
                                Some(ev) => {
                                    let vk = str::from_utf8(&r.id).unwrap().to_string();
                                    let val = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                                    println!(">>>>>>4 {:?}", &val);
                                    res.insert(vk, val);
                                },
                                None => continue
                            }
                        }
                        None => break
                    }
                },
                Err(e) => return Err(CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", e)))
            }
        }
        println!(">>>>>>>>1 {:?}", &res);
        serde_json::to_string(&res).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify ledger udpdate message {:?}.", err)))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::test::TestUtils;
    use services::microledger::helpers::tests::{valid_did_doc_storage_options, check_empty_storage, get_new_did_doc};

    #[test]
    fn test_setup_did_doc() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let options = valid_did_doc_storage_options();
        let doc = DidDoc::new(did, options).unwrap();
        assert_eq!(doc.did, did);
        check_empty_storage(doc.storage)
    }

    #[test]
    fn test_apply_invalid_txn() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);

        // Invalid JSON
        let invalid_txn_json_1 = r#"{"protocolVersion","txnVersion":1,"operation":{"authorizations":"all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        assert!(doc.apply_txn(invalid_txn_json_1).is_err());

        // No type field
        let invalid_txn_json_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        assert!(doc.apply_txn(invalid_txn_json_2).is_err());

        // Invalid type value
        let invalid_txn_json_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"9011","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        assert!(doc.apply_txn(invalid_txn_json_3).is_err());

        // No operation field
        let invalid_txn_json_4 = r#"{"protocolVersion":1,"txnVersion":1,"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}"#;
        assert!(doc.apply_txn(invalid_txn_json_4).is_err());
    }

    #[test]
    fn test_add_new_keys_in_did_doc() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        assert_eq!(doc.as_json().unwrap(), "{}");

        let txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(txn_1).unwrap();
        let expected_did_doc_1 = r#"{"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_1);
        /*let keys: HashMap<String, Vec<String>> = doc.get_all_keys();
        assert_eq!(&keys, );*/

        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_2).unwrap();
        let expected_did_doc_2 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["add_key"]},"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_2);

        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["rem_key"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(txn_3).unwrap();
        let expected_did_doc_3 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["add_key"]},"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"]},"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB":{"authorizations":["rem_key"]}}"#;;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_2);
    }

    #[test]
    fn test_update_old_keys_in_did_doc() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);

        let txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_1).unwrap();
        let expected_did_doc_1 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["add_key"]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_1);

        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key","rem_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_2).unwrap();
        let expected_did_doc_2 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["add_key","rem_key]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_2);

        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["rem_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_3).unwrap();
        let expected_did_doc_3 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["rem_key]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_3);
    }

    #[test]
    fn test_add_endpoint_txns_did_doc() {

    }
}