mod types;
mod catchup;

extern crate byteorder;
extern crate rust_base58;
extern crate zmq;

use self::byteorder::{ByteOrder, LittleEndian};
use self::rust_base58::FromBase58;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::{fmt, fs, io, thread};
use std::fmt::Debug;
use std::io::{BufRead, Write};
use std::error::Error;

use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use errors::crypto::CryptoError;
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
    worker: Option<thread::JoinHandle<Result<(), PoolError>>>,
}

struct PoolWorker {
    cmd_sock: zmq::Socket,
    open_cmd_id: i32,
    pool_id: i32,
    name: String,
    handler: PoolWorkerHandler,
}

enum PoolWorkerHandler {
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
        let msg = Message::from_raw_str(raw_msg).unwrap();
        match self {
            &mut PoolWorkerHandler::CatchupHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
            &mut PoolWorkerHandler::TransactionHandler(ref mut ch) => ch.process_msg(msg, raw_msg, src_ind),
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
                self.process_reply(&reply, raw_msg);
            }
            _ => {
                warn!("unhandled msg {:?}", msg);
            }
        };
        Ok(None)
    }

    fn process_reply(&mut self, reply: &Reply, raw_msg: &String) {
        let req_id = reply.result.req_id;
        let mut remove = false;
        if let Some(pend_cmd) = self.pending_commands.get_mut(&req_id) {
            pend_cmd.reply_cnt += 1;
            if pend_cmd.reply_cnt == self.f + 1 {
                for &cmd_id in &pend_cmd.cmd_ids {
                    CommandExecutor::instance().send(
                        Command::Ledger(LedgerCommand::SubmitAck(cmd_id, Ok(raw_msg.clone())))).unwrap();
                }
                remove = true;
            }
        }
        if remove {
            self.pending_commands.remove(&req_id);
        }
    }

    fn try_send_request(&mut self, cmd: &String, cmd_id: i32) {
        info!("cmd {:?}", cmd);
        let tmp = SimpleRequest::from_json(cmd).unwrap();
        if self.pending_commands.contains_key(&tmp.req_id) {
            self.pending_commands.get_mut(&tmp.req_id).unwrap().cmd_ids.push(cmd_id);
        } else {
            let pc = CommandProcess {
                cmd_ids: vec!(cmd_id),
                nack_cnt: 0,
                reply_cnt: 0,
            };
            self.pending_commands.insert(tmp.req_id, pc);
            for node in &self.nodes {
                let node: &RemoteNode = node;
                node.send_str(cmd);
            }
        }
    }
}

impl PoolWorker {
    fn connect_to_known_nodes(&mut self, merkle_tree: Option<&MerkleTree>) -> Result<(), PoolError> {
        let merkle_tree: MerkleTree = match merkle_tree {
            Some(mt) => {
                let mt: MerkleTree = mt.clone();
                mt
            }
            None => {
                match self.handler {
                    PoolWorkerHandler::CatchupHandler(ref ch) => ch.merkle_tree.clone(),
                    PoolWorkerHandler::TransactionHandler(_) => return Err(PoolError::InvalidState("Expect catchup state".to_string())),
                }
            }
        };
        let ctx: zmq::Context = zmq::Context::new();
        for gen_txn in &merkle_tree {
            let mut rn: RemoteNode = RemoteNode::new(gen_txn.as_str());
            rn.connect(&ctx);
            rn.zsock.as_ref().unwrap().send("pi".as_bytes(), 0).expect("send ping");
            self.handler.nodes_mut().push(rn);
        }
        self.handler.set_f(PoolWorker::get_f(merkle_tree.count())); //TODO set cnt to connect
        Ok(())
    }

