extern crate byteorder;
extern crate digest;
extern crate hex;
extern crate indy_crypto;
extern crate rand;
extern crate rmp_serde;
extern crate rust_base58;
extern crate sha2;
extern crate time;
extern crate zmq;

use self::byteorder::{ByteOrder, LittleEndian};
use self::zmq::Socket;

use std::{fs, io};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;

use serde_json;
use serde::de::DeserializeOwned;

use api::ledger::{CustomFree, CustomTransactionParser};
use domain::{
    pool::{PoolConfig, PoolOpenConfig},
    ledger::response::{
        Message,
        Reply,
        ResponseMetadata
    }
};
use errors::*;
use services::pool::pool::{Pool, ZMQPool};
use utils::environment;
use utils::sequence;

mod catchup;
mod commander;
mod events;
mod merkle_tree_factory;
mod networker;
mod pool;
mod request_handler;
mod state_proof;
mod types;

lazy_static! {
    static ref REGISTERED_SP_PARSERS: Mutex<HashMap<String, (CustomTransactionParser, CustomFree)>> = Mutex::new(HashMap::new());
}

pub struct PoolService {
    open_pools: RefCell<HashMap<i32, ZMQPool>>,
    pending_pools: RefCell<HashMap<i32, ZMQPool>>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            open_pools: RefCell::new(HashMap::new()),
            pending_pools: RefCell::new(HashMap::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<PoolConfig>) -> IndyResult<()> {
        //TODO: initialize all state machines
        trace!("PoolService::create {} with config {:?}", name, config);

        let mut path = environment::pool_path(name);
        let pool_config = config.unwrap_or(PoolConfig::default_for_name(name));

        if path.as_path().exists() {
            return Err(err_msg(IndyErrorKind::PoolConfigAlreadyExists, format!("Pool ledger config file with name \"{}\" already exists", name)));
        }

        // check that we can build MerkeleTree from genesis transaction file
        //TODO: move parse to correct place
        let mt = merkle_tree_factory::from_file(&pool_config.genesis_txn)?;

        if mt.count() == 0 {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Empty genesis transaction file"));
        }

        fs::create_dir_all(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config directory")?;

        path.push(name);
        path.set_extension("txn");

        {
            // fs::copy also copies attributes of the file
            // and copying permissions can be problem for some cases

            let mut gt_fin = fs::File::open(&pool_config.genesis_txn)
                .to_indy(IndyErrorKind::IOError, "Can't copy genesis txn file")?;

            let mut gt_fout = fs::File::create(path.as_path())
                .to_indy(IndyErrorKind::IOError, "Can't copy genesis txn file")?;

            io::copy(&mut gt_fin, &mut gt_fout)
                .to_indy(IndyErrorKind::IOError, "Can't copy genesis txn file")?;
        }

        path.pop();
        path.push("config");
        path.set_extension("json");

        let mut f: fs::File = fs::File::create(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config file")?;

        f
            .write_all({
                serde_json::to_string(&pool_config)
                    .to_indy(IndyErrorKind::InvalidState, "Can't serialize pool config")?
                    .as_bytes()
            })
            .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        f
            .flush()
            .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        // TODO probably create another one file pool.json with pool description,
        // but now there is no info to save (except name witch equal to directory)
        Ok(())
    }

    pub fn delete(&self, name: &str) -> IndyResult<()> {
        for ref pool in self.open_pools.try_borrow()?.values() {
            if pool.pool.get_name().eq(name) {
                return Err(err_msg(IndyErrorKind::InvalidState, "Can't delete pool config - pool is open now"));
            }
        }

        let path = environment::pool_path(name);

        fs::remove_dir_all(path)
            .to_indy(IndyErrorKind::IOError, "Can't delete pool config directory")
    }

    pub fn open(&self, name: &str, config: Option<PoolOpenConfig>) -> IndyResult<i32> {
        for ref pool in self.open_pools.try_borrow()?.values() {
            if name.eq(pool.pool.get_name()) {
                //TODO change error
                return Err(err_msg(IndyErrorKind::InvalidPoolHandle, "Pool with the same name is already opened"));
            }
        }

        let config = config.unwrap_or(PoolOpenConfig::default());

        let pool_handle: i32 = sequence::get_next_id();
        let mut new_pool = Pool::new(name, pool_handle, config);

        let zmq_ctx = zmq::Context::new();
        let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR)?;
        let inproc_sock_name: String = format!("inproc://pool_{}", name);

        recv_cmd_sock.bind(inproc_sock_name.as_str())?;

        send_cmd_sock.connect(inproc_sock_name.as_str())?;

        new_pool.work(recv_cmd_sock);
        self._send_msg(pool_handle, "connect", &send_cmd_sock, None, None)?;

        self.pending_pools.try_borrow_mut()?
            .insert(new_pool.get_id(), ZMQPool::new(new_pool, send_cmd_sock));
        return Ok(pool_handle);
    }

    pub fn add_open_pool(&self, pool_id: i32) -> IndyResult<i32> {
        let pool = self.pending_pools.try_borrow_mut()?
            .remove(&pool_id)
            .ok_or(err_msg(IndyErrorKind::InvalidPoolHandle, format!("No pool with requested handle {}", pool_id)))?;

        self.open_pools.try_borrow_mut()?.insert(pool_id, pool);

        Ok(pool_id)
    }


    pub fn send_tx(&self, handle: i32, msg: &str) -> IndyResult<i32> {
        self.send_action(handle, msg, None, None)
    }

    pub fn send_action(&self, handle: i32, msg: &str, nodes: Option<&str>, timeout: Option<i32>) -> IndyResult<i32> {
        let pools = self.open_pools.try_borrow()?;

        if let Some(ref pool) = pools.get(&handle) {
            let cmd_id: i32 = sequence::get_next_id();
            self._send_msg(cmd_id, msg, &pool.cmd_socket, nodes, timeout)?;
            Ok(cmd_id)
        } else {
            Err(err_msg(IndyErrorKind::InvalidPoolHandle, format!("No pool with requested handle {}", handle)))
        }
    }

    pub fn register_sp_parser(txn_type: &str,
                              parser: CustomTransactionParser, free: CustomFree) -> IndyResult<()> {
        if events::REQUESTS_FOR_STATE_PROOFS.contains(&txn_type) {
            return Err(err_msg(IndyErrorKind::InvalidStructure,
                               format!("Try to override StateProof parser for default TXN_TYPE {}", txn_type)));
        }

        REGISTERED_SP_PARSERS.lock()
            .map(|mut map| {
                map.insert(txn_type.to_owned(), (parser, free));
            })
            .unwrap(); // FIXME: Can we avoid unwrap?

        Ok(())
    }

    pub fn get_sp_parser(txn_type: &str) -> Option<(CustomTransactionParser, CustomFree)> {
        let parsers = REGISTERED_SP_PARSERS.lock().unwrap(); // FIXME: Can we avoid unwrap here?
        parsers.get(txn_type).map(Clone::clone)
    }

    pub fn close(&self, handle: i32) -> IndyResult<i32> {
        let cmd_id: i32 = sequence::get_next_id();

        let mut pools = self.open_pools.try_borrow_mut()?;

        match pools.remove(&handle) {
            Some(ref pool) => self._send_msg(cmd_id, "exit", &pool.cmd_socket, None, None)?,
            None => return Err(err_msg(IndyErrorKind::InvalidPoolHandle, format!("No pool with requested handle {}", handle)))
        }

        Ok(cmd_id)
    }

    pub fn refresh(&self, handle: i32) -> IndyResult<i32> {
        self.send_action(handle, "refresh", None, None)
    }

    fn _send_msg(&self, cmd_id: i32, msg: &str, socket: &Socket, nodes: Option<&str>, timeout: Option<i32>) -> IndyResult<()> {
        let mut buf = [0u8; 4];
        let mut buf_to = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let timeout = timeout.unwrap_or(-1);
        LittleEndian::write_i32(&mut buf_to, timeout);
        if let Some(nodes) = nodes {
            Ok(socket.send_multipart(&[msg.as_bytes(), &buf, &buf_to, nodes.as_bytes()], zmq::DONTWAIT)?)
        } else {
            Ok(socket.send_multipart(&[msg.as_bytes(), &buf, &buf_to], zmq::DONTWAIT)?)
        }
    }

    pub fn list(&self) -> IndyResult<Vec<serde_json::Value>> {
        let mut pool = Vec::new();
        let pool_home_path = environment::pool_home_path();

        if let Ok(entries) = fs::read_dir(pool_home_path) {
            for entry in entries {
                let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue; };
                if let Some(pool_name) = dir_entry.path().file_name().and_then(|os_str| os_str.to_str()) {
                    let json = json!({"pool":pool_name.to_owned()});
                    pool.push(json);
                }
            }
        }

        Ok(pool)
    }
}

