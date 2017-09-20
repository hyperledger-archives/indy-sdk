mod types;
mod catchup;
#[warn(dead_code)]
#[warn(unused_variables)]
mod state_proof;

extern crate byteorder;
extern crate digest;
extern crate hex;
extern crate rust_base58;
extern crate sha2;
extern crate zmq_pw as zmq;
extern crate rmp_serde;

use self::byteorder::{ByteOrder, LittleEndian};
use self::digest::{FixedOutput, Input};
use self::hex::ToHex;
use self::rust_base58::FromBase58;
use serde_json;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::{fmt, fs, io, thread};
use std::fmt::Debug;
use std::io::{BufRead, Write};
use std::error::Error;

use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use errors::common::CommonError;
use self::catchup::CatchupHandler;
use self::types::*;
use services::ledger::merkletree::merkletree::MerkleTree;
use utils::crypto::ed25519::ED25519;
use utils::environment::EnvironmentUtils;
use utils::json::{JsonDecodable, JsonEncodable};
use utils::sequence::SequenceUtils;

pub struct PoolService {
    pools: RefCell<HashMap<i32, Pool>>,
}

struct Pool {
    name: String,
    id: i32,
    cmd_sock: zmq::Socket,
    worker: Option<thread::JoinHandle<()>>,
}

struct PoolWorker {
    cmd_sock: zmq::Socket,
    open_cmd_id: i32,
    pool_id: i32,
    name: String,
    handler: PoolWorkerHandler,
}

enum PoolWorkerHandler {
    //TODO trait ?
    CatchupHandler(CatchupHandler),
    TransactionHandler(TransactionHandler),
}

struct TransactionHandler {
    f: usize,
    nodes: Vec<RemoteNode>,
    pending_commands: HashMap<u64 /* requestId */, CommandProcess>,
}

impl PoolWorkerHandler {
    fn process_msg(&mut self, raw_msg: &String, src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        let msg = Message::from_raw_str(raw_msg)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::IOError(
                    io::Error::from(io::ErrorKind::InvalidData)))?;
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
        }
    }

    fn send_request(&mut self, cmd: &str, cmd_id: i32) -> Result<(), PoolError> {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => {
                Err(PoolError::CommonError(
                    CommonError::InvalidState("Try send request while CatchUp.".to_string())))
            }
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => {
                ch.try_send_request(cmd, cmd_id)
            }
        }
    }

    fn flush_requests(&mut self, status: Result<(), PoolError>) -> Result<(), PoolError> {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.flush_requests(status),
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.flush_requests(status),
        }
    }

    fn nodes(&self) -> &Vec<RemoteNode> {
        match self {
            &PoolWorkerHandler::CatchupHandler(ref ch) => &ch.nodes,
            &PoolWorkerHandler::TransactionHandler(ref ch) => &ch.nodes,
        }
    }

    fn nodes_mut(&mut self) -> &mut Vec<RemoteNode> {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => &mut ch.nodes,
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => &mut ch.nodes,
        }
    }

    fn set_f(&mut self, f: usize) {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.f = f,
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.f = f,
        };
    }
}

impl TransactionHandler {
    fn process_msg(&mut self, msg: Message, raw_msg: &String, src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        match msg {
            Message::Reply(reply) => {
                self.process_reply(reply.result.req_id, raw_msg);
            }
            Message::PoolLedgerTxns(response) => {
                self.process_reply(response.txn.req_id, raw_msg);
            }
            Message::Reject(response) | Message::ReqNACK(response) => {
                self.process_reject(&response, raw_msg);
            }
            _ => {
                warn!("unhandled msg {:?}", msg);
            }
        };
        Ok(None)
    }

    fn process_reply(&mut self, req_id: u64, raw_msg: &String) {
        let mut remove = false;
        if let Some(pend_cmd) = self.pending_commands.get_mut(&req_id) {
            let pend_cmd: &mut CommandProcess = pend_cmd;
            let mut json_msg: HashableValue = HashableValue { inner: serde_json::from_str(raw_msg).unwrap() };
            if let Some(str) = json_msg.inner["result"]["data"].clone().as_str() {
                let tmp_obj: serde_json::Value = serde_json::from_str(str).unwrap();
                json_msg.inner["result"]["data"] = tmp_obj;
            }
            let reply_cnt: usize = *pend_cmd.replies.get(&json_msg).unwrap_or(&0usize);
            let consensus_reached = {
                let data_to_check_proof = TransactionHandler::parse_reply_for_proof_checking(&json_msg.inner["result"]);
                if let Some((proofs, root_hash, key, value)) = data_to_check_proof {
                    debug!("TransactionHandler::process_reply try to verify proofs in reply");
                    self::state_proof::verify_proof(proofs.from_base58().unwrap().as_slice(),
                                                    root_hash.from_base58().unwrap().as_slice(),
                                                    key.as_slice(),
                                                    value.as_ref().map(String::as_str))
                } else {
                    debug!("TransactionHandler::process_reply not enough data to verify proofs in reply, collect replies");
                    reply_cnt == self.f //already have f same replies and receive f+1 now
                }
            };
            debug!("TransactionHandler::process_reply consensus_reached {}", consensus_reached);
            if consensus_reached {
                for &cmd_id in &pend_cmd.cmd_ids {
                    CommandExecutor::instance().send(
                        Command::Ledger(LedgerCommand::SubmitAck(cmd_id, Ok(raw_msg.clone())))).unwrap();
                }
                remove = true;
            } else {
                pend_cmd.replies.insert(json_msg, reply_cnt + 1);
            }
        }
        if remove {
            self.pending_commands.remove(&req_id);
        }
    }

