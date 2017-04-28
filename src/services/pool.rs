extern crate base64;
extern crate rust_base58;
extern crate serde_json;
extern crate zmq;

use self::rust_base58::FromBase58;
use std::cell::RefCell;
use std::collections::{HashMap, BinaryHeap};
use std::{cmp, fmt, fs, io, thread};
use std::fmt::Debug;
use std::io::{BufRead, Write};

use commands::{Command, CommandExecutor};
use commands::pool::PoolCommand;
use errors::pool::PoolError;
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
}


struct CatchUpProcess {
    merkle_tree: MerkleTree,
    pending_reps: BinaryHeap<CatchupRep>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct LedgerStatus {
    txnSeqNo: usize,
    merkleRoot: String,
    ledgerType: u8,
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
struct ConsistencyProof {
    //TODO almost all fields Option<> or find better approach
    seqNoEnd: usize,
    seqNoStart: usize,
    ledgerType: usize,
    hashes: Vec<String>,
    oldMerkleRoot: String,
    newMerkleRoot: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CatchupReq {
    ledgerType: usize,
    seqNoStart: usize,
    seqNoEnd: usize,
    catchupTill: usize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CatchupRep {
    ledgerType: usize,
    consProof: Vec<String>,
    txns: HashMap<String, GenTransaction>,
}

impl CatchupRep {
    fn min_tx(&self) -> usize {
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
enum Message {
    #[serde(rename = "CONSISTENCY_PROOF")]
    ConsistencyProof(ConsistencyProof),
    #[serde(rename = "LEDGER_STATUS")]
    LedgerStatus(LedgerStatus),
    #[serde(rename = "CATCHUP_REQ")]
    CatchupReq(CatchupReq),
    #[serde(rename = "CATCHUP_REP")]
    CatchupRep(CatchupRep),
}

impl PoolWorker {
    fn restore_merkle_tree(&mut self) {
        let f = fs::File::open("pool_transactions_sandbox").expect("open file"); //FIXME use file for the pool
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
                        if "exit".eq(cmd.unwrap().expect("non-string command").as_str()) {
                            break;
                        }
                    }
                }
            }

            for &(ref msg, rn_ind) in &msgs_to_handle {
                self.process_msg(msg, rn_ind);
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

#[derive(Serialize, Deserialize)]
struct PoolConfig {
    genesis_txn: String
}

impl PoolConfig {
    fn default(name: &str) -> PoolConfig {
        let mut txn = name.to_string();
        txn += ".txn";
        PoolConfig { genesis_txn: txn }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct NodeData {
    alias: String,
    client_ip: String,
    client_port: u32,
    node_ip: String,
    node_port: u32,
    services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct GenTransaction {
    data: NodeData,
    dest: String,
    identifier: String,
    #[serde(rename = "txnId")]
    txn_id: String,
    #[serde(rename = "type")]
    txn_type: String,
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

    fn send_msg(&self, msg: &Message) {
        info!("Sending {:?}", msg);
        self.zsock.as_ref().unwrap()
            .send_str(serde_json::to_string(msg).unwrap().as_str(), zmq::DONTWAIT)
            .unwrap();
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
            None => PoolConfig::default(name)
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
    fn pool_service_can_be_created() {
        let pool_service = PoolService::new();
        assert!(true, "No crashes on PoolService::new");
    }

    #[test]
    fn pool_service_can_be_dropped() {
        fn drop_test() {
            let pool_service = PoolService::new();
        }

        drop_test();
        assert!(true, "No crashes on PoolService::drop");
    }
}