use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use serde_json;
use serde_json::Value as JValue;

use services::microledger::did_microledger::DidMicroledger;
use services::microledger::testing_utils::Peer;
use services::crypto::CryptoService;
use errors::common::CommonError;
use domain::crypto::key::KeyInfo;
use services::microledger::microledger::Microledger;
use services::microledger::messages::LedgerUpdate;


#[derive(Deserialize, Serialize, Debug)]
pub enum MsgTypes {
    Connection = 1,
    ConnectionResponse = 2,
    Message = 3
}

// NOTE: THIS STRUCT IS VERY LIKELY TO CHANGE
// ASSUMPTION: THERE IS A SECURE MECHANISM TO DELIVER THESE STRUCTS
#[derive(Deserialize, Serialize, Debug)]
pub struct Connection {
    #[serde(rename = "type")]
    pub type_: MsgTypes,
    pub id: String,
    pub message: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectionResponse {
    #[serde(rename = "type")]
    pub type_: MsgTypes,
    pub id: String,
    pub message: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: MsgTypes,
    pub message: String
}

impl Connection {
    pub fn new(id: &str, msg: &str) -> Self {
        Connection {
            type_: MsgTypes::Connection,
            id: id.to_string(),
            message: msg.to_string()
        }
    }
}


struct Agent<'a> {
    // TODO: FIX THIS!!!. Should have a wallet, not a signing key
    pub sigkey: String,
    pub verkey: String,
    pub managing_did: String,
    pub remote_did: Option<String>,
    pub m_ledgers: HashMap<String, DidMicroledger>,
    pub peer: Rc<RefCell<Peer<'a>>>
}

impl<'a> Agent<'a> {
    // TODO: Fix this, seed should not be required, a verkey should be passed and the given wallet should be checked for the verkey
    pub fn new(did: &str, seed: Option<String>, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let ml = DidMicroledger::new(did, options)?;
        let mut m_ledgers: HashMap<String, DidMicroledger> = HashMap::new();
        m_ledgers.insert(did.to_string(), ml);

        let crypto_service = CryptoService::new();
        let key_info = KeyInfo {
            seed: seed,
            crypto_type: None
        };
        let key = crypto_service.create_key(&key_info).map_err(|err|
            CommonError::InvalidState(format!("Cannot create a key {:?}.", err)))?;

        let peer = Rc::new(RefCell::new(Peer::new(did)));

        Ok(Agent {
            sigkey: key.signkey,
            verkey: key.verkey,
            managing_did: did.to_string(),
            remote_did: None,
            m_ledgers,
            peer
        })
    }

    pub fn get_self_microledger(&self) -> Result<&DidMicroledger, CommonError> {
        match self.m_ledgers.get(&self.managing_did) {
            Some(ml) => Ok(ml),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn get_self_microledger_mut(&mut self) -> Result<&mut DidMicroledger, CommonError> {
        match self.m_ledgers.get_mut(&self.managing_did) {
            Some(ml) => Ok(ml),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn has_microledger(&self, did: &str) -> bool {
        self.m_ledgers.get(did).is_some()
    }

    pub fn get_new_connection_msg(&self) -> Result<String, CommonError> {
        let ledger_update = LedgerUpdate::new_as_json(&self.managing_did,
                                                      self.get_self_microledger()?, 1)?;
        serde_json::to_string(&Connection::new(
            &self.managing_did,
            &ledger_update
        )).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify connection {:?}.", err)))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::testing_utils::tests::{get_new_network};
    use services::microledger::helpers::tests::{valid_storage_options, get_new_microledger};
    use utils::test::TestUtils;

    fn get_new_agent(did: &str, seed: String) -> Agent {
        Agent::new(did, Some(seed), valid_storage_options()).unwrap()
    }

    fn get_agent1_genesis_txns() -> Vec<String> {
        vec![
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"}}"#)
        ]
    }

    fn get_agent2_genesis_txns() -> Vec<String> {
        vec![]
    }

    #[test]
    fn test_new_agent_create_new_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let agent1 = get_new_agent(did, seed1);
        assert_eq!(agent1.managing_did, did);
        assert_eq!(agent1.verkey, "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC");
        assert!(agent1.m_ledgers.get(did).is_some());
        assert_eq!(agent1.get_self_microledger().unwrap().get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        let ml = agent1.m_ledgers.get(did).unwrap();
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        assert_eq!(agent1.has_microledger("somerandomstring"), false)
    }

    #[test]
    fn test_new_connection_message() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let mut agent1 = get_new_agent(did, seed1);
        let gen_txns = get_agent1_genesis_txns();
        let txns = get_agent1_genesis_txns();
        let txns = txns.iter().map(|s|s.as_ref()).collect();
        {
            let ml = agent1.get_self_microledger_mut().unwrap();
            ml.add_multiple(txns).unwrap();
        }
        let expected_message = r#"{"type":"Connection","id":"75KUW8tPUQNBS4W7ibFeY8","message":"{\"type\":\"ledgerUpdate\",\"state\":\"DID:75KUW8tPUQNBS4W7ibFeY8\",\"root\":\"c59e216c9207c5736670a70688e0caace20c2085333ba079842f0d9e1c250db3\",\"events\":[[1,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"dest\\\":\\\"75KUW8tPUQNBS4W7ibFeY8\\\",\\\"type\\\":\\\"1\\\"}}\"],[2,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"authorizations\\\":[\\\"all\\\"],\\\"type\\\":\\\"2\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"],[3,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"address\\\":\\\"https://agent.example.com\\\",\\\"type\\\":\\\"3\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"]]}"}"#;
        let conn = agent1.get_new_connection_msg().unwrap();
        assert_eq!(expected_message, conn);
    }
}