    fn init_catchup(&mut self) -> Result<(), PoolError> {
        let catchup_handler = CatchupHandler {
            f: 0,
            ledger_status_same: 0,
            nodes: Vec::new(),
            new_mt_size: 0,
            new_mt_vote: 0,
            merkle_tree: PoolWorker::_restore_merkle_tree(self.name.as_str())?,
            pending_catchup: None,
        };
        self.handler = PoolWorkerHandler::CatchupHandler(catchup_handler);
        self.connect_to_known_nodes(None)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), PoolError> {
        self.init_catchup()?; //TODO consider error as PoolOpen error

        'zmq_poll_loop: loop {
            trace!("zmq poll loop >>");

            let actions = self.poll_zmq();

            for action in &actions {
                match action {
                    &ZMQLoopAction::Terminate => {
                        //TODO terminate all pending commands?
                        break 'zmq_poll_loop;
                    }
                    &ZMQLoopAction::MessageToProcess(ref msg) => {
                        if let Some(new_mt) = self.handler.process_msg(&msg.message, msg.node_idx)? {
                            self.handler = PoolWorkerHandler::TransactionHandler(TransactionHandler {
                                nodes: Vec::new(),
                                pending_commands: HashMap::new(),
                                f: 0,
                            });
                            self.connect_to_known_nodes(Some(&new_mt))?;
                            CommandExecutor::instance().send(Command::Pool(
                                PoolCommand::OpenAck(self.open_cmd_id, Ok(self.pool_id)))).expect("send ack cmd"); //TODO send only once?
                        }
                    }
                    &ZMQLoopAction::RequestToSend(ref req) => {
                        match self.handler {
                            PoolWorkerHandler::CatchupHandler(_) => panic!("incorrect state"), //FIXME
                            PoolWorkerHandler::TransactionHandler(ref mut handler) => handler.try_send_request(&req.request, req.id),
                        }
                    }
                }
            }

            trace!("zmq poll loop <<");
        }
        info!("zmq poll loop finished");
        Ok(())
    }

    fn poll_zmq(&mut self) -> Vec<ZMQLoopAction> {
        let mut actions: Vec<ZMQLoopAction> = Vec::new();

        let mut poll_items = self.get_zmq_poll_items();
        let r = zmq::poll(poll_items.as_mut_slice(), -1).expect("poll");
        trace!("zmq poll {:?}", r);

        for i in 0..self.handler.nodes().len() {
            if poll_items[1 + i].is_readable() {
                if let Some(msg) = self.handler.nodes()[i].recv_msg().expect("recv msg") {
                    actions.push(ZMQLoopAction::MessageToProcess(MessageToProcess {
                        node_idx: i,
                        message: msg,
                    }));
                }
            }
        }
        if poll_items[0].is_readable() {
            let cmd = self.cmd_sock.recv_multipart(zmq::DONTWAIT).unwrap();
            trace!("cmd {:?}", cmd);
            let cmd_s = String::from_utf8(cmd[0].clone()).expect("non-string command");
            if "exit".eq(cmd_s.as_str()) {
                actions.push(ZMQLoopAction::Terminate);
            } else {
                actions.push(ZMQLoopAction::RequestToSend(RequestToSend {
                    id: LittleEndian::read_i32(cmd[1].as_slice()),
                    request: cmd_s,
                }));
            }
        }
        actions
    }

    fn get_zmq_poll_items(&self) -> Vec<zmq::PollItem> {
        let mut poll_items: Vec<zmq::PollItem> = Vec::new();
        poll_items.push(self.cmd_sock.as_poll_item(zmq::POLLIN));
        for ref node in self.handler.nodes() {
            let s: &zmq::Socket = node.zsock.as_ref().unwrap();
            poll_items.push(s.as_poll_item(zmq::POLLIN));
        }
        poll_items
    }


    fn _restore_merkle_tree(pool_name: &str) -> Result<MerkleTree, PoolError> {
        let mut p = EnvironmentUtils::pool_path(pool_name);
        let mut mt = MerkleTree::from_vec(Vec::new())?;
        //TODO firstly try to deserialize merkle tree
        p.push(pool_name);
        p.set_extension("txn");
        let f = fs::File::open(p)?;
        let reader = io::BufReader::new(&f);
        for line in reader.lines() {
            let line: String = line?;
            mt.append(line)?;
        }
        Ok(mt)
    }

    fn get_f(cnt: usize) -> usize {
        if cnt < 4 {
            return 0
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
                f: 0,
                ledger_status_same: 0,
                nodes: Vec::new(),
                new_mt_size: 0,
                new_mt_vote: 0,
                merkle_tree: MerkleTree::from_vec(Vec::new())?,
                pending_catchup: None,
            }),
        };

        Ok(Pool {
            name: name.to_string(),
            id: pool_id,
            cmd_sock: send_cmd_sock,
            worker: Some(thread::spawn(move || {
                pool_worker.run()
            })),
        })
    }

    pub fn send_tx(&self, cmd_id: i32, json: &str) {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        self.cmd_sock.send_multipart(&[json.as_bytes(), &buf], zmq::DONTWAIT).expect("send to cmd sock");
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        let target = format!("pool{}", self.name);
        info!(target: target.as_str(), "Drop started");
        self.cmd_sock.send("exit".as_bytes(), 0).expect("send exit command"); //TODO
        info!(target: target.as_str(), "Drop wait worker");
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join().unwrap().unwrap();
        info!(target: target.as_str(), "Drop finished");
    }
}

