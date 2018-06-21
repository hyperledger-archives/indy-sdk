mod types;
mod catchup;
//mod transaction_handler;
mod state_proof;
//mod pool_worker;

mod pool;
mod merkle_tree_factory;
mod networker;
mod commander;
mod events;
mod request_handler;

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


use self::byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;

use api::ledger::{CustomFree, CustomTransactionParser};
use errors::pool::PoolError;
use errors::common::CommonError;
use self::types::*;
use utils::crypto::box_::CryptoBox;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;
use services::pool::rust_base58::FromBase58;
use std::sync::Mutex;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use services::pool::pool::Pool;
use services::pool::networker::ZMQNetworker;
use services::pool::request_handler::RequestHandlerImpl;
use self::zmq::Socket;

lazy_static! {
    static ref REGISTERED_SP_PARSERS: Mutex<HashMap<String, (CustomTransactionParser, CustomFree)>> = Mutex::new(HashMap::new());
}

pub type PoolWorker = Pool<ZMQNetworker, RequestHandlerImpl<ZMQNetworker>>;

pub struct PoolService {
    open_pools: RefCell<HashMap<i32, (PoolWorker, zmq::Socket)>>,
    pending_pools: RefCell<HashMap<i32, (PoolWorker, zmq::Socket)>>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            open_pools: RefCell::new(HashMap::new()),
            pending_pools: RefCell::new(HashMap::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<&str>) -> Result<(), PoolError> {

        //TODO: initialize all state machines
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
        //TODO: move parse to correct place
        let mt = merkle_tree_factory::from_file(&pool_config.genesis_txn)?;
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
        for &(ref pool, _) in self.open_pools.try_borrow().map_err(CommonError::from)?.values() {
            if pool.get_name().eq(name) {
                return Err(PoolError::CommonError(CommonError::InvalidState("Can't delete pool config - pool is open now".to_string())));
            }
        }
        let path = EnvironmentUtils::pool_path(name);
        fs::remove_dir_all(path).map_err(PoolError::from)
    }

    pub fn open(&self, name: &str, _config: Option<&str>) -> Result<i32, PoolError> {
        for &(ref pool, _) in self.open_pools.try_borrow().map_err(CommonError::from)?.values() {
            if name.eq(pool.get_name()) {
                //TODO change error
                return Err(PoolError::InvalidHandle("Pool with same name already opened".to_string()));
            }
        }

        let pool_handle: i32 = SequenceUtils::get_next_id();
        let mut new_pool = Pool::new(name, pool_handle);
        //FIXME process config: check None (use default), transfer to Pool instance

        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let inproc_sock_name: String = format!("inproc://pool_{}", name);

        recv_cmd_sock.bind(inproc_sock_name.as_str())?;

        send_cmd_sock.connect(inproc_sock_name.as_str())?;

        new_pool.work(recv_cmd_sock);
        self._send_msg(pool_handle, "connect", &send_cmd_sock);

        self.pending_pools.try_borrow_mut().map_err(CommonError::from)?.insert(new_pool.get_id(), (new_pool, send_cmd_sock));
        return Ok(pool_handle);
    }

    pub fn add_open_pool(&self, pool_id: i32) -> Result<i32, PoolError> {
        let (pool, socket) = self.pending_pools.try_borrow_mut().map_err(CommonError::from)?
            .remove(&pool_id)
            .ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", pool_id)))?;

        self.open_pools.try_borrow_mut().map_err(CommonError::from)?.insert(pool_id, (pool, socket));

        Ok(pool_id)
    }