    //TODO correct handling of Reject
    fn process_reject(&mut self, response: &Response, raw_msg: &String) {
        let req_id = response.req_id;
        let mut remove = false;
        if let Some(pend_cmd) = self.pending_commands.get_mut(&req_id) {
            pend_cmd.nack_cnt += 1;
            if pend_cmd.nack_cnt == self.f + 1 {
                for &cmd_id in &pend_cmd.cmd_ids {
                    CommandExecutor::instance().send(
                        Command::Ledger(
                            LedgerCommand::SubmitAck(cmd_id,
                                                     Err(PoolError::Rejected(raw_msg.clone()))))
                    ).unwrap();
                }
                remove = true;
            }
        }
        if remove {
            self.pending_commands.remove(&req_id);
        }
    }

    fn try_send_request(&mut self, cmd: &str, cmd_id: i32) -> Result<(), PoolError> {
        info!("cmd {:?}", cmd);
        let request: Value = serde_json::from_str(cmd)
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Invalid request json: {}", err.description())))?;

        let request_id: u64 = request["reqId"]
            .as_u64()
            .ok_or(CommonError::InvalidStructure("No reqId in request".to_string()))?;

        if self.pending_commands.contains_key(&request_id) {
            self.pending_commands.get_mut(&request_id).unwrap().cmd_ids.push(cmd_id);
        } else {
            let pc = CommandProcess {
                cmd_ids: vec!(cmd_id),
                nack_cnt: 0,
                replies: HashMap::new(),
            };
            self.pending_commands.insert(request_id, pc);
            for node in &self.nodes {
                let node: &RemoteNode = node;
                node.send_str(cmd)?;
            }
        }
        Ok(())
    }

    fn flush_requests(&mut self, status: Result<(), PoolError>) -> Result<(), PoolError> {
        match status {
            Ok(()) => {
                return Err(PoolError::CommonError(
                    CommonError::InvalidState(
                        "Can't flash all transaction requests with common success status".to_string())));
            }
            Err(err) => {
                for (_, pending_cmd) in &self.pending_commands {
                    let pending_cmd: &CommandProcess = pending_cmd;
                    for cmd_id in &pending_cmd.cmd_ids {
                        CommandExecutor::instance()
                            .send(Command::Ledger(LedgerCommand::SubmitAck(
                                cmd_id.clone(), Err(PoolError::Terminate))))
                            .map_err(|err|
                                CommonError::InvalidState("Can't send ACK cmd".to_string()))?;
                    }
                }
                Ok(())
            }
        }
    }

    fn parse_reply_for_proof_checking(json_msg: &serde_json::Value)
                                      -> Option<(&str, &str, Vec<u8>, Option<String>)> {
        let raw = match json_msg["type"].as_str() {
            Some(super::ledger::constants::GET_ATTR) |
            Some(super::ledger::constants::GET_CLAIM_DEF) |
            Some(super::ledger::constants::GET_NYM) |
            Some(super::ledger::constants::GET_SCHEMA) => {
                (json_msg["state_proof"]["proof_nodes"].as_str(),
                 json_msg["state_proof"]["root_hash"].as_str(),
                 json_msg["dest"].as_str().or(json_msg["origin"].as_str())
                     .map(|v: &str| v.as_bytes().to_vec()))
            }
            //TODO Some(super::ledger::constants::GET_TXN) => check ledger MerkleTree proofs?
            //TODO Some(super::ledger::constants::GET_DDO) => support DDO
            _ => return None
        };
        if let (Some(proof), Some(root_hash), Some(mut dest)) = raw {
            let mut value = json_msg.get("data").map(Clone::clone);
            let key_suffix: String = match json_msg["type"].as_str() {
                Some(super::ledger::constants::GET_ATTR) => {
                    value = json_msg.get("data").map(|data| {
                        let mut hasher = sha2::Sha256::default();
                        hasher.process(serde_json::to_string(data).unwrap().as_bytes());
                        serde_json::Value::String(hasher.fixed_result().to_hex())
                    });
                    if let Some(attr_name) = json_msg["raw"].as_str()
                        .or(json_msg["enc"].as_str())
                        .or(json_msg["hash"].as_str()) {
                        let mut hasher = sha2::Sha256::default();
                        hasher.process(attr_name.as_bytes());
                        format!(":\x01:{}", hasher.fixed_result().to_hex())
                    } else {
                        return None;
                    }
                }
                Some(super::ledger::constants::GET_CLAIM_DEF) => {
                    if let (Some(sign_type), Some(sch_seq_no)) = (json_msg["signature_type"].as_str(),
                                                                  json_msg["ref"].as_u64()) {
                        format!(":\x03:{}:{}", sign_type, sch_seq_no)
                    } else {
                        return None;
                    }
                }
                Some(super::ledger::constants::GET_NYM) => {
                    let mut hasher = sha2::Sha256::default();
                    hasher.process(dest.as_slice());
                    dest = hasher.fixed_result().to_vec();
                    "".to_string()
                }
                Some(super::ledger::constants::GET_SCHEMA) => {
                    if let (Some(name), Some(ver)) = (json_msg["data"]["name"].as_str(),
                                                      json_msg["data"]["version"].as_str()) {
                        format!(":\x02:{}:{}", name, ver)
                    } else {
                        return None;
                    }
                }
                _ => return None
            };
            let mut out_value: Option<serde_json::Value> = None;
            if json_msg["type"].as_str().eq(&Some(::services::ledger::constants::GET_NYM)) {
                out_value = value.map(|mut value| {
                    value["seqNo"] = json_msg["seqNo"].clone();
                    if value["role"].as_str() == Some("") {
                        value["role"] = serde_json::Value::Null; //FIXME should be fixed on node side
                    }
                    let mut value = value.as_object_mut().unwrap().clone();
                    value.remove("dest");
                    serde_json::Value::from(value)
                });
            } else {
                if let (Some(data), Some(seq_no)) = (value, json_msg["seqNo"].as_u64()) {
                    out_value = Some(json!({ "lsn": seq_no, "val": data }));
                }
            }
            dest.extend_from_slice(key_suffix.as_bytes());
            Some((proof, root_hash, dest, out_value.map(|v| v.to_string())))
        } else {
            None
        }
    }
}

