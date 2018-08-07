use std::collections::HashMap;
use std::str;
use std::marker::PhantomData;
use std::collections::HashSet;

use serde_json;
use serde_json::Value as JValue;
use serde_json::Map;

use errors::common::CommonError;
use services::wallet::storage::WalletStorage;
use services::crypto::CryptoService;
use domain::ledger::constants::NYM;
use services::wallet::language::{Operator, TagName, TargetValue};
use services::wallet::storage::Tag;
use services::wallet::wallet::EncryptedValue;
use services::wallet::storage::StorageRecord;
use services::microledger::auth::Auth;
use services::microledger::helpers::parse_options;
use services::microledger::helpers::get_storage_path_from_options;
use services::microledger::helpers::get_rsm_storage;
use services::microledger::helpers::{create_storage_options, gen_enc_key};
use services::microledger::view::View;
use services::microledger::txn_builder::Txn;
use services::microledger::did_microledger::DidMicroledger;
use services::microledger::constants::{KEY_TXN, ENDPOINT_TXN, ENDPOINT_REM_TXN, VERKEY,
                                       AUTHORIZATIONS, ADDRESS, ENDPOINTS, AUTHZ_ALL, AUTHZ_ADD_KEY,
                                       AUTHZ_REM_KEY, IDENTIFIER, SIGNATURE};

const TYP: [u8; 3] = [1, 2, 3];

