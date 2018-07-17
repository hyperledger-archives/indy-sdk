use std::collections::HashMap;

use serde_json;
use errors::common::CommonError;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::wallet::storage::WalletStorage;
use services::wallet::storage::default::SQLiteStorageType;
use services::wallet::storage::WalletStorageType;
use errors::wallet::WalletStorageError;
use services::microledger::microledger::Microledger;

pub struct DidMicroledger {
    pub did: String,
    merkle_tree: MerkleTree,
    storage: Box<WalletStorage>
}

impl Microledger for DidMicroledger where Self: Sized {
    fn new(did: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let tree = MerkleTree::from_vec(vec![])?;
        let parsed_options = DidMicroledger::parse_options(options)?;
        let storage_path = DidMicroledger::get_storage_path_from_options(&parsed_options);
        let storage = DidMicroledger::get_ledger_storage(did, storage_path).map_err(|err|
            CommonError::InvalidStructure(format!("Error while getting storage for ledger {:?}.", err)))?;
        Ok(DidMicroledger {
            did: did.to_string(),
            merkle_tree: tree,
            storage
        })
    }

    fn get_root_hash(&self) -> String {
        self.merkle_tree.root_hash_hex()
    }

    fn get_size(&self) -> usize {
        self.merkle_tree.count
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

    // Temporary
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
        storage_type.create_storage(did, Some(&config), None,
                                    &DidMicroledger::_metadata())?;
        storage_type.open_storage(did, None, None)
    }

    pub fn get_storage_path_from_options(parsed_options: &HashMap<String, String>) -> &str {
        parsed_options.get("storage_path").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::environment::EnvironmentUtils;
    use utils::test::TestUtils;

    fn valid_storage_options() -> HashMap<String, String>{
        let mut options: HashMap<String, String> = HashMap::new();
        let mut path = EnvironmentUtils::tmp_path();
        path.push("did_ml_path");
        let storage_path = path.to_str().unwrap().to_owned();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        options.insert("storage_path".to_string(), storage_path);
        options
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
        let options = valid_storage_options();
        let ml = DidMicroledger::new(did, options).unwrap();
        assert_eq!(ml.did, did);
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        let mut storage_iterator = ml.storage.get_all().unwrap();
        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }
}