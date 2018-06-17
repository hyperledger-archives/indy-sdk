mod types;
mod catchup;
mod transaction_handler;
mod state_proof;

extern crate byteorder;
extern crate digest;
extern crate hex;
extern crate rand;
extern crate rust_base58;
extern crate sha2;
extern crate time;
extern crate zmq;
extern crate rmp_serde;
extern crate indy_crypto;


use self::byteorder::{ByteOrder, LittleEndian, WriteBytesExt, ReadBytesExt};
use self::rust_base58::FromBase58;
use std::str::from_utf8;
use self::time::{Duration, Tm};
use serde_json;
use serde_json::Value as SJsonValue;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::{fmt, fs, io, thread};
use std::fmt::Debug;
use std::io::{Read, BufRead, Write};
use std::ops::{Add, Sub};

use api::ledger::{CustomFree, CustomTransactionParser};
use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use errors::common::CommonError;
use self::catchup::CatchupHandler;
use self::transaction_handler::TransactionHandler;
use self::types::*;
use services::ledger::merkletree::merkletree::MerkleTree;
use utils::crypto::box_::CryptoBox;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;
use self::indy_crypto::bls::VerKey;
use std::path::PathBuf;
use std::sync::Mutex;
use domain::ledger::request::ProtocolVersion;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

pub struct PoolService {
    pending_pools: RefCell<HashMap<i32, Pool>>,
    open_pools: RefCell<HashMap<i32, Pool>>,
}

