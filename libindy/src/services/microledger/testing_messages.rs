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
use services::microledger::messages::ValidProtocolMessages;
use services::wallet::WalletService;
use services::wallet::RecordOptions;
use utils::crypto::base58::{encode, decode};
use services::microledger::view::View;
use services::microledger::did_doc::DidDoc;
use services::microledger::helpers::register_inmem_wallet;
use services::microledger::helpers::sign_msg;
use services::microledger::helpers::verify_msg;

#[derive(Deserialize, Serialize, Debug)]
pub enum MsgTypes {
    Connection,
    ConnectionResponse,
    Message
}

// NOTE: THIS STRUCT IS VERY LIKELY TO CHANGE
// ASSUMPTION: THERE IS A SECURE (CONFIDENTIALITY+INTEGRITY) MECHANISM TO DELIVER THESE STRUCTS
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
    pub did: String,
    pub verkey: String,
    pub payload: String,
    pub signature: String
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidMessages {
    Connection(Connection),
    ConnectionResponse(ConnectionResponse),
    Message(Message)
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

impl ConnectionResponse {
    pub fn new(id: &str, msg: &str) -> Self {
        ConnectionResponse {
            type_: MsgTypes::ConnectionResponse,
            id: id.to_string(),
            message: msg.to_string()
        }
    }
}

impl Message {
    pub fn new(payload: &str, did:&str, verkey: &str, sig: &str) -> Self {
        Message {
            type_: MsgTypes::Message,
            did: did.to_string(),
            verkey: verkey.to_string(),
            payload: payload.to_string(),
            signature: sig.to_string()
        }
    }
}

struct Agent<'a> {
    pub crypto_service: CryptoService,
    pub wallet_service: WalletService,
    pub wallet_handle: i32,
    pub verkey: String,
    pub managing_did: String,
    pub remote_did: Option<String>,
    pub m_ledgers: HashMap<String, DidMicroledger<'a>>,
    pub did_docs: HashMap<String, Rc<RefCell<DidDoc<'a>>>>,
    pub peer: Rc<RefCell<Peer<'a>>>
}

impl<'a> Agent<'a> {
    // TODO: Fix this, seed should not be required, a verkey should be passed and the given wallet should be checked for the verkey
    pub fn new(did: &str, seed: Option<String>, options: HashMap<String, String>) -> Result<Self, CommonError> {
        let crypto_service = CryptoService::new();
        let wallet_service = WalletService::new();
        register_inmem_wallet(&wallet_service);

        let ml = DidMicroledger::new(did, options)?;
        let mut m_ledgers: HashMap<String, DidMicroledger> = HashMap::new();
        m_ledgers.insert(did.to_string(), ml);

        let key_info = KeyInfo {
            seed: seed,
            crypto_type: None
        };
        let key = crypto_service.create_key(&key_info).map_err(|err|
            CommonError::InvalidState(format!("Cannot create a key {:?}.", err)))?;

        let id = format!("{}:{}", did, &key.verkey);
        let config = json!({"id": &id, "storage_type": "inmem"}).to_string();
        let credentials = json!({"key": &id}).to_string();
        wallet_service.create_wallet(&config, &credentials).unwrap();
        let wallet_handle = wallet_service.open_wallet(&config, &credentials).unwrap();

        wallet_service.add_indy_object(wallet_handle, &key.verkey, &key, &HashMap::new()).unwrap();

        let peer = Rc::new(RefCell::new(Peer::new(did)));

        Ok(Agent {
            crypto_service,
            wallet_service,
            wallet_handle,
            verkey: key.verkey,
            managing_did: did.to_string(),
            remote_did: None,
            m_ledgers,
            did_docs: HashMap::new(),
            peer
        })
    }

