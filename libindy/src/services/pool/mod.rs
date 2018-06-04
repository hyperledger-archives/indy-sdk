mod types;
//mod catchup;
//mod transaction_handler;
//mod state_proof;
//mod pool_worker;

mod pool;
mod consensus_collector;
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

use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use commands::pool::PoolCommand;
use errors::pool::PoolError;
use errors::common::CommonError;
//use self::catchup::CatchupHandler;
//use self::transaction_handler::TransactionHandler;
use self::types::*;
use services::ledger::merkletree::merkletree::MerkleTree;
use utils::crypto::box_::CryptoBox;
use utils::environment::EnvironmentUtils;
use utils::sequence::SequenceUtils;
use self::indy_crypto::bls::VerKey;
use std::path::PathBuf;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

pub struct PoolService {
    workers: RefCell<HashMap<i32, PoolWorker>>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            workers: RefCell::new(HashMap::new()),
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
                return Err(PoolError::InvalidHandle("Pool with same name already opened".to_string()));
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
        for entry in fs::read_dir(pool_home_path)? {
            let dir_entry = if let Ok(dir_entry) = entry { dir_entry } else { continue };
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
            |pool: &Pool| Ok(pool.name.clone()))
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
            let pool_name = "pool_drop_works";
            let gen_txn = format!("{{\"data\":{{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]}},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}}");

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

    pub const NODE1: &'static str = r#"{"data":{"alias":"Node1","client_ip":"192.168.1.35","client_port":9702,"node_ip":"192.168.1.35","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#;
    pub const NODE2: &'static str = r#"{"data":{"alias":"Node2","client_ip":"192.168.1.35","client_port":9704,"node_ip":"192.168.1.35","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#;

    pub const NODE1_NEW_FORMAT: &'static str = r#"{"reqSignature":{},"txn":{"type":"0","data":{"data":{"alias":"Node1","client_ip":"192.168.1.35","client_port":9702,"node_ip":"192.168.1.35","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"},"metadata":{"from":"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"}},"txnMetadata":{"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"},"ver":"1"}"#;
    pub const NODE2_NEW_FORMAT: &'static str = r#"{"reqSignature":{},"txn":{"type":"0","data":{"data":{"alias":"Node2","client_ip":"192.168.1.35","client_port":9704,"node_ip":"192.168.1.35","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"},"metadata":{"from":"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy"}},"txnMetadata":{"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"},"ver":"1"}"#;

    #[test]
    fn pool_worker_restore_merkle_tree_works_from_genesis_txns() {
        let txns_src = format!("{}\n{}", NODE1, NODE2);
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns_src.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();

        assert_eq!(merkle_tree.count(), 2, "test restored MT size");
        assert_eq!(merkle_tree.root_hash_hex(), "ae7fb19d399b0b03ed298285d0da19ee6c6ba9ed7c063c95228c435d7ff97b4d", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_connect_to_known_nodes_works() {
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
        serde_json::from_str::<NodeTransaction>(NODE1).unwrap();
        serde_json::from_str::<NodeTransaction>(NODE2).unwrap();

        let node1: NodeTransactionV0 = serde_json::from_str(NODE1).unwrap();
        let node2: NodeTransactionV0 = serde_json::from_str(NODE2).unwrap();

        let txn1_src = format!("{{\"data\":{{\"alias\":\"{}\",\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.data.alias, node1.data.node_ip.clone().unwrap(), node1.data.node_port.clone().unwrap(), node1.data.services.clone().unwrap(), node1.dest, node1.identifier, node1.txn_id.clone().unwrap());
        let txn2_src = format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.data.alias, node1.data.client_ip.clone().unwrap(), node1.data.client_port.clone().unwrap(), node1.data.node_ip.clone().unwrap(), node1.data.node_port.unwrap(), node1.dest, node1.identifier, node1.txn_id.clone().unwrap());
        let txn3_src = format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.data.alias, node2.data.client_ip.clone().unwrap(), node2.data.client_port.clone().unwrap(), node2.dest, node2.identifier, node2.txn_id.clone().unwrap());
        let txn4_src = format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.data.alias, node2.data.client_ip.clone().unwrap(), node2.data.client_port.clone().unwrap(), node2.data.node_ip.clone().unwrap(), node2.data.node_port.clone().unwrap(), node2.data.services.clone().unwrap(), node2.dest, node2.identifier, node2.txn_id.clone().unwrap());

        let txns = format!("{}\n{}\n{}\n{}", txn1_src, txn2_src, txn3_src, txn4_src);

        let txn1_json: serde_json::Value = serde_json::from_str(&txn1_src).unwrap();
        let txn2_json: serde_json::Value = serde_json::from_str(&txn2_src).unwrap();
        let txn3_json: serde_json::Value = serde_json::from_str(&txn3_src).unwrap();
        let txn4_json: serde_json::Value = serde_json::from_str(&txn4_src).unwrap();

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
        let node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();
    }