lazy_static! {
    static ref THRESHOLD: Mutex<u64> = Mutex::new(600);
}

pub fn set_freshness_threshold(threshold: u64) {
    let mut th = THRESHOLD.lock().unwrap();
    *th = ::std::cmp::max(threshold, 300);
}


pub fn parse_response_metadata(response: &str) -> IndyResult<ResponseMetadata> {
    trace!("indy::services::pool::parse_response_metadata << response: {}", response);
    let message: Message<serde_json::Value> = serde_json::from_str(response)
        .to_indy(IndyErrorKind::InvalidTransaction, "Cannot deserialize transaction Response")?;

    let response_object: Reply<serde_json::Value> = _handle_response_message_type(message)?;
    let response_result = response_object.result();

    let response_metadata = match response_result["ver"].as_str() {
        None => _parse_transaction_metadata_v0(&response_result),
        Some("1") => _parse_transaction_metadata_v1(&response_result),
        ver @ _ => return Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Unsupported transaction response version: {:?}", ver)))
    };

    trace!("indy::services::pool::parse_response_metadata >> response_metadata: {:?}", response_metadata);

    Ok(response_metadata)
}

pub fn get_last_signed_time(response: &str) -> Option<u64> {
    let c = parse_response_metadata(response);
    c.ok().and_then(|resp| resp.last_txn_time)
}