pub struct DidDoc<'a> {
    pub did: String,
    storage: Box<WalletStorage>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> View for DidDoc<'a> where Self: Sized {
    // initialize
    fn new(did: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let parsed_options = parse_options(options)?;
        // Create a new storage or load an existing storage
        let storage_path = get_storage_path_from_options(&parsed_options);
        let storage = get_rsm_storage(did, storage_path,
                                      &DidDoc::get_metadata()).map_err(|err|
            CommonError::InvalidStructure(format!("Error while getting storage for ledger: {:?}.", err)))?;
        Ok(DidDoc {
            did: did.to_string(),
            storage,
            phantom: PhantomData
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
                                        NYM => {
                                            println!("Encountered NYM txn to apply. Doing nothing");
                                            Ok(())
                                        },
                                        KEY_TXN => {
                                            println!("Encountered KEY_TXN txn to apply.");
                                            self.add_key_from_txn(&op)
                                        }
                                        ENDPOINT_TXN => {
                                            println!("Encountered ENDPOINT_TXN txn to apply.");
                                            self.add_endpoint_from_txn(&op)
                                        }
                                        ENDPOINT_REM_TXN => {
                                            println!("Encountered ENDPOINT_REM_TXN txn to apply.");
                                            self.remove_endpoint_from_txn(&op)
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

impl<'a> DidDoc<'a> {
    pub fn create_options(base_storage_path: Option<&str>, extra_path: Option<&str>) -> HashMap<String, String> {
        let mut extra_paths: Vec<&str> = vec![];
        if extra_path.is_some() {
            extra_paths.push(extra_path.unwrap())
        }
        extra_paths.push("did_doc_path");
        create_storage_options(base_storage_path, extra_paths)
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

    pub fn get_key_and_authz_from_operation(operation: &JValue) -> Result<(String, Vec<String>), CommonError> {
        match (operation.get(VERKEY), operation.get(AUTHORIZATIONS)) {
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

    pub fn get_key_and_endpoint_from_operation(operation: &JValue) -> Result<(String, String), CommonError> {
        match (operation.get(VERKEY), operation.get(ADDRESS)) {
            (Some(vk), Some(ep)) => match (vk.as_str(), ep.as_str()) {
                (Some(v), Some(e)) => {
                    Ok((v.to_string(), e.to_string()))
                },
                _ => return Err(CommonError::InvalidStructure(String::from("Cannot convert verkey/address to string")))
            },
            _ => Err(CommonError::InvalidStructure(String::from("Both verkey and address are needed")))
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

    fn new_key_entry(key: String, authorisations: Vec<String>, endpoint: Option<HashMap<String, JValue>>) -> Result<String, CommonError> {
        let mut m: Map<String, JValue> = Map::new();
        m.insert(AUTHORIZATIONS.to_string(), JValue::from(authorisations));
        let ep = match endpoint {
            Some(ref e) => {
                serde_json::to_value(e).map_err(|err|
                    CommonError::InvalidStructure(format!("Failed to jsonify: {:?}.", err)))?
            }
            None => {
                let m: HashMap<String, JValue> = HashMap::new();
                serde_json::to_value(m).map_err(|err|
                    CommonError::InvalidStructure(format!("Failed to jsonify: {:?}.", err)))?
            }
        };

        m.insert(ENDPOINTS.to_string(), ep);
        serde_json::to_string(&m).map_err(|err|
            CommonError::InvalidStructure(format!("Failed to jsonify: {:?}.", err)))
    }

    fn get_key_entry(&self, verkey: &str) -> Result<Option<StorageRecord>, CommonError> {
        /*let query = Operator::Eq(
            TagName::PlainTagName(VERKEY.as_bytes().to_vec()),
            TargetValue::Unencrypted(verkey.to_string())
        );

        let mut storage_iterator = self.storage.search(&TYP, &query,
                                                       Some(&DidDoc::get_search_options())).map_err(|err|
            CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", err)))?;
        storage_iterator.next().map_err(|e| CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", e)))*/
        // TODO: Use `storage.search` instead of `storage.get_all`
        let mut storage_iterator = self.storage.get_all().map_err(|e|
            CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", e)))?;
        let mut entry:Option<StorageRecord> = None;

        loop {
            match storage_iterator.next() {
                Ok(v) => {
                    match v {
                        Some(r) => {
                            let vk = str::from_utf8(&r.id).unwrap();
                            if vk == verkey {
                                entry = Some(r.clone());
                                break
                            }
                        }
                        None => break
                    }
                },
                Err(e) => return Err(CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", e)))
            }
        }
        Ok(entry)
    }

    pub fn add_key_from_txn(&mut self, operation: &JValue) -> Result<(), CommonError> {
        let (verkey, auths) = DidDoc::get_key_and_authz_from_operation(operation)?;
        let key_entry = self.get_key_entry(&verkey)?;
        match key_entry {
            Some(r) => {
                match r.value {
                    Some(ev) => {
                        let mut val: JValue = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                        val[AUTHORIZATIONS] = JValue::from(auths);
                        let data: String = serde_json::to_string(&val).map_err(|err|
                            CommonError::InvalidStructure(format!("Error jsonifying : {:?}.", err)))?;
                        let enc_data = EncryptedValue {data: data.as_bytes().to_vec(), key: ev.key.clone()};
                        println!("Updating existing verkey {} due to key txn", &verkey);
                        self.storage.update(&TYP, &r.id, &enc_data).map_err(|err|
                            CommonError::InvalidStructure(format!("Error while updating to DID doc storage: {:?}.", err)))
                    },
                    None => Err(CommonError::InvalidStructure(format!("No value found in record")))
                }
            }
            None => {
                let id = verkey.as_bytes();
                let tags: [Tag; 1] = [Tag::PlainText(id.to_vec(), verkey.clone()), ];
                let key_entry = DidDoc::new_key_entry(verkey.clone(), auths, None)?;
                let key_entry_bytes = key_entry.as_bytes().to_vec();
                let enc_key = gen_enc_key(key_entry_bytes.len());
                let enc_data = EncryptedValue::new(key_entry_bytes, enc_key);
                println!("Adding new verkey {} due to key txn", &verkey);
                self.storage.add(&TYP, &id, &enc_data, &tags).map_err(|err|
                    CommonError::InvalidStructure(format!("Error while adding to DID doc storage: {:?}.", err)))
            }
        }
    }

    pub fn add_endpoint_from_txn(&mut self, operation: &JValue) -> Result<(), CommonError> {
        self.update_endpoint(operation, &mut DidDoc::add_endpoint_in_json)
    }

    pub fn remove_endpoint_from_txn(&mut self, operation: &JValue) -> Result<(), CommonError> {
        self.update_endpoint(operation, &mut DidDoc::remove_endpoint_from_json)
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
        serde_json::to_string(&res).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify ledger udpdate message {:?}.", err)))
    }

    // Checks if DID doc has a particular verkey
    pub fn has_key(&self, verkey: &str) -> Result<bool, CommonError> {
        let key_entry = self.get_key_entry(&verkey)?;
        Ok(key_entry.is_some())
    }

    // Get the list of authorizations of a particular verkey
    pub fn get_key_authorisations(&self, verkey: &str) -> Result<Vec<String>, CommonError> {
        let key_entry = self.get_key_entry(&verkey)?;
        match key_entry {
            Some(r) => {
                match r.value {
                    Some(ev) => {
                        let val: JValue = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                        let auths: Vec<String> = serde_json::from_value(val[AUTHORIZATIONS].clone()).map_err(|err|
                            CommonError::InvalidStructure(format!("Cannot convert authorisations to vector of strings : {:?}.", err)))?;
                        let authz_all = AUTHZ_ALL.to_string();
                        if auths.contains(&authz_all) {
                            Ok(vec![authz_all])
                        } else {
                            Ok(auths)
                        }
                    },
                    None => Err(CommonError::InvalidStructure(format!("No value found in record")))
                }
            }
            None => Err(CommonError::InvalidStructure(format!("Key not found: {}", verkey)))
        }
    }

    // Get the list of endpoints (addresses) of a particular verkey
    pub fn get_key_endpoints(&self, verkey: &str) -> Result<Vec<String>, CommonError> {
        let key_entry = self.get_key_entry(&verkey)?;
        match key_entry {
            Some(r) => {
                match r.value {
                    Some(ev) => {
                        let val: JValue = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                        let endpoints: Map<String, JValue> = serde_json::from_value(val[ENDPOINTS].clone()).map_err(|err|
                            CommonError::InvalidStructure(format!("Cannot convert endpoints to vector of strings : {:?}.", err)))?;
                        let mut addresses: Vec<String> = vec![];
                        for k in endpoints.keys() {
                            addresses.push(k.to_string())
                        }
                        Ok(addresses)
                    },
                    None => {
                        let e_msg = format!("No value found in record for key {}", verkey);
                        println!("{}", &e_msg);
                        Err(CommonError::InvalidStructure(e_msg))
                    }
                }
            }
            None => {
                let e_msg = format!("Key not found: {}", verkey);
                println!("{}", &e_msg);
                Err(CommonError::InvalidStructure(e_msg))
            }
        }
    }

    // Get the list of keys with a particular authorisation
    pub fn get_keys_by_authorisation(&self, authz: &str) -> Result<Vec<String>, CommonError> {
        if !Auth::is_valid_auth(authz) {
            return Err(CommonError::InvalidStructure(format!("Invalid auth {}", authz)))
        }
        let mut res: Vec<String> = vec![];

        let mut storage_iterator = self.storage.get_all().map_err(|err|
            CommonError::InvalidStructure(format!("Error getting DID doc storage iterator: {:?}.", err)))?;
        let all_possible_auths = Auth::get_all();
        loop {
            match storage_iterator.next() {
                Ok(v) => {
                    match v {
                        Some(r) => {
                            match r.value {
                                Some(ev) => {
                                    let vk = str::from_utf8(&r.id).unwrap().to_string();
                                    let val: Map<String, JValue> = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                                    let auths: Vec<String> = serde_json::from_value(val[AUTHORIZATIONS].clone()).map_err(|err|
                                        CommonError::InvalidStructure(format!("Cannot convert authorisations to vector of strings : {:?}.", err)))?;
                                    let auths: HashSet<String> = auths.iter().cloned().collect();
                                    if auths.contains(authz) || auths.contains(AUTHZ_ALL) || (auths == all_possible_auths) {
                                        res.push(vk);
                                    }
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
        Ok(res)
    }

    pub fn is_valid_txn(txn: &str, did_doc: &DidDoc, crypto_service: &CryptoService) -> Result<bool, CommonError> {
        let mut j_txn: JValue = serde_json::from_str(txn).map_err(|err|
            CommonError::InvalidState(format!("Unable to parse json txn {:?}.", err)))?;
        match j_txn.as_object_mut() {
            Some(m_txn) => {
                let idr = m_txn.remove(IDENTIFIER);
                let sig = m_txn.remove(SIGNATURE);
                let j_txn = JValue::from(m_txn.clone());
                let txn: Txn<JValue> = serde_json::from_value(j_txn).map_err(|err|
                    CommonError::InvalidState(format!("Unable to create txn {:?}.", err)))?;
                let type_ = txn.operation.get("type");
                match type_ {
                    Some(t) => {
                        match t.as_str() {
                            Some(KEY_TXN) => {
                                if !DidMicroledger::is_valid_key_txn_schema(&txn.operation) {
                                    return Ok(false)
                                }
                                let j_vk = idr.unwrap();
                                if !DidMicroledger::is_valid_txn_sig(crypto_service, txn.clone(), j_vk.clone(), sig.unwrap())? {
                                    return Ok(false)
                                }
                                let vk = j_vk.as_str().unwrap();
                                if !DidDoc::is_valid_key_txn_auth(&txn.operation, vk, did_doc)? {
                                    return Ok(false)
                                }
                                Ok(true)
                            }
                            Some(ENDPOINT_TXN) => {
                                if !DidMicroledger::is_valid_endpoint_txn_schema(&txn.operation) {
                                    return Ok(false)
                                }
                                let j_vk = idr.unwrap();
                                if !DidMicroledger::is_valid_txn_sig(crypto_service, txn.clone(), j_vk.clone(), sig.unwrap())? {
                                    return Ok(false)
                                }
                                let vk = j_vk.as_str().unwrap();
                                if !DidDoc::is_valid_endpoint_txn_auth(&txn.operation, vk, did_doc)? {
                                    return Ok(false)
                                }
                                Ok(true)
                            }
                            _ => Err(CommonError::InvalidStructure(format!("Unknown txn type {:?}", t)))
                        }
                    },
                    None => Err(CommonError::InvalidStructure(String::from("Unable to find type in txn")))
                }
            }
            None => Err(CommonError::InvalidStructure(String::from("Unable to convert to json object")))
        }
    }

    pub fn is_valid_key_txn_auth(operation: &JValue, txn_author_vk: &str,
                                 did_doc: &DidDoc) -> Result<bool, CommonError> {
        let subject_vk = operation.get(VERKEY).unwrap();
        let subject_vk = subject_vk.as_str().unwrap();
        match did_doc.has_key(subject_vk)? {
            true => {
                let proposed_auths = operation.get(AUTHORIZATIONS).unwrap();
                let proposed_auths: Vec<String> = serde_json::from_value(proposed_auths.clone()).map_err(|err|
                    CommonError::InvalidStructure(format!("Cannot convert authorisations to vector of strings : {:?}.", err)))?;
                if proposed_auths.is_empty() {
                    if subject_vk == txn_author_vk {
                        return Ok(true)
                    }
                    match did_doc.get_key_authorisations(txn_author_vk) {
                        Ok(auths) => {
                            if !(auths.contains(&AUTHZ_ALL.to_string()) || auths.contains(&AUTHZ_REM_KEY.to_string())) {
                                return Ok(false)
                            }
                        }
                        Err(e) => {
                            println!("Cannot get authorisations for key {}. Error: {:?}", txn_author_vk, e);
                            return Ok(false)
                        }
                    }
                } else {
                    // TODO
                }
                Ok(true)
            }
            false => {
                match did_doc.get_key_authorisations(txn_author_vk) {
                    Ok(auths) => {
                        if !(auths.contains(&AUTHZ_ALL.to_string()) || auths.contains(&AUTHZ_ADD_KEY.to_string())) {
                            return Ok(false)
                        }
                    }
                    Err(e) => {
                        println!("Cannot get authorisations for key {}. Error: {:?}", txn_author_vk, e);
                        return Ok(false)
                    }
                }
                Ok(true)
            }
        }
    }

    pub fn is_valid_endpoint_txn_auth(operation: &JValue, txn_author_vk: &str,
                                      did_doc: &DidDoc) -> Result<bool, CommonError> {
        let subject_vk = operation.get(VERKEY).unwrap();
        let subject_vk = subject_vk.as_str().unwrap();
        match did_doc.get_key_endpoints(subject_vk) {
            Ok(endpoints) => {
                if subject_vk == txn_author_vk {
                    return Ok(true)
                }
                if endpoints.is_empty() {
                    match did_doc.get_key_authorisations(txn_author_vk) {
                        Ok(auths) => {
                            if !(auths.contains(&AUTHZ_ALL.to_string()) || auths.contains(&AUTHZ_ADD_KEY.to_string())) {
                                return Ok(false)
                            }
                        }
                        Err(e) => {
                            println!("Cannot get authorisations for key {}. Error: {:?}", txn_author_vk, e);
                            return Ok(false)
                        }
                    }
                } else {
                    // TODO
                }
                Ok(true)
            }
            _ => {
                println!("Cannot add endpoint for non-existent key {}", subject_vk);
                Ok(false)
            }
        }
    }

    fn extract_endpoints(key_entry: &mut JValue) -> Map<String, JValue> {
        match key_entry.get(ENDPOINTS) {
            Some(v) => {
                v.as_object().unwrap().clone()
            }
            None => {
                let m: Map<String, JValue> = Map::new();
                m
            }
        }
    }

    fn add_endpoint_in_json(key_entry: &mut JValue, endpoint: String) -> Result<JValue, CommonError> {
        let mut endpoints = DidDoc::extract_endpoints(key_entry);
        endpoints.insert(endpoint, JValue::Object(Map::new()));
        serde_json::to_value(endpoints).map_err(|e|
            CommonError::InvalidStructure(format!("Error jsonifying : {:?}.", e)))
    }

    fn remove_endpoint_from_json(key_entry: &mut JValue, endpoint: String) -> Result<JValue, CommonError> {
        let mut endpoints = DidDoc::extract_endpoints(key_entry);
        endpoints.remove(&endpoint);
        serde_json::to_value(endpoints).map_err(|e|
            CommonError::InvalidStructure(format!("Error jsonifying : {:?}.", e)))
    }

    fn update_endpoint(&self, operation: &JValue,
                       update_func: &mut FnMut(&mut JValue, String) -> Result<JValue, CommonError>) -> Result<(), CommonError> {
        let (verkey, endpoint) = DidDoc::get_key_and_endpoint_from_operation(operation)?;
        let key_entry = self.get_key_entry(&verkey)?;
        match key_entry {
            Some(r) => {
                match r.value {
                    Some(ev) => {
                        let mut val: JValue = serde_json::from_str(&str::from_utf8(&ev.data).unwrap().to_string()).unwrap();
                        val[ENDPOINTS.to_string()] = update_func(&mut val, endpoint)?;
                        let data: String = serde_json::to_string(&val).map_err(|err|
                            CommonError::InvalidStructure(format!("Error jsonifying : {:?}.", err)))?;
                        let enc_data = EncryptedValue {data: data.as_bytes().to_vec(), key: ev.key.clone()};
                        self.storage.update(&TYP, &r.id, &enc_data).map_err(|err|
                            CommonError::InvalidStructure(format!("Error while updating to DID doc storage: {:?}.", err)))?;
                        Ok(())
                    },
                    None => Err(CommonError::InvalidStructure(format!("No value found in record")))
                }
            }
            None => {
                Err(CommonError::InvalidStructure(format!("Key txn not present for: {}.", &verkey)))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::test::TestUtils;
    use services::microledger::helpers::tests::{valid_did_doc_storage_options, check_empty_storage, get_new_did_doc};
    use services::microledger::constants::AUTHZ_MPROX;
    use services::microledger::constants::AUTHZ_ADD_KEY;
    use services::microledger::constants::AUTHZ_REM_KEY;

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
        let expected_did_doc_2 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["add_key","rem_key"]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_2);

        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["rem_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_3).unwrap();
        let expected_did_doc_3 = r#"{"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW":{"authorizations":["rem_key"]}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_3);
    }

    #[test]
    fn test_add_rem_endpoint_txns_in_did_doc() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        let key_txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(key_txn_1).unwrap();

        let end_point_txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_1).unwrap();
        let expected_did_doc_1 = r#"{"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"],"endpoints":{"https://agent.example.com":{}}}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_1);

        let end_point_txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent2.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_2).unwrap();
        let expected_did_doc_2 = r#"{"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"],"endpoints":{"https://agent.example.com":{},"https://agent2.example.com":{}}}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_2);

        let end_point_txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent2.example.com","type":"4","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_3).unwrap();
        let expected_did_doc_3 = r#"{"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"],"endpoints":{"https://agent.example.com":{}}}}"#;
        assert_eq!(doc.as_json().unwrap(), expected_did_doc_3);

        let key_txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(key_txn_2).unwrap();

        let end_point_txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent3.example.com","type":"3","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(end_point_txn_4).unwrap();

        let expected_did_doc_4 = r#"{"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB":{"authorizations":["all"],"endpoints":{"https://agent3.example.com":{}}},"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"],"endpoints":{"https://agent.example.com":{}}}}"#;
        let expected_did_doc_4_1 = r#"{"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1":{"authorizations":["all"],"endpoints":{"https://agent.example.com":{}}},"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB":{"authorizations":["all"],"endpoints":{"https://agent3.example.com":{}}}}"#;
        assert_eq!((doc.as_json().unwrap() == expected_did_doc_4) || (doc.as_json().unwrap() == expected_did_doc_4_1), true);
    }

    #[test]
    fn test_has_keys() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        let txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_1).unwrap();
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(txn_2).unwrap();
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key","rem_key"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(txn_3).unwrap();

        assert!(doc.has_key("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW").unwrap());
        assert!(doc.has_key("6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1").unwrap());
        assert!(doc.has_key("46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB").unwrap());
        assert!(!doc.has_key("4Yk9HoDSfJv9QcmJbLcXdWVgS7nfvdUqiVcvbSu8VBru").unwrap());
    }

    #[test]
    fn test_get_key_authorisations() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        let txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key"],"type":"2","verkey":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"}}"#;
        doc.apply_txn(txn_1).unwrap();
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(txn_2).unwrap();
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key","rem_key"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(txn_3).unwrap();
        let txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["rem_key"],"type":"2","verkey":"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"}}"#;
        doc.apply_txn(txn_4).unwrap();
        let txn_5 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["mprox"],"type":"2","verkey":"3znAGhp6Tk4kmebhXnk9K3jaTMffu82PJfEG91AeRkq2"}}"#;
        doc.apply_txn(txn_5).unwrap();
        let txn_6 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all","add_key","rem_key"],"type":"2","verkey":"84hpoYb2cgCo4d5D2b5s7khE7SoHAJCLQNbfu1NsQNWy"}}"#;
        doc.apply_txn(txn_6).unwrap();

        assert_eq!(doc.get_key_authorisations("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW").unwrap(), vec!["add_key"]);
        assert_eq!(doc.get_key_authorisations("6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1").unwrap(), vec!["all"]);
        assert_eq!(doc.get_key_authorisations("46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB").unwrap(), vec!["add_key","rem_key"]);
        assert_eq!(doc.get_key_authorisations("4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ").unwrap(), vec!["rem_key"]);
        assert_eq!(doc.get_key_authorisations("3znAGhp6Tk4kmebhXnk9K3jaTMffu82PJfEG91AeRkq2").unwrap(), vec!["mprox"]);
        assert_eq!(doc.get_key_authorisations("84hpoYb2cgCo4d5D2b5s7khE7SoHAJCLQNbfu1NsQNWy").unwrap(), vec!["all"]);

        let txn_7 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key","mprox"],"type":"2","verkey":"3znAGhp6Tk4kmebhXnk9K3jaTMffu82PJfEG91AeRkq2"}}"#;
        doc.apply_txn(txn_7).unwrap();
        assert_eq!(doc.get_key_authorisations("3znAGhp6Tk4kmebhXnk9K3jaTMffu82PJfEG91AeRkq2").unwrap(), vec!["add_key","mprox"]);
    }

    #[test]
    fn test_get_key_endpoints() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        let key_txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(key_txn_1).unwrap();
        let key_txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(key_txn_2).unwrap();

        let end_point_txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_1).unwrap();
        // Getting endpoint for existent key passes
        assert_eq!(doc.get_key_endpoints("6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1").unwrap(), vec!["https://agent.example.com"]);

        // Getting endpoint for non-existent key fails
        assert!(doc.get_key_endpoints("41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74").is_err());

        // Getting endpoints for more than 1 endpoint
        let end_point_txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent2.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_2).unwrap();
        assert_eq!(doc.get_key_endpoints("6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1").unwrap(), vec!["https://agent.example.com",
                                                                                                        "https://agent2.example.com"]);

        // Getting endpoints after 1 endpoint removed
        let end_point_txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent2.example.com","type":"4","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(end_point_txn_3).unwrap();
        assert_eq!(doc.get_key_endpoints("6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1").unwrap(), vec!["https://agent.example.com"]);

        let end_point_txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent3.example.com","type":"3","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(end_point_txn_4).unwrap();
        assert_eq!(doc.get_key_endpoints("46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB").unwrap(), vec!["https://agent3.example.com"]);

        // Getting endpoints when no endpoints
        let end_point_txn_5 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent3.example.com","type":"4","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(end_point_txn_5).unwrap();
        let empty_str_vec: Vec<String> = vec![];
        assert_eq!(doc.get_key_endpoints("46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB").unwrap(), empty_str_vec);
    }

    #[test]
    fn test_get_keys_by_authorisation() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut doc = get_new_did_doc(did);
        let key_txn_1 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(key_txn_1).unwrap();
        let key_txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key"],"type":"2","verkey":"46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB"}}"#;
        doc.apply_txn(key_txn_2).unwrap();
        let key_txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key", "rem_key"],"type":"2","verkey":"41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74"}}"#;
        doc.apply_txn(key_txn_3).unwrap();
        let key_txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key", "rem_key", "mprox"],"type":"2","verkey":"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"}}"#;
        doc.apply_txn(key_txn_4).unwrap();

        assert!(doc.get_keys_by_authorisation("Some_incorrect_authz").is_err());
        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_ADD_KEY).unwrap(), vec!["6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                                                                               "46Kq4hASUdvUbwR7s7Pie3x8f4HRB3NLay7Z9jh9eZsB",
                                                                               "41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74",
                                                                               "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"]);
        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_REM_KEY).unwrap(), vec!["6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                                                                               "41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74",
                                                                               "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"]);
        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_MPROX).unwrap(), vec!["6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                                                                             "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"]);
        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_ALL).unwrap(), vec!["6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                                                                               "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"]);

        let key_txn_5 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key", "rem_key"],"type":"2","verkey":"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"}}"#;
        doc.apply_txn(key_txn_5).unwrap();

        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_MPROX).unwrap(), vec!["6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"]);

        let key_txn_6 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["add_key", "rem_key"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        doc.apply_txn(key_txn_6).unwrap();

        let empty_str_vec: Vec<String> = vec![];
        assert_eq!(doc.get_keys_by_authorisation(AUTHZ_MPROX).unwrap(), empty_str_vec);
    }
}