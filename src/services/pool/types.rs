use std::cmp;
use std::collections::{BinaryHeap, HashMap};

use services::ledger::merkletree::merkletree::MerkleTree;

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
    pub txn_id: String,
    #[serde(rename = "type")]
    pub txn_type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerStatus {
    pub txnSeqNo: usize,
    pub merkleRoot: String,
    pub ledgerType: u8,
}

impl Default for LedgerStatus {
    fn default() -> LedgerStatus {
        LedgerStatus {
            ledgerType: 0,
            merkleRoot: "".to_string(),
            txnSeqNo: 0,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConsistencyProof {
    //TODO almost all fields Option<> or find better approach
    pub seqNoEnd: usize,
    pub seqNoStart: usize,
    pub ledgerType: usize,
    pub hashes: Vec<String>,
    pub oldMerkleRoot: String,
    pub newMerkleRoot: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CatchupReq {
    pub ledgerType: usize,
    pub seqNoStart: usize,
    pub seqNoEnd: usize,
    pub catchupTill: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CatchupRep {
    pub ledgerType: usize,
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
}

#[derive(Serialize, Deserialize)]
pub struct PoolConfig {
    pub genesis_txn: String
}

impl PoolConfig {
    pub fn default_for_name(name: &str) -> PoolConfig {
        let mut txn = name.to_string();
        txn += ".txn";
        PoolConfig { genesis_txn: txn }
    }
}

pub struct CatchUpProcess {
    pub merkle_tree: MerkleTree,
    pub pending_reps: BinaryHeap<CatchupRep>,
}