impl Default for TransactionHandler {
    fn default() -> Self {
        TransactionHandler {
            pending_commands: HashMap::new(),
            f: 0,
            nodes: Vec::new(),
        }
    }
}

impl PoolWorker {
    fn connect_to_known_nodes(&mut self, merkle_tree: Option<&MerkleTree>) -> Result<(), PoolError> {
        let merkle_tree = match merkle_tree {
            Some(merkle_tree) => Some(merkle_tree.clone()),
            None => match self.handler {
                PoolWorkerHandler::CatchupHandler(ref ch) => Some(ch.merkle_tree.clone()),
                PoolWorkerHandler::TransactionHandler(_) => None
            }
        }
            .ok_or(CommonError::InvalidState("Expect catchup state".to_string()))?;

        let ctx: zmq::Context = zmq::Context::new();
        let key_pair = zmq::CurveKeyPair::new()?;
        for gen_txn in &merkle_tree {
            let gen_txn: GenTransaction = rmp_serde::decode::from_slice(gen_txn.as_slice())
                .map_err(|e|
                    CommonError::InvalidState(format!("MerkleTree contains invalid data {}", e)))?;

            let mut rn: RemoteNode = RemoteNode::new(&gen_txn)?;
            rn.connect(&ctx, &key_pair)?;
            rn.send_str("pi")?;
            self.handler.nodes_mut().push(rn);
        }
        self.handler.set_f(PoolWorker::get_f(merkle_tree.count())); //TODO set cnt to connect
        if let PoolWorkerHandler::CatchupHandler(ref mut handler) = self.handler {
            handler.reset_nodes_votes();
        }
        Ok(())
    }

    fn init_catchup(&mut self, refresh_cmd_id: Option<i32>) -> Result<(), PoolError> {
        let catchup_handler = CatchupHandler {
            merkle_tree: PoolWorker::_restore_merkle_tree(self.name.as_str())?,
            initiate_cmd_id: refresh_cmd_id.unwrap_or(self.open_cmd_id),
            is_refresh: refresh_cmd_id.is_some(),
            pool_id: self.pool_id,
            ..Default::default()
        };
        self.handler = PoolWorkerHandler::CatchupHandler(catchup_handler);
        self.connect_to_known_nodes(None)?;
        Ok(())
    }

    fn refresh(&mut self, cmd_id: i32) -> Result<(), PoolError> {
        match self.handler.flush_requests(Err(PoolError::Terminate)) {
            Ok(()) => self.init_catchup(Some(cmd_id)),
            Err(err) => CommandExecutor::instance().send(Command::Pool(PoolCommand::RefreshAck(cmd_id, Err(err)))).map_err(PoolError::from),
        }
    }

    pub fn run(&mut self) -> Result<(), PoolError> {
        self._run().or_else(|err: PoolError| {
            self.handler.flush_requests(Err(PoolError::Terminate))?;
            match err {
                PoolError::Terminate => Ok(()),
                _ => Err(err),
            }
        })
    }

    fn _run(&mut self) -> Result<(), PoolError> {
        self.init_catchup(None)?; //TODO consider error as PoolOpen error

        loop {
            trace!("zmq poll loop >>");

            let actions = self.poll_zmq()?;

            self.process_actions(actions).map_err(map_err_trace!("process_actions"))?;

            trace!("zmq poll loop <<");
        }
    }

    fn process_actions(&mut self, actions: Vec<ZMQLoopAction>) -> Result<(), PoolError> {
        for action in &actions {
            match action {
                &ZMQLoopAction::Terminate(cmd_id) => {
                    let res = self.handler.flush_requests(Err(PoolError::Terminate));
                    if cmd_id >= 0 {
                        CommandExecutor::instance().send(Command::Pool(PoolCommand::CloseAck(cmd_id, res)))?;
                    }
                    return Err(PoolError::Terminate);
                }
                &ZMQLoopAction::Refresh(cmd_id) => {
                    self.refresh(cmd_id)?;
                }
                &ZMQLoopAction::MessageToProcess(ref msg) => {
                    if let Some(new_mt) = self.handler.process_msg(&msg.message, msg.node_idx)? {
                        self.handler.flush_requests(Ok(()))?;
                        self.handler = PoolWorkerHandler::TransactionHandler(Default::default());
                        self.connect_to_known_nodes(Some(&new_mt))?;
                    }
                }
                &ZMQLoopAction::RequestToSend(ref req) => {
                    self.handler.send_request(req.request.as_str(), req.id).or_else(|err| {
                        CommandExecutor::instance()
                            .send(Command::Ledger(LedgerCommand::SubmitAck(req.id, Err(err))))
                            .map_err(|err| {
                                CommonError::InvalidState("Can't send ACK cmd".to_string())
                            })
                    })?;
                }
            }
        }
        Ok(())
    }