impl From<CryptoError> for PoolError {
    fn from(err: CryptoError) -> PoolError {
        PoolError::InvalidData(err.description().to_string())
    }
}

impl Debug for RemoteNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RemoteNode: {{ public_key: {:?}, verify_key {:?}, zaddr {:?}, zsock is_some {} }}",
               self.public_key, self.verify_key, self.zaddr, self.zsock.is_some())
    }
}

impl RemoteNode {
    fn new(txn: &str) -> RemoteNode {
        let gen_tx = GenTransaction::from_json(txn).expect("RemoteNode parsing");
        RemoteNode::from(gen_tx)
    }

    fn connect(&mut self, ctx: &zmq::Context) {
        let key_pair = zmq::CurveKeyPair::new().expect("create key pair");
        let s = ctx.socket(zmq::SocketType::DEALER).expect("socket for Node");
        s.set_identity(key_pair.public_key.as_bytes()).expect("set identity");
        s.set_curve_secretkey(key_pair.secret_key.as_str()).expect("set secret key");
        s.set_curve_publickey(key_pair.public_key.as_str()).expect("set public key");
        s.set_curve_serverkey(zmq::z85_encode(self.verify_key.as_slice()).unwrap().as_str()).expect("set verify key");
        s.set_linger(0).expect("set linger"); //TODO set correct timeout
        s.connect(self.zaddr.as_str()).expect("connect to Node");
        self.zsock = Some(s);
    }

    fn recv_msg(&self) -> Result<Option<String>, PoolError> {
        impl From<Vec<u8>> for PoolError {
            fn from(_: Vec<u8>) -> Self {
                PoolError::Io(io::Error::from(io::ErrorKind::InvalidData))
            }
        }
        let msg: String = self.zsock.as_ref().unwrap().recv_string(zmq::DONTWAIT)??;
        info!(target: "RemoteNode_recv_msg", "{} {}", self.name, msg);

        match msg.as_ref() {
            "pi" => Ok(None), //TODO send pong
            _ => Ok(Some(msg))
        }
    }

    fn send_str(&self, str: &str) {
        info!("Sending {:?}", str);
        self.zsock.as_ref().unwrap()
            .send_str(str, zmq::DONTWAIT)
            .unwrap();
    }

    fn send_msg(&self, msg: &Message) {
        self.send_str(msg.to_json().unwrap().as_str());
    }
}

