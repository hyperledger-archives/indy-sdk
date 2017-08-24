extern crate serde_json;
extern crate rmp_serde;

use std::cmp;
use std::cmp::Eq;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use super::zmq;

use services::ledger::merkletree::merkletree::MerkleTree;
use utils::json::{JsonDecodable, JsonEncodable};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NodeData {
    pub alias: String,
    pub client_ip: String,
    pub client_port: u32,
    pub node_ip: String,
    pub node_port: u32,
    pub services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GenTransaction {
    pub data: NodeData,
    pub dest: String,
    pub identifier: String,
    #[serde(rename = "txnId")]
    pub txn_id: Option<String>,
    #[serde(rename = "type")]
    pub txn_type: String,
}

impl JsonEncodable for GenTransaction {}

impl<'a> JsonDecodable<'a> for GenTransaction {}

impl GenTransaction {
    pub fn to_msg_pack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerStatus {
    pub txnSeqNo: usize,
    pub merkleRoot: String,
    pub ledgerId: u8,
    pub ppSeqNo: Option<String>,
    pub viewNo: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConsistencyProof {
    //TODO almost all fields Option<> or find better approach
    pub seqNoEnd: usize,
    pub seqNoStart: usize,
    pub ledgerId: usize,
    pub hashes: Vec<String>,
    pub oldMerkleRoot: String,
    pub newMerkleRoot: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CatchupReq {
    pub ledgerId: usize,
    pub seqNoStart: usize,
    pub seqNoEnd: usize,
    pub catchupTill: usize,
}

impl<'a> JsonDecodable<'a> for CatchupReq {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CatchupRep {
    pub ledgerId: usize,
    pub consProof: Vec<String>,
    pub txns: HashMap<String, GenTransaction>,
}

impl CatchupRep {
    pub fn min_tx(&self) -> usize {
        assert!(!self.txns.is_empty());
        (self.txns.keys().min().unwrap().parse::<usize>()).unwrap()
    }
}

impl cmp::Ord for CatchupRep {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.min_tx().cmp(&self.min_tx())
    }
}

impl cmp::PartialOrd for CatchupRep {
    fn partial_cmp(&self, other: &CatchupRep) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reply {
    pub result: Response,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub req_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolLedgerTxns {
    pub txn: Response,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleRequest {
    pub req_id: u64,
}

impl JsonEncodable for SimpleRequest {}

impl<'a> JsonDecodable<'a> for SimpleRequest {}

#[serde(tag = "op")]
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "CONSISTENCY_PROOF")]
    ConsistencyProof(ConsistencyProof),
    #[serde(rename = "LEDGER_STATUS")]
    LedgerStatus(LedgerStatus),
    #[serde(rename = "CATCHUP_REQ")]
    CatchupReq(CatchupReq),
    #[serde(rename = "CATCHUP_REP")]
    CatchupRep(CatchupRep),
    #[serde(rename = "REQACK")]
    ReqACK(Response),
    #[serde(rename = "REQNACK")]
    ReqNACK(Response),
    #[serde(rename = "REPLY")]
    Reply(Reply),
    #[serde(rename = "REJECT")]
    Reject(Response),
    #[serde(rename = "POOL_LEDGER_TXNS")]
    PoolLedgerTxns(PoolLedgerTxns),
    Ping,
    Pong,
}

impl Message {
    pub fn from_raw_str(str: &str) -> Result<Message, serde_json::Error> {
        match str {
            "po" => Ok(Message::Pong),
            "pi" => Ok(Message::Ping),
            _ => Message::from_json(str),
        }
    }
}

impl JsonEncodable for Message {}

impl<'a> JsonDecodable<'a> for Message {}

#[derive(Serialize, Deserialize)]
pub struct PoolConfig {
    pub genesis_txn: String
}

impl JsonEncodable for PoolConfig {}

impl<'a> JsonDecodable<'a> for PoolConfig {}

impl PoolConfig {
    pub fn default_for_name(name: &str) -> PoolConfig {
        let mut txn = name.to_string();
        txn += ".txn";
        PoolConfig { genesis_txn: txn }
    }
}

pub struct RemoteNode {
    pub name: String,
    pub public_key: Vec<u8>,
    pub zaddr: String,
    pub zsock: Option<zmq::Socket>,
    pub is_blacklisted: bool,
}

pub struct CatchUpProcess {
    pub merkle_tree: MerkleTree,
    pub pending_reps: BinaryHeap<(CatchupRep, usize)>,
}

#[derive(Debug)]
pub struct HashableValue {
    pub inner: serde_json::Value
}

impl Eq for HashableValue {}

impl Hash for HashableValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.inner).unwrap().hash(state); //TODO
    }
}

impl PartialEq for HashableValue {
    fn eq(&self, other: &HashableValue) -> bool {
        self.inner.eq(&other.inner)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandProcess {
    pub nack_cnt: usize,
    pub replies: HashMap<HashableValue, usize>,
    pub cmd_ids: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ZMQLoopAction {
    RequestToSend(RequestToSend),
    MessageToProcess(MessageToProcess),
    Terminate(i32),
    Refresh(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RequestToSend {
    pub request: String,
    pub id: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MessageToProcess {
    pub message: String,
    pub node_idx: usize,
}