    fn poll_zmq(&mut self) -> Result<Vec<ZMQLoopAction>, PoolError> {
        let mut actions: Vec<ZMQLoopAction> = Vec::new();

        let mut poll_items = self.get_zmq_poll_items()?;
        let r = zmq::poll(poll_items.as_mut_slice(), -1)?;
        trace!("zmq poll {:?}", r);

        for i in 0..self.handler.nodes().len() {
            if poll_items[1 + i].is_readable() {
                if let Some(msg) = self.handler.nodes()[i].recv_msg()? {
                    actions.push(ZMQLoopAction::MessageToProcess(MessageToProcess {
                        node_idx: i,
                        message: msg,
                    }));
                }
            }
        }
        if poll_items[0].is_readable() {
            let cmd = self.cmd_sock.recv_multipart(zmq::DONTWAIT)?;
            trace!("cmd {:?}", cmd);
            let cmd_s = String::from_utf8(cmd[0].clone())
                .map_err(|err|
                    CommonError::InvalidState("Invalid command received".to_string()))?;
            let id = cmd.get(1).map(|cmd: &Vec<u8>| LittleEndian::read_i32(cmd.as_slice()))
                .unwrap_or(-1);
            if "exit".eq(cmd_s.as_str()) {
                actions.push(ZMQLoopAction::Terminate(id));
            } else if "refresh".eq(cmd_s.as_str()) {
                actions.push(ZMQLoopAction::Refresh(id));
            } else {
                actions.push(ZMQLoopAction::RequestToSend(RequestToSend {
                    id: id,
                    request: cmd_s,
                }));
            }
        }
        Ok(actions)
    }

    fn get_zmq_poll_items(&self) -> Result<Vec<zmq::PollItem>, PoolError> {
        let mut poll_items: Vec<zmq::PollItem> = Vec::new();
        poll_items.push(self.cmd_sock.as_poll_item(zmq::POLLIN));
        for ref node in self.handler.nodes() {
            let s: &zmq::Socket = node.zsock.as_ref()
                .ok_or(CommonError::InvalidState(
                    "Try to poll from ZMQ socket for unconnected RemoteNode".to_string()))?;
            poll_items.push(s.as_poll_item(zmq::POLLIN));
        }
        Ok(poll_items)
    }


    fn _restore_merkle_tree(pool_name: &str) -> Result<MerkleTree, PoolError> {
        let mut p = EnvironmentUtils::pool_path(pool_name);
        let mut mt = MerkleTree::from_vec(Vec::new()).map_err(map_err_trace!())?;
        //TODO firstly try to deserialize merkle tree
        p.push(pool_name);
        p.set_extension("txn");
        let f = fs::File::open(p).map_err(map_err_trace!())?;
        let reader = io::BufReader::new(&f);
        for line in reader.lines() {
            let line: String = line.map_err(map_err_trace!())?;
            let genesis_txn: serde_json::Value = serde_json::from_str(line.as_str()).unwrap(); /* FIXME resolve unwrap */
            let bytes = rmp_serde::encode::to_vec_named(&genesis_txn).unwrap(); /* FIXME resolve unwrap */
            mt.append(bytes).map_err(map_err_trace!())?;
        }
        Ok(mt)
    }

    #[allow(unreachable_code)]
    fn get_f(cnt: usize) -> usize {
        return cnt / 2; /* FIXME ugly hack to work with pool instability, remove after pool will be fixed */
        if cnt < 4 {
            return 0;
        }
        (cnt - 1) / 3
    }
}

impl Pool {
    pub fn new(name: &str, cmd_id: i32) -> Result<Pool, PoolError> {
        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let inproc_sock_name: String = format!("inproc://pool_{}", name);

        recv_cmd_sock.bind(inproc_sock_name.as_str())?;

        send_cmd_sock.connect(inproc_sock_name.as_str())?;
        let pool_id = SequenceUtils::get_next_id();
        let mut pool_worker: PoolWorker = PoolWorker {
            cmd_sock: recv_cmd_sock,
            open_cmd_id: cmd_id,
            pool_id: pool_id,
            name: name.to_string(),
            handler: PoolWorkerHandler::CatchupHandler(CatchupHandler {
                initiate_cmd_id: cmd_id,
                pool_id: pool_id,
                ..Default::default()
            }),
        };

        Ok(Pool {
            name: name.to_string(),
            id: pool_id,
            cmd_sock: send_cmd_sock,
            worker: Some(thread::spawn(move || {
                pool_worker.run().unwrap_or_else(|err| {
                    error!("Pool worker thread finished with error {:?}", err);
                })
            })),
        })
    }

    pub fn send_tx(&self, cmd_id: i32, json: &str) -> Result<(), PoolError> {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        Ok(self.cmd_sock.send_multipart(&[json.as_bytes(), &buf], zmq::DONTWAIT)?)
    }

