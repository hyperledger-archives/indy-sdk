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


#[derive(Deserialize, Serialize, Debug)]
pub enum MsgTypes {
    Connection = 1,
    Message = 2
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
pub struct Message {
    #[serde(rename = "type")]
    pub type_: MsgTypes,
    pub message: String
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

    pub fn has_microledger(&self, did: &str) -> bool {
        self.m_ledgers.get(did).is_some()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::testing_utils::tests::{get_new_network};
    use services::microledger::helpers::tests::{valid_storage_options, get_new_microledger};
    use super::super::super::super::utils::test::TestUtils;

    #[test]
    fn test_new_agent_create_new_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let agent1 = Agent::new(did, Some(seed1), valid_storage_options()).unwrap();
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

    }
}