    pub fn get_self_microledger(&self) -> Result<&DidMicroledger<'a>, CommonError> {
        match self.m_ledgers.get(&self.managing_did) {
            Some(ml) => Ok(ml),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn get_self_microledger_mut(&mut self) -> Result<&mut DidMicroledger<'a>, CommonError> {
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

    pub fn get_peer_id(&self) -> String {
        self.managing_did.clone()
    }

    pub fn process_inbox(&mut self) -> Result<(), CommonError> {
        let mut msgs_to_sent: Vec<(String, String)> = vec![];

        // TODO: Process one by one as error on a single message will cause error and early return
        let recvd_msgs = self.peer.borrow_mut().process();

        for msg in recvd_msgs {
            let j: JValue = serde_json::from_str(&msg).map_err(|err|
                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
            let j1 = j.clone();
            let t = j1.get("type");
            match t {
                Some(t) => {
                    match t.as_str() {
                        Some("Connection") => {
                            let c: Connection = serde_json::from_value(j).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            let msg = c.message;
                            let jpm: ValidProtocolMessages = serde_json::from_str(&msg).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            match jpm {
                                ValidProtocolMessages::LedgerUpdate(l) => {
                                    println!("{} Parsing inner connection message", &self.managing_did);
                                    if self.process_ledger_update_from_connection(l)? {
                                        println!("{} Inner connection message success", &self.managing_did);
                                        let self_ml = self.get_self_microledger()?;
                                        let conn_resp = self.get_connection_resp(&c.id, self_ml)?;
                                        msgs_to_sent.push((c.id.to_string(), conn_resp));
                                    }
                                },
                                _ => return Err(CommonError::InvalidStructure(String::from(
                                    "Cannot parse inner message")))
                            }
                        }
                        Some("ConnectionResponse") => {
                            let r: ConnectionResponse = serde_json::from_value(j).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            let msg = r.message;
                            let jpm: ValidProtocolMessages = serde_json::from_str(&msg).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            match jpm {
                                ValidProtocolMessages::LedgerUpdate(l) => {
                                    println!("{} Parsing inner connection response message", &self.managing_did);
                                    self.process_ledger_update_from_connection(l)?;
                                },
                                _ => return Err(CommonError::InvalidStructure(String::from(
                                    "Cannot parse inner message")))
                            }
                        }
                        Some("Message") => {
                            let m: Message = serde_json::from_value(j).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            let remote_did = &m.did;
                            let remote_verkey = &m.verkey;
                            if !self.verify_msg(remote_verkey, m.payload.clone().as_bytes(),
                                                &decode(&m.signature).unwrap()).unwrap() {
                                return Err(CommonError::InvalidStructure(String::from(
                                    "Verification failed")));
                            }
                            let payload_json: JValue = serde_json::from_str(&m.payload).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;


                        }
                        _ => return Err(CommonError::InvalidStructure(String::from("Cannot find required type")))
                    }
                }
                None => return Err(CommonError::InvalidStructure(String::from("Cannot find type")))
            }
            /*let j: ValidMessages = serde_json::from_str(&msg).map_err(|err|
                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
            println!("{:#?}", &j);
            match j {
                ValidMessages::Connection(c) => {
                    let msg = c.message;
                    let jpm: ValidProtocolMessages = serde_json::from_str(&msg).map_err(|err|
                        CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                    match jpm {
                        ValidProtocolMessages::LedgerUpdate(l) => {
                            println!("{} Parsing inner connection message", &self.managing_did);
                            if self.process_ledger_update_from_connection(l)? {
                                println!("{} Inner connection message success", &self.managing_did);
                                let self_ml = self.get_self_microledger()?;
                                let conn_resp = self.get_connection_resp(&c.id, self_ml)?;
                                msgs_to_sent.push((c.id.to_string(), conn_resp));
                            }
                        },
                        _ => return Err(CommonError::InvalidStructure(String::from(
                            "Cannot parse inner message")))
                    }
                },
                ValidMessages::ConnectionResponse(r) => {
                    let msg = r.message;
                    let jpm: ValidProtocolMessages = serde_json::from_str(&msg).map_err(|err|
                        CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                    match jpm {
                        ValidProtocolMessages::LedgerUpdate(l) => {
                            println!("{} Parsing inner connection response message", &self.managing_did);
                            self.process_ledger_update_from_connection(l)?;
                        },
                        _ => return Err(CommonError::InvalidStructure(String::from(
                            "Cannot parse inner message")))
                    }
                }
                _ => return Err(CommonError::InvalidStructure(String::from("Cannot parse message")))
            }*/
        }

        for (peer_id, msg) in msgs_to_sent {
            self.peer.borrow_mut().add_to_outbox(&peer_id, &msg);
        }
        Ok(())
    }

    fn process_ledger_update_from_connection(&mut self, l: LedgerUpdate) -> Result<bool, CommonError> {
        // TODO: Move part of this in microledger since processing LedgerUpdate is a core function
        let did = l.get_state_id();
        if self.m_ledgers.get(&did).is_none() {
            println!("Don't have ledger for {}", &did);
            let txns = Agent::get_validate_ledger_update_events(l.events)?;
            let s_opts = DidMicroledger::create_options(None);
            let mut ml = DidMicroledger::new(&did, s_opts)?;
            let s_opts = DidDoc::create_options(None);
            let mut doc = Rc::new(RefCell::new(DidDoc::new(&did, s_opts)?));
            ml.register_did_doc(Rc::clone(&doc));
            println!("Existing size {}", &ml.get_size());
            let txns: Vec<&str> = txns.iter().map(|t|t.as_ref()).collect();
            println!("Inserting in ledger {} txns", &txns.len());
            ml.add_multiple(txns)?;
            self.m_ledgers.insert(did.to_string(), ml);
            self.did_docs.insert(did.to_string(), Rc::clone(&doc));
            self.remote_did = Some(did.to_string());
            Ok(true)
        } else {
            println!("Already have ledger for {}", &did);
            Ok(false)
        }
    }

    fn get_validate_ledger_update_events(events: Vec<(u64, String)>) -> Result<Vec<String>, CommonError> {
        // TODO: Move this to microledger
        let mut txns: Vec<String> = vec![];
        let mut i = 0u64;
        for (j, txn) in events {
            if j - i != 1 {
                return Err(CommonError::InvalidStructure(format!("seq no should be {} but was {}", i+1, j)))
            }
            txns.push(txn);
            i += 1;
        }
        Ok(txns)
    }

    pub fn get_connection_resp(&self, from_agent_id: &str, ml: &DidMicroledger) -> Result<String, CommonError> {
        let ledger_update = LedgerUpdate::new_as_json(&ml.did, &ml, 1)?;
        let conn_resp = serde_json::to_string(&ConnectionResponse::new(
            from_agent_id,
            &ledger_update
        )).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify connection {:?}.", err)))?;
        Ok(conn_resp)
    }

    fn sign_msg(&self, msg: &[u8]) -> Result<String, CommonError> {
        sign_msg(&self.wallet_service, &self.crypto_service, self.wallet_handle, &self.verkey, msg)
    }

    fn verify_msg(&self, verkey: &str, msg: &[u8], sig: &[u8]) -> Result<bool, CommonError> {
        verify_msg(&self.crypto_service, verkey, msg, sig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::testing_utils::tests::{get_new_network};
    use services::microledger::helpers::tests::{valid_did_ml_storage_options, get_new_microledger, valid_did_doc_storage_options};
    use services::microledger::helpers::{create_storage_options};
    use utils::test::TestUtils;
    use utils::environment::EnvironmentUtils;
    use services::microledger::testing_utils::Network;
    use services::microledger::constants::AUTHZ_ADD_KEY;

    pub fn gen_storage_options(extra_path: Option<&str>) -> HashMap<String, String>{
        let mut path = EnvironmentUtils::tmp_path();
        let mut extra_paths = vec!["did_ml_path"];
        if extra_path.is_some() {
            extra_paths.push(extra_path.unwrap());
        }
        create_storage_options(path.to_str(), extra_paths)
    }

    fn get_new_agent(did: &str, seed: String, extra_path: String) -> Agent {
        Agent::new(did, Some(seed), gen_storage_options(Some(&extra_path))).unwrap()
    }

    fn get_agent1_genesis_txns() -> Vec<String> {
        vec![
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC"}}"#)
        ]
    }

    fn get_agent2_genesis_txns() -> Vec<String> {
        vec![
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"84qiTnsJrdefBDMrF49kfa","type":"1"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"}}"#),
            String::from(r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent2.example.com","type":"3","verkey":"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ"}}"#)
        ]
    }

    fn bootstrap_agent1(did: &str, seed: String) -> (Agent, String) {
        let mut agent1 = get_new_agent(did, seed, String::from("agent1"));
        let txns = get_agent1_genesis_txns();
        let txns = txns.iter().map(|s|s.as_ref()).collect();
        {
            let ml = agent1.get_self_microledger_mut().unwrap();
            ml.add_multiple(txns).unwrap();
        }
        let msg = agent1.get_new_connection_msg().unwrap();
        (agent1, msg)
    }

    fn bootstrap_agent2(did: &str, seed: String) -> (Agent, String) {
        let mut agent2 = get_new_agent(did, seed, String::from("agent2"));
        let txns = get_agent2_genesis_txns();
        let txns = txns.iter().map(|s|s.as_ref()).collect();
        {
            let ml = agent2.get_self_microledger_mut().unwrap();
            ml.add_multiple(txns).unwrap();
        }
        let msg = agent2.get_new_connection_msg().unwrap();
        (agent2, msg)
    }

    fn connected_agents<'a>() -> (Rc<RefCell<Network<'a>>>, Agent<'a>, Agent<'a>) {

        let network = get_new_network("n");
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let seed2 = String::from("99999999999999999999999999999999");

        let (mut agent1, conn1) = bootstrap_agent1(did1, seed1);
        let (mut agent2, _) = bootstrap_agent2(did2, seed2);

        network.borrow_mut().register_peer(Rc::clone(&agent1.peer));
        network.borrow_mut().register_peer(Rc::clone(&agent2.peer));

        network.borrow().send_message(&conn1, &agent2.get_peer_id()).unwrap();
        agent2.process_inbox().unwrap();

        network.borrow_mut().process_outboxes_for_all_peers().unwrap();
        agent1.process_inbox().unwrap();
        (network, agent1, agent2)
    }

    #[test]
    fn test_new_agent_create_new_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let agent1 = get_new_agent(did, seed1, String::from(""));
        assert_eq!(agent1.managing_did, did);
        assert_eq!(agent1.verkey, "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC");
        assert_eq!(agent1.get_peer_id(), did.to_string());
        assert!(agent1.m_ledgers.get(did).is_some());
        assert_eq!(agent1.get_self_microledger().unwrap().get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        let ml = agent1.m_ledgers.get(did).unwrap();
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        assert_eq!(agent1.has_microledger("somerandomstring"), false);
    }

    #[test]
    fn test_parse_ledger_events() {
        let events_1: Vec<(u64, String)> = vec![(0, "t1".into()), (1, "t2".into()), (2, "t3".into())];
        let events_2: Vec<(u64, String)> = vec![(1, "t1".into()), (3, "t2".into()), (4, "t3".into())];
        let events_3: Vec<(u64, String)> = vec![(2, "t1".into()), (3, "t2".into()), (4, "t3".into())];
        let events_4: Vec<(u64, String)> = vec![(1, "t1".into()), (2, "t2".into()), (4, "t3".into())];
        let events_5: Vec<(u64, String)> = vec![(1, "t1".into()), (2, "t2".into()), (3, "t3".into())];

        assert!(Agent::get_validate_ledger_update_events(events_1).is_err());
        assert!(Agent::get_validate_ledger_update_events(events_2).is_err());
        assert!(Agent::get_validate_ledger_update_events(events_3).is_err());
        assert!(Agent::get_validate_ledger_update_events(events_4).is_err());
        let e: Vec<String> = vec!["t1".into(), "t2".into(), "t3".into()];
        assert_eq!(Agent::get_validate_ledger_update_events(events_5).unwrap(), e);
    }


    #[test]
    fn test_new_connection_message() {
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let expected_message1 = r#"{"type":"Connection","id":"75KUW8tPUQNBS4W7ibFeY8","message":"{\"type\":\"ledgerUpdate\",\"state\":\"DID:75KUW8tPUQNBS4W7ibFeY8\",\"root\":\"c59e216c9207c5736670a70688e0caace20c2085333ba079842f0d9e1c250db3\",\"events\":[[1,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"dest\\\":\\\"75KUW8tPUQNBS4W7ibFeY8\\\",\\\"type\\\":\\\"1\\\"}}\"],[2,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"authorizations\\\":[\\\"all\\\"],\\\"type\\\":\\\"2\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"],[3,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"address\\\":\\\"https://agent.example.com\\\",\\\"type\\\":\\\"3\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"]]}"}"#;
        let (agent1, conn1) = bootstrap_agent1(did1, seed1);
        assert_eq!(expected_message1, conn1);

        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let seed2 = String::from("99999999999999999999999999999999");
        let expected_message2 = r#"{"type":"Connection","id":"84qiTnsJrdefBDMrF49kfa","message":"{\"type\":\"ledgerUpdate\",\"state\":\"DID:84qiTnsJrdefBDMrF49kfa\",\"root\":\"63a09c731f706aeb38874e648da92f8194284d5f5d2aea5957f28573e51208f3\",\"events\":[[1,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"dest\\\":\\\"84qiTnsJrdefBDMrF49kfa\\\",\\\"type\\\":\\\"1\\\"}}\"],[2,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"authorizations\\\":[\\\"all\\\"],\\\"type\\\":\\\"2\\\",\\\"verkey\\\":\\\"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ\\\"}}\"],[3,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"address\\\":\\\"https://agent2.example.com\\\",\\\"type\\\":\\\"3\\\",\\\"verkey\\\":\\\"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ\\\"}}\"]]}"}"#;
        let (agent2, conn2) = bootstrap_agent2(did2, seed2);
        assert_eq!(expected_message2, conn2);
    }

    #[test]
    fn test_connection_response() {
        TestUtils::cleanup_temp();
        let network = get_new_network("n");
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let seed2 = String::from("99999999999999999999999999999999");

        let (mut agent1, conn1) = bootstrap_agent1(did1, seed1);
        let (mut agent2, _) = bootstrap_agent2(did2, seed2);

        network.borrow_mut().register_peer(Rc::clone(&agent1.peer));
        network.borrow_mut().register_peer(Rc::clone(&agent2.peer));

        assert!(&agent1.remote_did.is_none());
        assert!(&agent2.remote_did.is_none());
        assert!(&agent1.m_ledgers.get(did2).is_none());
        assert!(&agent1.m_ledgers.get(did1).is_some());
        assert!(&agent2.m_ledgers.get(did1).is_none());
        assert!(&agent2.m_ledgers.get(did2).is_some());

        // Send connection message
        network.borrow().send_message(&conn1, &agent2.get_peer_id()).unwrap();
        agent2.process_inbox().unwrap();

        // Process received connection response message
        network.borrow_mut().process_outboxes_for_all_peers().unwrap();
        agent1.process_inbox().unwrap();

        assert_eq!(&agent1.remote_did.clone().unwrap(), did2);
        assert_eq!(&agent2.remote_did.clone().unwrap(), did1);

        let a1_ml_s = agent1.get_self_microledger().unwrap();
        let a2_ml_s = agent2.get_self_microledger().unwrap();

        let a1_ml_s_root_hash = a1_ml_s.get_root_hash();
        let a2_ml_s_root_hash = a2_ml_s.get_root_hash();

        let a1_ml_r = agent1.m_ledgers.get(did2).unwrap();
        let a2_ml_r = agent2.m_ledgers.get(did1).unwrap();
        let a1_ml_r_root_hash = a1_ml_r.get_root_hash();
        let a2_ml_r_root_hash = a2_ml_r.get_root_hash();

        assert_eq!(&a1_ml_s_root_hash, &a2_ml_r_root_hash);
        assert_eq!(&a2_ml_s_root_hash, &a1_ml_r_root_hash);
    }

    #[test]
    fn test_messaging() {
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let payload = json!({
            "type": "greetings",
            "msg": "hey there"
        }).to_string();

        // Correct sig
        let sig = agent1.sign_msg(payload.as_bytes()).unwrap();
        let msg = Message::new(&payload, did1, &agent1.verkey, &sig);
        network.borrow().send_message(&serde_json::to_string(&msg).unwrap(), &agent2.get_peer_id()).unwrap();
        assert!(agent2.process_inbox().is_ok());

        // Incorrect sig
        let msg = Message::new(&payload, did1, &agent1.verkey, "4Be93xNcmaoHzUVK89Qz4aeQg9zMiC2PooegFWEY5aQEfzZo9uNgdjJJDQPj3K5Jj4gE5mERBetqLUBUu6G5cyX2");
        network.borrow().send_message(&serde_json::to_string(&msg).unwrap(), &agent2.get_peer_id()).unwrap();
        assert!(agent2.process_inbox().is_err());
    }

    #[test]
    fn test_key_rotation() {
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        /*{
            let ml = agent1.get_self_microledger_mut().unwrap();
            let new_verkey = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
            let key_txn = ml.add_key_txn(new_verkey, &vec![AUTHZ_ADD_KEY]).unwrap();
        }*/
    }
}