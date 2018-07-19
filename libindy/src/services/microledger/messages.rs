use serde_json;
use serde_json::Value as JValue;

use services::microledger::microledger::Microledger;
use services::microledger::constants::LEDGER_UPDATE;

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerUpdate {
    #[serde(rename = "type")]
    pub type_: String,
    pub state: String,
    pub root: String,
    pub events: Vec<(u64, String)>
}

impl LedgerUpdate {
    pub fn new(did: &str, root: &str, events: Vec<(u64, String)>) -> Self {
        LedgerUpdate {
            type_: LEDGER_UPDATE.to_string(),
            state: format!("DID:{}", did),
            root: root.to_string(),
            events: events
        }
    }
}

pub mod tests {
    use super::*;
//    use utils::test::TestUtils;
    use services::microledger::helpers::tests::{valid_storage_options, get_new_microledger, get_10_txns};

    #[test]
    fn test_create_ledger_update_message() {
        // TODO: Uncomment this, tmp dir has to be cleared otherwise
//        TestUtils::cleanup_temp();
        let txns = get_10_txns();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let mut ml = get_new_microledger(did);
        for txn in txns {
            ml.add(&txn).unwrap();
        }
        let root = ml.get_root_hash();
        let txns_1 = ml.get_with_seq_no(6, None).unwrap();
        let expected_msg_1 = r#"{"type":"ledgerUpdate","state":"DID:75KUW8tPUQNBS4W7ibFeY8","root":"ea9ffad1e936f7cafb99741607d59ad44108e73dc206beda81e143e6bb8edb97","events":[[6,"{\"protocolVersion\":2,\"txnVersion\":1,\"operation\":{\"address\":\"http://agent2.example.org\",\"type\":\"3\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[7,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"dest\":\"75KUW8tPUQNBS4W7ibFeY8\",\"type\":\"1\"}}"],[8,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"dest\":\"75KUW8tPUQNBS4W7ibFeY8\",\"type\":\"1\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[9,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"authorizations\":[\"all\"],\"type\":\"2\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[10,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"address\":\"https://agent.example.com\",\"type\":\"3\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"]]}"#;
        let ledger_update_1 = LedgerUpdate::new(did, &root, txns_1);
        let ledger_update_1_msg = serde_json::to_string(&ledger_update_1).unwrap();
        println!("{}", &ledger_update_1_msg);
        assert_eq!(ledger_update_1_msg, expected_msg_1);
    }
}