    pub fn close(&self, cmd_id: i32) -> Result<(), PoolError> {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        Ok(self.cmd_sock.send_multipart(&["exit".as_bytes(), &buf], zmq::DONTWAIT)?)
    }

    pub fn refresh(&self, cmd_id: i32) -> Result<(), PoolError> {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        Ok(self.cmd_sock.send_multipart(&["refresh".as_bytes(), &buf], zmq::DONTWAIT)?)
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        let target = format!("pool{}", self.name);
        info!(target: target.as_str(), "Drop started");

        if let Err(err) = self.cmd_sock.send("exit".as_bytes(), zmq::DONTWAIT) {
            warn!("Can't send exit command to pool worker thread (may be already finished) {}", err);
        }

        // Option worker type and this kludge is workaround for rust
        if let Some(worker) = self.worker.take() {
            info!(target: target.as_str(), "Drop wait worker");
            worker.join().unwrap();
        }
        info!(target: target.as_str(), "Drop finished");
    }
}

impl Debug for RemoteNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RemoteNode: {{ public_key {:?}, zaddr {:?}, zsock is_some {} }}",
               self.public_key, self.zaddr, self.zsock.is_some())
    }
}

impl RemoteNode {
    fn new(txn: &GenTransaction) -> Result<RemoteNode, PoolError> {
        let node_verkey = txn.dest.as_str().from_base58()
            .map_err(|e| { CommonError::InvalidStructure("Invalid field dest in genesis transaction".to_string()) })?;
        Ok(RemoteNode {
            public_key: ED25519::vk_to_curve25519(&node_verkey)?,
            zaddr: format!("tcp://{}:{}", txn.data.client_ip, txn.data.client_port),
            zsock: None,
            name: txn.data.alias.clone(),
            is_blacklisted: false,
        })
    }

    fn connect(&mut self, ctx: &zmq::Context, key_pair: &zmq::CurveKeyPair) -> Result<(), PoolError> {
        let s = ctx.socket(zmq::SocketType::DEALER)?;
        s.set_identity(zmq::z85_encode(&key_pair.public_key).unwrap().as_bytes())?;
        s.set_curve_secretkey(&key_pair.secret_key)?;
        s.set_curve_publickey(&key_pair.public_key)?;
        s.set_curve_serverkey(self.public_key.as_slice())?;
        s.set_linger(0)?; //TODO set correct timeout
        s.connect(self.zaddr.as_str())?;
        self.zsock = Some(s);
        Ok(())
    }

    fn recv_msg(&self) -> Result<Option<String>, PoolError> {
        impl From<Vec<u8>> for PoolError {
            fn from(_: Vec<u8>) -> Self {
                PoolError::CommonError(
                    CommonError::IOError(
                        io::Error::from(io::ErrorKind::InvalidData)))
            }
        }
        let msg: String = self.zsock.as_ref()
            .ok_or(CommonError::InvalidState("Try to receive msg for unconnected RemoteNode".to_string()))?
            .recv_string(zmq::DONTWAIT)??;
        info!("RemoteNode::recv_msg {} {}", self.name, msg);

        Ok(Some(msg))
    }

    fn send_str(&self, str: &str) -> Result<(), PoolError> {
        info!("Sending {:?}", str);
        self.zsock.as_ref()
            .ok_or(CommonError::InvalidState("Try to send str for unconnected RemoteNode".to_string()))?
            .send(str, zmq::DONTWAIT)?;
        Ok(())
    }

    fn send_msg(&self, msg: &Message) -> Result<(), PoolError> {
        self.send_str(
            msg.to_json()
                .map_err(|err|
                    CommonError::InvalidState(format!("Can't serialize message: {}", err.description())))?
                .as_str())
    }
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            pools: RefCell::new(HashMap::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<&str>) -> Result<(), PoolError> {
        trace!("PoolService::create {} with config {:?}", name, config);
        let mut path = EnvironmentUtils::pool_path(name);
        let pool_config = match config {
            Some(config) => PoolConfig::from_json(config)
                .map_err(|err|
                    CommonError::InvalidStructure(format!("Invalid pool config format: {}", err.description())))?,
            None => PoolConfig::default_for_name(name)
        };

        if path.as_path().exists() {
            return Err(PoolError::AlreadyExists(format!("Pool ledger config file with name \"{}\" already exists", name)));
        }

        fs::create_dir_all(path.as_path()).map_err(map_err_trace!())?;

        path.push(name);
        path.set_extension("txn");
        fs::copy(&pool_config.genesis_txn, path.as_path()).map_err(map_err_trace!())?;
        path.pop();

        path.push("config");
        path.set_extension("json");
        let mut f: fs::File = fs::File::create(path.as_path()).map_err(map_err_trace!())?;

        f.write(pool_config
            .to_json()
            .map_err(|err|
                CommonError::InvalidState(format!("Can't serialize pool config: {}", err.description()))).map_err(map_err_trace!())?
            .as_bytes()).map_err(map_err_trace!())?;
        f.flush().map_err(map_err_trace!())?;

        // TODO probably create another one file pool.json with pool description,
        // but now there is no info to save (except name witch equal to directory)

        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), PoolError> {
        for pool in self.pools.try_borrow().map_err(CommonError::from)?.values() {
            if pool.name.eq(name) {
                return Err(PoolError::CommonError(CommonError::InvalidState("Can't delete pool config - pool is open now".to_string())));
            }
        }
        let path = EnvironmentUtils::pool_path(name);
        fs::remove_dir_all(path).map_err(PoolError::from)
    }

