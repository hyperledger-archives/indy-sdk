use std::collections::HashMap;

use byteorder::{LittleEndian, WriteBytesExt};
use serde_json;

use errors::common::CommonError;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::wallet::storage::WalletStorage;
use services::wallet::storage::default::SQLiteStorageType;
use services::wallet::storage::WalletStorageType;
use services::wallet::wallet::EncryptedValue;
use errors::wallet::WalletStorageError;
use services::microledger::microledger::Microledger;
use services::microledger::txn_builder::TxnBuilder;

const TYP: [u8; 3] = [0, 1, 2];

pub struct DidMicroledger {
    pub did: String,
    merkle_tree: MerkleTree,
    storage: Box<WalletStorage>
}

impl Microledger for DidMicroledger where Self: Sized {
    // Creates a persistent ledger in a sqlite file. Loads the sqlite file if found. Uses the DID as the db name
    // Creates an in-memory merkle tree and loads the records from ledger database and populates it
    fn new(did: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let tree = MerkleTree::from_vec(vec![])?;
        // Parse options to see if all required are present
        let parsed_options = DidMicroledger::parse_options(options)?;
        // Create a new storage or load an existing storage
        let storage_path = DidMicroledger::get_storage_path_from_options(&parsed_options);
        let storage = DidMicroledger::get_ledger_storage(did, storage_path).map_err(|err|
            CommonError::InvalidStructure(format!("Error while getting storage for ledger: {:?}.", err)))?;
        let mut ml = DidMicroledger {
            did: did.to_string(),
            merkle_tree: tree,
            storage
        };
        // Build a merkle tree from ledger storage
        ml.populate_merkle_tree()?;
        Ok(ml)
    }

    fn get_root_hash(&self) -> String {
        self.merkle_tree.root_hash_hex()
    }

    fn get_size(&self) -> usize {
        self.merkle_tree.count
    }

    fn add(&mut self, txn: &str) -> Result<usize, CommonError> {
        let txn_bytes = txn.as_bytes().to_vec();
        let txn_bytes_len = txn_bytes.len();
        self.merkle_tree.append(txn_bytes.clone())?;
        // TODO: Fix this, the key should be generated
        let enc = EncryptedValue::new(txn_bytes, vec![0; txn_bytes_len]);
        let new_size = self.get_size();
        let mut wtr: Vec<u8> = Vec::new();
        wtr.write_u64::<LittleEndian>(new_size as u64).unwrap();
        self.storage.add(&TYP, &wtr, &enc, &vec![]).map_err(|err|
            CommonError::InvalidStructure(format!("Error while adding to ledger storage: {:?}.", err)))?;
        Ok(new_size)
    }
}

impl DidMicroledger {
    fn parse_options(options: HashMap<String, String>) -> Result<HashMap<String, String>, CommonError> {
        // TODO: Support inmemory storage type
        match options.get("storage_type") {
            Some(s) => {
                if s != "sqlite" {
                    return Err(CommonError::InvalidStructure(format!("storage_type needs to be sqlite")))
                }
            }
            None => return Err(CommonError::InvalidStructure(format!("storage_type needs to be provided")))
        }
        if options.get("storage_path").is_none() {
            // TODO: Make sure storage path is valid OsString
            return Err(CommonError::InvalidStructure(format!("storage_path needs to be provided")))
        }
        Ok(options)
    }

