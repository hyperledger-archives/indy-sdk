use std::cmp;
use std::collections::BinaryHeap;

use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::common::CommonError;
use errors::pool::PoolError;
use super::{
    MerkleTree,
    RemoteNode,
};
use super::rust_base58::{FromBase58, ToBase58};
use super::types::*;
use utils::json::JsonEncodable;

enum CatchupStepResult {
    Finished,
    Continue,
    FailedAtNode(usize),
}

pub struct CatchupHandler {
    pub f: usize,
    pub ledger_status_same: usize,
    pub merkle_tree: MerkleTree,
    pub target_mt_size: usize,
    pub target_mt_root: Vec<u8>,
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
            target_mt_size: 0,
            new_mt_vote: 0,
            target_mt_root: Vec::new(),
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
                    self.target_mt_size = cmp::max(cons_proof.seqNoEnd, self.target_mt_size);
                    self.new_mt_vote += 1;
                    self.target_mt_root = cons_proof.newMerkleRoot.from_base58().unwrap();
                    debug!("merkle tree expected size now {}", self.target_mt_size);
                }
                if self.new_mt_vote == self.f + 1 {
                    self.start_catchup()?;
                }
            }
            Message::CatchupRep(catchup) => {
                if let Some(new_mt) = self.process_catchup_rep(catchup, src_ind)? {
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
        if self.merkle_tree.count() != self.nodes.len() {
            return Err(PoolError::CommonError(
                CommonError::InvalidState(
                    "Merkle tree doesn't equal nodes count".to_string())));
        }

        let node_cnt = self.nodes.iter().filter(|node| !node.is_blacklisted).count();
        let cnt_to_catchup = self.target_mt_size - self.merkle_tree.count();
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
            catchupTill: self.target_mt_size,
        };
        for node in &self.nodes {
            if node.is_blacklisted {
                continue;
            }
            node.send_msg(&Message::CatchupReq(catchup_req.clone()))?;
            catchup_req.seqNoStart += portion;
            catchup_req.seqNoEnd = cmp::min(catchup_req.seqNoStart + portion - 1,
                                            catchup_req.catchupTill);
        }
        Ok(())
    }

    pub fn process_catchup_rep(&mut self, catchup: CatchupRep, node_idx: usize) -> Result<Option<MerkleTree>, PoolError> {
        trace!("append {:?}", catchup);
        let catchup_finished = self.catchup_step(catchup, node_idx)?;
        match catchup_finished {
            CatchupStepResult::Finished => return Ok(Some(self.finish_catchup()?)),
            CatchupStepResult::Continue => { /* nothing to do */ }
            CatchupStepResult::FailedAtNode(failed_node_idx) => {
                warn!("Fail to continue catch-up by response from node with idx {}. Node will be blacklisted and catchup will be restarted", failed_node_idx);
                self.nodes[failed_node_idx].is_blacklisted = true;
                self.pending_catchup = None;
                self.start_catchup()?
            }
        }
        Ok(None)
    }

    fn catchup_step(&mut self, catchup: CatchupRep, node_idx: usize) -> Result<CatchupStepResult, PoolError> {
        let mut process = self.pending_catchup.as_mut()
            .ok_or(CommonError::InvalidState("Process non-existing CatchUp".to_string()))?;
        process.pending_reps.push((catchup, node_idx));
        while !process.pending_reps.is_empty()
            && process.pending_reps.peek().unwrap().0.min_tx() - 1 == process.merkle_tree.count() {
            let (mut first_resp, node_idx) = process.pending_reps.pop().unwrap();
            let mut temp_mt = process.merkle_tree.clone();
            while !first_resp.txns.is_empty() {
                let key = first_resp.min_tx().to_string();
                if let Ok(new_gen_tx) = first_resp.txns.remove(&key).unwrap().to_json() {
                    trace!("append to tree {}", new_gen_tx);
                    temp_mt.append(new_gen_tx)?;
                } else {
                    return Ok(CatchupStepResult::FailedAtNode(node_idx));
                }
            }

            if CatchupHandler::check_cons_proofs(&temp_mt, &first_resp.consProof, &self.target_mt_root, self.target_mt_size).is_err() {
                return Ok(CatchupStepResult::FailedAtNode(node_idx));
            }

            process.merkle_tree = temp_mt;
        }
        trace!("updated mt hash {}, tree {:?}", process.merkle_tree.root_hash().as_slice().to_base58(), process.merkle_tree);
        if &process.merkle_tree.count() == &self.target_mt_size {
            if process.merkle_tree.root_hash().ne(&self.target_mt_root) {
                return Err(PoolError::CommonError(CommonError::InvalidState(
                    "CatchUp failed: all transactions added, proofs checked, but root hash differ with target".to_string())));
            }
            return Ok(CatchupStepResult::Finished);
        } else {
            return Ok(CatchupStepResult::Continue);
        }
    }

    fn check_cons_proofs(mt: &MerkleTree, cons_proofs: &Vec<String>, target_mt_root: &Vec<u8>, target_mt_size: usize) -> Result<(), CommonError> {
        let mut bytes_proofs: Vec<Vec<u8>> = Vec::new();
        for cons_proof in cons_proofs {
            let cons_proof: &String = cons_proof;
            bytes_proofs.push(cons_proof.from_base58().map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't decode node consistency proof: {}", err)))?)
        }
        assert!(mt.consistency_proof(target_mt_root, target_mt_size, &bytes_proofs)?);
        Ok(())
    }

    fn finish_catchup(&mut self) -> Result<MerkleTree, PoolError> {
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
