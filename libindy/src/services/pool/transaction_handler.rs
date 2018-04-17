extern crate digest;
extern crate hex;
extern crate rand;
extern crate rust_base58;
extern crate sha2;
extern crate time;
extern crate rmp_serde;
extern crate indy_crypto;

use base64;
use self::digest::{FixedOutput, Input};
use self::hex::ToHex;
use self::rand::Rng;
use self::rust_base58::FromBase58;
use self::time::{Duration, Tm};
use serde_json;
use serde_json::Value as SJsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Add;

use super::state_proof;
use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use errors::pool::PoolError;
use errors::common::CommonError;
use super::types::*;
use services::ledger::constants;
use services::ledger::merkletree::merkletree::MerkleTree;
use self::indy_crypto::bls::Generator;

const REQUESTS_FOR_STATE_PROOFS: [&'static str; 4] = [constants::GET_NYM, constants::GET_SCHEMA, constants::GET_CRED_DEF, constants::GET_ATTR];
const RESENDABLE_REQUEST_TIMEOUT: i64 = 1;
const REQUEST_TIMEOUT_ACK: i64 = 10;
const REQUEST_TIMEOUT_REPLY: i64 = 100;

pub struct TransactionHandler {
    gen: Generator,
    pub f: usize,
    pub nodes: Vec<RemoteNode>,
    pending_commands: HashMap<u64 /* requestId */, CommandProcess>,
}

impl TransactionHandler {
    pub fn process_msg(&mut self, msg: Message, raw_msg: &String, _src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        match msg {
            Message::Reply(reply) => {
                self.process_reply(reply.result.req_id, raw_msg);
            }
            Message::Reject(response) | Message::ReqNACK(response) => {
                self.process_reject(&response, raw_msg);
            }
            Message::ReqACK(ack) => {
                self.process_ack(&ack, raw_msg);
            }
            _ => {
                warn!("unhandled msg {:?}", msg);
            }
        };
        Ok(None)
    }

    fn process_ack(&mut self, ack: &Response, raw_msg: &str) {
        trace!("TransactionHandler::process_ack: >>> ack: {:?}, raw_msg: {:?}", ack, raw_msg);

        self.pending_commands.get_mut(&ack.req_id).map(|cmd| {
            debug!("TransactionHandler::process_ack: update timeout for req_id: {:?}", ack.req_id);
            cmd.full_cmd_timeout
                = Some(time::now_utc().add(Duration::seconds(REQUEST_TIMEOUT_REPLY)))
        });

        trace!("TransactionHandler::process_ack: <<<");
    }

    fn process_reply(&mut self, req_id: u64, raw_msg: &str) {
        trace!("TransactionHandler::process_reply: >>> req_id: {:?}, raw_msg: {:?}", req_id, raw_msg);

        if !self.pending_commands.contains_key(&req_id) {
            return warn!("TransactionHandler::process_reply: <<< No pending command for request");
        }

        let msg_result: SJsonValue = match serde_json::from_str::<SJsonValue>(raw_msg) {
            Ok(raw_msg) => raw_msg["result"].clone(),
            Err(err) => return warn!("{:?}", err)
        };
        let mut msg_result_without_proof: SJsonValue = msg_result.clone();
        msg_result_without_proof.as_object_mut().map(|obj| obj.remove("state_proof"));
        if msg_result_without_proof["data"].is_object() {
            msg_result_without_proof["data"].as_object_mut().map(|obj| obj.remove("stateProofFrom"));
        }
        let msg_result_without_proof = HashableValue { inner: msg_result_without_proof };

        let reply_cnt = *self.pending_commands
            .get(&req_id).unwrap()
            .replies.get(&msg_result_without_proof).unwrap_or(&0usize);
        trace!("TransactionHandler::process_reply: reply_cnt: {:?}, f: {:?}", reply_cnt, self.f);

        let consensus_reached = reply_cnt >= self.f || {
            debug!("TransactionHandler::process_reply: Try to verify proof and signature");

            let data_to_check_proof = TransactionHandler::parse_reply_for_proof_checking(&msg_result);
            let data_to_check_proof_signature = TransactionHandler::parse_reply_for_proof_signature_checking(&msg_result);

            data_to_check_proof.is_some() && data_to_check_proof_signature.is_some() && {
                debug!("TransactionHandler::process_reply: Proof and signature are present");

                let (proofs, root_hash, key, value) = data_to_check_proof.unwrap();

                let proof_valid = state_proof::verify_proof(
                    base64::decode(proofs).unwrap().as_slice(),
                    root_hash.from_base58().unwrap().as_slice(),
                    key.as_slice(),
                    value.as_ref().map(String::as_str));


                debug!("TransactionHandler::process_reply: proof_valid: {:?}", proof_valid);

                proof_valid && {
                    let (signature, participants, value) = data_to_check_proof_signature.unwrap();
                    let signature_valid = state_proof::verify_proof_signature(
                        signature,
                        participants.as_slice(),
                        &value,
                        self.nodes.as_slice(), self.f, &self.gen).map_err(|err| warn!("{:?}", err)).unwrap_or(false);

                    debug!("TransactionHandler::process_reply: signature_valid: {:?}", signature_valid);
                    signature_valid
                }
            }
        };

        debug!("TransactionHandler::process_reply: consensus_reached {}", consensus_reached);

        if consensus_reached {
            let cmd_ids = self.pending_commands.get(&req_id).unwrap().parent_cmd_ids.clone();

            for cmd_id in cmd_ids {
                CommandExecutor::instance().send(
                    Command::Ledger(LedgerCommand::SubmitAck(cmd_id, Ok(raw_msg.to_owned())))).unwrap();
            }

            self.pending_commands.remove(&req_id);
        } else {
            let pend_cmd: &mut CommandProcess = self.pending_commands.get_mut(&req_id).unwrap();
            pend_cmd.replies.insert(msg_result_without_proof, reply_cnt + 1);
            pend_cmd.try_send_to_next_node_if_exists(&self.nodes);
        }

        trace!("TransactionHandler::process_reply: <<<");
    }

    //TODO correct handling of Reject
    fn process_reject(&mut self, response: &Response, raw_msg: &String) {
        let req_id = response.req_id;
        let mut remove = false;
        if let Some(pend_cmd) = self.pending_commands.get_mut(&req_id) {
            pend_cmd.nack_cnt += 1;
            if pend_cmd.nack_cnt == self.f + 1 {
                for &cmd_id in &pend_cmd.parent_cmd_ids {
                    CommandExecutor::instance().send(
                        Command::Ledger(
                            LedgerCommand::SubmitAck(cmd_id,
                                                     Ok(raw_msg.clone())))
                    ).unwrap();
                }
                remove = true;
            } else {
                pend_cmd.try_send_to_next_node_if_exists(&self.nodes);
            }
        }
        if remove {
            self.pending_commands.remove(&req_id);
        }
    }

    pub fn try_send_request(&mut self, req_str: &str, cmd_id: i32) -> Result<(), PoolError> {
        info!("cmd {:?}", req_str);
        let req_json: SJsonValue = serde_json::from_str(req_str)
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid request json: {}", err.description())))?;

        let req_id: u64 = req_json["reqId"]
            .as_u64()
            .ok_or(CommonError::InvalidStructure("No reqId in request".to_string()))?;

        if let Some(in_progress_req) = self.pending_commands.get_mut(&req_id) {
            in_progress_req.parent_cmd_ids.push(cmd_id);
            let new_req_differ_cached = in_progress_req
                .resendable_request.as_ref()
                // TODO pop request filed from ResendableRequest to CommandProcess and check always
                .map(|req| req.request.ne(req_str)).unwrap_or(false);
            if new_req_differ_cached {
                return Err(PoolError::CommonError(CommonError::InvalidStructure(
                    "Different request already sent with same request ID".to_string())));
            } else {
                return Ok(());
            }
        }

        let mut new_request = CommandProcess {
            parent_cmd_ids: vec!(cmd_id),
            nack_cnt: 0,
            replies: HashMap::new(),
            resendable_request: None,
            full_cmd_timeout: Some(time::now_utc().add(Duration::seconds(REQUEST_TIMEOUT_ACK))),
        };

        if REQUESTS_FOR_STATE_PROOFS.contains(&req_json["operation"]["type"].as_str().unwrap_or("")) {
            let start_node = rand::StdRng::new().unwrap().gen_range(0, self.nodes.len());
            let resendable_request = ResendableRequest {
                request: req_str.to_string(),
                start_node,
                next_node: (start_node + 1) % self.nodes.len(),
                next_try_send_time: Some(time::now_utc().add(Duration::seconds(RESENDABLE_REQUEST_TIMEOUT))),
            };
            trace!("try_send_request schedule next sending to {:?}", resendable_request.next_try_send_time);
            new_request.resendable_request = Some(resendable_request);
            self.nodes[start_node].send_str(req_str)?;
        } else {
            for node in &self.nodes {
                node.send_str(req_str)?;
            }
        }
        self.pending_commands.insert(req_id, new_request);
        Ok(())
    }

    pub fn flush_requests(&mut self, status: Result<(), PoolError>) -> Result<(), PoolError> {
        match status {
            Ok(()) => {
                return Err(PoolError::CommonError(
                    CommonError::InvalidState(
                        "Can't flash all transaction requests with common success status".to_string())));
            }
            Err(_) => {
                for (_, pending_cmd) in &mut self.pending_commands {
                    pending_cmd.terminate_parent_cmds(false)?
                }
                Ok(())
            }
        }
    }

    fn parse_reply_for_proof_checking(json_msg: &SJsonValue)
                                      -> Option<(&str, &str, Vec<u8>, Option<String>)> {
        trace!("TransactionHandler::parse_reply_for_proof_checking: >>> json_msg: {:?}", json_msg);

        let xtype = if let Some(xtype) = json_msg["type"].as_str() {
            trace!("TransactionHandler::parse_reply_for_proof_checking: xtype: {:?}", xtype);
            xtype
        } else {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No type field");
            return None;
        };

        if !REQUESTS_FOR_STATE_PROOFS.contains(&xtype) {
            //TODO GET_DDO, GET_TXN
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< type not supported");
            return None;
        }

        let proof = if let Some(proof) = json_msg["state_proof"]["proof_nodes"].as_str() {
            trace!("TransactionHandler::parse_reply_for_proof_checking: proof: {:?}", proof);
            proof
        } else {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No proof");
            return None;
        };

        let root_hash = if let Some(root_hash) = json_msg["state_proof"]["root_hash"].as_str() {
            trace!("TransactionHandler::parse_reply_for_proof_checking: root_hash: {:?}", root_hash);
            root_hash
        } else {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No root hash");
            return None;
        };

        // TODO: FIXME: It is a workaround for Node's problem. Node returns some transactions as strings and some as objects.
        // If node returns marshaled json it can contain spaces and it can cause invalid hash.
        // So we have to save the original string too.
        // See https://jira.hyperledger.org/browse/INDY-699
        let (data, parsed_data): (Option<String>, SJsonValue) = match json_msg["data"] {
            SJsonValue::Null => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: Data is null");
                (None, SJsonValue::Null)
            }
            SJsonValue::String(ref str) => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: Data is string");
                if let Ok(parsed_data) = serde_json::from_str(str) {
                    (Some(str.to_owned()), parsed_data)
                } else {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Data field is invalid json");
                    return None;
                }
            }
            SJsonValue::Object(ref map) => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: Data is object");
                (Some(json_msg["data"].to_string()), SJsonValue::from(map.clone()))
            }
            _ => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Data field is invalid type");
                return None;
            }
        };

        trace!("TransactionHandler::parse_reply_for_proof_checking: data: {:?}, parsed_data: {:?}", data, parsed_data);

        let key_suffix: String = match xtype {
            constants::GET_ATTR => {
                if let Some(attr_name) = json_msg["raw"].as_str()
                    .or(json_msg["enc"].as_str())
                    .or(json_msg["hash"].as_str()) {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: GET_ATTR attr_name {:?}", attr_name);

                    let mut hasher = sha2::Sha256::default();
                    hasher.process(attr_name.as_bytes());
                    format!(":\x01:{}", hasher.fixed_result().to_hex())
                } else {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_ATTR No key suffix");
                    return None;
                }
            }
            constants::GET_CRED_DEF => {
                if let (Some(sign_type), Some(sch_seq_no)) = (json_msg["signature_type"].as_str(),
                                                              json_msg["ref"].as_u64()) {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: GET_CRED_DEF sign_type {:?}, sch_seq_no: {:?}", sign_type, sch_seq_no);
                    format!(":\x03:{}:{}", sign_type, sch_seq_no)
                } else {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_CRED_DEF No key suffix");
                    return None;
                }
            }
            constants::GET_NYM => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_NYM");
                "".to_string()
            }
            constants::GET_SCHEMA => {
                if let (Some(name), Some(ver)) = (parsed_data["name"].as_str(),
                                                  parsed_data["version"].as_str()) {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: GET_SCHEMA name {:?}, ver: {:?}", name, ver);
                    format!(":\x02:{}:{}", name, ver)
                } else {
                    trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_SCHEMA No key suffix");
                    return None;
                }
            }
            _ => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Unknown transaction");
                return None;
            }
        };

        let key = if let Some(dest) = json_msg["dest"].as_str().or(json_msg["origin"].as_str()) {
            let mut dest = if xtype == constants::GET_NYM {
                let mut hasher = sha2::Sha256::default();
                hasher.process(dest.as_bytes());
                hasher.fixed_result().to_vec()
            } else {
                dest.as_bytes().to_vec()
            };

            dest.extend_from_slice(key_suffix.as_bytes());

            trace!("TransactionHandler::parse_reply_for_proof_checking: dest: {:?}", dest);
            dest
        } else {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No dest");
            return None;
        };

        let value: Option<String> = match TransactionHandler::parse_reply_for_proof_value(json_msg, data, parsed_data, xtype) {
            Ok(value) => value,
            Err(err_str) => {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< {}", err_str);
                return None;
            }
        };

        trace!("parse_reply_for_proof_checking: <<< proof {:?}, root_hash: {:?}, dest: {:?}, value: {:?}", proof, root_hash, key, value);
        Some((proof, root_hash, key, value))
    }

    fn parse_reply_for_proof_value(json_msg: &SJsonValue, data: Option<String>, parsed_data: SJsonValue, xtype: &str) -> Result<Option<String>, String> {
        if let Some(data) = data {
            let mut value = json!({});

            let (seq_no, time) = (json_msg["seqNo"].clone(), json_msg["txnTime"].clone());
            if xtype.eq(constants::GET_NYM) {
                value["seqNo"] = seq_no;
                value["txnTime"] = time;
            } else {
                value["lsn"] = seq_no;
                value["lut"] = time;
            }

            match xtype {
                //TODO constants::GET_TXN => check ledger MerkleTree proofs?
                //TODO constants::GET_DDO => support DDO
                constants::GET_NYM => {
                    value["identifier"] = parsed_data["identifier"].clone();
                    value["role"] = parsed_data["role"].clone();
                    value["verkey"] = parsed_data["verkey"].clone();
                }
                constants::GET_ATTR => {
                    let mut hasher = sha2::Sha256::default();
                    hasher.process(data.as_bytes());
                    value["val"] = SJsonValue::String(hasher.fixed_result().to_hex());
                }
                constants::GET_CRED_DEF => {
                    value["val"] = parsed_data;
                }
                constants::GET_SCHEMA => {
                    if let Some(map) = parsed_data.as_object() {
                        let mut map = map.clone();
                        map.remove("name");
                        map.remove("version");
                        if map.is_empty() {
                            return Ok(None); // TODO FIXME remove after INDY-699 will be fixed
                        } else {
                            value["val"] = SJsonValue::from(map)
                        }
                    } else {
                        return Err("Invalid data for GET_SCHEMA".to_string());
                    };
                }
                _ => {
                    return Err("Unknown transaction".to_string());
                }
            }

            Ok(Some(value.to_string()))
        } else {
            Ok(None)
        }
    }

    fn parse_reply_for_proof_signature_checking(json_msg: &SJsonValue) -> Option<(&str, Vec<&str>, Vec<u8>)> {
        match (json_msg["state_proof"]["multi_signature"]["signature"].as_str(),
               json_msg["state_proof"]["multi_signature"]["participants"].as_array(),
               rmp_serde::to_vec_named(&json_msg["state_proof"]["multi_signature"]["value"])
                   .map_err(map_err_trace!())) {
            (Some(signature), Some(participants), Ok(value)) => {
                let participants_unwrap: Vec<&str> = participants
                    .iter()
                    .flat_map(SJsonValue::as_str)
                    .collect();

                if participants.len() == participants_unwrap.len() {
                    Some((signature, participants_unwrap, value))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    pub fn get_upcoming_timeout(&self) -> Option<time::Tm> {
        self.pending_commands.iter().fold(None, |acc, (_, ref cur)| {
            let resend_tm: Option<Tm> = cur.resendable_request.as_ref()
                .and_then(|resend: &ResendableRequest| resend.next_try_send_time);
            let full_tm = cur.full_cmd_timeout;
            let tms = [resend_tm, full_tm, acc];
            tms.iter().fold(None, |acc, cur| {
                match (acc, *cur) {
                    (None, cur) => cur,
                    (Some(acc), None) => Some(acc),
                    (Some(acc), Some(cur)) => Some(acc.min(cur)),
                }
            })
        })
    }

    pub fn process_timeout(&mut self) -> Result<(), PoolError> {
        let timeout_cmds: Vec<u64> = self.pending_commands.iter_mut()
            .filter(|&(_, ref cur)| match cur.full_cmd_timeout {
                Some(tm) => tm <= time::now_utc(),
                None => false
            })
            .map(|(k, cmd)| {
                cmd.terminate_parent_cmds(true).map_err(map_err_trace!()).ok();
                *k
            }).collect();
        for cmd in timeout_cmds {
            self.pending_commands.remove(&cmd);
        }

        for (_, pc) in &mut self.pending_commands {
            let is_timeout = pc.resendable_request.as_ref()
                .and_then(|resend| resend.next_try_send_time)
                .map(|next_try_send_time| next_try_send_time <= time::now_utc())
                .unwrap_or(false);
            if is_timeout {
                pc.try_send_to_next_node_if_exists(&self.nodes);
            }
        }

        Ok(())
    }
}

impl Default for TransactionHandler {
    fn default() -> Self {
        TransactionHandler {
            gen: Generator::from_bytes(&"3LHpUjiyFC2q2hD7MnwwNmVXiuaFbQx2XkAFJWzswCjgN1utjsCeLzHsKk1nJvFEaS4fcrUmVAkdhtPCYbrVyATZcmzwJReTcJqwqBCPTmTQ9uWPwz6rEncKb2pYYYFcdHa8N17HzVyTqKfgPi4X9pMetfT3A5xCHq54R2pDNYWVLDX".from_base58().unwrap()).unwrap(),
            pending_commands: HashMap::new(),
            f: 0,
            nodes: Vec::new(),
        }
    }
}

impl CommandProcess {
    //TODO return err or bool for more complex handling
    fn try_send_to_next_node_if_exists(&mut self, nodes: &Vec<RemoteNode>) {
        if let Some(ref mut resend) = self.resendable_request {
            resend.next_try_send_time = Some(time::now_utc().add(Duration::seconds(RESENDABLE_REQUEST_TIMEOUT)));
            trace!("try_send_to_next_node_if_exists schedule next sending to {:?}", resend.next_try_send_time);
            while resend.next_node != resend.start_node {
                let cur_node = resend.next_node;
                resend.next_node = (cur_node + 1) % nodes.len();
                match nodes[cur_node].send_str(&resend.request) {
                    Ok(()) => return,
                    Err(err) => warn!("Can't send request to the next node, skip it ({})", err),
                }
            }
            resend.next_try_send_time = None;
        }
    }

    fn terminate_parent_cmds(&mut self, is_timeout: bool) -> Result<(), CommonError> {
        for cmd_id in &self.parent_cmd_ids {
            CommandExecutor::instance()
                .send(Command::Ledger(LedgerCommand::SubmitAck(
                    *cmd_id,
                    Err(if is_timeout { PoolError::Timeout } else { PoolError::Terminate }))))
                .map_err(|err| CommonError::InvalidState(format!("Can't send ACK cmd: {:?}", err)))?;
        }
        self.parent_cmd_ids.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    #[test]
    fn transaction_handler_process_reply_works() {
        use utils::logger::LoggerUtils;
        LoggerUtils::init();

        let mut th: TransactionHandler = Default::default();
        th.f = 1;
        let mut pc = CommandProcess {
            parent_cmd_ids: Vec::new(),
            replies: HashMap::new(),
            nack_cnt: 0,
            resendable_request: None,
            full_cmd_timeout: None,
        };
        let json = json!({"value":1});
        pc.replies.insert(HashableValue { inner: json.clone() }, 1);
        let req_id = 1;
        th.pending_commands.insert(req_id, pc);
        let json_result: SJsonValue = json!({"result":json});

        th.process_reply(req_id, &serde_json::to_string(&json_result).unwrap());

        assert_eq!(th.pending_commands.len(), 0);
    }

    #[test]
    fn transaction_handler_process_reply_works_for_different_replies_with_same_req_id() {
        let mut th: TransactionHandler = Default::default();
        th.f = 1;
        let mut pc = CommandProcess {
            parent_cmd_ids: Vec::new(),
            replies: HashMap::new(),
            nack_cnt: 0,
            resendable_request: None,
            full_cmd_timeout: None,
        };
        let json1 = json!({"value":1});
        let json2 = json!({"value":2});
        pc.replies.insert(HashableValue { inner: json1 }, 1);
        let req_id = 1;
        th.pending_commands.insert(req_id, pc);
        let json2_result: SJsonValue = json!({"result":json2});

        th.process_reply(req_id, &serde_json::to_string(&json2_result).unwrap());

        assert_eq!(th.pending_commands.len(), 1);
        assert_eq!(th.pending_commands.get(&req_id).unwrap().replies.len(), 2);
    }

    #[test]
    fn transaction_handler_try_send_request_works_for_new_req_id() {
        let mut th: TransactionHandler = Default::default();

        let req_id = 2;
        let cmd_id = 1;
        let cmd = format!("{{\"reqId\": {}}}", req_id);

        th.try_send_request(&cmd, cmd_id).unwrap();
        let expected_timeout = time::now_utc().add(Duration::seconds(REQUEST_TIMEOUT_ACK));

        assert_eq!(th.pending_commands.len(), 1);
        let pending_cmd = th.pending_commands.get(&req_id).unwrap();
        let exp_command_process = CommandProcess {
            nack_cnt: 0,
            replies: HashMap::new(),
            parent_cmd_ids: vec!(cmd_id),
            resendable_request: None,
            full_cmd_timeout: pending_cmd.full_cmd_timeout /* just copy for eq check other fields*/,
        };
        assert_eq!(pending_cmd, &exp_command_process);
        let diff: Duration = expected_timeout.sub(pending_cmd.full_cmd_timeout.unwrap());
        assert!(diff <= Duration::milliseconds(10));
        assert!(diff >= Duration::zero());
    }
}