fn _handle_response_message_type<T>(message: Message<T>) -> IndyResult<Reply<T>> where T: DeserializeOwned + ::std::fmt::Debug {
    trace!("handle_response_message_type >>> message {:?}", message);

    match message {
        Message::Reject(response) | Message::ReqNACK(response) =>
            Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Transaction has been failed: {:?}", response.reason))),
        Message::Reply(reply) =>
            Ok(reply)
    }
}

fn _parse_transaction_metadata_v0(message: &serde_json::Value) -> ResponseMetadata {
    ResponseMetadata {
        seq_no: message["seqNo"].as_u64(),
        txn_time: message["txnTime"].as_u64(),
        last_txn_time: message["state_proof"]["multi_signature"]["value"]["timestamp"].as_u64(),
        last_seq_no: None,
    }
}

fn _parse_transaction_metadata_v1(message: &serde_json::Value) -> ResponseMetadata {
    ResponseMetadata {
        seq_no: message["txnMetadata"]["seqNo"].as_u64(),
        txn_time: message["txnMetadata"]["txnTime"].as_u64(),
        last_txn_time: message["multiSignature"]["signedState"]["stateMetadata"]["timestamp"].as_u64(),
        last_seq_no: None,
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use domain::ledger::request::ProtocolVersion;
    use services::pool::types::*;
    use utils::test;

    use super::*;

    const TEST_PROTOCOL_VERSION: usize = 2;

    fn _set_protocol_version(version: usize) {
        ProtocolVersion::set(version);
    }

    mod pool_service {
        use std::path;

        use libc::c_char;

        use api::ErrorCode;

        use super::*;

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
            test::cleanup_storage();

            let ps = PoolService::new();
            let pool_id = sequence::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.open_pools.borrow_mut().insert(pool_id, ZMQPool::new(Pool::new("", pool_id, PoolOpenConfig::default()), send_soc));
            let cmd_id = ps.close(pool_id).unwrap();
            let recv = recv_soc.recv_multipart(zmq::DONTWAIT).unwrap();
            assert_eq!(recv.len(), 3);
            assert_eq!("exit", String::from_utf8(recv[0].clone()).unwrap());
            assert_eq!(cmd_id, LittleEndian::read_i32(recv[1].as_slice()));
        }

        #[test]
        fn pool_service_refresh_works() {
            test::cleanup_storage();

            let ps = PoolService::new();
            let pool_id = sequence::get_next_id();
            let ctx = zmq::Context::new();
            let send_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_soc = ctx.socket(zmq::SocketType::PAIR).unwrap();
            recv_soc.bind("inproc://test").unwrap();
            send_soc.connect("inproc://test").unwrap();
            ps.open_pools.borrow_mut().insert(pool_id, ZMQPool::new(Pool::new("", pool_id, PoolOpenConfig::default()), send_soc));
            let cmd_id = ps.refresh(pool_id).unwrap();
            let recv = recv_soc.recv_multipart(zmq::DONTWAIT).unwrap();
            assert_eq!(recv.len(), 3);
            assert_eq!("refresh", String::from_utf8(recv[0].clone()).unwrap());
            assert_eq!(cmd_id, LittleEndian::read_i32(recv[1].as_slice()));
        }

        #[test]
        fn pool_service_delete_works() {
            test::cleanup_storage();

            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works";
            let path: path::PathBuf = environment::pool_path(pool_name);
            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            ps.delete(pool_name).unwrap();
            assert!(!path.exists());
        }

        #[test]
        fn pool_service_delete_works_for_opened() {
            test::cleanup_storage();

            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works";
            let path: path::PathBuf = environment::pool_path(pool_name);
            let pool_id = sequence::get_next_id();

            let inproc_sock_name: String = format!("inproc://pool_{}", pool_name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();

            let pool = Pool::new(pool_name, pool_id, PoolOpenConfig::default());
            ps.open_pools.borrow_mut().insert(pool_id, ZMQPool::new(pool, send_cmd_sock));

            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            let res = ps.delete(pool_name);
            assert_eq!(IndyErrorKind::InvalidState, res.unwrap_err().kind());
            assert!(path.exists());
        }

        #[test]
        fn pool_send_tx_works() {
            test::cleanup_storage();

            let name = "test";
            let zmq_ctx = zmq::Context::new();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let inproc_sock_name: String = format!("inproc://pool_{}", name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
            let pool = Pool::new(name, 0, PoolOpenConfig::default());
            let ps = PoolService::new();
            ps.open_pools.borrow_mut().insert(-1, ZMQPool::new(pool, send_cmd_sock));
            let test_data = "str_instead_of_tx_json";
            ps.send_tx(-1, test_data).unwrap();
            assert_eq!(recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), test_data);
        }

        #[test]
        fn pool_send_tx_works_for_closed_socket() {
            test::cleanup_storage();

            let name = "test";
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();

            let pool = Pool::new(name, 0, PoolOpenConfig::default());
            let ps = PoolService::new();
            ps.open_pools.borrow_mut().insert(-1, ZMQPool::new(pool, send_cmd_sock));
            let res = ps.send_tx(-1, "test_data");
            assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());
        }

        #[test]
        fn pool_send_tx_works_for_invalid_handle() {
            test::cleanup_storage();
            let ps = PoolService::new();
            let res = ps.send_tx(-1, "txn");
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        #[test]
        fn pool_send_action_works() {
            test::cleanup_storage();

            let name = "test";
            let zmq_ctx = zmq::Context::new();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let inproc_sock_name: String = format!("inproc://pool_{}", name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
            let pool = Pool::new(name, 0, PoolOpenConfig::default());
            let ps = PoolService::new();
            ps.open_pools.borrow_mut().insert(-1, ZMQPool::new(pool, send_cmd_sock));
            let test_data = "str_instead_of_tx_json";
            ps.send_action(-1, test_data, None, None).unwrap();
            assert_eq!(recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(), test_data);
        }

        #[test]
        fn pool_close_works_for_invalid_handle() {
            test::cleanup_storage();
            let ps = PoolService::new();
            let res = ps.close(-1);
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        #[test]
        fn pool_refresh_works_for_invalid_handle() {
            test::cleanup_storage();
            let ps = PoolService::new();
            let res = ps.refresh(-1);
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        #[test]
        fn pool_register_sp_parser_works() {
            test::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            extern fn test_sp(_reply_from_node: *const c_char, _parsed_sp: *mut *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            extern fn test_free(_data: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
        }

        #[test]
        fn pool_get_sp_parser_works() {
            test::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            extern fn test_sp(_reply_from_node: *const c_char, _parsed_sp: *mut *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            extern fn test_free(_data: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }
            PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
            PoolService::get_sp_parser("test").unwrap();
        }

        #[test]
        fn pool_get_sp_parser_works_for_invalid_name() {
            test::cleanup_storage();
            REGISTERED_SP_PARSERS.lock().unwrap().clear();
            assert_eq!(None, PoolService::get_sp_parser("test"));
        }

        #[test]
        pub fn pool_add_open_pool_works() {
            test::cleanup_storage();
            let name = "test";
            let ps = PoolService::new();
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let pool = Pool::new(name, 0, PoolOpenConfig::default());
            ps.pending_pools.borrow_mut().insert(-1, ZMQPool::new(pool, send_cmd_sock));
            assert_match!(Ok(-1), ps.add_open_pool(-1));
        }

        #[test]
        pub fn pool_add_open_pool_works_for_no_pending_pool() {
            test::cleanup_storage();
            let ps = PoolService::new();
            let res = ps.add_open_pool(-1);
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }
    }

    #[test]
    fn pool_drop_works_for_after_close() {
        use utils::test;
        use std::time;

        test::cleanup_storage();

        fn drop_test() {
            test::cleanup_storage();
            _set_protocol_version(TEST_PROTOCOL_VERSION);
            let ps = PoolService::new();

            let pool_name = "pool_drop_works";
            let gen_txn = test::gen_txns()[0].clone();

            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
            let inproc_sock_name: String = format!("inproc://pool_{}", pool_name);
            recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
            send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();

            // create minimal fs config stub before Pool::new()
            let mut pool_path = environment::pool_path(pool_name);
            fs::create_dir_all(&pool_path).unwrap();
            pool_path.push(pool_name);
            pool_path.set_extension("txn");
            let mut file = fs::File::create(pool_path).unwrap();
            file.write(&gen_txn.as_bytes()).unwrap();

            let mut pool = Pool::new(pool_name, -1, PoolOpenConfig::default());
            pool.work(recv_cmd_sock);
            ps.open_pools.borrow_mut().insert(-1, ZMQPool::new(pool, send_cmd_sock));
            thread::sleep(time::Duration::from_secs(1));
            ps.close(-1).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }

        drop_test();
        test::cleanup_storage();
    }

    pub mod nodes_emulator {
        extern crate sodiumoxide;

        use services::pool::rust_base58::{FromBase58, ToBase58};
        use utils::crypto::ed25519_sign;

        use super::*;

        use self::indy_crypto::bls::{Generator, SignKey, VerKey};

        pub static POLL_TIMEOUT: i64 = 1_000; /* in ms */

        pub fn node() -> NodeTransactionV1 {
            let blskey = VerKey::new(&Generator::from_bytes(&"3LHpUjiyFC2q2hD7MnwwNmVXiuaFbQx2XkAFJWzswCjgN1utjsCeLzHsKk1nJvFEaS4fcrUmVAkdhtPCYbrVyATZcmzwJReTcJqwqBCPTmTQ9uWPwz6rEncKb2pYYYFcdHa8N17HzVyTqKfgPi4X9pMetfT3A5xCHq54R2pDNYWVLDX".from_base58().unwrap()).unwrap(),
                                     &SignKey::new(None).unwrap()).unwrap().as_bytes().to_base58();

            NodeTransactionV1 {
                txn: Txn {
                    txn_type: "1".to_string(),
                    protocol_version: None,
                    data: TxnData {
                        data: NodeData {
                            alias: "n1".to_string(),
                            client_ip: Some("127.0.0.1".to_string()),
                            client_port: Some(9000),
                            node_ip: Some(String::new()),
                            node_port: Some(0),
                            services: Some(vec!["VALIDATOR".to_string()]),
                            blskey: Some(blskey.to_string()),
                            blskey_pop: None,
                        },
                        dest: "Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv".to_string(),
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
            }
        }

        pub fn node_2() -> NodeTransactionV1 {
            let blskey = VerKey::new(&Generator::from_bytes(&"3LHpUjiyFC2q2hD7MnwwNmVXiuaFbQx2XkAFJWzswCjgN1utjsCeLzHsKk1nJvFEaS4fcrUmVAkdhtPCYbrVyATZcmzwJReTcJqwqBCPTmTQ9uWPwz6rEncKb2pYYYFcdHa8N17HzVyTqKfgPi4X9pMetfT3A5xCHq54R2pDNYWVLDX".from_base58().unwrap()).unwrap(),
                                     &SignKey::new(None).unwrap()).unwrap().as_bytes().to_base58();

            NodeTransactionV1 {
                txn: Txn {
                    txn_type: "1".to_string(),
                    protocol_version: None,
                    data: TxnData {
                        data: NodeData {
                            alias: "n2".to_string(),
                            client_ip: Some("127.0.0.1".to_string()),
                            client_port: Some(9001),
                            node_ip: Some(String::new()),
                            node_port: Some(0),
                            services: Some(vec!["VALIDATOR".to_string()]),
                            blskey: Some(blskey.to_string()),
                            blskey_pop: None,
                        },
                        dest: "Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv".to_string(),
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
            }
        }

        pub fn start(gt: &mut NodeTransactionV1) -> zmq::Socket {
            let (vk, sk) = sodiumoxide::crypto::sign::ed25519::gen_keypair();
            let (vk, sk) = (ed25519_sign::PublicKey::from_slice(&vk[..]).unwrap(), ed25519_sign::SecretKey::from_slice(&sk[..]).unwrap());
            let pkc = ed25519_sign::vk_to_curve25519(&vk).expect("Invalid pkc");
            let skc = ed25519_sign::sk_to_curve25519(&sk).expect("Invalid skc");
            let ctx = zmq::Context::new();
            let s: zmq::Socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();

            gt.txn.data.dest = (&vk[..]).to_base58();

            s.set_curve_publickey(&zmq::z85_encode(&pkc[..]).unwrap()).expect("set public key");
            s.set_curve_secretkey(&zmq::z85_encode(&skc[..]).unwrap()).expect("set secret key");
            s.set_curve_server(true).expect("set curve server");

            s.bind("tcp://127.0.0.1:*").expect("bind");

            let parts = s.get_last_endpoint().unwrap().unwrap();
            let parts = parts.rsplit(":").collect::<Vec<&str>>();

            gt.txn.data.data.client_port = Some(parts[0].parse::<u64>().unwrap());

            s
        }

        pub fn next(s: &zmq::Socket) -> Option<String> {
            let poll_res = s.poll(zmq::POLLIN, POLL_TIMEOUT).expect("poll");
            if poll_res == 1 {
                let v = s.recv_multipart(zmq::DONTWAIT).expect("recv mulp");
                trace!("Node emulator poll recv {:?}", v);
                s.send_multipart(&[v[0].as_slice(), "po".as_bytes()], zmq::DONTWAIT).expect("send mulp");
                Some(String::from_utf8(v[1].clone()).unwrap())
            } else {
                warn!("Node emulator poll return {}", poll_res);
                None
            }
        }
    }
}
