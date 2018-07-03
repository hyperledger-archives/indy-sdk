extern crate indy_crypto;
extern crate rmp_serde;
extern crate serde;
extern crate serde_json;
extern crate time;

use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use super::zmq;
use errors::common::CommonError;
use utils::crypto::verkey_builder::build_full_verkey;

use self::indy_crypto::bls;

use services::ledger::merkletree::merkletree::MerkleTree;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NodeData {
    pub alias: String,
    pub client_ip: Option<String>,
    #[serde(deserialize_with = "string_or_number")]
    #[serde(default)]
    pub client_port: Option<u64>,
    pub node_ip: Option<String>,
    #[serde(deserialize_with = "string_or_number")]
    #[serde(default)]
    pub node_port: Option<u64>,
    pub services: Option<Vec<String>>,
    pub blskey: Option<String>
}

fn string_or_number<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: serde::Deserializer<'de>
{
    let deser_res: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(serde_json::Value::String(s)) => match s.parse::<u64>() {
            Ok(num) => Ok(Some(num)),
            Err(err) => Err(serde::de::Error::custom(format!("Invalid Node transaction: {:?}", err)))
        },
        Ok(serde_json::Value::Number(n)) => match n.as_u64() {
            Some(num) => Ok(Some(num)),
            None => Err(serde::de::Error::custom(format!("Invalid Node transaction")))
        },
        Ok(serde_json::Value::Null) => Ok(None),
        _ => Err(serde::de::Error::custom(format!("Invalid Node transaction"))),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeTransaction {
    NodeTransactionV0(NodeTransactionV0),
    NodeTransactionV1(NodeTransactionV1)
}

impl JsonEncodable for NodeTransaction {}

impl<'a> JsonDecodable<'a> for NodeTransaction {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NodeTransactionV0 {
    pub data: NodeData,
    pub dest: String,
    pub identifier: String,
    #[serde(rename = "txnId")]
    pub txn_id: Option<String>,
    pub verkey: Option<String>,
    #[serde(rename = "type")]
    pub txn_type: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeTransactionV1 {
    pub txn: Txn,
    pub txn_metadata: Metadata,
    pub req_signature: ReqSignature,
    pub ver: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Txn {
    #[serde(rename = "type")]
    pub txn_type: String,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<i32>,
    pub data: TxnData,
    pub metadata: TxnMetadata
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub creation_time: Option<u64>,
    pub seq_no: Option<i32>,
    pub txn_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReqSignature {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub values: Option<Vec<ReqSignatureValue>>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ReqSignatureValue {
    pub from: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct TxnData {
    pub data: NodeData,
    pub dest: String,
    pub verkey: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TxnMetadata {
    pub req_id: Option<i64>,
    pub from: String
}

impl From<NodeTransaction> for NodeTransactionV1 {
    fn from(node_txn: NodeTransaction) -> Self {
        match node_txn {
            NodeTransaction::NodeTransactionV1(n_txn) => n_txn,
            NodeTransaction::NodeTransactionV0(n_txn) => {
                let txn = Txn {
                    txn_type: n_txn.txn_type,
                    protocol_version: None,
                    data: TxnData {
                        data: n_txn.data,
                        dest: n_txn.dest,
                        verkey: n_txn.verkey
                    },
                    metadata: TxnMetadata {
                        req_id: None,
                        from: n_txn.identifier
                    },
                };
                NodeTransactionV1 {
                    txn,
                    txn_metadata: Metadata {
                        seq_no: None,
                        txn_id: n_txn.txn_id,
                        creation_time: None
                    },
                    req_signature: ReqSignature {
                        type_: None,
                        values: None
                    },
                    ver: "1".to_string(),
                }
            }
        }
    }
}

impl NodeTransactionV1 {
    pub fn update(&mut self, other: &mut NodeTransactionV1) -> Result<(), CommonError> {
        assert_eq!(self.txn.data.dest, other.txn.data.dest);
        assert_eq!(self.txn.data.data.alias, other.txn.data.data.alias);

        if let Some(ref mut client_ip) = other.txn.data.data.client_ip {
            self.txn.data.data.client_ip = Some(client_ip.to_owned());
        }
        if let Some(ref mut client_port) = other.txn.data.data.client_port {
            self.txn.data.data.client_port = Some(client_port.to_owned());
        }
        if let Some(ref mut node_ip) = other.txn.data.data.node_ip {
            self.txn.data.data.node_ip = Some(node_ip.to_owned());
        }
        if let Some(ref mut node_port) = other.txn.data.data.node_port {
            self.txn.data.data.node_port = Some(node_port.to_owned());
        }
        if let Some(ref mut blskey) = other.txn.data.data.blskey {
            self.txn.data.data.blskey = Some(blskey.to_owned());
        }
        if let Some(ref mut services) = other.txn.data.data.services {
            self.txn.data.data.services = Some(services.to_owned());
        }
        if other.txn.data.verkey.is_some() {
            self.txn.data.verkey = Some(build_full_verkey(&self.txn.data.dest, other.txn.data.verkey.as_ref().map(String::as_str))?);
        }
        Ok(())
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CatchupRep {
    pub ledgerId: usize,
    pub consProof: Vec<String>,
    pub txns: HashMap<String, serde_json::Value>,
}

impl CatchupRep {
    pub fn min_tx(&self) -> Result<usize, CommonError> {
        let mut min = None;
        for (k, _) in self.txns.iter() {
            let val = k.parse::<usize>()
                .map_err(|err| CommonError::InvalidStructure(format!("{:?}", err)))?;
            match min {
                None => min = Some(val),
                Some(m) => if val < m { min = Some(val) }
            }
        }
        min.ok_or(CommonError::InvalidStructure(format!("Empty Map")))
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    ReplyV0(ReplyV0),
    ReplyV1(ReplyV1)
}

impl Reply {
    pub fn req_id(self) -> u64 {
        match self {
            Reply::ReplyV0(reply) => reply.result.req_id,
            Reply::ReplyV1(reply) => reply.result.txn.metadata.req_id
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyV0 {
    pub result: ResponseMetadata
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyV1 {
    pub result: ReplyResultV1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyResultV1 {
    pub txn: ReplyTxnV1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyTxnV1 {
    pub metadata: ResponseMetadata
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    ResponseV0(ResponseV0),
    ResponseV1(ResponseV1)
}

impl Response {
    pub fn req_id(&self) -> u64 {
        match self {
            &Response::ResponseV0(ref res) => res.req_id,
            &Response::ResponseV1(ref res) => res.metadata.req_id
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseV0 {
    pub req_id: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseV1 {
    pub metadata: ResponseMetadata
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    pub req_id: u64
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum PoolLedgerTxn {
    PoolLedgerTxnV0(PoolLedgerTxnV0),
    PoolLedgerTxnV1(PoolLedgerTxnV1)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolLedgerTxnV0 {
    pub txn: Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolLedgerTxnV1 {
    pub txn: PoolLedgerTxnDataV1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolLedgerTxnDataV1 {
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
    PoolLedgerTxns(PoolLedgerTxn),
    Ping,
    Pong,
}

impl Message {
    pub fn from_raw_str(str: &str) -> Result<Message, CommonError> {
        match str {
            "po" => Ok(Message::Pong),
            "pi" => Ok(Message::Ping),
            _ => Message::from_json(str).map_err(CommonError::from),
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
    pub blskey: Option<bls::VerKey>
}

pub struct CatchUpProcess {
    pub merkle_tree: MerkleTree,
    pub pending_reps: Vec<(CatchupRep, usize)>,
    pub resp_not_received_node_idx: HashSet<usize>,
}

pub trait MinValue {
    fn get_min_index(&self) -> Result<usize, CommonError>;
}

impl MinValue for Vec<(CatchupRep, usize)> {
    fn get_min_index(&self) -> Result<usize, CommonError> {
        let mut res = None;
        for (index, &(ref catchup_rep, _)) in self.iter().enumerate() {
            match res {
                None => { res = Some((catchup_rep, index)); }
                Some((min_rep, _)) => if catchup_rep.min_tx()? < min_rep.min_tx()? {
                    res = Some((catchup_rep, index));
                }
            }
        }
        Ok(res.ok_or(CommonError::InvalidStructure("Element not Found".to_string()))?.1)
    }
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
pub struct ResendableRequest {
    pub request: String,
    pub start_node: usize,
    pub next_node: usize,
    pub next_try_send_time: Option<time::Tm>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommandProcess {
    pub nack_cnt: usize,
    pub replies: HashMap<HashableValue, usize>,
    pub accum_replies : Option<HashableValue>,
    pub parent_cmd_ids: Vec<i32>,
    pub resendable_request: Option<ResendableRequest>,
    pub full_cmd_timeout: Option<time::Tm>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ZMQLoopAction {
    RequestToSend(RequestToSend),
    MessageToProcess(MessageToProcess),
    Terminate(i32),
    Refresh(i32),
    Timeout,
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