    pub fn open(&self, name: &str, config: Option<&str>) -> Result<i32, PoolError> {
        for pool in self.pools.try_borrow().map_err(CommonError::from)?.values() {
            if name.eq(pool.name.as_str()) {
                //TODO change error
                return Err(PoolError::InvalidHandle("Pool with same name already opened".to_string()));
            }
        }

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let new_pool = Pool::new(name, cmd_id)?;
        //FIXME process config: check None (use default), transfer to Pool instance

        self.pools.try_borrow_mut().map_err(CommonError::from)?.insert(new_pool.id, new_pool);
        return Ok(cmd_id);
    }

    pub fn send_tx(&self, handle: i32, json: &str) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.pools.try_borrow().map_err(CommonError::from)?
            .get(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .send_tx(cmd_id, json)?;
        Ok(cmd_id)
    }

    pub fn close(&self, handle: i32) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.pools.try_borrow_mut().map_err(CommonError::from)?
            .remove(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .close(cmd_id)
            .map(|()| cmd_id)
    }

    pub fn refresh(&self, handle: i32) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.pools.try_borrow_mut().map_err(CommonError::from)?
            .get(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .refresh(cmd_id)
            .map(|()| cmd_id)
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, PoolError> {
        self.pools.try_borrow().map_err(CommonError::from)?.get(&handle).map_or(
            Err(PoolError::InvalidHandle(format!("Pool doesn't exists for handle {}", handle))),
            |pool: &Pool| Ok(pool.name.clone()))
    }
}

#[cfg(test)]
mod mocks {
    use super::*;

    use std::cell::RefCell;

    pub struct PoolService {
        create_results: RefCell<Vec<Result<(), PoolError>>>,
        delete_results: RefCell<Vec<Result<(), PoolError>>>,
        open_results: RefCell<Vec<Result<i32, PoolError>>>,
        close_results: RefCell<Vec<Result<(), PoolError>>>,
        refresh_results: RefCell<Vec<Result<(), PoolError>>>
    }

    impl PoolService {
        pub fn new() -> PoolService {
            PoolService {
                create_results: RefCell::new(Vec::new()),
                delete_results: RefCell::new(Vec::new()),
                open_results: RefCell::new(Vec::new()),
                close_results: RefCell::new(Vec::new()),
                refresh_results: RefCell::new(Vec::new())
            }
        }

        pub fn create(&self, name: &str, config: &str) -> Result<(), PoolError> {
            //self.create_results.pop().unwrap()
            unimplemented!()
        }

        pub fn delete(&self, name: &str) -> Result<(), PoolError> {
            //self.delete_results.pop().unwrap()
            unimplemented!()
        }

        pub fn open(&self, name: &str, config: &str) -> Result<i32, PoolError> {
            //self.open_results.pop().unwrap()
            unimplemented!()
        }

        pub fn close(&self, handle: i32) -> Result<(), PoolError> {
            //self.close_results.pop().unwrap()
            unimplemented!()
        }

