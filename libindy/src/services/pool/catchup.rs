extern crate rmp_serde;
extern crate time;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::common::CommonError;
use errors::pool::PoolError;
use self::time::Duration;
use super::{
    MerkleTree,
    RemoteNode,
};
use super::rust_base58::{FromBase58, ToBase58};
use super::types::*;

pub const CATCHUP_ROUND_TIMEOUT: i64 = 50;

enum CatchupStepResult {
    Finished,
    Continue,
    FailedAtNode(usize),
}

enum CatchupProgress {
    ShouldBeStarted,
    NotNeeded,
    Finished(MerkleTree),
    InProgress,
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
    pub timeout: time::Tm,
    pub pool_id: i32,
    pub nodes_votes: Vec<Option<(String, usize)>>,
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
            nodes_votes: Vec::new(),
            timeout: time::now_utc(),
        }
    }
}

impl CatchupHandler {
    pub fn process_msg(&mut self, msg: Message, _raw_msg: &String, src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        let catchup_status: CatchupProgress = match msg {
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
                CatchupProgress::InProgress
            }
            Message::LedgerStatus(ledger_status) => {
                self.nodes_votes[src_ind] = Some((ledger_status.merkleRoot, ledger_status.txnSeqNo));
                self.check_nodes_responses_on_status()?
            }
            Message::ConsistencyProof(cons_proof) => {
                self.nodes_votes[src_ind] = Some((cons_proof.newMerkleRoot, cons_proof.seqNoEnd));
                self.check_nodes_responses_on_status()?
            }
            Message::CatchupRep(catchup) => {
                self.process_catchup_rep(catchup, src_ind)?
            }
            _ => {
                warn!("unhandled msg {:?}", msg);
                CatchupProgress::InProgress
            }
        };

