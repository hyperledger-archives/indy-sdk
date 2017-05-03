mod types;

extern crate base64;
extern crate rust_base58;
extern crate serde_json;
extern crate zmq;

use self::rust_base58::FromBase58;
use std::cell::RefCell;
use std::collections::{HashMap, BinaryHeap};
use std::{cmp, fmt, fs, io, thread, usize};
use std::fmt::Debug;
use std::io::{BufRead, Write};

use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use self::types::*;
use services::ledger::merkletree::merkletree::MerkleTree;
use utils::crypto::Ed25519ToCurve25519;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;

pub struct PoolService {
    pools: RefCell<HashMap<i32, Pool>>,
}

struct Pool {
    name: String,
    id: i32,
    send_sock: zmq::Socket,
    worker: Option<thread::JoinHandle<()>>,
}

struct PoolWorker {
    cmd_sock: zmq::Socket,
    open_cmd_id: i32,
    pool_id: i32,
    nodes: Vec<RemoteNode>,
    merkle_tree: MerkleTree,
    new_mt_size: usize,
    new_mt_vote: usize,
    f: usize,
    pending_catchup: Option<CatchUpProcess>,
    name: String,
}

impl PoolWorker {
    fn restore_merkle_tree(&mut self) {
        let mut p = EnvironmentUtils::pool_path(self.name.as_str());
        //TODO firstly try to deserialize merkle tree
        p.push(&self.name);
        p.set_extension("txn");
        let f = fs::File::open(p).expect("open file");
        let reader = io::BufReader::new(&f);
        for line in reader.lines() {
            let line: String = line.expect("read transaction line");
            self.merkle_tree.append(line);
        }
    }

    fn connect_to_known_nodes(&mut self) {
        if self.merkle_tree.is_empty() {
            self.restore_merkle_tree();
        }
        let ctx: zmq::Context = zmq::Context::new();
        for gen_txn in &self.merkle_tree {
            let mut rn: RemoteNode = RemoteNode::new(gen_txn.as_str());
            rn.connect(&ctx);
            rn.zsock.as_ref().unwrap().send("pi".as_bytes(), 0).expect("send ping");
            self.nodes.push(rn);
        }
        self.f = self.merkle_tree.count(); //FIXME
        info!("merkle tree {:?}", self.merkle_tree);
    }

    fn start_catchup(&mut self) {
        assert!(self.pending_catchup.is_none());
        self.pending_catchup = Some(CatchUpProcess {
            merkle_tree: self.merkle_tree.clone(),
            pending_reps: BinaryHeap::new(),
        });
        let node_cnt = self.nodes.len();
        assert!(self.merkle_tree.count() == node_cnt);

        let cnt_to_catchup = self.new_mt_size - self.merkle_tree.count();
        assert!(cnt_to_catchup > 0);
        let portion = (cnt_to_catchup + node_cnt - 1) / node_cnt; //TODO check standard round up div
        let mut catchup_req = CatchupReq {
            ledgerType: 0,
            seqNoStart: node_cnt + 1,
            seqNoEnd: node_cnt + 1 + portion - 1,
            catchupTill: self.new_mt_size,
        };
        for node in &self.nodes {
            node.send_msg(&Message::CatchupReq(catchup_req.clone()));
            catchup_req.seqNoStart += portion;
            catchup_req.seqNoEnd = cmp::min(catchup_req.seqNoStart + portion - 1,
                                            catchup_req.catchupTill);
        }
    }

    fn process_catchup(&mut self, catchup: CatchupRep) {
        trace!("append {:?}", catchup);
        let catchup_finished = {
            let mut process = self.pending_catchup.as_mut().unwrap();
            process.pending_reps.push(catchup);
            while !process.pending_reps.is_empty()
                && process.pending_reps.peek().unwrap().min_tx() - 1 == process.merkle_tree.count() {
                let mut first_resp = process.pending_reps.pop().unwrap();
                while !first_resp.txns.is_empty() {
                    let key = first_resp.min_tx().to_string();
                    let new_gen_tx = serde_json::to_string(&first_resp.txns.remove(&key)).unwrap();
                    trace!("append to tree {}", new_gen_tx);
                    process.merkle_tree.append(
                        new_gen_tx
                    );
                }
            }
            trace!("updated mt hash {}, tree {:?}", base64::encode(process.merkle_tree.root_hash()), process.merkle_tree);
            if &process.merkle_tree.count() == &self.new_mt_size {
                //TODO check also root hash?
                true
            } else {
                false
            }
        };
        if catchup_finished {
            self.finish_catchup();
        }
    }

    fn finish_catchup(&mut self) {
        self.merkle_tree = self.pending_catchup.take().unwrap().merkle_tree;
        self.nodes.clear();
        self.connect_to_known_nodes();
    }