        pub fn refresh(&self, handle: i32) -> Result<(), PoolError> {
            //self.refresh_results.pop().unwrap()
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pool_service {
        use super::*;
        use std::path;

        #[test]
        fn pool_service_new_works() {
            let pool_service = PoolService::new();
            assert!(true, "No crashes on PoolService::new");
        }

        #[test]
        fn pool_service_drop_works() {
            fn drop_test() {
                let pool_service = PoolService::new();
            }

            drop_test();
            assert!(true, "No crashes on PoolService::drop");
        }

        #[test]
        fn pool_service_close_works() {
            let ps = PoolService::new();
            let pool_id = SequenceUtils::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.pools.borrow_mut().insert(pool_id, Pool {
                name: String::new(),
                id: pool_id,
                worker: None,
                cmd_sock: send_soc,
            });
            let cmd_id = ps.close(pool_id).unwrap();
            let recv = recv_soc.recv_multipart(zmq::DONTWAIT).unwrap();
            assert_eq!(recv.len(), 2);
            assert_eq!("exit", String::from_utf8(recv[0].clone()).unwrap());
            assert_eq!(cmd_id, LittleEndian::read_i32(recv[1].as_slice()));
        }

        #[test]
        fn pool_service_refresh_works() {
            let ps = PoolService::new();
            let pool_id = SequenceUtils::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.pools.borrow_mut().insert(pool_id, Pool {
                name: String::new(),
                id: pool_id,
                worker: None,
                cmd_sock: send_soc,
            });
            let cmd_id = ps.refresh(pool_id).unwrap();
            let recv = recv_soc.recv_multipart(zmq::DONTWAIT).unwrap();
            assert_eq!(recv.len(), 2);
            assert_eq!("refresh", String::from_utf8(recv[0].clone()).unwrap());
            assert_eq!(cmd_id, LittleEndian::read_i32(recv[1].as_slice()));
        }

        #[test]
        fn pool_service_delete_works() {
            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works";
            let path: path::PathBuf = EnvironmentUtils::pool_path(pool_name);
            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            ps.delete(pool_name).unwrap();
            assert!(!path.exists());
        }

        #[test]
        fn pool_service_delete_works_for_opened() {
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works";
            let path: path::PathBuf = EnvironmentUtils::pool_path(pool_name);
            let pool_id = SequenceUtils::get_next_id();

            let inproc_sock_name: String = format!("inproc://pool_{}", pool_name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();

            let pool = Pool {
                worker: None,
                name: pool_name.to_string(),
                cmd_sock: recv_cmd_sock,
                id: pool_id
            };
            ps.pools.borrow_mut().insert(pool_id, pool);

            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            let res = ps.delete(pool_name);
            assert_match!(Err(PoolError::CommonError(CommonError::InvalidState(_))), res);
            assert!(path.exists());
        }
    }

    #[test]
    fn pool_drop_works_for_after_close() {
        use utils::logger::LoggerUtils;
        use utils::test::TestUtils;
        use std::time;

        TestUtils::cleanup_storage();
        LoggerUtils::init();

        fn drop_test() {
            let pool_name = "pool_drop_works";

            // create minimal fs config stub before Pool::new()
            let mut pool_path = EnvironmentUtils::pool_path(pool_name);
            fs::create_dir_all(&pool_path).unwrap();
            pool_path.push(pool_name);
            pool_path.set_extension("txn"); //empty genesis txns file - pool will not try to connect to somewhere
            fs::File::create(pool_path).unwrap();

            let pool = Pool::new(pool_name, -1).unwrap();
            thread::sleep(time::Duration::from_secs(1));
            pool.close(-1).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }

        drop_test();
        TestUtils::cleanup_storage();
    }

    #[test]
    fn pool_send_tx_works() {
        let name = "test";
        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let inproc_sock_name: String = format!("inproc://pool_{}", name);
        recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
        send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
        let pool = Pool {
            worker: None,
            name: name.to_string(),
            id: 0,
            cmd_sock: send_cmd_sock,
        };
        let test_data = "str_instead_of_tx_json";
        pool.send_tx(0, test_data).unwrap();
        assert_eq!(recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), test_data);
    }

    impl Default for PoolWorker {
        fn default() -> Self {
            PoolWorker {
                pool_id: 0,
                cmd_sock: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
                open_cmd_id: 0,
                name: "".to_string(),
                handler: PoolWorkerHandler::CatchupHandler(Default::default()),
            }
        }
    }

    #[test]
    fn pool_worker_restore_merkle_tree_works_from_genesis_txns() {
        let txns_src = format!("{}\n{}\n{}\n{}\n",
                               "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"192.168.1.35\",\"client_port\":9702,\"node_ip\":\"192.168.1.35\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"192.168.1.35\",\"client_port\":9704,\"node_ip\":\"192.168.1.35\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"192.168.1.35\",\"client_port\":9706,\"node_ip\":\"192.168.1.35\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"192.168.1.35\",\"client_port\":9708,\"node_ip\":\"192.168.1.35\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns_src.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let merkle_tree = PoolWorker::_restore_merkle_tree("test").unwrap();

        assert_eq!(merkle_tree.count(), 4, "test restored MT size");
        assert_eq!(merkle_tree.root_hash_hex(), "7c7e209a5bee34e467f7a2b6e233b8c61b74ddfd099bd9ad8a9a764cdf671981", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_connect_to_known_nodes_works() {
        let mut pw: PoolWorker = Default::default();
        let (gt, handle) = nodes_emulator::start();
        let mut merkle_tree: MerkleTree = MerkleTree::from_vec(Vec::new()).unwrap();
        merkle_tree.append(gt.to_msg_pack().unwrap()).unwrap();

        pw.connect_to_known_nodes(Some(&merkle_tree)).unwrap();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        assert_eq!("pi", emulator_msgs[0]);
    }

    #[test]
    fn pool_worker_poll_zmq_works_for_terminate() {
        let ctx = zmq::Context::new();
        let mut pw = PoolWorker {
            cmd_sock: ctx.socket(zmq::SocketType::PAIR).expect("socket"),
            ..Default::default()
        };
        let pair_socket_addr = "inproc://test_pool_worker_poll_zmq_works_for_terminate";
        let send_cmd_sock = ctx.socket(zmq::SocketType::PAIR).expect("socket");
        pw.cmd_sock.bind(pair_socket_addr).expect("bind");
        send_cmd_sock.connect(pair_socket_addr).expect("connect");

        let handle: thread::JoinHandle<Vec<ZMQLoopAction>> = thread::spawn(move || {
            pw.poll_zmq().unwrap()
        });
        send_cmd_sock.send("exit", zmq::DONTWAIT).expect("send");
        let actions: Vec<ZMQLoopAction> = handle.join().unwrap();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], ZMQLoopAction::Terminate(-1));
    }

    #[test]
    fn pool_worker_get_zmq_poll_items_works() {
        let pw: PoolWorker = Default::default();

        let poll_items = pw.get_zmq_poll_items().unwrap();

        assert_eq!(poll_items.len(), pw.handler.nodes().len() + 1);
        //TODO compare poll items
    }

    #[test]
    #[ignore] /* FIXME remove after get_f will be restored */
    fn pool_worker_get_f_works() {
        assert_eq!(PoolWorker::get_f(0), 0);
        assert_eq!(PoolWorker::get_f(3), 0);
        assert_eq!(PoolWorker::get_f(4), 1);
        assert_eq!(PoolWorker::get_f(5), 1);
        assert_eq!(PoolWorker::get_f(6), 1);
        assert_eq!(PoolWorker::get_f(7), 2);
    }

    #[test]
    fn transaction_handler_process_reply_works() {
        let mut th: TransactionHandler = Default::default();
        th.f = 1;
        let mut pc = super::types::CommandProcess {
            cmd_ids: Vec::new(),
            replies: HashMap::new(),
            nack_cnt: 0,
        };
        let json = "{\"value\":1}";
        pc.replies.insert(HashableValue { inner: serde_json::from_str(json).unwrap() }, 1);
        let req_id = 1;
        th.pending_commands.insert(req_id, pc);

        th.process_reply(req_id, &json.to_string());

        assert_eq!(th.pending_commands.len(), 0);
    }

    #[test]
    fn transaction_handler_process_reply_works_for_different_replies_with_same_req_id() {
        let mut th: TransactionHandler = Default::default();
        th.f = 1;
        let mut pc = super::types::CommandProcess {
            cmd_ids: Vec::new(),
            replies: HashMap::new(),
            nack_cnt: 0,
        };
        let json1 = "{\"value\":1}";
        let json2 = "{\"value\":2}";
        pc.replies.insert(HashableValue { inner: serde_json::from_str(json1).unwrap() }, 1);
        let req_id = 1;
        th.pending_commands.insert(req_id, pc);

        th.process_reply(req_id, &json2.to_string());

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

        assert_eq!(th.pending_commands.len(), 1);
        let pending_cmd = th.pending_commands.get(&req_id).unwrap();
        let exp_command_process = CommandProcess {
            nack_cnt: 0,
            replies: HashMap::new(),
            cmd_ids: vec!(cmd_id),
        };
        assert_eq!(pending_cmd, &exp_command_process);
    }

    #[test]
    fn catchup_handler_start_catchup_works() {
        let mut ch: CatchupHandler = Default::default();
        let (gt, handle) = nodes_emulator::start();
        ch.merkle_tree.append(gt.to_msg_pack().unwrap()).unwrap();
        let mut rn: RemoteNode = RemoteNode::new(&gt).unwrap();
        rn.connect(&zmq::Context::new(), &zmq::CurveKeyPair::new().unwrap()).unwrap();
        ch.nodes.push(rn);
        ch.target_mt_size = 2;

        ch.start_catchup().unwrap();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        let expected_resp: CatchupReq = CatchupReq {
            ledgerId: 0,
            seqNoStart: 2,
            seqNoEnd: 2,
            catchupTill: 2,
        };
        let act_resp = CatchupReq::from_json(emulator_msgs[0].as_str()).unwrap();
        assert_eq!(expected_resp, act_resp);
    }

    #[test]
    fn remote_node_connect_works_and_can_ping_pong() {
        let (gt, handle) = nodes_emulator::start();
        let mut rn: RemoteNode = RemoteNode::new(&gt).unwrap();
        let ctx = zmq::Context::new();
        rn.connect(&ctx, &zmq::CurveKeyPair::new().unwrap()).unwrap();
        rn.send_str("pi").expect("send");
        rn.zsock.as_ref().expect("sock").poll(zmq::POLLIN, nodes_emulator::POLL_TIMEOUT).expect("poll");
        assert_eq!("po", rn.zsock.as_ref().expect("sock").recv_string(zmq::DONTWAIT).expect("recv").expect("string").as_str());
        handle.join().expect("join");
    }

    mod nodes_emulator {
        extern crate sodiumoxide;

        use services::pool::rust_base58::ToBase58;
        use std::thread;
        use super::*;

        pub static POLL_TIMEOUT: i64 = 1000; /* in ms */

        pub fn start() -> (GenTransaction, thread::JoinHandle<Vec<String>>) {
            let (vk, sk) = sodiumoxide::crypto::sign::ed25519::gen_keypair();
            let pkc = ED25519::vk_to_curve25519(&Vec::from(&vk.0 as &[u8])).expect("Invalid pkc");
            let skc = ED25519::sk_to_curve25519(&Vec::from(&sk.0 as &[u8])).expect("Invalid skc");
            let ctx = zmq::Context::new();
            let s: zmq::Socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();
            let gt = GenTransaction {
                identifier: "".to_string(),
                data: NodeData {
                    alias: "n1".to_string(),
                    services: Vec::new(),
                    client_port: 9700,
                    client_ip: "127.0.0.1".to_string(),
                    node_ip: "".to_string(),
                    node_port: 0,
                },
                txn_id: None,
                txn_type: "0".to_string(),
                dest: (&vk.0 as &[u8]).to_base58(),
            };
            let addr = format!("tcp://{}:{}", gt.data.client_ip, gt.data.client_port);
            s.set_curve_publickey(pkc.as_slice()).expect("set public key");
            s.set_curve_secretkey(skc.as_slice()).expect("set secret key");
            s.set_curve_server(true).expect("set curve server");
            s.bind(addr.as_str()).expect("bind");
            let handle = thread::spawn(move || {
                let mut received_msgs: Vec<String> = Vec::new();
                let poll_res = s.poll(zmq::POLLIN, POLL_TIMEOUT).expect("poll");
                if poll_res == 1 {
                    let v = s.recv_multipart(zmq::DONTWAIT).expect("recv mulp");
                    trace!("Node emulator poll recv {:?}", v);
                    s.send_multipart(&[v[0].as_slice(), "po".as_bytes()], zmq::DONTWAIT).expect("send mulp");
                    received_msgs.push(String::from_utf8(v[1].clone()).unwrap());
                } else {
                    warn!("Node emulator poll return {}", poll_res)
                }
                received_msgs
            });
            (gt, handle)
        }
    }
}