    #[test]
    fn pool_worker_build_node_state_works() {
        serde_json::from_str::<NodeTransaction>(NODE1).unwrap();
        serde_json::from_str::<NodeTransaction>(NODE2).unwrap();

        let node1: NodeTransactionV0 = serde_json::from_str(NODE1).unwrap();
        let node2: NodeTransactionV0 = serde_json::from_str(NODE2).unwrap();

        let txns_src = format!("{}\n{}\n{}\n{}\n",
                               format!("{{\"data\":{{\"alias\":\"{}\",\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.data.alias, node1.data.node_ip.clone().unwrap(), node1.data.node_port.clone().unwrap(), node1.data.services.clone().unwrap(), node1.dest, node1.identifier, node1.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.data.alias, node1.data.client_ip.clone().unwrap(), node1.data.client_port.clone().unwrap(), node1.data.node_ip.clone().unwrap(), node1.data.node_port.unwrap(), node1.dest, node1.identifier, node1.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.data.alias, node2.data.client_ip.clone().unwrap(), node2.data.client_port.clone().unwrap(), node2.dest, node2.identifier, node2.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.data.alias, node2.data.client_ip.clone().unwrap(), node2.data.client_port.clone().unwrap(), node2.data.node_ip.clone().unwrap(), node2.data.node_port.clone().unwrap(), node2.data.services.clone().unwrap(), node2.dest, node2.identifier, node2.txn_id.clone().unwrap()));
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns_src.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();

        assert_eq!(2, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], NodeTransactionV1::from(NodeTransaction::NodeTransactionV0(node1)));
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], NodeTransactionV1::from(NodeTransaction::NodeTransactionV0(node2)));
    }

    #[test]
    fn pool_worker_build_node_state_works_for_new_format() {
        let node1: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransaction>(NODE1_NEW_FORMAT).unwrap());
        let node2: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransaction>(NODE2_NEW_FORMAT).unwrap());

        let txns_src = format!("{}\n{}\n{}\n{}\n",
                               format!("{{\"data\":{{\"alias\":\"{}\",\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.txn.data.data.alias, node1.txn.data.data.node_ip.clone().unwrap(), node1.txn.data.data.node_port.clone().unwrap(), node1.txn.data.data.services.clone().unwrap(), node1.txn.data.dest, node1.txn.metadata.from, node1.txn_metadata.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node1.txn.data.data.alias, node1.txn.data.data.client_ip.clone().unwrap(), node1.txn.data.data.client_port.clone().unwrap(), node1.txn.data.data.node_ip.clone().unwrap(), node1.txn.data.data.node_port.unwrap(), node1.txn.data.dest, node1.txn.metadata.from, node1.txn_metadata.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.txn.data.data.alias, node2.txn.data.data.client_ip.clone().unwrap(), node2.txn.data.data.client_port.clone().unwrap(), node2.txn.data.dest, node2.txn.metadata.from, node2.txn_metadata.txn_id.clone().unwrap()),
                               format!("{{\"data\":{{\"alias\":\"{}\",\"client_ip\":\"{}\",\"client_port\":{},\"node_ip\":\"{}\",\"node_port\":{},\"services\":{:?}}},\"dest\":\"{}\",\"identifier\":\"{}\",\"txnId\":\"{}\",\"type\":\"0\"}}",
                                       node2.txn.data.data.alias, node2.txn.data.data.client_ip.clone().unwrap(), node2.txn.data.data.client_port.clone().unwrap(), node2.txn.data.data.node_ip.clone().unwrap(), node2.txn.data.data.node_port.clone().unwrap(), node2.txn.data.data.services.clone().unwrap(), node2.txn.data.dest, node2.txn.metadata.from, node2.txn_metadata.txn_id.clone().unwrap()));
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns_src.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();

        let merkle_tree = PoolWorker::restore_merkle_tree_from_pool_name("test").unwrap();
        let node_state = PoolWorker::_build_node_state(&merkle_tree).unwrap();

        assert_eq!(2, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], node1);
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], node2);
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
        send_cmd_sock.send("exit".as_bytes(), zmq::DONTWAIT).expect("send");
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
    fn pool_worker_get_f_works() {
        assert_eq!(PoolWorker::get_f(0), 0);
        assert_eq!(PoolWorker::get_f(3), 0);
        assert_eq!(PoolWorker::get_f(4), 1);
        assert_eq!(PoolWorker::get_f(5), 1);
        assert_eq!(PoolWorker::get_f(6), 1);
        assert_eq!(PoolWorker::get_f(7), 2);
    }

    #[test]
    fn catchup_handler_start_catchup_works() {
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

            let gt = NodeTransactionV1::from(NodeTransaction::NodeTransactionV0(NodeTransactionV0 {
                identifier: "".to_string(),
                data: NodeData {
                    alias: "n1".to_string(),
                    blskey: Some(blskey),
                    services: Some(vec!["VALIDATOR".to_string()]),
                    client_port: Some(9700),
                    client_ip: Some("127.0.0.1".to_string()),
                    node_ip: Some("".to_string()),
                    node_port: Some(0)
                },
                txn_id: None,
                verkey: None,
                txn_type: "0".to_string(),
                dest: (&vk.0 as &[u8]).to_base58(),
            }));
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