    fn process_msg(&mut self, msg: &String, src_ind: usize) {
        let resp: Option<String> = if msg.eq("po") {
            //sending ledger status
            //TODO not send ledger status directly as response on ping, wait pongs from all nodes?
            let ls: LedgerStatus = LedgerStatus {
                txnSeqNo: self.nodes.len(),
                merkleRoot: base64::encode(self.merkle_tree.root_hash()),
                ..Default::default()
            };
            let msg: Message = Message::LedgerStatus(ls);
            Some(serde_json::to_string(&msg).unwrap())
        } else {
            let msg: Message = serde_json::from_str(msg.as_str()).unwrap();
            match msg {
                Message::LedgerStatus(ledger_status) => {
                    //TODO nothing?
                }
                Message::ConsistencyProof(cons_proof) => {
                    trace!("{:?}", cons_proof);
                    if cons_proof.seqNoStart == self.merkle_tree.count()
                        && cons_proof.seqNoEnd > self.merkle_tree.count() {
                        self.new_mt_size = cmp::max(cons_proof.seqNoEnd, self.new_mt_size);
                        self.new_mt_vote += 1;
                        debug!("merkle tree expected size now {}", self.new_mt_size);
                    }
                    if self.new_mt_vote == self.f {
                        self.start_catchup();
                    }
                }
                Message::CatchupRep(catchup) => {
                    self.process_catchup(catchup);
                }
                _ => {
                    info!("unhandled msg {:?}", msg);
                }
            }
            None
        };
        if resp.is_some() {
            self.nodes[src_ind].zsock.as_ref().unwrap().send(resp.unwrap().as_bytes(), zmq::DONTWAIT).expect("send resp msg");
        }
    }


    pub fn run(&mut self) {
        self.connect_to_known_nodes();

        CommandExecutor::instance().send(Command::Pool(
            PoolCommand::OpenAck(self.open_cmd_id, Ok(self.pool_id)))).expect("send ack cmd"); //TODO send only after catch-up?

        loop {
            trace!("zmq poll loop >>");
            let mut msgs_to_handle: Vec<(String, usize)> = Vec::new();
            {
                let mut ss_to_poll: Vec<zmq::PollItem> = Vec::new();
                ss_to_poll.push(self.cmd_sock.as_poll_item(zmq::POLLIN));
                for ref node in &self.nodes {
                    let s: &zmq::Socket = node.zsock.as_ref().unwrap();
                    ss_to_poll.push(s.as_poll_item(zmq::POLLIN));
                }

                let r = zmq::poll(ss_to_poll.as_mut_slice(), -1).expect("poll");
                trace!("zmq poll loop ... {:?}", r);
                for i in 0..self.nodes.len() {
                    if ss_to_poll[1 + i].is_readable() {
                        let msg: Option<String> = self.nodes[i].recv_msg().expect("recv msg");
                        if msg.is_some() {
                            msgs_to_handle.push((msg.unwrap(), i));
                        }
                    }
                }
                if ss_to_poll[0].is_readable() {
                    let cmd = self.cmd_sock.recv_string(zmq::DONTWAIT);
                    trace!("cmd {:?}", cmd);
                    if cmd.is_ok() {
                        let cmd = cmd.unwrap().expect("non-string command");
                        if "exit".eq(cmd.as_str()) {
                            break;
                        } else {
                            msgs_to_handle.push((cmd, usize::MAX));
                        }
                    }
                }
            }

            for &(ref msg, rn_ind) in &msgs_to_handle {
                if rn_ind == usize::MAX {
                    for node in &self.nodes {
                        let node: &RemoteNode = node;
                        node.send_str(msg);
                    }
                } else {
                    self.process_msg(msg, rn_ind);
                }
            }

            trace!("zmq poll loop <<");
        }
        info!("zmq poll loop finished");
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
            nodes: Vec::new(),
            merkle_tree: MerkleTree::from_vec(Vec::new()),
            new_mt_size: 0,
            new_mt_vote: 0,
            f: 0,
            pending_catchup: None,
            name: name.to_string(),
        };

        Ok(Pool {
            name: name.to_string(),
            id: pool_id,
            send_sock: send_cmd_sock,
            worker: Some(thread::spawn(move || {
                pool_worker.run();
            })),
        })
    }

    pub fn send_tx(&self, cmd_id: i32, json: &str) {
        self.send_sock.send_str(json, 0).expect("send to cmd sock");
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        let target = format!("pool{}", self.name);
        info!(target: target.as_str(), "Drop started");
        self.send_sock.send("exit".as_bytes(), 0).expect("send exit command"); //TODO
        info!(target: target.as_str(), "Drop wait worker");
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join().unwrap();
        info!(target: target.as_str(), "Drop finished");
    }
}

struct RemoteNode {
    name: String,
    public_key: Vec<u8>,
    verify_key: Vec<u8>,
    zaddr: String,
    zsock: Option<zmq::Socket>,
}

impl Debug for RemoteNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RemoteNode: {{ public_key: {:?}, verify_key {:?}, zaddr {:?}, zsock is_some {} }}",
               self.public_key, self.verify_key, self.zaddr, self.zsock.is_some())
    }
}