    pub fn send_tx(&self, handle: i32, msg: &str) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.open_pools.try_borrow().map_err(CommonError::from)?
            .get(&handle)
            .map(|&(_, ref socket)| {
                self._send_msg(cmd_id, msg, socket);
            }).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?;
        Ok(cmd_id)
    }

    pub fn register_sp_parser(txn_type: &str,
                              parser: CustomTransactionParser, free: CustomFree) -> Result<(), PoolError> {
        if events::REQUESTS_FOR_STATE_PROOFS.contains(&txn_type) {
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
            .remove(&handle).map(|(_, ref socket)| {
                self._send_msg(cmd_id, "exit", socket);
            }).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?;
        Ok(cmd_id)
    }

    pub fn refresh(&self, handle: i32) -> Result<i32, PoolError> {
        let cmd_id: i32 = SequenceUtils::get_next_id();
        self.open_pools.try_borrow_mut().map_err(CommonError::from)?
            .get(&handle).map(|&(_, ref socket)| {
                self._send_msg(cmd_id, "refresh", socket);
            }).ok_or(PoolError::InvalidHandle(format!("No pool with requested handle {}", handle)))?;
        Ok(cmd_id)
    }

    fn _send_msg(&self, cmd_id: i32, msg: &str, socket: &Socket) {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        socket.send_multipart(&[msg.as_bytes(), &buf], zmq::DONTWAIT).expect("FIXME");
    }

    pub fn list(&self) -> Result<Vec<serde_json::Value>, PoolError> {
        let mut pool = Vec::new();

        let pool_home_path = EnvironmentUtils::pool_home_path();
        for entry in fs::read_dir(pool_home_path)? {
            let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue; };
            if let Some(pool_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                let json = json!({"pool":pool_name.to_owned()});
                pool.push(json);
            }
        }

        Ok(pool)
    }

    pub fn get_pool_name(&self, handle: i32) -> Result<String, PoolError> {
        self.open_pools.try_borrow().map_err(CommonError::from)?.get(&handle).map_or(
            Err(PoolError::InvalidHandle(format!("Pool doesn't exists for handle {}", handle))),
            |&(ref pool, _)| Ok(pool.get_name().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::test::TestUtils;
    use domain::ledger::request::ProtocolVersion;
    use std::thread;
    use services::ledger::merkletree::merkletree::MerkleTree;
    use time::Duration;
    use std::marker::PhantomData;

    pub const NODE1: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"10.0.0.2","client_port":9702,"node_ip":"10.0.0.2","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"},"metadata":{"from":"Th7MpTaRZVRYnPiabds81Y"},"type":"0"},"txnMetadata":{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"},"ver":"1"}"#;
    const TEST_PROTOCOL_VERSION: usize = 2;

    fn _set_protocol_version(version: usize) {
        ProtocolVersion::set(version);
    }

    mod pool_service {
        use super::*;
        use std::path;
        use std::marker::PhantomData;
        use api::ErrorCode;
        use std::os::raw::c_char;

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
            ps.open_pools.borrow_mut().insert(pool_id, (Pool::new("", pool_id), send_soc));
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
            ps.open_pools.borrow_mut().insert(pool_id, (Pool::new("", pool_id), send_soc));
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

            let pool = Pool::new(pool_name, pool_id);
            ps.open_pools.borrow_mut().insert(pool_id, (pool, send_cmd_sock));

            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            let res = ps.delete(pool_name);
            assert_match!(Err(PoolError::CommonError(CommonError::InvalidState(_))), res);
            assert!(path.exists());
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
            let pool = Pool::new(name, 0);
            let ps = PoolService::new();
            ps.open_pools.borrow_mut().insert(-1, (pool, send_cmd_sock));
            let test_data = "str_instead_of_tx_json";
            ps.send_tx(-1, test_data).unwrap();
            assert_eq!(recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), test_data);
        }

        #[test]
        fn pool_get_pool_name_works() {
            TestUtils::cleanup_storage();
            let name = "test";
            let ps = PoolService::new();
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let pool = Pool::new(name, 0);
            ps.open_pools.borrow_mut().insert(-1, (pool, send_cmd_sock));
            assert_eq!(ps.get_pool_name(-1).unwrap(), name);
        }

        #[test]
        fn pool_get_pool_name_works_for_invalid_handle() {
            TestUtils::cleanup_storage();
            let ps = PoolService::new();
            assert_match!(Err(PoolError::InvalidHandle(_)), ps.get_pool_name(-1));
        }

        #[test]
        fn pool_send_tx_works_for_invalid_handle() {
            TestUtils::cleanup_storage();
            let ps = PoolService::new();
            assert_match!(Err(PoolError::InvalidHandle(_)), ps.send_tx(-1, "txn"));
        }

        #[test]
        fn pool_close_works_for_invalid_handle() {
            TestUtils::cleanup_storage();
            let ps = PoolService::new();
            assert_match!(Err(PoolError::InvalidHandle(_)), ps.close(-1));
        }

        #[test]
        fn pool_refresh_works_for_invalid_handle() {
            TestUtils::cleanup_storage();
            let ps = PoolService::new();
            assert_match!(Err(PoolError::InvalidHandle(_)), ps.refresh(-1));
        }

        #[test]
        fn pool_register_sp_parser_works() {
            TestUtils::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            extern fn test_sp(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            extern fn test_free(data: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
        }

        #[test]
        fn pool_get_sp_parser_works() {
            TestUtils::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            extern fn test_sp(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            extern fn test_free(data: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
            PoolService::get_sp_parser("test").unwrap();
        }

        #[test]
        fn pool_get_sp_parser_works_for_invalid_name() {
            TestUtils::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            assert_eq!(None, PoolService::get_sp_parser("test"));
        }

        #[test]
        pub fn pool_add_open_pool_works() {
            TestUtils::cleanup_storage();
            let name = "test";
            let ps = PoolService::new();
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let pool = Pool::new(name, 0);
            ps.pending_pools.borrow_mut().insert(-1, (pool, send_cmd_sock));
            assert_match!(Ok(-1), ps.add_open_pool(-1));
        }

        #[test]
        pub fn pool_add_open_pool_works_for_no_pending_pool() {
            TestUtils::cleanup_storage();
            let ps = PoolService::new();
            assert_match!(Err(PoolError::InvalidHandle(_)), ps.add_open_pool(-1));
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
            let ps = PoolService::new();

            let pool_name = "pool_drop_works";
            let gen_txn = NODE1;

            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let inproc_sock_name: String = format!("inproc://pool_{}", pool_name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();

            // create minimal fs config stub before Pool::new()
            let mut pool_path = EnvironmentUtils::pool_path(pool_name);
            fs::create_dir_all(&pool_path).unwrap();
            pool_path.push(pool_name);
            pool_path.set_extension("txn");
            let mut file = fs::File::create(pool_path).unwrap();
            file.write(&gen_txn.as_bytes()).unwrap();

            let mut pool = Pool::new(pool_name, -1);
            pool.work(recv_cmd_sock);
            ps.open_pools.borrow_mut().insert(-1, (pool, send_cmd_sock));
            thread::sleep(time::Duration::from_secs(1));
            ps.close(-1).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }

        drop_test();
        TestUtils::cleanup_storage();
    }
//
//    impl Default for PoolWorker {
//        fn default() -> Self {
//            PoolWorker {
//                pool_id: 0,
//                cmd_sock: zmq::Context::new().socket(zmq::SocketType::PAIR).unwrap(),
//                open_cmd_id: 0,
//                name: "".to_string(),
//                handler: PoolWorkerHandler::CatchupHandler(CatchupHandler {
//                    timeout: time::now_utc().add(Duration::seconds(2)),
//                    ..Default::default()
//                }),
//            }
//        }
//    }

//    #[test]
//    fn pool_worker_get_zmq_poll_items_works() {
//        TestUtils::cleanup_storage();
//
//        let pw: PoolWorker = Default::default();
//
//        let poll_items = pw.get_zmq_poll_items().unwrap();
//
//        assert_eq!(poll_items.len(), pw.handler.nodes().len() + 1);
//        //TODO compare poll items
//    }

//    #[test]
//    fn catchup_handler_start_catchup_works() {
//        TestUtils::cleanup_storage();
//
//        let mut ch: CatchupHandler = Default::default();
//        let (gt, handle) = nodes_emulator::start();
//        ch.merkle_tree.append(rmp_serde::to_vec_named(&gt).unwrap()).unwrap();
//        let mut rn: RemoteNode = RemoteNode::new(&gt).unwrap();
//        rn.connect(&zmq::Context::new(), &zmq::CurveKeyPair::new().unwrap()).unwrap();
//        ch.nodes.push(rn);
//        ch.target_mt_size = 2;
//
//        ch.start_catchup().unwrap();
//
//        let emulator_msgs: Vec<String> = handle.join().unwrap();
//        assert_eq!(1, emulator_msgs.len());
//        let expected_resp: CatchupReq = CatchupReq {
//            ledgerId: 0,
//            seqNoStart: 2,
//            seqNoEnd: 2,
//            catchupTill: 2,
//        };
//        let act_resp = CatchupReq::from_json(emulator_msgs[0].as_str()).unwrap();
//        assert_eq!(expected_resp, act_resp);
//    }

//    #[test]
//    fn remote_node_connect_works_and_can_ping_pong() {
//        TestUtils::cleanup_storage();
//
//        let (gt, handle) = nodes_emulator::start();
//        let mut rn: RemoteNode = RemoteNode::new(&gt).unwrap();
//        let ctx = zmq::Context::new();
//        rn.connect(&ctx, &zmq::CurveKeyPair::new().unwrap()).unwrap();
//        rn.send_str("pi").expect("send");
//        rn.zsock.as_ref().expect("sock").poll(zmq::POLLIN, nodes_emulator::POLL_TIMEOUT).expect("poll");
//        assert_eq!("po", rn.zsock.as_ref().expect("sock").recv_string(zmq::DONTWAIT).expect("recv").expect("string").as_str());
//        handle.join().expect("join");
//    }

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
