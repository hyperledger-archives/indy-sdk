use serde_json;
use serde_json::Value as JValue;

use services::microledger::microledger::Microledger;
use services::microledger::constants::LEDGER_UPDATE;
use services::microledger::did_microledger::DidMicroledger;
use errors::common::CommonError;

#[derive(Deserialize, Serialize, Debug)]
pub struct LedgerUpdate {
    #[serde(rename = "type")]
    pub type_: String,
    pub state: String,
    pub root: String,
    pub events: Vec<(u64, String)>
}

impl LedgerUpdate {
    pub fn new(did: &str, ml: &DidMicroledger, from: u64) -> Result<Self, CommonError> {
        let root = ml.get_root_hash();
        let events = ml.get_with_seq_no(from, None)?;
        Ok(LedgerUpdate {
            type_: LEDGER_UPDATE.to_string(),
            state: format!("DID:{}", did),
            root: root.to_string(),
            events
        })
    }

    pub fn get_state_id(&self) -> String {
        self.state.chars().skip(4).collect()
    }

    pub fn new_as_json(did: &str, ml: &DidMicroledger, from: u64) -> Result<String, CommonError> {
        LedgerUpdate::new(did, ml, from)?.as_json()

    }

    pub fn as_json(&self) -> Result<String, CommonError> {
        serde_json::to_string(self).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify ledger udpdate message {:?}.", err)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidProtocolMessages {
    LedgerUpdate(LedgerUpdate)
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

        let expected_msg_1 = r#"{"type":"ledgerUpdate","state":"DID:75KUW8tPUQNBS4W7ibFeY8","root":"ea9ffad1e936f7cafb99741607d59ad44108e73dc206beda81e143e6bb8edb97","events":[[6,"{\"protocolVersion\":2,\"txnVersion\":1,\"operation\":{\"address\":\"http://agent2.example.org\",\"type\":\"3\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[7,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"dest\":\"75KUW8tPUQNBS4W7ibFeY8\",\"type\":\"1\"}}"],[8,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"dest\":\"75KUW8tPUQNBS4W7ibFeY8\",\"type\":\"1\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[9,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"authorizations\":[\"all\"],\"type\":\"2\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[10,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"address\":\"https://agent.example.com\",\"type\":\"3\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"]]}"#;
        let ledger_update_1 = LedgerUpdate::new(did, &ml, 6).unwrap();
        let ledger_update_1_msg = serde_json::to_string(&ledger_update_1).unwrap();
        assert_eq!(ledger_update_1_msg, expected_msg_1);
        assert_eq!(LedgerUpdate::new_as_json(did, &ml, 6).unwrap(), expected_msg_1);

        let expected_msg_2 = r#"{"type":"ledgerUpdate","state":"DID:75KUW8tPUQNBS4W7ibFeY8","root":"ea9ffad1e936f7cafb99741607d59ad44108e73dc206beda81e143e6bb8edb97","events":[[9,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"authorizations\":[\"all\"],\"type\":\"2\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"],[10,"{\"protocolVersion\":1,\"txnVersion\":1,\"operation\":{\"address\":\"https://agent.example.com\",\"type\":\"3\",\"verkey\":\"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1\"}}"]]}"#;
        let ledger_update_2 = LedgerUpdate::new(did, &ml, 9).unwrap();
        let ledger_update_2_msg = serde_json::to_string(&ledger_update_2).unwrap();
        assert_eq!(LedgerUpdate::new_as_json(did, &ml, 9).unwrap(), expected_msg_2);
    }
}