impl From<GenTransaction> for RemoteNode {
    fn from(tx: GenTransaction) -> RemoteNode {
        let public_key = tx.dest.as_str().from_base58().expect("dest field in GenTransaction isn't valid");
        RemoteNode {
            verify_key: ED25519::pk_to_curve25519(&public_key),
            public_key: public_key,
            zaddr: format!("tcp://{}:{}", tx.data.client_ip, tx.data.client_port),
            zsock: None,
            name: tx.data.alias,
        }
    }
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            pools: RefCell::new(HashMap::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<&str>) -> Result<(), PoolError> {
        let mut path = EnvironmentUtils::pool_path(name);
        let pool_config = match config {
            Some(config) => PoolConfig::from_json(config)?,
            None => PoolConfig::default_for_name(name)
        };

        if path.as_path().exists() {
            return Err(PoolError::NotCreated("Already created".to_string()));
        }

        fs::create_dir_all(path.as_path())?;

        path.push(name);
        path.set_extension("txn");
        fs::copy(&pool_config.genesis_txn, path.as_path())?;
        path.pop();

        path.push("config");
        path.set_extension("json");
        let mut f: fs::File = fs::File::create(path.as_path())?;
        f.write(pool_config.to_json()?.as_bytes())?;
        f.flush()?;

        // TODO probably create another one file pool.json with pool description,
        // but now there is no info to save (except name witch equal to directory)

        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn open(&self, name: &str, config: Option<&str>) -> Result<i32, PoolError> {
        for pool in self.pools.try_borrow()?.values() {
            if name.eq(pool.name.as_str()) {
                //TODO change error
                return Err(PoolError::InvalidHandle("Already opened".to_string()));
            }
        }

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let new_pool = Pool::new(name, cmd_id)?;
        //FIXME process config: check None (use default), transfer to Pool instance

        self.pools.try_borrow_mut()?.insert(new_pool.id, new_pool);
        return Ok(cmd_id);
    }

    pub fn send_tx(&self, handle: i32, json: &str) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.pools.try_borrow()?
            .get(&handle).ok_or(PoolError::InvalidHandle("No pool with requested handle".to_string()))?
            .send_tx(cmd_id, json);
        Ok(cmd_id)
    }