lazy_static! {
    static ref REGISTERED_SP_PARSERS: Mutex<HashMap<String, (CustomTransactionParser, CustomFree)>> = Mutex::new(HashMap::new());
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

impl PoolWorkerHandler {
    fn process_msg(&mut self, raw_msg: &String, src_ind: usize) -> Result<Option<MerkleTree>, PoolError> {
        let msg = Message::from_raw_str(raw_msg)
            .map_err(map_err_trace!())
            .map_err(|_|
                CommonError::IOError(
                    io::Error::from(io::ErrorKind::InvalidData)))?;
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
        }
    }

    fn send_request(&mut self, cmd: &str, cmd_id: i32) -> Result<(), PoolError> {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(_) => {
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

    fn get_upcoming_timeout(&self) -> Option<Tm> {
        match self {
            &PoolWorkerHandler::CatchupHandler(ref ch) => ch.get_upcoming_timeout(),
            &PoolWorkerHandler::TransactionHandler(ref ch) => ch.get_upcoming_timeout(),
        }
    }

    fn process_timeout(&mut self) -> Result<(), PoolError> {
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.process_timeout(),
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.process_timeout(),
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

        let gen_tnxs = PoolWorker::_build_node_state(&merkle_tree)?;

        for (_, gen_txn) in &gen_tnxs {
            let mut rn: RemoteNode = match RemoteNode::new(&gen_txn) {
                Ok(rn) => rn,
                Err(err) => {
                    warn!("{:?}", err);
                    continue
                }
            };
            rn.connect(&ctx, &key_pair)?;
            rn.send_str("pi")?;
            self.handler.nodes_mut().push(rn);
        }

        let cnt = self.handler.nodes().len();
        self.handler.set_f(PoolWorker::get_f(cnt));
        if let PoolWorkerHandler::CatchupHandler(ref mut handler) = self.handler {
            handler.reset_nodes_votes();
        }
        Ok(())
    }

    fn _build_node_state(merkle_tree: &MerkleTree) -> Result<HashMap<String, NodeTransactionV1>, PoolError> {
        let mut gen_tnxs: HashMap<String, NodeTransactionV1> = HashMap::new();

        for gen_txn in merkle_tree {
            let gen_txn: NodeTransaction =
                rmp_serde::decode::from_slice(gen_txn.as_slice())
                    .map_err(|e|
                        CommonError::InvalidState(format!("MerkleTree contains invalid data {:?}", e)))?;

            let protocol_version = ProtocolVersion::get();

            let mut gen_txn = match gen_txn {
                NodeTransaction::NodeTransactionV0(txn) => {
                    if protocol_version != 1 {
                        return Err(PoolError::PoolIncompatibleProtocolVersion(
                            format!("Libindy PROTOCOL_VERSION is {} but Pool Genesis Transactions are of version {}.\
                             Call indy_set_protocol_version(1) to set correct PROTOCOL_VERSION", protocol_version, NodeTransactionV0::VERSION)));
                    }
                    NodeTransactionV1::from(txn)
                }
                NodeTransaction::NodeTransactionV1(txn) => {
                    if protocol_version != 2 {
                        return Err(PoolError::PoolIncompatibleProtocolVersion(
                            format!("Libindy PROTOCOL_VERSION is {} but Pool Genesis Transactions are of version {}.\
                             Call indy_set_protocol_version(2) to set correct PROTOCOL_VERSION", protocol_version, NodeTransactionV1::VERSION)));
                    }
                    txn
                }
            };

            if gen_tnxs.contains_key(&gen_txn.txn.data.dest) {
                gen_tnxs.get_mut(&gen_txn.txn.data.dest).unwrap().update(&mut gen_txn)?;
            } else {
                gen_tnxs.insert(gen_txn.txn.data.dest.clone(), gen_txn);
            }
        }
        Ok(gen_tnxs)
    }

    fn init_catchup(&mut self, refresh_cmd_id: Option<i32>) -> Result<(), PoolError> {
        let mt = PoolWorker::restore_merkle_tree_from_pool_name(self.name.as_str())?;
        if mt.count() == 0 {
            return Err(PoolError::CommonError(
                CommonError::InvalidState("Invalid Genesis Transaction file".to_string())));
        }

        let catchup_handler = CatchupHandler {
            merkle_tree: mt,
            initiate_cmd_id: refresh_cmd_id.unwrap_or(self.open_cmd_id),
            is_refresh: refresh_cmd_id.is_some(),
            pool_id: self.pool_id,
            timeout: time::now_utc().add(Duration::seconds(catchup::CATCHUP_ROUND_TIMEOUT)),
            pool_name: self.name.clone(),
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
            self.handler.flush_requests(Err(err.clone()))?;
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
        for action in actions {
            match action {
                ZMQLoopAction::Terminate(cmd_id) => {
                    let res = self.handler.flush_requests(Err(PoolError::Terminate));
                    if cmd_id >= 0 {
                        CommandExecutor::instance().send(Command::Pool(PoolCommand::CloseAck(cmd_id, res)))?;
                    }
                    return Err(PoolError::Terminate);
                }
                ZMQLoopAction::Refresh(cmd_id) => {
                    self.refresh(cmd_id)?;
                }
                ZMQLoopAction::MessageToProcess(ref msg) => {
                    if let Some(new_mt) = self.handler.process_msg(&msg.message, msg.node_idx)? {
                        self.handler.flush_requests(Ok(()))?;
                        self.handler = PoolWorkerHandler::TransactionHandler(Default::default());
                        self.connect_to_known_nodes(Some(&new_mt))?;
                    }
                }
                ZMQLoopAction::RequestToSend(ref req) => {
                    self.handler.send_request(req.request.as_str(), req.id).or_else(|err| {
                        CommandExecutor::instance()
                            .send(Command::Ledger(LedgerCommand::SubmitAck(req.id, Err(err))))
                            .map_err(|err| {
                                CommonError::InvalidState(format!("Can't send ACK cmd: {:?}", err))
                            })
                    })?;
                }
                ZMQLoopAction::Timeout => {
                    self.handler.process_timeout()?;
                }
            }
        }
        Ok(())
    }

    fn poll_zmq(&mut self) -> Result<Vec<ZMQLoopAction>, PoolError> {
        let mut actions: Vec<ZMQLoopAction> = Vec::new();

        let mut poll_items = self.get_zmq_poll_items()?;
        let t = self.get_zmq_poll_timeout();
        let r = zmq::poll(poll_items.as_mut_slice(), t)?;
        trace!("zmq poll {:?}", r);
        if r == 0 {
            actions.push(ZMQLoopAction::Timeout);
        }

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
                    CommonError::InvalidState(format!("Invalid command received: {:?}", err)))?;
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

    fn get_zmq_poll_timeout(&self) -> i64 {
        let first_event: time::Tm = match self.handler.get_upcoming_timeout() {
            None => return -1,
            Some(tm) => tm,
        };
        let now_utc = time::now_utc();
        trace!("get_zmq_poll_timeout first_event {:?}", first_event);
        trace!("get_zmq_poll_timeout now_utc {:?}", now_utc);
        let diff: Duration = first_event.sub(now_utc);
        trace!("get_zmq_poll_timeout diff Duration {:?}", diff);
        let diff: i64 = max(diff.num_milliseconds(), 1);
        trace!("get_zmq_poll_timeout diff ms {}", diff);
        return diff;
    }


    fn _restore_merkle_tree_from_file(txn_file: &str) -> Result<MerkleTree, PoolError> {
        PoolWorker::_restore_merkle_tree_from_genesis(&PathBuf::from(txn_file))
    }

    fn _restore_merkle_tree_from_genesis(file_name: &PathBuf) -> Result<MerkleTree, PoolError> {
        let mut mt = MerkleTree::from_vec(Vec::new()).map_err(map_err_trace!())?;

        let f = fs::File::open(file_name).map_err(map_err_trace!())?;

        let reader = io::BufReader::new(&f);
        for line in reader.lines() {
            let line: String = line.map_err(map_err_trace!())?.trim().to_string();
            if line.is_empty() { continue };
            let genesis_txn: SJsonValue = serde_json::from_str(line.as_str())
                .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
            let bytes = rmp_serde::encode::to_vec_named(&genesis_txn)
                .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
            mt.append(bytes).map_err(map_err_trace!())?;
        }
        Ok(mt)
    }

    fn _restore_merkle_tree_from_cache(file_name: &PathBuf) -> Result<MerkleTree, PoolError> {
        let mut mt = MerkleTree::from_vec(Vec::new()).map_err(map_err_trace!())?;

        let mut f = fs::File::open(file_name).map_err(map_err_trace!())?;

        while let Ok(bytes) = f.read_u64::<LittleEndian>().map_err(CommonError::IOError).map_err(PoolError::from) {
            let mut buf = vec![0; bytes as usize];
            f.read(buf.as_mut()).map_err(map_err_trace!())?;
            mt.append(buf.to_vec()).map_err(map_err_trace!())?;
        }
        Ok(mt)
    }

    pub fn restore_merkle_tree_from_pool_name(pool_name: &str) -> Result<MerkleTree, PoolError> {
        let mut p = EnvironmentUtils::pool_path(pool_name);

        let mut p_stored = p.clone();
        p_stored.push("stored");
        p_stored.set_extension("btxn");

        if !p_stored.exists() {
            p.push(pool_name);
            p.set_extension("txn");

            if !p.exists() {
                return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
            }

            PoolWorker::_restore_merkle_tree_from_genesis(&p)
        } else {
            PoolWorker::_restore_merkle_tree_from_cache(&p_stored)
        }
    }

    fn _parse_txn_from_json(txn: &[u8]) -> Result<Vec<u8>, CommonError> {
        let txn_str = from_utf8(txn).map_err(|_| CommonError::InvalidStructure(format!("Can't parse valid UTF-8 string from this array: {:?}", txn)))?;

        if txn_str.trim().is_empty() {
            return Ok(vec![]);
        }

        let genesis_txn: SJsonValue = serde_json::from_str(txn_str.trim())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
        rmp_serde::encode::to_vec_named(&genesis_txn)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))
    }

    pub fn dump_new_txns(pool_name: &str, txns: &Vec<Vec<u8>>) -> Result<(), PoolError> {
        let mut p = EnvironmentUtils::pool_path(pool_name);

        p.push("stored");
        p.set_extension("btxn");
        if !p.exists() {
            PoolWorker::_dump_genesis_to_stored(&p, pool_name)?;
        }

        let mut file = fs::OpenOptions::new().append(true).open(p)
            .map_err(|e| CommonError::IOError(e))
            .map_err(map_err_err!())?;

        PoolWorker::_dump_vec_to_file(txns, &mut file)
    }

    fn _dump_genesis_to_stored(p: &PathBuf, pool_name: &str) -> Result<(), PoolError> {
        let mut file = fs::File::create(p)
            .map_err(|e| CommonError::IOError(e))
            .map_err(map_err_err!())?;

        let mut p_genesis = EnvironmentUtils::pool_path(pool_name);
        p_genesis.push(pool_name);
        p_genesis.set_extension("txn");

        if !p_genesis.exists() {
            return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
        }

        let genesis_vec = PoolWorker::_genesis_to_binary(&p_genesis)?;
        PoolWorker::_dump_vec_to_file(&genesis_vec, &mut file)
    }

    fn _dump_vec_to_file(v: &Vec<Vec<u8>>, file: &mut fs::File) -> Result<(), PoolError> {
        v.into_iter().map(|vec| {
            file.write_u64::<LittleEndian>(vec.len() as u64).map_err(map_err_trace!())?;
            file.write_all(vec).map_err(map_err_trace!())
        }).fold(Ok(()), |acc, next| {
            match (acc, next) {
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(PoolError::CommonError(CommonError::IOError(e))),
                _ => Ok(()),
            }
        })
    }

    fn _genesis_to_binary(p: &PathBuf) -> Result<Vec<Vec<u8>>, PoolError> {
        let f = fs::File::open(p).map_err(map_err_trace!())?;
        let reader = io::BufReader::new(&f);
        reader
            .lines()
            .into_iter()
            .map(|res| {
                let line = res.map_err(map_err_trace!())?;
                PoolWorker::_parse_txn_from_json(line.trim().as_bytes()).map_err(PoolError::from).map_err(map_err_err!())
            })
            .fold(Ok(Vec::new()), |acc, next| {
                match (acc, next) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(mut acc), Ok(res)) => {
                        let mut vec = vec![];
                        vec.append(&mut acc);
                        vec.push(res);
                        Ok(vec)
                    }
                }
            })
    }

    pub fn drop_saved_txns(pool_name: &str) -> Result<(), PoolError> {
        warn!("Cache is invalid -- dropping it!");
        let mut p = EnvironmentUtils::pool_path(pool_name);

        p.push("stored");
        p.set_extension("btxn");
        if p.exists() {
            fs::remove_file(p).map_err(CommonError::IOError).map_err(PoolError::from)?;
            Ok(())
        } else {
            Err(PoolError::CommonError(CommonError::InvalidState("Can't recover to genesis -- no txns stored. Possible problems in genesis txns.".to_string())))
        }
    }

    fn get_f(cnt: usize) -> usize {
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
            pool_id,
            name: name.to_string(),
            handler: PoolWorkerHandler::CatchupHandler(CatchupHandler {
                initiate_cmd_id: cmd_id,
                pool_id,
                pool_name: name.to_string(),
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
    fn new(txn: &NodeTransactionV1) -> Result<RemoteNode, PoolError> {
        let node_verkey = txn.txn.data.dest.as_str().from_base58()
            .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field dest in genesis transaction: {:?}", err)) })?;

        if txn.txn.data.data.services.is_none() || !txn.txn.data.data.services.as_ref().unwrap().contains(&"VALIDATOR".to_string()) {
            return Err(PoolError::CommonError(CommonError::InvalidState("Node is not a Validator".to_string())));
        }

        let address = match (&txn.txn.data.data.client_ip, &txn.txn.data.data.client_port) {
            (&Some(ref client_ip), &Some(ref client_port)) => format!("tcp://{}:{}", client_ip, client_port),
            _ => return Err(PoolError::CommonError(CommonError::InvalidState("Client address not found".to_string())))
        };

        let blskey = match txn.txn.data.data.blskey {
            Some(ref blskey) => {
                let key = blskey.as_str().from_base58()
                    .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field blskey in genesis transaction: {:?}", err)) })?;
                Some(VerKey::from_bytes(key.as_slice())
                    .map_err(|err| { CommonError::InvalidStructure(format!("Invalid field blskey in genesis transaction: {:?}", err)) })?)
            }
            None => None
        };

        Ok(RemoteNode {
            public_key: CryptoBox::vk_to_curve25519(&node_verkey)?,
            zaddr: address,
            zsock: None,
            name: txn.txn.data.data.alias.clone(),
            is_blacklisted: false,
            blskey: blskey
        })
    }

    fn connect(&mut self, ctx: &zmq::Context, key_pair: &zmq::CurveKeyPair) -> Result<(), PoolError> {
        let s = ctx.socket(zmq::SocketType::DEALER)?;
        s.set_identity(key_pair.public_key.as_bytes())?;
        s.set_curve_secretkey(&key_pair.secret_key)?;
        s.set_curve_publickey(&key_pair.public_key)?;
        s.set_curve_serverkey(
            zmq::z85_encode(self.public_key.as_slice())
                .map_err(|err| { CommonError::InvalidStructure(format!("Can't encode server key as z85: {:?}", err)) })?
                .as_str())?;
        s.set_linger(0)?; //TODO set correct timeout
        s.connect(&self.zaddr)?;
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
            .recv_string(zmq::DONTWAIT)
            .map_err(map_err_trace!())?
            .map_err(|err| {
                trace!("Can't parse UTF-8 string from bytes {:?}", err);
                err
            })?;
        info!("RemoteNode::recv_msg {} {}", self.name, msg);

        Ok(Some(msg))
    }

    fn send_str(&self, str: &str) -> Result<(), PoolError> {
        info!("RemoteNode::send_str {} {}", self.name, str);
        self.zsock.as_ref()
            .ok_or(CommonError::InvalidState("Try to send str for unconnected RemoteNode".to_string()))?
            .send(str.as_bytes(), 0)
            .map_err(map_err_trace!())?;
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
            pending_pools: RefCell::new(HashMap::new()),
            open_pools: RefCell::new(HashMap::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<&str>) -> Result<(), PoolError> {
        trace!("PoolService::create {} with config {:?}", name, config);
        let mut path = EnvironmentUtils::pool_path(name);
        let pool_config: PoolConfig = match config {
            Some(config) => PoolConfig::from_json(config)
                .map_err(|err|
                    CommonError::InvalidStructure(format!("Invalid pool config format: {}", err.description())))?,
            None => PoolConfig::default_for_name(name)
        };

        if path.as_path().exists() {
            return Err(PoolError::AlreadyExists(format!("Pool ledger config file with name \"{}\" already exists", name)));
        }

        // check that we can build MerkeleTree from genesis transaction file
        let mt = PoolWorker::_restore_merkle_tree_from_file(&pool_config.genesis_txn)?;
        if mt.count() == 0 {
            return Err(PoolError::CommonError(
                CommonError::InvalidStructure("Invalid Genesis Transaction file".to_string())));
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
        for pool in self.open_pools.try_borrow().map_err(CommonError::from)?.values() {
            if pool.name.eq(name) {
                return Err(PoolError::CommonError(CommonError::InvalidState("Can't delete pool config - pool is open now".to_string())));
            }
        }
        let path = EnvironmentUtils::pool_path(name);
        fs::remove_dir_all(path).map_err(PoolError::from)
    }

    pub fn open(&self, name: &str, _config: Option<&str>) -> Result<i32, PoolError> {
        for pool in self.open_pools.try_borrow().map_err(CommonError::from)?.values() {
            if name.eq(pool.name.as_str()) {
                //TODO change error
                return Err(PoolError::AlreadyExists("Pool with same name already opened".to_string()));
            }
        }

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let new_pool = Pool::new(name, cmd_id)?;
        //FIXME process config: check None (use default), transfer to Pool instance

        self.pending_pools.try_borrow_mut().map_err(CommonError::from)?.insert(new_pool.id, new_pool);
        return Ok(cmd_id);
    }

    pub fn add_open_pool(&self, pool_id: i32) -> Result<i32, PoolError> {
        let pool = self.pending_pools.try_borrow_mut().map_err(CommonError::from)?
            .remove(&pool_id)
            .ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", pool_id)))?;

        self.open_pools.try_borrow_mut().map_err(CommonError::from)?.insert(pool_id, pool);

        Ok(pool_id)
    }

    pub fn send_tx(&self, handle: i32, json: &str) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.open_pools.try_borrow().map_err(CommonError::from)?
            .get(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .send_tx(cmd_id, json)?;
        Ok(cmd_id)
    }

    pub fn register_sp_parser(txn_type: &str,
                              parser: CustomTransactionParser, free: CustomFree) -> Result<(), PoolError> {
        if transaction_handler::REQUESTS_FOR_STATE_PROOFS.contains(&txn_type) {
            return Err(PoolError::CommonError(CommonError::InvalidStructure(
                format!("Try to override StateProof parser for default TXN_TYPE {}", txn_type))));
        }
        REGISTERED_SP_PARSERS.lock()
            .map(|mut map| {
                map.insert(txn_type.to_owned(), (parser, free));
            })
            .map_err(|_| PoolError::CommonError(CommonError::InvalidState(
                "Can't register new SP parser: mutex lock error".to_owned())))
    }

    pub fn get_sp_parser(txn_type: &str) -> Option<(CustomTransactionParser, CustomFree)> {
        REGISTERED_SP_PARSERS.lock().ok().and_then(|map| {
            map.get(txn_type).map(Clone::clone)
        })
    }

    pub fn close(&self, handle: i32) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.open_pools.try_borrow_mut().map_err(CommonError::from)?
            .remove(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .close(cmd_id)
            .map(|()| cmd_id)
    }

    pub fn refresh(&self, handle: i32) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.open_pools.try_borrow_mut().map_err(CommonError::from)?
            .get(&handle).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?
            .refresh(cmd_id)
            .map(|()| cmd_id)
    }

    pub fn list(&self) -> Result<Vec<serde_json::Value>, PoolError> {
        let mut pool = Vec::new();
        let pool_home_path = EnvironmentUtils::pool_home_path();

        if let Ok(entries) = fs::read_dir(pool_home_path) {
            for entry in entries {
                let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue };
                if let Some(pool_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                    let json = json!({"pool":pool_name.to_owned()});
                    pool.push(json);
                }
            }
        }
        Ok(pool)
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, PoolError> {
        self.open_pools.try_borrow().map_err(CommonError::from)?.get(&handle).map_or(
            Err(PoolError::InvalidHandle(format!("Pool doesn't exists for handle {}", handle))),
            |pool: &Pool| Ok(pool.name.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::test::TestUtils;

    const TEST_PROTOCOL_VERSION: usize = 2;

    fn _set_protocol_version(version: usize) {
        ProtocolVersion::set(version);
    }

    mod pool_service {
        use super::*;
        use std::path;

        #[test]
        fn pool_service_new_works() {
            PoolService::new();
            assert!(true, "No crashes on PoolService::new");
        }

        #[test]
        fn pool_service_drop_works() {
            fn drop_test() {
                PoolService::new();
            }

            drop_test();
            assert!(true, "No crashes on PoolService::drop");
        }

        #[test]
        fn pool_service_close_works() {
            TestUtils::cleanup_storage();

            let ps = PoolService::new();
            let pool_id = SequenceUtils::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.open_pools.borrow_mut().insert(pool_id, Pool {
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
            TestUtils::cleanup_storage();

            let ps = PoolService::new();
            let pool_id = SequenceUtils::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.open_pools.borrow_mut().insert(pool_id, Pool {
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
            TestUtils::cleanup_storage();

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
            TestUtils::cleanup_storage();

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
            ps.open_pools.borrow_mut().insert(pool_id, pool);

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
            TestUtils::cleanup_storage();
            _set_protocol_version(TEST_PROTOCOL_VERSION);

            let pool_name = "pool_drop_works";
            let gen_txn = NODE1;

            // create minimal fs config stub before Pool::new()
            let mut pool_path = EnvironmentUtils::pool_path(pool_name);
            fs::create_dir_all(&pool_path).unwrap();
            pool_path.push(pool_name);
            pool_path.set_extension("txn");
            let mut file = fs::File::create(pool_path).unwrap();
            file.write(&gen_txn.as_bytes()).unwrap();

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
        TestUtils::cleanup_storage();

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
                handler: PoolWorkerHandler::CatchupHandler(CatchupHandler {
                    timeout: time::now_utc().add(Duration::seconds(2)),
                    ..Default::default()
                }),
            }
        }
    }

    pub const NODE1_OLD: &'static str = r#"{"data":{"alias":"Node1","client_ip":"192.168.1.35","client_port":9702,"node_ip":"192.168.1.35","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#;
    pub const NODE2_OLD: &'static str = r#"{"data":{"alias":"Node2","client_ip":"192.168.1.35","client_port":9704,"node_ip":"192.168.1.35","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#;

    pub const NODE1: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"10.0.0.2","client_port":9702,"node_ip":"10.0.0.2","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"},"metadata":{"from":"Th7MpTaRZVRYnPiabds81Y"},"type":"0"},"txnMetadata":{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"},"ver":"1"}"#;
    pub const NODE2: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"10.0.0.2","client_port":9704,"node_ip":"10.0.0.2","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"},"metadata":{"from":"EbP4aYNeTHL6q385GuVpRV"},"type":"0"},"txnMetadata":{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"},"ver":"1"}"#;
    pub const NODE3: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"10.0.0.2","client_port":9706,"node_ip":"10.0.0.2","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"},"metadata":{"from":"4cU41vWW82ArfxJxHkzXPG"},"type":"0"},"txnMetadata":{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"},"ver":"1"}"#;
    pub const NODE4: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"10.0.0.2","client_port":9708,"node_ip":"10.0.0.2","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"},"metadata":{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"},"type":"0"},"txnMetadata":{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"},"ver":"1"}"#;

    fn _write_genesis_txns(txns: &str) {
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();
    }

    #[test]
    fn pool_worker_restore_merkle_tree_works_from_genesis_txns() {
        TestUtils::cleanup_storage();

        let txns_src = format!("{}\n{}", NODE1, NODE2);
        _write_genesis_txns(&txns_src);

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();

        assert_eq!(merkle_tree.count(), 2, "test restored MT size");
        assert_eq!(merkle_tree.root_hash_hex(), "3768ef5b25a01d19c0fda687f2354b29e004821bce8557e70085379f536907ed", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_connect_to_known_nodes_works() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let mut pw: PoolWorker = Default::default();
        let (gt, handle) = nodes_emulator::start();
        let mut merkle_tree: MerkleTree = MerkleTree::from_vec(Vec::new()).unwrap();
        merkle_tree.append(rmp_serde::to_vec_named(&gt).unwrap()).unwrap();

        pw.connect_to_known_nodes(Some(&merkle_tree)).unwrap();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        assert_eq!("pi", emulator_msgs[0]);
    }

    #[test]
    pub fn pool_worker_works_for_deserialize_cache() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let txn1_json: serde_json::Value = serde_json::from_str(NODE1).unwrap();
        let txn2_json: serde_json::Value = serde_json::from_str(NODE2).unwrap();
        let txn3_json: serde_json::Value = serde_json::from_str(NODE3).unwrap();
        let txn4_json: serde_json::Value = serde_json::from_str(NODE4).unwrap();

        let pool_cache = vec![rmp_serde::to_vec_named(&txn1_json).unwrap(),
                              rmp_serde::to_vec_named(&txn2_json).unwrap(),
                              rmp_serde::to_vec_named(&txn3_json).unwrap(),
                              rmp_serde::to_vec_named(&txn4_json).unwrap()];

        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push("stored");
        path.set_extension("btxn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        pool_cache.iter().for_each(|vec| {
            f.write_u64::<LittleEndian>(vec.len() as u64).unwrap();
            f.write_all(vec).unwrap();
        });

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let _node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();
    }

    #[test]
    fn pool_worker_build_node_state_works_for_old_format() {
        TestUtils::cleanup_storage();

        _set_protocol_version(1);

        let node1: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransactionV0>(NODE1_OLD).unwrap());
        let node2: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransactionV0>(NODE2_OLD).unwrap());

        let txns_src = format!("{}\n{}\n", NODE1_OLD, NODE2_OLD);

        _write_genesis_txns(&txns_src);

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();

        assert_eq!(1, ProtocolVersion::get());

        assert_eq!(2, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], node1);
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], node2);
    }

    #[test]
    fn pool_worker_build_node_state_works_for_new_format() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let node1: NodeTransactionV1 = serde_json::from_str(NODE1).unwrap();
        let node2: NodeTransactionV1 = serde_json::from_str(NODE2).unwrap();

        let txns_src = format!("{}\n{}\n{}\n{}\n", NODE1, NODE2, NODE3, NODE4);

        _write_genesis_txns(&txns_src);

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();

        assert_eq!(2, ProtocolVersion::get());

        assert_eq!(4, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], node1);
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], node2);
    }

    #[test]
    fn pool_worker_build_node_state_works_for_old_txns_format_and_2_protocol_version() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let txns_src = format!("{}\n{}\n", NODE1_OLD, NODE2_OLD);

        _write_genesis_txns(&txns_src);

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let res = PoolWorker::_build_node_state(&merkle_tree);
        assert_match!(Err(PoolError::PoolIncompatibleProtocolVersion(_)), res);
    }

    #[test]
    fn pool_worker_build_node_state_works_for_new_txns_format_and_1_protocol_version() {
        TestUtils::cleanup_storage();

        _set_protocol_version(1);

        let txns_src = format!("{}\n{}\n", NODE1, NODE2);

        _write_genesis_txns(&txns_src);

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let res = PoolWorker::_build_node_state(&merkle_tree);
        assert_match!(Err(PoolError::PoolIncompatibleProtocolVersion(_)), res);
    }

    #[test]
    fn pool_worker_poll_zmq_works_for_terminate() {
        TestUtils::cleanup_storage();

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
        send_cmd_sock.send("exit".as_bytes(), zmq::DONTWAIT).expect("send");
        let actions: Vec<ZMQLoopAction> = handle.join().unwrap();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], ZMQLoopAction::Terminate(-1));
    }

    #[test]
    fn pool_worker_get_zmq_poll_items_works() {
        TestUtils::cleanup_storage();

        let pw: PoolWorker = Default::default();

        let poll_items = pw.get_zmq_poll_items().unwrap();

        assert_eq!(poll_items.len(), pw.handler.nodes().len() + 1);
        //TODO compare poll items
    }

    #[test]
    fn pool_worker_get_f_works() {
        TestUtils::cleanup_storage();

        assert_eq!(PoolWorker::get_f(0), 0);
        assert_eq!(PoolWorker::get_f(3), 0);
        assert_eq!(PoolWorker::get_f(4), 1);
        assert_eq!(PoolWorker::get_f(5), 1);
        assert_eq!(PoolWorker::get_f(6), 1);
        assert_eq!(PoolWorker::get_f(7), 2);
    }

    #[test]
    fn catchup_handler_start_catchup_works() {
        TestUtils::cleanup_storage();

        let mut ch: CatchupHandler = Default::default();
        let (gt, handle) = nodes_emulator::start();
        ch.merkle_tree.append(rmp_serde::to_vec_named(&gt).unwrap()).unwrap();
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
        TestUtils::cleanup_storage();

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
        use self::indy_crypto::bls::{Generator, SignKey, VerKey};

        pub static POLL_TIMEOUT: i64 = 5_000; /* in ms */

        pub fn start() -> (NodeTransactionV1, thread::JoinHandle<Vec<String>>) {
            let (vk, sk) = sodiumoxide::crypto::sign::ed25519::gen_keypair();
            let pkc = CryptoBox::vk_to_curve25519(&Vec::from(&vk.0 as &[u8])).expect("Invalid pkc");
            let skc = CryptoBox::sk_to_curve25519(&Vec::from(&sk.0 as &[u8])).expect("Invalid skc");
            let ctx = zmq::Context::new();
            let s: zmq::Socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();

            let blskey = VerKey::new(&Generator::from_bytes(&"3LHpUjiyFC2q2hD7MnwwNmVXiuaFbQx2XkAFJWzswCjgN1utjsCeLzHsKk1nJvFEaS4fcrUmVAkdhtPCYbrVyATZcmzwJReTcJqwqBCPTmTQ9uWPwz6rEncKb2pYYYFcdHa8N17HzVyTqKfgPi4X9pMetfT3A5xCHq54R2pDNYWVLDX".from_base58().unwrap()).unwrap(),
                                     &SignKey::new(None).unwrap()).unwrap().as_bytes().to_base58();

            let gt = NodeTransactionV1 {
                txn: Txn {
                    txn_type: "1".to_string(),
                    protocol_version: None,
                    data: TxnData {
                        data: NodeData {
                            alias: "n1".to_string(),
                            client_ip: Some("127.0.0.1".to_string()),
                            client_port: Some(9700),
                            node_ip: Some("".to_string()),
                            node_port: Some(0),
                            services: Some(vec!["VALIDATOR".to_string()]),
                            blskey: Some(blskey),
                        },
                        dest: (&vk.0 as &[u8]).to_base58(),
                        verkey: None,
                    },
                    metadata: TxnMetadata { req_id: None, from: String::new() },
                },
                txn_metadata: Metadata {
                    creation_time: None,
                    seq_no: None,
                    txn_id: None,
                },
                req_signature: ReqSignature { type_: None, values: None },
                ver: String::new(),
            };
            let addr = format!("tcp://{}:{}", gt.txn.data.data.client_ip.clone().unwrap(), gt.txn.data.data.client_port.clone().unwrap());
            s.set_curve_publickey(&zmq::z85_encode(pkc.as_slice()).unwrap()).expect("set public key");
            s.set_curve_secretkey(&zmq::z85_encode(skc.as_slice()).unwrap()).expect("set secret key");
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
