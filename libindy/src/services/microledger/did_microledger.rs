use std::collections::HashMap;
use std::str;

use serde_json;
use rand::{thread_rng, Rng};

use errors::common::CommonError;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::wallet::storage::WalletStorage;
use services::wallet::storage::default::SQLiteStorageType;
use services::wallet::storage::WalletStorageType;
use services::wallet::wallet::EncryptedValue;
use errors::wallet::WalletStorageError;
use services::microledger::microledger::Microledger;
use services::microledger::txn_builder::TxnBuilder;
use services::microledger::helpers::{byte_array_to_usize, usize_to_byte_array, parse_options,
                                     create_storage_options, gen_enc_key};
use utils::environment::EnvironmentUtils;
use std::path::PathBuf;
use services::microledger::did_doc::DidDoc;
use services::microledger::helpers::get_storage_path_from_options;
use services::microledger::helpers::get_ledger_storage;

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
        let parsed_options = parse_options(options)?;
        // Create a new storage or load an existing storage
        let storage_path = get_storage_path_from_options(&parsed_options);
        let storage = get_ledger_storage(did, storage_path,
                                         &DidMicroledger::get_metadata()).map_err(|err|
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

    // TODO: Resolve usize and u64 in `get_size` and `add*`

    fn get_size(&self) -> usize {
        self.merkle_tree.count
    }

    fn add(&mut self, txn: &str) -> Result<usize, CommonError> {
        let txn_bytes = txn.as_bytes().to_vec();
        let txn_bytes_len = txn_bytes.len();
        self.merkle_tree.append(txn_bytes.clone())?;
        // TODO: Fix this, find out the correct size of the key
        let key = gen_enc_key(txn_bytes_len);
        let enc = EncryptedValue::new(txn_bytes, key);
        let new_size = self.get_size();
        let id = usize_to_byte_array(new_size);
        self.storage.add(&TYP, &id, &enc, &vec![]).map_err(|err|
            CommonError::InvalidStructure(format!("Error while adding to ledger storage: {:?}.", err)))?;
        Ok(new_size)
    }

    fn add_multiple(&mut self, txns: Vec<&str>) -> Result<(usize, usize), CommonError> {
        let mut start = 0;
        let mut end = 0;

        for txn in txns {
            let s = self.add(txn)?;
            if start == 0 {
                start = s;
            }
            end = s;
        }
        Ok((start, end))
    }

    fn get(&self, from: u64, to: Option<u64>) -> Result<Vec<String>, CommonError> {
        if from < 1 {
            return Err(CommonError::InvalidStructure(format!("Invalid seq no: {}", from)))
        }

        match to {
            Some(t) => {
                let ledger_size = self.get_size() as u64;
                if t > ledger_size {
                    return Err(CommonError::InvalidStructure(format!("`to` greater than ledger size: to={}, ledger size={}", t, ledger_size)))
                }
            },
            None => ()
        }

        // TODO: Use `storage.search` instead of `storage.get_all`
        let mut storage_iterator = self.storage.get_all().map_err(|err|
            CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", err)))?;

        let mut res: Vec<String> = Vec::new();
        // TODO Duplicated from `populate_merkle_tree`, change when changing iterator
        loop {
            match storage_iterator.next() {
                Ok(v) => {
                    match v {
                        Some(r) => {
                            let id = byte_array_to_usize(r.id) as u64;
                            match r.value {
                                Some(ev) => {
                                    if id >= from && (to.is_none() || to.unwrap() >= id) {
                                        res.push(str::from_utf8(&ev.data).unwrap().to_string())
                                    }
                                },
                                None => continue
                            }
                        }
                        None => break
                    }
                },
                Err(e) => return Err(CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", e)))
            }
        }

        Ok(res)
    }

    fn get_with_seq_no(&self, from: u64, to: Option<u64>) -> Result<Vec<(u64, String)>, CommonError> {
        let txns = self.get(from, to)?;
        let mut res: Vec<(u64, String)> = Vec::new();
        let mut start = from;
        for txn in txns {
            res.push((start, txn));
            start += 1;
        }
        Ok(res)
    }
}

impl DidMicroledger {
    pub fn create_options(storage_path: Option<&str>) -> HashMap<String, String> {
        create_storage_options(storage_path, vec!["did_ml_path"])
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

    fn populate_merkle_tree(&mut self) -> Result<(), CommonError> {
        let mut storage_iterator = self.storage.get_all().map_err(|err|
            CommonError::InvalidStructure(format!("Error getting ledger storage iterator: {:?}.", err)))?;
        loop {
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

    pub fn add_endpoint_rem_txn(&mut self, verkey: &str, address: &str) -> Result<usize, CommonError> {
        let ep_txn = TxnBuilder::build_endpoint_rem_txn(verkey, address)?;
        self.add(&ep_txn)
    }

    fn register_did_doc(&self, view: DidDoc) {

    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::environment::EnvironmentUtils;
    use super::super::super::super::utils::test::TestUtils;
    use services::microledger::constants::*;
    use services::microledger::helpers::tests::{valid_did_ml_storage_options, get_new_microledger,
                                                get_4_txns, check_empty_storage};

    fn add_4_txns(ml: &mut DidMicroledger) -> usize {
        for txn in get_4_txns() {
            ml.add(&txn).unwrap();
        }
        ml.get_size()

    }

    #[test]
    fn test_did_create_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let ml = get_new_microledger(did);
        assert_eq!(ml.did, did);
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        check_empty_storage(ml.storage)
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
    fn test_add_multiple_to_did_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut ml = get_new_microledger(did);
        let txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let seq_nos = ml.add_multiple(vec![txn, txn_2, txn_3]).unwrap();
        assert_eq!(seq_nos.0, 1usize);
        assert_eq!(seq_nos.1, 3usize);
    }

    #[test]
    fn test_rebuild_merkle_tree() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut root_hash = String::from("");
        let mut size = 0;

        // Create a new microledger and fill it with some txns
        {
            let mut ml = get_new_microledger(did);
            let s = add_4_txns(&mut ml);
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

    #[test]
    fn test_add_endpoint_rem_txn() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let verkey = "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1";
        let address = "https://agent.example.com";
        let mut ml = get_new_microledger(did);
        let s = ml.add_endpoint_txn(verkey, address).unwrap();
        assert_eq!(s, 1);
        let t = ml.add_endpoint_rem_txn(verkey, address).unwrap();
        assert_eq!(t, 2);
    }

    #[test]
    fn test_get_txns() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut ml = get_new_microledger(did);
        let s = add_4_txns(&mut ml);
        assert_eq!(s, 4);
        let txns = get_4_txns();

        let t = ml.get(0, None);
        assert!(t.is_err());

        let t = ml.get(1, None).unwrap();
        assert_eq!(t, txns.clone());

        let t = ml.get(1, Some(1)).unwrap();
        assert_eq!(t[0], txns[0].to_owned());

        let t = ml.get(1, Some(2)).unwrap();
        assert_eq!(t, txns[0..2].to_vec());

        let t = ml.get(1, Some(3)).unwrap();
        assert_eq!(t, txns[0..3].to_vec());

        let t = ml.get(1, Some(4)).unwrap();
        assert_eq!(t, txns[0..4].to_vec());

        let t = ml.get(2, Some(4)).unwrap();
        assert_eq!(t, txns[1..4].to_vec());

        let t = ml.get(2, Some(3)).unwrap();
        assert_eq!(t, txns[1..3].to_vec());

        let t = ml.get(2, Some(2)).unwrap();
        assert_eq!(t, txns[1..2].to_vec());

        let t = ml.get(1, Some(5));
        assert!(t.is_err());
    }

    #[test]
    fn test_get_with_seq_no_txns() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut ml = get_new_microledger(did);
        let s = add_4_txns(&mut ml);
        let txns = get_4_txns();

        let t = ml.get_with_seq_no(1, None).unwrap();
        for i in 0..4usize {
            assert_eq!(t[i], (i as u64 + 1, txns[i].clone()))
        }

        let t = ml.get_with_seq_no(2, None).unwrap();
        for i in 1..4usize {
            assert_eq!(t[i-1], (i as u64 + 1, txns[i].clone()))
        }

        let t = ml.get_with_seq_no(2, Some(3)).unwrap();
        for i in 1..3usize {
            assert_eq!(t[i-1], (i as u64 + 1, txns[i].clone()))
        }
    }
}