    // TODO: Temporary, fix it
    fn _metadata() -> Vec<u8> {
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

    pub fn get_ledger_storage(did: &str, storage_path: &str)  -> Result<Box<WalletStorage>, WalletStorageError> {
        let config = json!({
            "path": storage_path
        }).to_string();
        let storage_type = SQLiteStorageType::new();
        match storage_type.create_storage(did, Some(&config), None,
                                    &DidMicroledger::_metadata()) {
            Ok(_) => (),
            Err(WalletStorageError::AlreadyExists) => (),
            Err(e) => return Err(e)
        }
        storage_type.open_storage(did, Some(&config), None)
    }

    pub fn get_storage_path_from_options(parsed_options: &HashMap<String, String>) -> &str {
        parsed_options.get("storage_path").unwrap()
    }

    fn populate_merkle_tree(&mut self) -> Result<(), CommonError> {
        let mut storage_iterator = self.storage.get_all().map_err(|err|
            CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", err)))?;
        while true {
            match storage_iterator.next() {
                Ok(v) => {
                    match v {
                        Some(r) => {
                            match r.value {
                                Some(ev) => {
                                    self.merkle_tree.append(ev.data)?
                                },
                                None => continue
                            }
                            continue
                        }
                        None => break
                    }
                },
                Err(e) => return Err(CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", e)))
            }
        }
        Ok(())
    }

    pub fn add_nym_txn(&mut self, did: &str, verkey: Option<&str>) -> Result<usize, CommonError> {
        let nym_txn = TxnBuilder::build_nym_txn(did, verkey)?;
        self.add(&nym_txn)
    }

    pub fn add_key_txn(&mut self, verkey: &str, authorisations: &Vec<&str>) -> Result<usize, CommonError> {
        let key_txn = TxnBuilder::build_key_txn(verkey, authorisations)?;
        self.add(&key_txn)
    }

    pub fn add_endpoint_txn(&mut self, verkey: &str, address: &str) -> Result<usize, CommonError> {
        let ep_txn = TxnBuilder::build_endpoint_txn(verkey, address)?;
        self.add(&ep_txn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::environment::EnvironmentUtils;
    use utils::test::TestUtils;
    use services::microledger::constants::*;

    fn valid_storage_options() -> HashMap<String, String>{
        let mut options: HashMap<String, String> = HashMap::new();
        let mut path = EnvironmentUtils::tmp_path();
        path.push("did_ml_path");
        let storage_path = path.to_str().unwrap().to_owned();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        options.insert("storage_path".to_string(), storage_path);
        options
    }

    fn get_new_microledger(did: &str) -> DidMicroledger{
        let options = valid_storage_options();
        DidMicroledger::new(did, options).unwrap()
    }

    #[test]
    fn test_parse_valid_options() {
        let options = valid_storage_options();
        let expected_options: HashMap<String, String> = options.clone();
        assert_eq!(DidMicroledger::parse_options(options).unwrap(), expected_options);
    }

    #[test]
    fn test_parse_options_without_required_keys() {
        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        assert!(DidMicroledger::parse_options(options).is_err());

        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_path".to_string(), "storage_path".to_string());
        assert!(DidMicroledger::parse_options(options).is_err());

        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("unknown key".to_string(), "unknown value".to_string());
        assert!(DidMicroledger::parse_options(options).is_err());
    }

    #[test]
    fn test_parse_options_incorrect_storage_type() {
        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_type".to_string(), "mysql".to_string());
        options.insert("storage_path".to_string(), "/tmp".to_string());
        let expected_options: HashMap<String, String> = options.clone();
        assert!(DidMicroledger::parse_options(options).is_err());
    }

    #[test]
    fn test_get_ledger_storage() {
        TestUtils::cleanup_temp();
        let options = valid_storage_options();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let storage = DidMicroledger::get_ledger_storage(
            did, DidMicroledger::get_storage_path_from_options(&options)).unwrap();
        let mut storage_iterator = storage.get_all().unwrap();
        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());

        /*let parsed_options = DidMicroledger::parse_options(options).unwrap();
        let storage_path = DidMicroledger::get_storage_path_from_options(&parsed_options);
        let config = json!({
            "path": storage_path
        }).to_string();
        let storage_type = SQLiteStorageType::new();
        storage_type.delete_storage(did, Some(&config), None).unwrap();*/
    }

    #[test]
    fn test_did_create_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let ml = get_new_microledger(did);
        assert_eq!(ml.did, did);
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        let mut storage_iterator = ml.storage.get_all().unwrap();
        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn test_add_to_did_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut ml = get_new_microledger(did);
        let txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let seq_no = ml.add(txn).unwrap();
        assert_eq!(seq_no, 1);
        assert_eq!(ml.merkle_tree.root_hash_hex(), "f2d6693205eb9af52888e5326522cc5af82866a8761540fa13283e12b690eae3");
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let seq_no_2 = ml.add(txn_2).unwrap();
        assert_eq!(seq_no_2, 2);
        assert_eq!(ml.merkle_tree.root_hash_hex(), "37f096e724a587c37ed15fdba2ad1a6e4b1b5dbf1cd88ea1c1c5e29fd3fd9c44");
    }

    #[test]
    fn test_rebuild_merkle_tree() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let mut root_hash = String::from("");
        let mut size = 0;

        // Create a new microledger and fill it with some txns
        {
            let mut ml = get_new_microledger(did);
            ml.add(txn).unwrap();
            ml.add(txn_2).unwrap();
            ml.add(txn_3).unwrap();
            let s = ml.add(txn_4).unwrap();
            assert_eq!(s, 4);
            let s = ml.get_size();
            assert_eq!(s, 4);
            root_hash = ml.merkle_tree.root_hash_hex();
            size = s;
            ml.storage.close().unwrap();
        }

        // Reload the microledger and see if all size and root hash matches
        let mut ml = get_new_microledger(did);
        assert!(ml.get_size() > 0);
        assert_eq!(ml.get_size(), size);
        assert_eq!(ml.merkle_tree.root_hash_hex(), root_hash);

        // Add txn to the reloaded merkle tree
        let txn_5 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        ml.add(txn_5).unwrap();
        assert_eq!(ml.get_size(), size+1);
        assert_eq!(ml.merkle_tree.root_hash_hex(), "100a9616d3a74b481d74199b36d3e56dbf41f14453a8d99567a795952c12ea48");
    }

    #[test]
    fn test_add_nym_txn() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let mut ml = get_new_microledger(did);
        let s = ml.add_nym_txn(did, Some(verkey)).unwrap();
        assert_eq!(s, 1);
    }

    #[test]
    fn test_add_key_txn() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let authorisations: Vec<&str> = vec![AUTHZ_ALL, AUTHZ_ADD_KEY, AUTHZ_REM_KEY];
        let mut ml = get_new_microledger(did);
        let s = ml.add_key_txn(verkey, &authorisations).unwrap();
        assert_eq!(s, 1);
    }

    #[test]
    fn test_add_endpoint_txn() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let address = "https://agent.example.com";
        let mut ml = get_new_microledger(did);
        let s = ml.add_endpoint_txn(verkey, address).unwrap();
        assert_eq!(s, 1);
    }
}