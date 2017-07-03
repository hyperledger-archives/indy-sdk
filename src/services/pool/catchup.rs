use std::cmp;
use std::collections::{BinaryHeap};
use std::error::Error;

use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::common::CommonError;
use errors::pool::PoolError;
use super::{
    MerkleTree,
    RemoteNode,
};
use super::rust_base58::ToBase58;
use super::types::*;
use utils::json::JsonEncodable;

pub struct CatchupHandler {
    pub f: usize,
    pub ledger_status_same: usize,
    pub merkle_tree: MerkleTree,
    pub new_mt_size: usize,
    pub new_mt_vote: usize,
    pub nodes: Vec<RemoteNode>,
    pub initiate_cmd_id: i32,
    pub is_refresh: bool,
    pub pending_catchup: Option<CatchUpProcess>,
    pub pool_id: i32,
}

impl Default for CatchupHandler {
    fn default() -> Self {
        CatchupHandler {
            f: 0,
            ledger_status_same: 0,
            merkle_tree: MerkleTree::from_vec(Vec::new()).unwrap(),
            nodes: Vec::new(),
            new_mt_size: 0,
            new_mt_vote: 0,
            pending_catchup: None,
            initiate_cmd_id: 0,
            is_refresh: false,
            pool_id: 0,
        }
    }
}

impl CatchupHandler {
    pub fn process_msg(&mut self, msg: Message, raw_msg: &String, src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        match msg {
            Message::Pong => {
                //sending ledger status
                //TODO not send ledger status directly as response on ping, wait pongs from all nodes?
                let ls: LedgerStatus = LedgerStatus {
                    txnSeqNo: self.nodes.len(),
                    merkleRoot: self.merkle_tree.root_hash().as_slice().to_base58(),
                    ledgerId: 0,
                    ppSeqNo: None,
                    viewNo: None,
                };
                let resp_msg: Message = Message::LedgerStatus(ls);
                self.nodes[src_ind].send_msg(&resp_msg)?;
            }
            Message::LedgerStatus(ledger_status) => {
                if self.merkle_tree.root_hash().as_slice().to_base58().ne(ledger_status.merkleRoot.as_str()) {
                    return Err(PoolError::CommonError(
                        CommonError::InvalidState(
                            "Ledger merkle tree doesn't acceptable for current tree.".to_string())));
                }
                self.ledger_status_same += 1;
                if self.ledger_status_same == self.f + 1 {
                    return Ok(Some(self.merkle_tree.clone()));
                }
            }
            Message::ConsistencyProof(cons_proof) => {
                trace!("{:?}", cons_proof);
                if cons_proof.seqNoStart == self.merkle_tree.count()
                    && cons_proof.seqNoEnd > self.merkle_tree.count() {
                    self.new_mt_size = cmp::max(cons_proof.seqNoEnd, self.new_mt_size);
                    self.new_mt_vote += 1;
                    debug!("merkle tree expected size now {}", self.new_mt_size);
                }
                if self.new_mt_vote == self.f + 1 {
                    self.start_catchup()?;
                }
            }
            Message::CatchupRep(catchup) => {
                if let Some(new_mt) = self.process_catchup_rep(catchup)? {
                    return Ok(Some(new_mt));
                }
            }
            _ => {
                warn!("unhandled msg {:?}", msg);
            }
        };
        Ok(None)
    }

    pub fn start_catchup(&mut self) -> Result<(), PoolError> {
        trace!("start_catchup");
        if self.pending_catchup.is_some() {
            return Err(PoolError::CommonError(
                CommonError::InvalidState(
                    "CatchUp already started for the pool".to_string())));
        }
        let node_cnt = self.nodes.len();
        if self.merkle_tree.count() != node_cnt {
            return Err(PoolError::CommonError(
                CommonError::InvalidState(
                    "Merkle tree doesn't equal nodes count".to_string())));
        }
        let cnt_to_catchup = self.new_mt_size - self.merkle_tree.count();
        if cnt_to_catchup <= 0 {
            return Err(PoolError::CommonError(CommonError::InvalidState(
                "Nothing to CatchUp, but started".to_string())));
        }

        self.pending_catchup = Some(CatchUpProcess {
            merkle_tree: self.merkle_tree.clone(),
            pending_reps: BinaryHeap::new(),
        });

        let portion = (cnt_to_catchup + node_cnt - 1) / node_cnt; //TODO check standard round up div
        let mut catchup_req = CatchupReq {
            ledgerId: 0,
            seqNoStart: node_cnt + 1,
            seqNoEnd: node_cnt + 1 + portion - 1,
            catchupTill: self.new_mt_size,
        };
        for node in &self.nodes {
            node.send_msg(&Message::CatchupReq(catchup_req.clone()))?;
            catchup_req.seqNoStart += portion;
            catchup_req.seqNoEnd = cmp::min(catchup_req.seqNoStart + portion - 1,
                                            catchup_req.catchupTill);
        }
        Ok(())
    }

    pub fn process_catchup_rep(&mut self, catchup: CatchupRep) -> Result<Option<MerkleTree>, PoolError> {
        trace!("append {:?}", catchup);
        let catchup_finished = {
            let mut process = self.pending_catchup.as_mut()
                .ok_or(CommonError::InvalidState("Process non-existing CatchUp".to_string()))?;
            process.pending_reps.push(catchup);
            while !process.pending_reps.is_empty()
                && process.pending_reps.peek().unwrap().min_tx() - 1 == process.merkle_tree.count() {
                let mut first_resp = process.pending_reps.pop().unwrap();
                while !first_resp.txns.is_empty() {
                    let key = first_resp.min_tx().to_string();
                    let new_gen_tx = first_resp.txns
                        .remove(&key)
                        .unwrap()
                        .to_json()
                        .map_err(|err|
                            CommonError::InvalidState(
                                format!("Can't serialize gen-tx json: {}", err.description())))?;
                    trace!("append to tree {}", new_gen_tx);
                    process.merkle_tree.append(
                        new_gen_tx
                    )?;
                }
            }
            trace!("updated mt hash {}, tree {:?}", process.merkle_tree.root_hash().as_slice().to_base58(), process.merkle_tree);
            if &process.merkle_tree.count() == &self.new_mt_size {
                //TODO check also root hash?
                true
            } else {
                false
            }
        };
        if catchup_finished {
            return Ok(Some(self.finish_catchup()?));
        }
        Ok(None)
    }

    pub fn finish_catchup(&mut self) -> Result<MerkleTree, PoolError> {
        Ok(self.pending_catchup.take().
            ok_or(CommonError::InvalidState("Try to finish non-existing CatchUp".to_string()))?
            .merkle_tree)
    }

    pub fn flush_requests(&mut self, status: Result<(), PoolError>) -> Result<(), PoolError> {
        let cmd = if self.is_refresh {
            PoolCommand::RefreshAck(self.initiate_cmd_id, status)
        } else {
            PoolCommand::OpenAck(self.initiate_cmd_id, status.map(|()| self.pool_id))
        };
        CommandExecutor::instance()
            .send(Command::Pool(cmd))
            .map_err(|err|
                PoolError::CommonError(
                    CommonError::InvalidState("Can't send ACK cmd".to_string())))
    }
}
