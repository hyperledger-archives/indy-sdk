use rand::{thread_rng, Rng};

use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::collections::HashMap;
use errors::common::CommonError;
use std::path::PathBuf;
use utils::environment::EnvironmentUtils;
use services::wallet::storage::default::SQLiteStorageType;
use services::wallet::storage::WalletStorage;
use errors::wallet::WalletStorageError;
use services::wallet::storage::WalletStorageType;
use services::wallet::WalletService;
use utils::inmem_wallet::InmemWallet;
use services::wallet::RecordOptions;
use services::crypto::CryptoService;
use utils::crypto::base58::encode;
use domain::crypto::key::KeyInfo;

pub fn usize_to_byte_array(n: usize) -> Vec<u8> {
    let mut wtr: Vec<u8> = Vec::new();
    wtr.write_u64::<LittleEndian>(n as u64).unwrap();
    wtr
}

pub fn byte_array_to_usize(v: Vec<u8>) -> usize {
    let mut rdr = Cursor::new(v);
    rdr.read_u64::<LittleEndian>().unwrap() as usize
}

pub fn parse_options(options: HashMap<String, String>) -> Result<HashMap<String, String>, CommonError> {
    // TODO: Support in-memory storage type
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

// TODO: This should be enhanced further
pub fn create_storage_options(base_storage_path: Option<&str>, extra_paths: Vec<&str>) -> HashMap<String, String> {
    let mut options: HashMap<String, String> = HashMap::new();
    options.insert("storage_type".to_string(), "sqlite".to_string());
    let mut path = match base_storage_path {
        Some(m) => {
            let mut pf = PathBuf::new();
            pf.push(m);
            pf
        },
        None => {
            EnvironmentUtils::tmp_path()
        }
    };
    for ep in extra_paths{
        path.push(ep);
    }
    let storage_path = path.to_str().unwrap().to_owned();
    options.insert("storage_path".to_string(), storage_path);
    options
}

pub fn get_storage_path_from_options(parsed_options: &HashMap<String, String>) -> &str {
    parsed_options.get("storage_path").unwrap()
}

pub fn get_rsm_storage(did: &str, storage_path: &str, metadata: &[u8]) -> Result<Box<WalletStorage>, WalletStorageError> {
    let config = json!({
            "path": storage_path
        }).to_string();
    let storage_type = SQLiteStorageType::new();
    match storage_type.create_storage(did, Some(&config), None,
                                      &metadata) {
        Ok(_) => (),
        Err(WalletStorageError::AlreadyExists) => (),
        Err(e) => return Err(e)
    }
    storage_type.open_storage(did, Some(&config), None)
}

pub fn gen_enc_key(size: usize) -> Vec<u8> {
    gen_random_bytes(size)
}

pub fn gen_random_bytes(size: usize) -> Vec<u8> {
    thread_rng().gen_iter().take(size).collect()
}

pub fn register_inmem_wallet(wallet_service: &WalletService) {
    wallet_service
        .register_wallet_storage(
            "inmem",
            InmemWallet::create,
            InmemWallet::open,
            InmemWallet::close,
            InmemWallet::delete,
            InmemWallet::add_record,
            InmemWallet::update_record_value,
            InmemWallet::update_record_tags,
            InmemWallet::add_record_tags,
            InmemWallet::delete_record_tags,
            InmemWallet::delete_record,
            InmemWallet::get_record,
            InmemWallet::get_record_id,
            InmemWallet::get_record_type,
            InmemWallet::get_record_value,
            InmemWallet::get_record_tags,
            InmemWallet::free_record,
            InmemWallet::get_storage_metadata,
            InmemWallet::set_storage_metadata,
            InmemWallet::free_storage_metadata,
            InmemWallet::search_records,
            InmemWallet::search_all_records,
            InmemWallet::get_search_total_count,
            InmemWallet::fetch_search_next_record,
            InmemWallet::free_search
        )
        .unwrap();
}

pub fn sign_msg(wallet_service: &WalletService, crypto_service: &CryptoService,
                wallet_handle: i32, verkey: &str, msg: &[u8]) -> Result<String, CommonError> {
    let key = wallet_service.get_indy_object(wallet_handle, verkey,
                                                  &RecordOptions::id_value(),
                                                  &mut String::new()).map_err(|err|
        CommonError::InvalidState(format!("Cannot get key {:?}.", err)))?;
    let res = crypto_service.sign(&key, msg).map_err(|err|
        CommonError::InvalidState(format!("Cannot sign {:?}.", err)))?;
    Ok(encode(&res))
}

pub fn verify_msg(crypto_service: &CryptoService, verkey: &str, msg: &[u8], sig: &[u8]) -> Result<bool, CommonError> {
    let res = crypto_service.verify(verkey, &msg, &sig).map_err(|err|
        CommonError::InvalidState(format!("Cannot verify {:?}.", err)))?;
    Ok(res)
}

pub fn in_memory_wallet_with_key(wallet_service: &WalletService, seed: Option<String>) -> i32 {
    let crypto_service = CryptoService::new();
    let key_info = KeyInfo {
        seed: seed,
        crypto_type: None
    };
    let key = crypto_service.create_key(&key_info).map_err(|err|
        CommonError::InvalidState(format!("Cannot create a key {:?}.", err))).unwrap();

    register_inmem_wallet(wallet_service);
    let config = json!({"id": &key.verkey, "storage_type": "inmem"}).to_string();
    let credentials = json!({"key": &key.verkey}).to_string();
    wallet_service.create_wallet(&config, &credentials).unwrap();
    let wallet_handle = wallet_service.open_wallet(&config, &credentials).unwrap();
    wallet_service.add_indy_object(wallet_handle, &key.verkey, &key,
                                   &HashMap::new()).unwrap();
    wallet_handle
}

pub fn in_memory_wallets_cleanup() {
    InmemWallet::cleanup()
}

pub mod tests {
    use super::*;
    use std::fs;
    use utils;
    // This import does not work
//    use utils::test::TestUtils;
    use services::microledger::constants::*;
    use std::collections::HashMap;
    use services::microledger::microledger::Microledger;
    use services::microledger::did_microledger::DidMicroledger;
    use services::microledger::did_doc::DidDoc;
    use services::microledger::view::View;

    pub fn test_data_cleanup() {
        let path = EnvironmentUtils::tmp_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
        in_memory_wallets_cleanup();
    }

    pub fn valid_did_ml_storage_options() -> HashMap<String, String>{
        /*let mut options: HashMap<String, String> = HashMap::new();
        let mut path = EnvironmentUtils::tmp_path();
        path.push("did_ml_path");
        let storage_path = path.to_str().unwrap().to_owned();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        options.insert("storage_path".to_string(), storage_path);
        options*/
        create_storage_options(EnvironmentUtils::tmp_path().to_str(),
                               vec!["did_ml_path"])
    }

    pub fn valid_did_doc_storage_options() -> HashMap<String, String>{
        create_storage_options(EnvironmentUtils::tmp_path().to_str(),
                               vec!["did_doc_path"])
    }

    pub fn check_empty_storage(storage: Box<WalletStorage>) {
        let mut storage_iterator = storage.get_all().unwrap();
        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    pub fn get_new_microledger(did: &str) -> DidMicroledger {
        let options = valid_did_ml_storage_options();
        DidMicroledger::new(did, options).unwrap()
    }

    pub fn get_new_did_doc(did: &str) -> DidDoc {
        let options = valid_did_doc_storage_options();
        DidDoc::new(did, options).unwrap()
    }

    pub fn get_4_txns() -> Vec<String> {
        let txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        vec![txn.to_string(), txn_2.to_string(), txn_3.to_string(), txn_4.to_string()]
    }

    pub fn get_10_txns() -> Vec<String> {
        let txns = vec![
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":[],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all","add_key","rem_key"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"tcp://123.88.912.091:9876","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":2,"txnVersion":2,"operation":{"address":"https://agent1.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":2,"txnVersion":1,"operation":{"address":"http://agent2.example.org","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#
        ];
        let mut txns: Vec<String> = txns.iter().map(|s|s.to_string()).collect();
        for txn in get_4_txns() {
            txns.push(txn)
        }
        txns
    }

    #[test]
    fn test_parse_valid_options() {
        let options = valid_did_ml_storage_options();
        let expected_options: HashMap<String, String> = options.clone();
        assert_eq!(parse_options(options).unwrap(), expected_options);
    }

    #[test]
    fn test_parse_options_without_required_keys() {
        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        assert!(parse_options(options).is_err());

        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_path".to_string(), "storage_path".to_string());
        assert!(parse_options(options).is_err());

        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("unknown key".to_string(), "unknown value".to_string());
        assert!(parse_options(options).is_err());
    }

    #[test]
    fn test_parse_options_incorrect_storage_type() {
        let mut options: HashMap<String, String> = HashMap::new();
        options.insert("storage_type".to_string(), "mysql".to_string());
        options.insert("storage_path".to_string(), "/tmp".to_string());
        let expected_options: HashMap<String, String> = options.clone();
        assert!(parse_options(options).is_err());
    }

    #[test]
    fn test_get_ledger_storage() {
        test_data_cleanup();
        let options = valid_did_ml_storage_options();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let metadata = vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
        let storage = get_rsm_storage(
            did, get_storage_path_from_options(&options),
            &metadata).unwrap();
        check_empty_storage(storage);

        /*let parsed_options = DidMicroledger::parse_options(options).unwrap();
        let storage_path = get_storage_path_from_options(&parsed_options);
        let config = json!({
            "path": storage_path
        }).to_string();
        let storage_type = SQLiteStorageType::new();
        storage_type.delete_storage(did, Some(&config), None).unwrap();*/
    }
}