impl RemoteNode {
    fn new(txn: &str) -> RemoteNode {
        let gen_tx: GenTransaction = serde_json::from_str(txn).expect("RemoteNode parsing");
        RemoteNode::from(gen_tx)
    }

    fn connect(&mut self, ctx: &zmq::Context) {
        let key_pair = zmq::CurveKeyPair::new().expect("create key pair");
        let s = ctx.socket(zmq::SocketType::DEALER).expect("socket for Node");
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
        self.send_str(serde_json::to_string(msg).unwrap().as_str());
    }
}

impl From<GenTransaction> for RemoteNode {
    fn from(tx: GenTransaction) -> RemoteNode {
        let public_key = tx.dest.as_str().from_base58().expect("dest field in GenTransaction isn't valid");
        RemoteNode {
            verify_key: Ed25519ToCurve25519::crypto_sign_ed25519_pk_to_curve25519(&public_key),
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
        let pool_config: PoolConfig = match config {
            Some(config) => serde_json::from_str(config)?,
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
        f.write(serde_json::to_string(&pool_config)?.as_bytes())?;
        f.flush()?;

        // TODO probably create another one file pool.json with pool description,
        // but now there is no info to save (except name witch equal to directory)

        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn open(&self, name: &str, config: Option<&str>) -> Result<i32, PoolError> {
        for pool in self.pools.borrow().values() {
            if name.eq(pool.name.as_str()) {
                //TODO change error
                return Err(PoolError::InvalidHandle("Already opened".to_string()));
            }
        }

        let cmd_id: i32 = SequenceUtils::get_next_id();
        let new_pool = Pool::new(name, cmd_id)?;
        //FIXME process config: check None (use default), transfer to Pool instance

        self.pools.borrow_mut().insert(new_pool.id, new_pool);
        return Ok(cmd_id);
    }

    pub fn send_tx(&self, handle: i32, json: &str) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.pools.borrow().get(&handle).unwrap().send_tx(cmd_id, json);
        Ok(cmd_id)
    }

    pub fn close(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn refresh(&self, handle: i32) -> Result<(), PoolError> {
        unimplemented!()
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, PoolError> {
        self.pools.borrow().get(&handle).map_or(
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
            worker: Some(thread::spawn(|| {})),
            name: name.to_string(),
            id: 0,
            send_sock: send_cmd_sock,
        };
        let test_data = "str_instead_of_tx_json";
        pool.send_tx(0, test_data);
        assert_eq!(recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), test_data);
    }

    impl Default for PoolWorker {
        fn default() -> Self {
            PoolWorker {
                merkle_tree: MerkleTree::from_vec(Vec::new()),
                pool_id: 0,
                cmd_sock: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
                f: 0,
                pending_catchup: None,
                open_cmd_id: 0,
                nodes: Vec::new(),
                new_mt_size: 0,
                new_mt_vote: 0,
                name: "".to_string(),
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

        let mut pw: PoolWorker = PoolWorker {
            name: pool_name.to_string(),
            ..Default::default()
        };
        pw.restore_merkle_tree();

        assert_eq!(pw.merkle_tree.count(), 4, "test restored MT size");
        assert_eq!(pw.merkle_tree.root_hash_hex(), "1285070cf01debc1155cef8dfd5ba54c05abb919a4c08c8632b079fb1e1e5e7c", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_connect_to_known_nodes_works() {
        let mut pw: PoolWorker = Default::default();
        let (gt, handle) = nodes_emulator::start();
        pw.merkle_tree.append(serde_json::to_string(&gt).unwrap());

        pw.connect_to_known_nodes();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        assert_eq!("pi", emulator_msgs[0]);
    }

    #[test]
    fn pool_worker_start_catchup_works() {
        let mut pw: PoolWorker = Default::default();
        let (gt, handle) = nodes_emulator::start();
        pw.merkle_tree.append(serde_json::to_string(&gt).unwrap());
        let mut rn: RemoteNode = RemoteNode::from(gt);
        rn.connect(&zmq::Context::new());
        pw.nodes.push(rn);
        pw.new_mt_size = 2;

        pw.start_catchup();

        let emulator_msgs: Vec<String> = handle.join().unwrap();
        assert_eq!(1, emulator_msgs.len());
        let expected_resp: CatchupReq = CatchupReq {
            ledgerType: 0,
            seqNoStart: 2,
            seqNoEnd: 2,
            catchupTill: 2,
        };
        let act_resp: CatchupReq = serde_json::from_str(emulator_msgs[0].as_str()).unwrap();
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
            let pkc = Ed25519ToCurve25519::crypto_sign_ed25519_pk_to_curve25519(&Vec::from(&pk.0 as &[u8]));
            let skc = Ed25519ToCurve25519::crypto_sign_ed25519_sk_to_curve25519(&Vec::from(&sk.0 as &[u8]));
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