        match catchup_status {
            CatchupProgress::Finished(mt) => return Ok(Some(mt)),
            CatchupProgress::NotNeeded => return Ok(Some(self.merkle_tree.clone())),
            CatchupProgress::ShouldBeStarted => self.start_catchup()?,
            CatchupProgress::InProgress => { /* nothing to do */ }
        }
        Ok(None)
    }

    fn check_nodes_responses_on_status(&mut self) -> Result<CatchupProgress, PoolError> {
        if self.pending_catchup.is_some() {
            return Ok(CatchupProgress::InProgress);
        }
        let mut votes: HashMap<(String, usize), usize> = HashMap::new();
        for node_vote in &self.nodes_votes {
            if let &Some(ref node_vote) = node_vote {
                let cnt = *votes.get(&node_vote).unwrap_or(&0) + 1;
                votes.insert((node_vote.0.clone(), node_vote.1), cnt);
            }
        }
        if let Some((most_popular_vote, votes_cnt)) = votes.iter().max_by_key(|entry| entry.1) {
            if *votes_cnt == self.nodes.len() - self.f {
                let &(ref target_mt_root, target_mt_size) = most_popular_vote;
                let cur_mt_size = self.merkle_tree.count();
                let cur_mt_hash = self.merkle_tree.root_hash().to_base58();
                if target_mt_size == cur_mt_size {
                    if cur_mt_hash.eq(target_mt_root) {
                        return Ok(CatchupProgress::NotNeeded);
                    } else {
                        return Err(PoolError::CommonError(CommonError::InvalidState(
                            "Ledger merkle tree doesn't acceptable for current tree.".to_string())));
                    }
                } else if target_mt_size > cur_mt_size {
                    self.target_mt_size = target_mt_size;
                    self.target_mt_root = target_mt_root.from_base58().map_err(|_|
                        CommonError::InvalidStructure(
                            "Can't parse target MerkleTree hash from nodes responses".to_string()))?;
                    return Ok(CatchupProgress::ShouldBeStarted);
                } else {
                    return Err(PoolError::CommonError(CommonError::InvalidState(
                        "Local merkle tree greater than mt from ledger".to_string())));
                }
            }
        }
        Ok(CatchupProgress::InProgress)
    }

    pub fn reset_nodes_votes(&mut self) {
        self.nodes_votes.clear();
        self.nodes_votes.resize(self.nodes.len(), None);
    }

    pub fn start_catchup(&mut self) -> Result<(), PoolError> {
        trace!("start_catchup");
        if self.pending_catchup.is_some() {
            return Err(PoolError::CommonError(
                CommonError::InvalidState(
                    "CatchUp already started for the pool".to_string())));
        }

        let active_node_cnt = self.nodes.iter().filter(|node| !node.is_blacklisted).count();

        if active_node_cnt == 0 {
            // TODO FIXME
            return Err(PoolError::Terminate);
        }

        let txns_cnt_in_cur_mt = self.merkle_tree.count();
        let cnt_to_catchup = self.target_mt_size - txns_cnt_in_cur_mt;
        if cnt_to_catchup <= 0 {
            return Err(PoolError::CommonError(CommonError::InvalidState(
                "Nothing to CatchUp, but started".to_string())));
        }

        self.pending_catchup = Some(CatchUpProcess {
            merkle_tree: self.merkle_tree.clone(),
            pending_reps: Vec::new(),
            resp_not_received_node_idx: HashSet::new(),
        });
        self.timeout = time::now_utc().add(Duration::seconds(CATCHUP_ROUND_TIMEOUT));

        let portion = (cnt_to_catchup + active_node_cnt - 1) / active_node_cnt; //TODO check standard round up div
        let mut catchup_req = CatchupReq {
            ledgerId: 0,
            seqNoStart: txns_cnt_in_cur_mt + 1,
            seqNoEnd: txns_cnt_in_cur_mt + 1 + portion - 1,
            catchupTill: self.target_mt_size,
        };
        for idx in 0..self.nodes.len() {
            let node = &self.nodes[idx];
            //TODO do not perform duplicate requests, update resp_not_received_node_idx
            if node.is_blacklisted {
                continue;
            }
            self.pending_catchup.as_mut().map(|pc| {
                pc.resp_not_received_node_idx.insert(idx);
            });
            node.send_msg(&Message::CatchupReq(catchup_req.clone()))?;
            catchup_req.seqNoStart += portion;
            catchup_req.seqNoEnd = cmp::min(catchup_req.seqNoStart + portion - 1,
                                            catchup_req.catchupTill);

            if catchup_req.seqNoStart > catchup_req.seqNoEnd {
                // We don't have more portions to ask
                break;
            }
        }
        Ok(())
    }

    fn process_catchup_rep(&mut self, catchup: CatchupRep, node_idx: usize) -> Result<CatchupProgress, PoolError> {
        trace!("append {:?}", catchup);
        let catchup_finished = self.catchup_step(catchup, node_idx)?;
        match catchup_finished {
            CatchupStepResult::Finished => return Ok(CatchupProgress::Finished(self.finish_catchup()?)),
            CatchupStepResult::Continue => { /* nothing to do */ }
            CatchupStepResult::FailedAtNode(failed_node_idx) => {
                warn!("Fail to continue catch-up by response from node with idx {}. Node will be blacklisted and catchup will be restarted", failed_node_idx);
                self.nodes[failed_node_idx].is_blacklisted = true;
                self.pending_catchup = None;
                // TODO may be send ledger status again and re-obtain target MerkleTree params
                self.start_catchup()?
            }
        }
        Ok(CatchupProgress::InProgress)
    }

    fn catchup_step(&mut self, catchup: CatchupRep, node_idx: usize) -> Result<CatchupStepResult, PoolError> {
        let process = self.pending_catchup.as_mut()
            .ok_or(CommonError::InvalidState("Process non-existing CatchUp".to_string()))?;
        process.pending_reps.push((catchup, node_idx));
        process.resp_not_received_node_idx.remove(&node_idx);

        while !process.pending_reps.is_empty() {
            let index = process.pending_reps.get_min_index()?;
            {
                let &mut (ref mut first_resp, node_idx) = process.pending_reps.get_mut(index)
                    .ok_or(CommonError::InvalidStructure(format!("Element not Found")))?;
                if first_resp.min_tx()? - 1 != process.merkle_tree.count() { break; }

                let mut temp_mt = process.merkle_tree.clone();
                while !first_resp.txns.is_empty() {
                    let key = first_resp.min_tx()?.to_string();
                    let new_gen_tx = first_resp.txns.remove(&key).unwrap();
                    if let Ok(new_get_txn_bytes) = rmp_serde::to_vec_named(&new_gen_tx) {
                        temp_mt.append(new_get_txn_bytes)?;
                    } else {
                        return Ok(CatchupStepResult::FailedAtNode(node_idx));
                    }
                }

                if CatchupHandler::check_cons_proofs(&temp_mt, &first_resp.consProof, &self.target_mt_root, self.target_mt_size)
                    .map_err(map_err_err!()).is_err() {
                    return Ok(CatchupStepResult::FailedAtNode(node_idx));
                }

                process.merkle_tree = temp_mt;
            }
            process.pending_reps.remove(index);
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
        if !mt.consistency_proof(target_mt_root, target_mt_size, &bytes_proofs)? {
            return Err(CommonError::InvalidStructure("Consistency proof verification failed".to_string()));
        }
        Ok(())
    }

    fn finish_catchup(&mut self) -> Result<MerkleTree, PoolError> {
        Ok(self.pending_catchup.take().
            ok_or(CommonError::InvalidState("Try to finish non-existing CatchUp".to_string()))?
            .merkle_tree)
    }

    pub fn flush_requests(&mut self, status: Result<(), PoolError>) -> Result<(), PoolError> {
        if self.initiate_cmd_id == -1 {
            return Ok(());
        }
        let cmd = if self.is_refresh {
            PoolCommand::RefreshAck(self.initiate_cmd_id, status)
        } else {
            PoolCommand::OpenAck(self.initiate_cmd_id, self.pool_id, status)
        };
        self.initiate_cmd_id = -1;
        CommandExecutor::instance()
            .send(Command::Pool(cmd))
            .map_err(|err|
                PoolError::CommonError(
                    CommonError::InvalidState(format!("Can't send ACK cmd: {:?}", err))))
    }

    pub fn get_upcoming_timeout(&self) -> Option<time::Tm> {
        Some(self.timeout)
    }

    pub fn process_timeout(&mut self) -> Result<(), PoolError> {
        let pc = if let Some(pc) = self.pending_catchup.take() { pc } else {
            return Err(PoolError::Timeout);
        };
        warn!("Fail to continue catch-up response(s) not received from nodes with idx {:?}. Node will be blacklisted and catchup will be restarted", pc.resp_not_received_node_idx);
        pc.resp_not_received_node_idx.iter()
            .for_each(|idx| self.nodes[*idx].is_blacklisted = true);
        // TODO may be send ledger status again and re-obtain target MerkleTree params
        self.start_catchup()
    }
}
