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
use services::microledger::helpers::{sign_msg, verify_msg};
use services::microledger::txn_builder::Txn;
use services::microledger::constants::{SIGNATURE, IDENTIFIER, KEY_TXN, AUTHZ_ALL, AUTHZ_ADD_KEY,
                                       AUTHZ_REM_KEY, AUTHZ_MPROX, VERKEY, AUTHORIZATIONS,
                                       ENDPOINT_TXN, ADDRESS};
use services::microledger::helpers::gen_random_bytes;

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
    pub id: String,
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
    pub fn new(payload: &str, id:&str, verkey: &str, sig: &str) -> Self {
        Message {
            type_: MsgTypes::Message,
            id: id.to_string(),
            verkey: verkey.to_string(),
            payload: payload.to_string(),
            signature: sig.to_string()
        }
    }
}

struct Agent<'a> {
    pub crypto_service: CryptoService,
    pub wallet_service: Rc<WalletService>,
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

        let peer_id = encode(&gen_random_bytes(6));
        let s_opts = DidMicroledger::create_options(None, Some(&peer_id));
        let mut ml = DidMicroledger::new(did, s_opts)?;
        let mut m_ledgers: HashMap<String, DidMicroledger> = HashMap::new();

        let mut did_docs: HashMap<String, Rc<RefCell<DidDoc<'a>>>> = HashMap::new();
        let s_opts = DidDoc::create_options(None, Some(&peer_id));
        let doc = Rc::new(RefCell::new(DidDoc::new(&did, s_opts)?));
        ml.register_did_doc(Rc::clone(&doc));

        m_ledgers.insert(did.to_string(), ml);
        did_docs.insert(did.to_string(), Rc::clone(&doc));

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

        let peer = Rc::new(RefCell::new(Peer::new(&peer_id)));

        Ok(Agent {
            crypto_service,
            wallet_service: Rc::new(wallet_service),
            wallet_handle,
            verkey: key.verkey,
            managing_did: did.to_string(),
            remote_did: None,
            m_ledgers,
            did_docs,
            peer
        })
    }

    pub fn get_self_microledger(&self) -> Result<&DidMicroledger<'a>, CommonError> {
        let did = self.managing_did.clone();
        self.get_microledger(&did)
    }

    pub fn get_self_microledger_mut(&mut self) -> Result<&mut DidMicroledger<'a>, CommonError> {
        let did = self.managing_did.clone();
        self.get_microledger_mut(&did)
    }

    pub fn get_remote_microledger(&self) -> Result<Option<&DidMicroledger<'a>>, CommonError> {
        let did = self.remote_did.clone();
        match did {
            Some(ref did) => {
                let ml = self.get_microledger(&did)?;
                Ok(Some(ml))
            }
            None => Ok(None)
        }
    }

    pub fn get_remote_microledger_mut(&mut self) -> Result<Option<&mut DidMicroledger<'a>>, CommonError> {
        let did = self.remote_did.clone();
        match did {
            Some(ref did) => {
                let mut ml = self.get_microledger_mut(&did)?;
                Ok(Some(ml))
            }
            None => Ok(None)
        }
    }

    pub fn get_microledger(&self, did: &str) -> Result<&DidMicroledger<'a>, CommonError> {
        match self.m_ledgers.get(did) {
            Some(ml) => Ok(ml),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn get_microledger_mut(&mut self, did: &str) -> Result<&mut DidMicroledger<'a>, CommonError> {
        match self.m_ledgers.get_mut(did) {
            Some(ml) => Ok(ml),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn has_microledger(&self, did: &str) -> bool {
        self.m_ledgers.get(did).is_some()
    }

    pub fn get_did_doc(&self, did: &str) -> Result<&Rc<RefCell<DidDoc<'a>>>, CommonError> {
        match self.did_docs.get(did) {
            Some(doc) => Ok(doc),
            None => Err(CommonError::InvalidState(String::from("Microledger not present")))
        }
    }

    pub fn get_new_connection_msg(&self) -> Result<String, CommonError> {
        let ledger_update = LedgerUpdate::new_as_json(&self.managing_did,
                                                      self.get_self_microledger()?, 1)?;
        serde_json::to_string(&Connection::new(
            &self.get_peer_id(),
            &ledger_update
        )).map_err(|err|
            CommonError::InvalidState(format!("Unable to jsonify connection {:?}.", err)))
    }

    pub fn get_peer_id(&self) -> String {
        self.peer.borrow().name.clone()
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
                                    if l.events.len() > 0 {
                                        self.process_ledger_update_from_connection(l)?;
                                    }
                                },
                                _ => return Err(CommonError::InvalidStructure(String::from(
                                    "Cannot parse inner message")))
                            }
                        }
                        Some("Message") => {
                            let m: Message = serde_json::from_value(j).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            let remote_id = &m.id;
                            let remote_verkey = &m.verkey;
                            if !self.verify_msg(remote_verkey, m.payload.clone().as_bytes(),
                                                &decode(&m.signature).unwrap()).unwrap() {
                                return Err(CommonError::InvalidStructure(String::from(
                                    "Verification failed")));
                            }
                            let payload_json: JValue = serde_json::from_str(&m.payload).map_err(|err|
                                CommonError::InvalidState(format!("Unable to parse json message {:?}.", err)))?;
                            match payload_json.get("type") {
                                Some(val) => {
                                    match val.as_str() {
                                        Some("greetings") => {
                                            println!("Greetings received {:?}", &payload_json);
                                        }
                                        Some(LEDGER_UPDATE) => {
                                            println!("{} Parsing LEDGER_UPDATE in message payload", &self.managing_did);
                                            let l: LedgerUpdate = serde_json::from_value(payload_json.clone()).map_err(|err|
                                                CommonError::InvalidState(format!("Unable to convert to ledger update {:?}.", err)))?;
                                            if l.events.len() > 0 {
                                                self.process_ledger_update(l)?;
                                            }
                                        }
                                        None => {
                                            println!("Bad message payload, type should be string: {:?}", &payload_json);
                                        }
                                    }
                                }
                                None => {
                                    println!("Bad message payload, without type: {:?}", &payload_json);
                                }
                            }

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

    pub fn add_new_ledger_and_did_doc(agent: &mut Agent, did: &str, txns: Vec<String>) -> Result<(), CommonError> {
        let s_opts = DidMicroledger::create_options(None, Some(&agent.get_peer_id()));
        let mut ml = DidMicroledger::new(&did, s_opts)?;
        let s_opts = DidDoc::create_options(None,Some(&agent.get_peer_id()));
        let doc = Rc::new(RefCell::new(DidDoc::new(&did, s_opts)?));
        ml.register_did_doc(Rc::clone(&doc));
        agent.m_ledgers.insert(did.to_string(), ml);
        agent.did_docs.insert(did.to_string(), Rc::clone(&doc));
        {
            let mut ml = agent.m_ledgers.get_mut(did).unwrap();
            let txns: Vec<&str> = txns.iter().map(|t|t.as_ref()).collect();
            ml.add_multiple(txns)?;
        }
        // TODO: Check final root hash by cloning merkle tree

        agent.remote_did = Some(did.to_string());
        Ok(())
    }

    fn process_ledger_update_for_new_did(&mut self, did: &str, l: LedgerUpdate) -> Result<bool, CommonError> {
        let len = l.events.len() as u64;
        let txns = DidMicroledger::get_validated_ledger_update_events(l.events, 1,
                                                             len)?;
        self.remote_did = Some(did.to_string());
        Agent::add_new_ledger_and_did_doc(self, did, txns)?;
        Ok(true)
    }

    fn process_ledger_update_for_existing_did(&mut self, did: &str, l: LedgerUpdate) -> Result<bool, CommonError> {
        let ml = self.m_ledgers.get_mut(did).unwrap();
        let doc = self.did_docs.get(did).unwrap();
        let existing_size = ml.get_size() as u64;
        let events = DidMicroledger::get_unseen_events(existing_size, l.events);
        let len = events.len() as u64;
        let txns = DidMicroledger::get_validated_ledger_update_events(events, existing_size+1,
                                                             existing_size+len)?;
        for txn in &txns {
            if !DidDoc::is_valid_txn(txn, &doc.borrow(), &self.crypto_service)? {
                return Ok(false);
            } else {
                ml.add(txn)?;
            }
        }
        Ok(true)
    }

    fn process_ledger_update_from_connection(&mut self, l: LedgerUpdate) -> Result<bool, CommonError> {
        // TODO: Move part of this in microledger since processing LedgerUpdate is a core function
        let did = l.get_state_id();
        if self.m_ledgers.get(&did).is_none() {
            self.process_ledger_update_for_new_did(&did, l)
        } else {
            println!("Already have ledger for {}. Ignoring!", &did);
            Ok(false)
        }
    }

    fn process_ledger_update(&mut self, l: LedgerUpdate) -> Result<bool, CommonError> {
        let did = l.get_state_id();
        if self.m_ledgers.get(&did).is_none() {
            self.process_ledger_update_for_new_did(&did, l)
        } else {
            println!("Already have ledger for {}", &did);
            self.process_ledger_update_for_existing_did(&did, l)
        }
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
    use services::microledger::txn_builder::TxnBuilder;
    use services::microledger::constants::AUTHZ_ALL;
    use services::microledger::constants::AUTHZ_MPROX;
    use std::collections::HashSet;

    /*pub fn gen_storage_options(extra_path: Option<&str>) -> HashMap<String, String>{
        let mut path = EnvironmentUtils::tmp_path();
        let mut extra_paths = vec!["did_ml_path"];
        if extra_path.is_some() {
            extra_paths.push(extra_path.unwrap());
        }
        create_storage_options(path.to_str(), extra_paths)
    }*/

    fn get_new_agent(did: &str, seed: String, extra_path: String) -> Agent {
        Agent::new(did, Some(seed), HashMap::new()).unwrap()
    }

    fn get_did1_genesis_txns() -> Vec<String> {
        let mut txns: Vec<String> = vec![];
        txns.push(TxnBuilder::build_nym_txn("75KUW8tPUQNBS4W7ibFeY8", None).unwrap());
        txns.push(TxnBuilder::build_key_txn("5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC", &vec![AUTHZ_ALL]).unwrap());

        let t3 = TxnBuilder::build_endpoint_txn("5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC", "https://agent.example.com").unwrap();
        let st3 = TxnBuilder::add_signature_to_txn(&t3, "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC", "3NhdyVztm92qYVfQn34n9uRYuHFe7HZwcgg8jaFVVhB7CY3HBE8hocPUX4jifUCNRbkKdJP5VdLER4pQ5fiK67rE").unwrap();
        txns.push(st3);

        txns
    }

    fn get_did2_genesis_txns() -> Vec<String> {
        let mut txns: Vec<String> = vec![];
        txns.push(TxnBuilder::build_nym_txn("84qiTnsJrdefBDMrF49kfa", None).unwrap());
        txns.push(TxnBuilder::build_key_txn("4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ", &vec![AUTHZ_ALL]).unwrap());

        let t3 = TxnBuilder::build_endpoint_txn("4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ", "https://agent2.example.com").unwrap();
        let st3 = TxnBuilder::add_signature_to_txn(&t3, "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ", "3PrU8GxZPVAcTEJdmPFpd4w9b4oMEoRv3guWeuXwgrKLHqfSkHkx7VSKJNRokj836Q6gzubqu6SCfybKi9NVAXJr").unwrap();
        txns.push(st3);

        txns
    }

    fn bootstrap_agent1(did: &str, seed: String) -> (Agent, String) {
        let agent1 = bootstrap_agent(did, seed, "agent1", get_did1_genesis_txns(), None, None);
        let msg = agent1.get_new_connection_msg().unwrap();
        (agent1, msg)
    }

    fn bootstrap_agent2(did: &str, seed: String) -> (Agent, String) {
        let agent2 = bootstrap_agent(did, seed, "agent2", get_did2_genesis_txns(), None, None);
        let msg = agent2.get_new_connection_msg().unwrap();
        (agent2, msg)
    }

    fn bootstrap_agent<'a>(did: &'a str, seed: String, name: &'a str,
                           gen_txns_self: Vec<String>, other_did: Option<&str>, gen_txns_other: Option<Vec<String>>) -> Agent<'a> {
        let mut agent = get_new_agent(did, seed, String::from(name));
        let txns = gen_txns_self.iter().map(|s|s.as_ref()).collect();
        {
            let ml = agent.get_self_microledger_mut().unwrap();
            ml.add_multiple(txns).unwrap();
        }
        match (other_did, gen_txns_other) {
            (Some(d), Some(t)) => {
                let txns: Vec<String> = t.iter().map(|s|s.to_owned()).collect();
                Agent::add_new_ledger_and_did_doc(&mut agent, d, txns).unwrap();
            }
            _ => ()
        }
        agent
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

    fn new_ledger_update_msg(did: &str, ml: &DidMicroledger, from: u64, agent: &Agent) -> Message {
        let payload = LedgerUpdate::new_as_json(did, &ml, from).unwrap();
        let sig = agent.sign_msg(payload.as_bytes()).unwrap();
        Message::new(&payload, did, &agent.verkey, &sig)
    }

    fn deliver_msg_successfully(network: &Rc<RefCell<Network>>, msg: &Message, agent: &mut Agent) {
        network.borrow().send_message(&serde_json::to_string(msg).unwrap(), &agent.get_peer_id()).unwrap();
        let a_did = &agent.managing_did.clone();
        let a_vk = &agent.verkey.clone();

        match agent.process_inbox() {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("{} {} got error {:?}", a_did, a_vk, e);
                assert!(false)
            }
        }
    }

    fn deliver_msg_unsuccessfully(network: &Rc<RefCell<Network>>, msg: &Message, agent: &mut Agent) {
        network.borrow().send_message(&serde_json::to_string(msg).unwrap(), &agent.get_peer_id()).unwrap();
        assert!(agent.process_inbox().is_err());
    }

    fn add_new_agent<'a>(network: &'a Rc<RefCell<Network<'a>>>, acting_agent: Rc<RefCell<Agent<'a>>>,
                         other_agents: Vec<Rc<RefCell<Agent<'a>>>>, edge_verkey: &'a str, cloud_agent_did: &'a str,
                         new_agent_verkey: &'a str, new_agent_seed: &'a str, new_agent_name: &'a str,
                         new_agent_address: &'a str, new_agent_authz: Vec<&str>) -> Agent<'a> {
        let mut old_seq_no = 0;
        let new_seq_no = {
            let ws1 = Rc::clone(&acting_agent.borrow().wallet_service);
            let wh1 = (&acting_agent.borrow()).wallet_handle.clone();
            let mut a = acting_agent.borrow_mut();
            let ml = a.get_self_microledger_mut().unwrap();
            old_seq_no = ml.get_size();
            ml.add_key_txn(new_agent_verkey, &new_agent_authz,
                           Some(&ws1),
                           Some(wh1),
                           Some(edge_verkey)).unwrap();
            let new_seq_no = ml.add_endpoint_txn(new_agent_verkey, new_agent_address,
                                                 Some(&ws1),
                                                 Some(wh1),
                                                 Some(edge_verkey)).unwrap();
            assert_eq!((ml.get_size() - old_seq_no), 2);
            new_seq_no as u64
        };

        let mut other_did = None;
        if cloud_agent_did == "75KUW8tPUQNBS4W7ibFeY8" {
            other_did = Some("84qiTnsJrdefBDMrF49kfa");
        } else if cloud_agent_did == "84qiTnsJrdefBDMrF49kfa" {
            other_did = Some("75KUW8tPUQNBS4W7ibFeY8");
        } else { panic!("Unacceptable did {}", &cloud_agent_did) }

        let gen_txns_self = {
            let mut a = acting_agent.borrow_mut();
            let ml = a.get_self_microledger().unwrap();
            ml.get(1, None).unwrap()
        };

        let gen_txns_other = {
            let mut a = acting_agent.borrow_mut();
            let ml = a.get_remote_microledger().unwrap();
            Some(ml.unwrap().get(1, None).unwrap())
        };

        let mut agent3 = bootstrap_agent(cloud_agent_did,
                                         String::from(new_agent_seed),
                                         new_agent_name, gen_txns_self, other_did, gen_txns_other);
        network.borrow_mut().register_peer(Rc::clone(&agent3.peer));

        let msg = {
            let a1 = acting_agent.borrow();
            let ml1 = a1.get_self_microledger().unwrap();
            new_ledger_update_msg(cloud_agent_did, &ml1, (old_seq_no+1) as u64, &acting_agent.borrow())
        };

        {
            for agent in other_agents {
                deliver_msg_successfully(&network, &msg, &mut agent.borrow_mut());
            }
            deliver_msg_successfully(&network, &msg, &mut agent3);
        }

        agent3
    }

    fn check_same_microledger_for_agents(agents: Vec<&Rc<RefCell<Agent>>>, did: &str) {
        let mut sizes: HashSet<usize> = HashSet::new();
        let mut root_hashes: HashSet<String> = HashSet::new();
        for agent in agents {
            let a1 = agent.borrow();
            let ml = a1.m_ledgers.get(did).unwrap();
            println!("{} {} {} has size {} for did {}", &a1.managing_did, &a1.verkey, &a1.wallet_handle, ml.get_size(), did);
            sizes.insert(ml.get_size());
            root_hashes.insert(ml.get_root_hash());
        }
        assert_eq!(sizes.len(), 1);
        assert_eq!(root_hashes.len(), 1);
    }

    fn rotate_key(old_key: &str, new_key: &str, new_key_authz: Vec<&str>, author_key: &str, author_agent: Rc<RefCell<Agent>>) -> u64 {
        let ws1 = Rc::clone(&author_agent.borrow().wallet_service);
        let wh1 = (&author_agent.borrow()).wallet_handle.clone();
        let mut a = author_agent.borrow_mut();
        let ml = a.get_self_microledger_mut().unwrap();
        let old_seq_no = ml.get_size();
        ml.add_key_txn(new_key, &new_key_authz,
                       Some(&ws1),
                       Some(wh1),
                       Some(author_key)).unwrap();
        let new_seq_no = ml.add_key_txn(old_key, &vec![],
                                        Some(&ws1),
                                        Some(wh1),
                                        Some(author_key)).unwrap();
        assert_eq!((ml.get_size() - old_seq_no), 2);
        new_seq_no as u64
    }

    fn gen_ledger_update_and_deliver_to_others(network: &Rc<RefCell<Network>>, generating_agent: Rc<RefCell<Agent>>,
                                               from_seq_no: u64, receiving_agents: Vec<Rc<RefCell<Agent>>>) {
        let msg = {
            let a = generating_agent.borrow();
            let ml = a.get_self_microledger().unwrap();
            new_ledger_update_msg(a.managing_did.as_str(), &ml, from_seq_no, &generating_agent.borrow())
        };

        {
            for agent in receiving_agents {
                deliver_msg_successfully(&network, &msg, &mut agent.borrow_mut());
            }
        }
    }

    #[test]
    fn test_new_agent_create_new_microledger() {
        TestUtils::cleanup_temp();
        let did = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let agent1 = get_new_agent(did, seed1, String::from(""));
        assert_eq!(agent1.managing_did, did);
        assert_eq!(agent1.verkey, "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC");
        assert!(agent1.m_ledgers.get(did).is_some());
        assert_eq!(agent1.get_self_microledger().unwrap().get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        let ml = agent1.m_ledgers.get(did).unwrap();
        assert_eq!(ml.get_root_hash(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(ml.get_size(), 0);
        assert_eq!(agent1.has_microledger("somerandomstring"), false);
    }

    #[test]
    fn test_new_connection_message() {
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let seed1 = String::from("11111111111111111111111111111111");
        let (agent1, conn1) = bootstrap_agent1(did1, seed1);
        let expected_message1 = r#"{"message":"{\"type\":\"ledgerUpdate\",\"state\":\"DID:75KUW8tPUQNBS4W7ibFeY8\",\"root\":\"c59e216c9207c5736670a70688e0caace20c2085333ba079842f0d9e1c250db3\",\"events\":[[1,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"dest\\\":\\\"75KUW8tPUQNBS4W7ibFeY8\\\",\\\"type\\\":\\\"1\\\"}}\"],[2,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"authorizations\\\":[\\\"all\\\"],\\\"type\\\":\\\"2\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"],[3,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"address\\\":\\\"https://agent.example.com\\\",\\\"type\\\":\\\"3\\\",\\\"verkey\\\":\\\"5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC\\\"}}\"]]}"}"#;
        println!("{}", &conn1);
        assert!(conn1.contains(expected_message1));
        assert!(conn1.contains(agent1.get_peer_id().as_str()));

        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let seed2 = String::from("99999999999999999999999999999999");
        let (agent2, conn2) = bootstrap_agent2(did2, seed2);
        let expected_message2 = r#"{"message":"{\"type\":\"ledgerUpdate\",\"state\":\"DID:84qiTnsJrdefBDMrF49kfa\",\"root\":\"63a09c731f706aeb38874e648da92f8194284d5f5d2aea5957f28573e51208f3\",\"events\":[[1,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"dest\\\":\\\"84qiTnsJrdefBDMrF49kfa\\\",\\\"type\\\":\\\"1\\\"}}\"],[2,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"authorizations\\\":[\\\"all\\\"],\\\"type\\\":\\\"2\\\",\\\"verkey\\\":\\\"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ\\\"}}\"],[3,\"{\\\"protocolVersion\\\":1,\\\"txnVersion\\\":1,\\\"operation\\\":{\\\"address\\\":\\\"https://agent2.example.com\\\",\\\"type\\\":\\\"3\\\",\\\"verkey\\\":\\\"4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ\\\"}}\"]]}"}"#;
        assert!(conn2.contains(expected_message2));
        assert!(conn2.contains(agent2.get_peer_id().as_str()));
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

        // Correct sig, correct key
        let sig = agent1.sign_msg(payload.as_bytes()).unwrap();
        let msg = Message::new(&payload, did1, &agent1.verkey, &sig);
        {
            deliver_msg_successfully(&network, &msg, &mut agent2);
        }

        // Incorrect sig, correct key
        let msg = Message::new(&payload, did1, &agent1.verkey, "4Be93xNcmaoHzUVK89Qz4aeQg9zMiC2PooegFWEY5aQEfzZo9uNgdjJJDQPj3K5Jj4gE5mERBetqLUBUu6G5cyX2");
        {
            deliver_msg_unsuccessfully(&network, &msg, &mut agent2);
        }
    }

    #[test]
    fn test_key_rotation1() {
        // 2 parties with each having 1 edge agent only, 1 party rotates its (only agent's) key
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut old_seq_no = 0;
        let old_verkey = "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC";
        let new_verkey = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
        let new_seq_no = {
            let ws1 = Rc::clone(&agent1.wallet_service);
            let wh1 = (&agent1).wallet_handle.clone();
            let ml = agent1.get_self_microledger_mut().unwrap();
            old_seq_no = ml.get_size();
            ml.add_key_txn(new_verkey, &vec![AUTHZ_ADD_KEY],
                           Some(&ws1),
                           Some(wh1),
                           Some(old_verkey)).unwrap();
            let new_seq_no = ml.add_key_txn(old_verkey, &vec![],
                                            Some(&ws1),
                                            Some(wh1),
                                            Some(old_verkey)).unwrap();
            assert_eq!((ml.get_size() - old_seq_no), 2);
            new_seq_no as u64
        };

        let ml1 = agent1.get_self_microledger().unwrap();

        let msg = new_ledger_update_msg(did1, &ml1, (old_seq_no+1) as u64, &agent1);
        {
            deliver_msg_successfully(&network, &msg, &mut agent2);
        }

        // Microledger matches
        let ml2 = agent2.m_ledgers.get(did1).unwrap();
        assert_eq!(ml1.get_root_hash(), ml2.get_root_hash());
        assert_eq!(ml1.get_size(), ml2.get_size());

        // DID doc has correct authorisations
        let doc = agent2.get_did_doc(did1).unwrap();
        let empty_str_vec: Vec<String> = vec![];
        assert_eq!(doc.borrow().get_key_authorisations(old_verkey).unwrap(), empty_str_vec);
        assert_eq!(doc.borrow().get_key_authorisations(new_verkey).unwrap(), vec![AUTHZ_ADD_KEY]);
    }

    fn alice_adds_cloud_agent<'a>(network: &'a Rc<RefCell<Network<'a>>>, acting_agent: Rc<RefCell<Agent<'a>>>,
                              other_agents: Vec<Rc<RefCell<Agent<'a>>>>, cloud_agent_authz: Vec<&'a str>) -> Rc<RefCell<Agent<'a>>> {
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let edge_verkey = "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC";
        let cloud_verkey = "2ru5PcgeQzxF7QZYwQgDkG2K13PRqyigVw99zMYg8eML";
        let cloud_agent_address = "https://agent1.example.com:9080";

        let mut agent3 =
            add_new_agent(network, acting_agent, other_agents, edge_verkey,
                          did1, cloud_verkey, "00000000000000000000000000000000",
                          "agent3", cloud_agent_address, cloud_agent_authz);
        Rc::new(RefCell::new(agent3))
    }

    fn bob_adds_cloud_agent<'a>(network: &'a Rc<RefCell<Network<'a>>>, acting_agent: Rc<RefCell<Agent<'a>>>,
                                other_agents: Vec<Rc<RefCell<Agent<'a>>>>, cloud_agent_authz: Vec<&'a str>) -> Rc<RefCell<Agent<'a>>> {
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let edge_verkey_2 = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
        let cloud_verkey_2 = "6X7FHuUPcfFm7HAqugAwpus6n9Pwk4RVjK5UtRsnGhxk";
        let cloud_agent_address_2 = "https://agent2.example.com:9090";

        let mut agent4 =
            add_new_agent(&network, acting_agent, other_agents, edge_verkey_2,
                          did2, cloud_verkey_2, "10101010101010101010101010101010",
                          "agent4", cloud_agent_address_2, vec![AUTHZ_MPROX]);
        Rc::new(RefCell::new(agent4))
    }

    fn both_alice_bob_add_cloud_agent<'a>(network: &'a Rc<RefCell<Network<'a>>>, alice_acting_agent: Rc<RefCell<Agent<'a>>>,
                                          bob_acting_agent: Rc<RefCell<Agent<'a>>>, alice_notifies_agents: Vec<Rc<RefCell<Agent<'a>>>>,
                                          bob_notifies_agents: Vec<Rc<RefCell<Agent<'a>>>>,
                                          alice_cloud_agent_authz: Vec<&'a str>, bob_cloud_agent_authz: Vec<&'a str>) -> (Rc<RefCell<Agent<'a>>>, Rc<RefCell<Agent<'a>>>){
        let mut agent3 = alice_adds_cloud_agent(network, alice_acting_agent, alice_notifies_agents, alice_cloud_agent_authz);
        let mut bob_notifies_agents = bob_notifies_agents.clone();
        bob_notifies_agents.push(Rc::clone(&agent3));
        let mut agent4 = bob_adds_cloud_agent(network, bob_acting_agent, bob_notifies_agents, bob_cloud_agent_authz);
        (agent3, agent4)
    }

    #[test]
    fn test_new_cloud_agent() {
        // Alice and Bob both have edge agents. Alice adds a new cloud agent
        TestUtils::cleanup_temp();
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));

        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let mut agent3 = alice_adds_cloud_agent(&network, a1, a2, vec![AUTHZ_MPROX]);

        // Microledger matches on cloud agent and other party's agent
        check_same_microledger_for_agents(vec![&Rc::clone(&agent1), &Rc::clone(&agent2), &Rc::clone(&agent3)], did1);
        check_same_microledger_for_agents(vec![&Rc::clone(&agent1), &Rc::clone(&agent2), &Rc::clone(&agent3)], did2);

        // DID doc has correct key authorisations
        let a2 = agent2.borrow();
        let doc = a2.get_did_doc(did1).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(&agent3.borrow().verkey).unwrap(), vec![AUTHZ_MPROX]);
    }

    #[test]
    fn test_key_rotation2() {
        // 2 parties, Alice and Bob. Alice has 1 cloud agent and 1 edge agent. Bob has 1 edge agent.
        // Alice uses its edge agent (`agent1`) to change cloud agent's (`agent3`) verkey

        TestUtils::cleanup_temp();

        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));

        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let mut agent3 = alice_adds_cloud_agent(&network, a1, a2, vec![AUTHZ_MPROX]);

        let agent1_verkey = "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC";
        let cloud_old_verkey = "2ru5PcgeQzxF7QZYwQgDkG2K13PRqyigVw99zMYg8eML";
        let cloud_new_verkey = "2btLJAAb1S3x6hZYdVyAePjqtQYi2ZBSRGy4569RZu8h";

        let new_seq_no = rotate_key(cloud_old_verkey, cloud_new_verkey,
                                    vec![AUTHZ_MPROX], agent1_verkey, Rc::clone(&agent1));

        gen_ledger_update_and_deliver_to_others(&network, Rc::clone(&agent1),
                                                new_seq_no-1,
                                                vec![Rc::clone(&agent2), Rc::clone(&agent3)]);

        check_same_microledger_for_agents(vec![&Rc::clone(&agent1),
                                               &Rc::clone(&agent2), &Rc::clone(&agent3)], did1);

        // DID doc has correct key authorisations
        let empty_str_vec: Vec<String> = vec![];
        let a2 = agent2.borrow();
        let doc = a2.get_did_doc(did1).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(cloud_new_verkey).unwrap(), vec![AUTHZ_MPROX]);
        assert_eq!(doc.borrow().get_key_authorisations(cloud_old_verkey).unwrap(), empty_str_vec);
        let a3 = agent3.borrow();
        let doc = a3.get_did_doc(did1).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(cloud_new_verkey).unwrap(), vec![AUTHZ_MPROX]);
        assert_eq!(doc.borrow().get_key_authorisations(cloud_old_verkey).unwrap(), empty_str_vec);
    }

    #[test]
    fn test_key_rotation3() {
        // 2 parties, Alice and Bob. Alice has 1 cloud agent and 1 edge agent. Bob has 1 edge agent.
        // Alice uses its cloud agent (`agent3`) to change edge agent's (`agent1`) verkey

        TestUtils::cleanup_temp();

        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let edge_verkey = "5rArie7XKukPCaEwq5XGQJnM9Fc5aZE3M9HAPVfMU2xC";
        let cloud_verkey = "2ru5PcgeQzxF7QZYwQgDkG2K13PRqyigVw99zMYg8eML";
        let cloud_agent_address = "https://agent1.example.com:9080";

        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));
        let mut agent3 =
            add_new_agent(&network, Rc::clone(&agent1), vec![Rc::clone(&agent2)], edge_verkey,
                          did1, cloud_verkey, "00000000000000000000000000000000",
                          "agent3", cloud_agent_address, vec![AUTHZ_ALL]);
        let mut agent3 = Rc::new(RefCell::new(agent3));

        check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3], did1);
        check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3], did2);

        let edge_new_verkey = "2btLJAAb1S3x6hZYdVyAePjqtQYi2ZBSRGy4569RZu8h";

        let new_seq_no = rotate_key(edge_verkey, edge_new_verkey,
                                    vec![AUTHZ_ADD_KEY, AUTHZ_REM_KEY], cloud_verkey,
                                    Rc::clone(&agent3));

        gen_ledger_update_and_deliver_to_others(&network, Rc::clone(&agent3),
                                                new_seq_no-1,
                                                vec![Rc::clone(&agent1), Rc::clone(&agent2)]);

        check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3], did1);

        // DID doc has correct key authorisations
        let empty_str_vec: Vec<String> = vec![];
        let a2 = agent2.borrow();
        let doc = a2.get_did_doc(did1).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(edge_new_verkey).unwrap(), vec![AUTHZ_ADD_KEY, AUTHZ_REM_KEY]);
        assert_eq!(doc.borrow().get_key_authorisations(edge_verkey).unwrap(), empty_str_vec);
        let a1 = agent1.borrow();
        let doc = a1.get_did_doc(did1).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(edge_new_verkey).unwrap(), vec![AUTHZ_ADD_KEY, AUTHZ_REM_KEY]);
        assert_eq!(doc.borrow().get_key_authorisations(edge_verkey).unwrap(), empty_str_vec);
    }

    #[test]
    fn test_key_rotation4() {
        // 2 parties, Alice and Bob. Alice has 1 cloud agent and 1 edge agent. Bob has 1 edge agent.
        // Bob rotates it's edge agent key (`agent2`)
        TestUtils::cleanup_temp();

        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));
        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let mut agent3 = alice_adds_cloud_agent(&network, a1, a2, vec![AUTHZ_ALL]);

        let old_edge_key = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
        let new_edge_key = "6X7FHuUPcfFm7HAqugAwpus6n9Pwk4RVjK5UtRsnGhxk";

        let new_seq_no = rotate_key(old_edge_key, new_edge_key,
                                    vec![AUTHZ_ALL], old_edge_key,
                                    Rc::clone(&agent2));

        gen_ledger_update_and_deliver_to_others(&network, Rc::clone(&agent2),
                                                new_seq_no-1,
                                                vec![Rc::clone(&agent1), Rc::clone(&agent3)]);

        check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3], did2);

        // DID doc has correct key authorisations
        let empty_str_vec: Vec<String> = vec![];
        let a1 = agent1.borrow();
        let doc = a1.get_did_doc(did2).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(new_edge_key).unwrap(), vec![AUTHZ_ALL]);
        assert_eq!(doc.borrow().get_key_authorisations(old_edge_key).unwrap(), empty_str_vec);
        let a3 = agent3.borrow();
        let doc = a3.get_did_doc(did2).unwrap();
        assert_eq!(doc.borrow().get_key_authorisations(new_edge_key).unwrap(), vec![AUTHZ_ALL]);
        assert_eq!(doc.borrow().get_key_authorisations(old_edge_key).unwrap(), empty_str_vec);
    }

    #[test]
    fn test_new_cloud_agent_1() {
        // Alice and Bob both have edge agents. Both Alice and Bob add a new cloud agent each.
        // Alice's cloud agent is `agent3`, Bob's cloud agent is `agent4`

        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));

        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let a3 = Rc::clone(&agent2);
        let a4 = vec![Rc::clone(&agent1)];
        let (mut agent3, mut agent4) = both_alice_bob_add_cloud_agent(&network,
                                                                      a1, a3,
                                                                      a2, a4,
                                                                      vec![AUTHZ_MPROX], vec![AUTHZ_MPROX]);

        {
            check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3, &agent4], did1);
            check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3, &agent4], did2);
        }
    }

    #[test]
    fn test_key_rotation5() {
        // 2 parties, Alice and Bob. Both Alice and Bob have 1 cloud agent and 1 edge agent each.
        // Bob rotates it's cloud agent key (`agent4`)

        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));

        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let a3 = Rc::clone(&agent2);
        let a4 = vec![Rc::clone(&agent1)];
        let (mut agent3, mut agent4) = both_alice_bob_add_cloud_agent(&network,
                                                                      a1, a3,
                                                                      a2, a4,
                                                                      vec![AUTHZ_MPROX], vec![AUTHZ_MPROX]);

        let bob_cloud_old_verkey = "6X7FHuUPcfFm7HAqugAwpus6n9Pwk4RVjK5UtRsnGhxk";
        let bob_cloud_new_verkey = "41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74";
        let agent_2_verkey = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
        let new_seq_no = rotate_key(bob_cloud_old_verkey, bob_cloud_new_verkey,
                                    vec![AUTHZ_MPROX], agent_2_verkey, Rc::clone(&agent2));

        gen_ledger_update_and_deliver_to_others(&network, Rc::clone(&agent2),
                                                new_seq_no-1,
                                                vec![Rc::clone(&agent1), Rc::clone(&agent3), Rc::clone(&agent4)]);

        check_same_microledger_for_agents(vec![&Rc::clone(&agent1),
                                               &Rc::clone(&agent2), &Rc::clone(&agent3), &Rc::clone(&agent4)], did2);
    }

    #[test]
    fn test_new_edge_agent() {
        // 2 parties, Alice and Bob. Both Alice and Bob have 1 cloud agent and 1 edge agent each.
        // Bob adds new edge agent (`agent5`)
        TestUtils::cleanup_temp();
        let did1 = "75KUW8tPUQNBS4W7ibFeY8";
        let did2 = "84qiTnsJrdefBDMrF49kfa";
        let (mut network, mut agent1, mut agent2) = connected_agents();
        let mut agent1 = Rc::new(RefCell::new(agent1));
        let mut agent2 = Rc::new(RefCell::new(agent2));

        let a1 = Rc::clone(&agent1);
        let a2 = vec![Rc::clone(&agent2)];
        let a3 = Rc::clone(&agent2);
        let a4 = vec![Rc::clone(&agent1)];
        let (mut agent3, mut agent4) = both_alice_bob_add_cloud_agent(&network,
                                                                      a1, a3,
                                                                      a2, a4,
                                                                      vec![AUTHZ_MPROX], vec![AUTHZ_MPROX]);
        let edge_verkey = "4AdS22kC7xzb4bcqg9JATuCfAMNcQYcZa1u5eWzs6cSJ";
        let new_edge_verkey = "41bgpk11WQ4NBHzbJH9YiRFFkkvzQrc25J4Y8839Dx74";
        let new_edge_agent_address = "https://agent100.example.com:9090";
        let other_agents = vec![Rc::clone(&agent1), Rc::clone(&agent3), Rc::clone(&agent4)];

        let mut agent5 =
            add_new_agent(&network, Rc::clone(&agent2), other_agents, edge_verkey,
                          did2, new_edge_verkey, "01010101010101010101010101010101",
                          "agent5", new_edge_agent_address, vec![AUTHZ_MPROX]);
        let agent5 = Rc::new(RefCell::new(agent5));

        {
            check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3, &agent4, &agent5], did1);
            check_same_microledger_for_agents(vec![&agent1, &agent2, &agent3, &agent4, &agent5], did2);
        }
    }
}