    pub fn close(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn refresh(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, PoolError> {
        self.pools.try_borrow()?.get(&handle).map_or(
            Err(PoolError::InvalidHandle("Doesn't exists".to_string())),
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
    fn pool_send_tx_works() {
        let name = "test";
        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
        let inproc_sock_name: String = format!("inproc://pool_{}", name);
        recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
        send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
        let pool = Pool {
            worker: Some(thread::spawn(|| { Ok(()) })),
            name: name.to_string(),
            id: 0,
            cmd_sock: send_cmd_sock,
        };
        let test_data = "str_instead_of_tx_json";
        pool.send_tx(0, test_data);
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

    #[test]
    fn pool_worker_restore_merkle_tree_works_from_genesis_txns() {
        let txns_src = format!("{}\n{}\n{}\n{}\n",
                               "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"192.168.1.35\",\"client_port\":9702,\"node_ip\":\"192.168.1.35\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"192.168.1.35\",\"client_port\":9704,\"node_ip\":\"192.168.1.35\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"192.168.1.35\",\"client_port\":9706,\"node_ip\":\"192.168.1.35\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
                               "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"192.168.1.35\",\"client_port\":9708,\"node_ip\":\"192.168.1.35\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}");
        let pool_name = "test";
        let mut path = ::utils::environment::EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns_src.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let merkle_tree = PoolWorker::_restore_merkle_tree("test").unwrap();

        assert_eq!(merkle_tree.count(), 4, "test restored MT size");
        assert_eq!(merkle_tree.root_hash_hex(), "1285070cf01debc1155cef8dfd5ba54c05abb919a4c08c8632b079fb1e1e5e7c", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_connect_to_known_nodes_works() {
        let mut pw: PoolWorker = Default::default();
        let (gt, handle) = nodes_emulator::start();
        let mut merkle_tree: MerkleTree = MerkleTree::from_vec(Vec::new()).unwrap();
        merkle_tree.append(gt.to_json().unwrap()).unwrap();

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
            pw.poll_zmq()
        });
        send_cmd_sock.send_str("exit", zmq::DONTWAIT).expect("send");
        let actions: Vec<ZMQLoopAction> = handle.join().unwrap();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], ZMQLoopAction::Terminate);
    }

    #[test]
    fn pool_worker_get_zmq_poll_items_works() {
        let pw: PoolWorker = Default::default();

        let poll_items = pw.get_zmq_poll_items();

        assert_eq!(poll_items.len(), pw.handler.nodes().len() + 1 );
        //TODO compare poll items
    }

    #[test]
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
        let pc = super::types::CommandProcess {
            cmd_ids: Vec::new(),
            reply_cnt: th.f,
            nack_cnt: 0,
        };
        let req_id = 1;
        th.pending_commands.insert(req_id, pc);
        let reply = super::types::Reply {
            result: super::types::Response {
                req_id: req_id,
            },
        };

        th.process_reply(&reply, &"".to_string());

        assert_eq!(th.pending_commands.len(), 0);
    }

    #[test]
    fn transaction_handler_try_send_request_works_for_new_req_id() {
        let mut th: TransactionHandler = Default::default();

        let req_id = 2;
        let cmd_id = 1;
        let req = SimpleRequest {
            req_id: req_id,
        };
        let cmd = req.to_json().unwrap();

        th.try_send_request(&cmd, cmd_id);

        assert_eq!(th.pending_commands.len(), 1);
        let pending_cmd = th.pending_commands.get(&req_id).unwrap();
        let exp_command_process = CommandProcess {
            nack_cnt: 0,
            reply_cnt: 0,
            cmd_ids: vec!(cmd_id),
        };
        assert_eq!(pending_cmd, &exp_command_process);
    }

    #[test]
    fn catchup_handler_start_catchup_works() {
        let mut ch: CatchupHandler = Default::default();
        let (gt, handle) = nodes_emulator::start();
        ch.merkle_tree.append(gt.to_json().unwrap()).unwrap();
        let mut rn: RemoteNode = RemoteNode::from(gt);
        rn.connect(&zmq::Context::new());
        ch.nodes.push(rn);
        ch.new_mt_size = 2;

        ch.start_catchup();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        let expected_resp: CatchupReq = CatchupReq {
            ledgerType: 0,
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
        let mut rn: RemoteNode = RemoteNode::from(gt);
        let ctx = zmq::Context::new();
        rn.connect(&ctx);
        rn.zsock.as_ref().expect("sock").send_str("pi", zmq::DONTWAIT).expect("send");
        rn.zsock.as_ref().expect("sock").poll(zmq::POLLIN, 100).expect("poll");
        assert_eq!("po", rn.zsock.as_ref().expect("sock").recv_string(zmq::DONTWAIT).expect("recv").expect("string").as_str());
        handle.join().expect("join");
    }

    mod nodes_emulator {
        extern crate sodiumoxide;

        use services::pool::rust_base58::ToBase58;
        use std::thread;
        use super::*;

        pub fn start() -> (GenTransaction, thread::JoinHandle<Vec<String>>) {
            let (pk, sk) = sodiumoxide::crypto::sign::ed25519::gen_keypair();
            let pkc = ED25519::pk_to_curve25519(&Vec::from(&pk.0 as &[u8]));
            let skc = ED25519::sk_to_curve25519(&Vec::from(&sk.0 as &[u8]));
            let ctx = zmq::Context::new();
            let s: zmq::Socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();
            let gt = GenTransaction {
                identifier: "".to_string(),
                data: NodeData {
                    alias: "n1".to_string(),
                    services: Vec::new(),
                    client_port: 9701,
                    client_ip: "0.0.0.0".to_string(),
                    node_ip: "".to_string(),
                    node_port: 0,
                },
                txn_id: "".to_string(),
                txn_type: "0".to_string(),
                dest: (&pk.0 as &[u8]).to_base58(),
            };
            let addr = format!("tcp://{}:{}", gt.data.client_ip, gt.data.client_port);
            s.set_curve_publickey(zmq::z85_encode(pkc.as_slice()).unwrap().as_str()).expect("set public key");
            s.set_curve_secretkey(zmq::z85_encode(skc.as_slice()).unwrap().as_str()).expect("set secret key");
            s.set_curve_server(true).expect("set curve server");
            s.bind(addr.as_str()).expect("bind");
            let handle = thread::spawn(move || {
                let mut received_msgs: Vec<String> = Vec::new();
                if s.poll(zmq::POLLIN, 100).expect("poll") == 1 {
                    let v = s.recv_multipart(zmq::DONTWAIT).expect("recv mulp");
                    s.send_multipart(&[v[0].as_slice(), "po".as_bytes()], zmq::DONTWAIT).expect("send mulp");
                    received_msgs.push(String::from_utf8(v[1].clone()).unwrap());
                }
                received_msgs
            });
            (gt, handle)
        }
    }
}
