use byteorder::{ByteOrder, LittleEndian};

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::Arc;
use std::{fs, io};

use serde::de::DeserializeOwned;
use serde_json;

//use crate::api::ledger::{CustomFree, CustomTransactionParser}; FIXME: !!!
use crate::domain::{
    ledger::response::{Message, Reply, ResponseMetadata},
    pool::{PoolConfig, PoolOpenConfig},
};

use crate::services::pool::events::{COMMAND_CONNECT, COMMAND_EXIT, COMMAND_REFRESH};
use crate::services::pool::pool::{Pool, ZMQPool};
use crate::utils::environment;
use indy_api_types::errors::*;
use indy_api_types::{CommandHandle, PoolHandle};
use indy_utils::{next_command_handle, next_pool_handle};
use ursa::bls::VerKey;

use futures::{channel::oneshot, lock::Mutex};

use log_derive::logfn;

use zmq::Socket;

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
    //static ref REGISTERED_SP_PARSERS: Mutex<HashMap<String, (CustomTransactionParser, CustomFree)>> = Mutex::new(HashMap::new()); FXIME: !!!
    static ref POOL_HANDLE_SENDERS: Arc<Mutex<HashMap<PoolHandle, oneshot::Sender<IndyResult<PoolHandle>>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref SUBMIT_SENDERS: Arc<Mutex<HashMap<PoolHandle, oneshot::Sender<IndyResult<String>>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref CLOSE_SENDERS: Arc<Mutex<HashMap<PoolHandle, oneshot::Sender<IndyResult<()>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

type Nodes = HashMap<String, Option<VerKey>>;

pub struct PoolService {
    open_pools: Mutex<HashMap<PoolHandle, Arc<ZMQPool>>>,
    pending_pools: Mutex<HashSet<String>>,
}

impl PoolService {
    pub fn new() -> PoolService {
        PoolService {
            open_pools: Mutex::new(HashMap::new()),
            pending_pools: Mutex::new(HashSet::new()),
        }
    }

    pub fn create(&self, name: &str, config: Option<PoolConfig>) -> IndyResult<()> {
        //TODO: initialize all state machines
        trace!("PoolService::create {} with config {:?}", name, config);

        let mut path = environment::pool_path(name);
        let pool_config = config.unwrap_or_else(|| PoolConfig::default_for_name(name));

        if path.as_path().exists() {
            return Err(err_msg(
                IndyErrorKind::PoolConfigAlreadyExists,
                format!(
                    "Pool ledger config file with name \"{}\" already exists",
                    name
                ),
            ));
        }

        // check that we can build MerkeleTree from genesis transaction file
        //TODO: move parse to correct place
        let mt = merkle_tree_factory::from_file(&pool_config.genesis_txn)?;

        if mt.count() == 0 {
            return Err(err_msg(
                IndyErrorKind::InvalidStructure,
                "Empty genesis transaction file",
            ));
        }

        fs::create_dir_all(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config directory")?;

        path.push(name);
        path.set_extension("txn");

        {
            // fs::copy also copies attributes of the file
            // and copying permissions can be problem for some cases

            let mut gt_fin = fs::File::open(&pool_config.genesis_txn).to_indy(
                IndyErrorKind::IOError,
                format!("Can't open genesis txn file {:?}", &pool_config.genesis_txn),
            )?;

            let mut gt_fout = fs::File::create(path.as_path()).to_indy(
                IndyErrorKind::IOError,
                format!("Can't create genesis txn file {:?}", path.as_path()),
            )?;

            io::copy(&mut gt_fin, &mut gt_fout).to_indy(
                IndyErrorKind::IOError,
                format!(
                    "Can't copy genesis txn file from {:?} to {:?}",
                    &pool_config.genesis_txn,
                    path.as_path()
                ),
            )?;
        }

        path.pop();
        path.push("config");
        path.set_extension("json");

        let mut f: fs::File = fs::File::create(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config file")?;

        f.write_all({
            serde_json::to_string(&pool_config)
                .to_indy(IndyErrorKind::InvalidState, "Can't serialize pool config")?
                .as_bytes()
        })
        .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        f.flush()
            .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        // TODO probably create another one file pool.json with pool description,
        // but now there is no info to save (except name witch equal to directory)
        Ok(())
    }

    pub async fn delete(&self, name: &str) -> IndyResult<()> {
        if self
            .open_pools
            .lock()
            .await
            .values()
            .find(|pool| pool.pool.lock().unwrap().get_name().eq(name))
            .is_some()
        {
            return Err(err_msg(
                IndyErrorKind::InvalidState,
                "Can't delete pool config - pool is open now",
            ));
        }

        let path = environment::pool_path(name);

        fs::remove_dir_all(path)
            .to_indy(IndyErrorKind::IOError, "Can't delete pool config directory")
    }

    pub async fn open(
        &self,
        name: String,
        config: Option<PoolOpenConfig>,
    ) -> IndyResult<PoolHandle> {
        if self
            .open_pools
            .lock()
            .await
            .values()
            .find(|pool| pool.pool.lock().unwrap().get_name().eq(&name))
            .is_some()
        {
            //TODO change error
            return Err(err_msg(
                IndyErrorKind::InvalidPoolHandle,
                "Pool with the same name is already opened",
            ));
        }

        if self.pending_pools.lock().await.contains(&name) {
            return Err(err_msg(
                IndyErrorKind::InvalidPoolHandle,
                "Pool with the same name is already being opened",
            ));
        }

        let config = config.unwrap_or_default();

        let pool_handle: PoolHandle = next_pool_handle();
        let mut new_pool = Pool::new(&name, pool_handle, config);

        let (send_cmd_sock, recv_cmd_sock) = pool_create_pair_of_sockets(&format!("pool_{}", name));

        new_pool.work(recv_cmd_sock);
        self._send_msg(pool_handle, COMMAND_CONNECT, &send_cmd_sock, None, None)?;

        let (sender, receiver) = oneshot::channel::<IndyResult<PoolHandle>>();

        self.pending_pools.lock().await.insert(name.clone());
        POOL_HANDLE_SENDERS.lock().await.insert(pool_handle, sender);

        let res = receiver.await?;

        self.pending_pools.lock().await.remove(&name);
        
        if res.is_ok() {
            self.open_pools
                .lock()
                .await
                .insert(new_pool.get_id(), Arc::new(ZMQPool::new(new_pool, send_cmd_sock)));
        }

        res
    }

    //#[logfn(trace)] FIXME:
    pub async fn open_ack(pool_hanlde: PoolHandle, result: IndyResult<()>) {
        let sender: futures::channel::oneshot::Sender<IndyResult<PoolHandle>> = POOL_HANDLE_SENDERS
            .lock()
            .await
            .remove(&pool_hanlde)
            .unwrap();
        sender.send(result.map(|()| pool_hanlde)).unwrap(); //FIXME
    }

    //#[logfn(trace)] FIXME:
    pub async fn refresh_ack(cmd_id: CommandHandle, result: IndyResult<()>) {
        let sender: futures::channel::oneshot::Sender<IndyResult<String>> =
            SUBMIT_SENDERS.lock().await.remove(&cmd_id).unwrap();
        sender.send(result.map(|()| String::new())).unwrap(); //FIXME
    }

    pub async fn send_tx(&self, handle: PoolHandle, msg: &str) -> IndyResult<String> {
        self.send_action(handle, msg, None, None).await
    }

    pub async fn send_action(
        &self,
        handle: PoolHandle,
        msg: &str,
        nodes: Option<&str>,
        timeout: Option<i32>,
    ) -> IndyResult<String> {
        trace!("send_action >>");

        let receiver = {

            let pools = self.open_pools.lock().await;

            let pool = pools
                .get(&handle)
                .ok_or(err_msg(
                    IndyErrorKind::InvalidPoolHandle,
                    format!("No pool with requested handle {:?}", handle),
                ))
                .clone()?;

            let cmd_id: CommandHandle = next_command_handle();

            let (sender, receiver) = oneshot::channel::<IndyResult<String>>();
            SUBMIT_SENDERS.lock().await.insert(cmd_id, sender);

            self._send_msg(cmd_id, msg, &pool.cmd_socket.lock().unwrap(), nodes, timeout)?;

            receiver
        };

        let res = receiver.await?;

        trace!("send_action <<< {:?}", res);

        res
    }

    //#[logfn(trace)] FIXME:
    async fn submit_ack(cmd_id: CommandHandle, result: IndyResult<String>) {
        let sender: futures::channel::oneshot::Sender<IndyResult<String>> =
            SUBMIT_SENDERS.lock().await.remove(&cmd_id).unwrap();
        sender.send(result).unwrap(); //FIXME
    }

    // FIXME: !!!

    // pub fn register_sp_parser(txn_type: &str,
    //                           parser: CustomTransactionParser, free: CustomFree) -> IndyResult<()> {
    //     if events::REQUESTS_FOR_STATE_PROOFS.contains(&txn_type) {
    //         return Err(err_msg(IndyErrorKind::InvalidStructure,
    //                            format!("Try to override StateProof parser for default TXN_TYPE {}", txn_type)));
    //     }

    //     REGISTERED_SP_PARSERS.lock()
    //         .map(|mut map| {
    //             map.insert(txn_type.to_owned(), (parser, free));
    //         })
    //         .unwrap(); // FIXME: Can we avoid unwrap?

    //     Ok(())
    // }

    // FIXME:!!!

    // pub fn get_sp_parser(txn_type: &str) -> Option<(CustomTransactionParser, CustomFree)> {
    //     let parsers = REGISTERED_SP_PARSERS.lock().unwrap(); // FIXME: Can we avoid unwrap here?
    //     parsers.get(txn_type).map(Clone::clone)
    // }

    pub async fn close(&self, handle: PoolHandle) -> IndyResult<()> {
        let pool = self.open_pools.lock().await.remove(&handle);

        let (sender, receiver) = oneshot::channel::<IndyResult<()>>();
        CLOSE_SENDERS.lock().await.insert(handle, sender);

        match pool {
            Some(pool) => self._send_msg(handle, COMMAND_EXIT, &pool.cmd_socket.lock().unwrap(), None, None)?,
            None => {
                return Err(err_msg(
                    IndyErrorKind::InvalidPoolHandle,
                    format!("No pool with requested handle {}", handle),
                ))
            }
        }

        receiver.await?
    }

    //#[logfn(trace)] FIXME:!!!
    async fn close_ack(cmd_id: CommandHandle, result: IndyResult<()>) {
        let sender: futures::channel::oneshot::Sender<IndyResult<()>> =
            CLOSE_SENDERS.lock().await.remove(&cmd_id).unwrap();
        sender.send(result).unwrap(); //FIXME
    }

    pub async fn refresh(&self, handle: PoolHandle) -> IndyResult<()> {
        self.send_action(handle, COMMAND_REFRESH, None, None)
            .await
            .map(|_| ())
    }

    fn _send_msg(
        &self,
        cmd_id: CommandHandle,
        msg: &str,
        socket: &Socket,
        nodes: Option<&str>,
        timeout: Option<i32>,
    ) -> IndyResult<()> {
        let mut buf = [0u8; 4];
        let mut buf_to = [0u8; 4];
        LittleEndian::write_i32(&mut buf, cmd_id);
        let timeout = timeout.unwrap_or(-1);
        LittleEndian::write_i32(&mut buf_to, timeout);
        if let Some(nodes) = nodes {
            Ok(socket.send_multipart(
                &[msg.as_bytes(), &buf, &buf_to, nodes.as_bytes()],
                zmq::DONTWAIT,
            )?)
        } else {
            Ok(socket.send_multipart(&[msg.as_bytes(), &buf, &buf_to], zmq::DONTWAIT)?)
        }
    }

    pub fn list(&self) -> IndyResult<Vec<serde_json::Value>> {
        let mut pool = Vec::new();
        let pool_home_path = environment::pool_home_path();

        if let Ok(entries) = fs::read_dir(pool_home_path) {
            for entry in entries {
                let dir_entry = if let Ok(dir_entry) = entry {
                    dir_entry
                } else {
                    continue;
                };
                if let Some(pool_name) = dir_entry
                    .path()
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                {
                    let json = json!({"pool":pool_name.to_owned()});
                    pool.push(json);
                }
            }
        }

        Ok(pool)
    }
}

lazy_static! {
    static ref THRESHOLD: std::sync::Mutex<u64> = std::sync::Mutex::new(600);
}

pub fn set_freshness_threshold(threshold: u64) {
    *THRESHOLD.lock().unwrap() = ::std::cmp::max(threshold, 300);
}

pub fn parse_response_metadata(response: &str) -> IndyResult<ResponseMetadata> {
    trace!(
        "indy::services::pool::parse_response_metadata << response: {}",
        response
    );
    let message: Message<serde_json::Value> = serde_json::from_str(response).to_indy(
        IndyErrorKind::InvalidTransaction,
        "Cannot deserialize transaction Response",
    )?;

    let response_object: Reply<serde_json::Value> = _handle_response_message_type(message)?;
    let response_result = response_object.result();

    let response_metadata = match response_result["ver"].as_str() {
        None => _parse_transaction_metadata_v0(&response_result),
        Some("1") => _parse_transaction_metadata_v1(&response_result),
        ver => {
            return Err(err_msg(
                IndyErrorKind::InvalidTransaction,
                format!("Unsupported transaction response version: {:?}", ver),
            ))
        }
    };

    trace!(
        "indy::services::pool::parse_response_metadata >> response_metadata: {:?}",
        response_metadata
    );

    Ok(response_metadata)
}

pub fn get_last_signed_time(response: &str) -> Option<u64> {
    let c = parse_response_metadata(response);
    c.ok().and_then(|resp| resp.last_txn_time)
}

fn _handle_response_message_type<T>(message: Message<T>) -> IndyResult<Reply<T>>
where
    T: DeserializeOwned + ::std::fmt::Debug,
{
    trace!("handle_response_message_type >>> message {:?}", message);

    match message {
        Message::Reject(response) | Message::ReqNACK(response) => Err(err_msg(
            IndyErrorKind::InvalidTransaction,
            format!("Transaction has been failed: {:?}", response.reason),
        )),
        Message::Reply(reply) => Ok(reply),
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
        last_txn_time: message["multiSignature"]["signedState"]["stateMetadata"]["timestamp"]
            .as_u64(),
        last_seq_no: None,
    }
}

pub fn pool_create_pair_of_sockets(addr: &str) -> (zmq::Socket, zmq::Socket) {
    let zmq_ctx = zmq::Context::new();
    let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();
    let recv_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();

    let inproc_sock_name: String = format!("inproc://{}", addr);
    recv_cmd_sock.bind(inproc_sock_name.as_str()).unwrap();
    send_cmd_sock.connect(inproc_sock_name.as_str()).unwrap();
    (send_cmd_sock, recv_cmd_sock)
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub async fn fake_pool_handle_for_poolsm() -> (
        indy_api_types::PoolHandle,
        oneshot::Receiver<IndyResult<indy_api_types::PoolHandle>>,
    ) {
        let pool_handle = indy_utils::next_pool_handle();
        let (sender, receiver) = oneshot::channel();
        super::POOL_HANDLE_SENDERS
            .lock()
            .await
            .insert(pool_handle, sender);
        (pool_handle, receiver)
    }

    pub async fn fake_cmd_id() -> (
        indy_api_types::CommandHandle,
        oneshot::Receiver<IndyResult<String>>,
    ) {
        let cmd_id = indy_utils::next_command_handle();
        let (sender, receiver) = oneshot::channel();
        super::SUBMIT_SENDERS.lock().await.insert(cmd_id, sender);
        (cmd_id, receiver)
    }

    pub async fn fake_pool_handle_for_close_cmd() -> (
        indy_api_types::CommandHandle,
        oneshot::Receiver<IndyResult<()>>,
    ) {
        let pool_handle = indy_utils::next_command_handle();
        let (sender, receiver) = oneshot::channel();
        super::CLOSE_SENDERS
            .lock()
            .await
            .insert(pool_handle, sender);
        (pool_handle, receiver)
    }
}

#[cfg(test)]
pub mod tests {
    use std::thread;

    use futures::executor::block_on;

    use crate::domain::ledger::request::ProtocolVersion;
    use crate::services::pool::types::*;
    use crate::utils::test;

    use super::*;

    const TEST_PROTOCOL_VERSION: usize = 2;

    fn _set_protocol_version(version: usize) {
        ProtocolVersion::set(version);
    }

    mod send_sync {
        use super::*;

        #[async_std::test]
        async fn pool_service_sync_send() {
            use futures::{channel::oneshot, executor::ThreadPool};
            use std::sync::Arc;

            let executor = ThreadPool::new().expect("Failed to new ThreadPool");
            let service = Arc::new(Box::new(PoolService::new()));
            let (tx, rx) = oneshot::channel::<IndyResult<()>>();
            let s = service.clone();

            let future = async move {
                let res = s.create("test", None);
                tx.send(res).unwrap();
            };

            executor.spawn_ok(future);
            let res = rx.await;
            println!("----> {:?}", res);
        }
    }

    mod pool_service {
        use std::path;

        use libc::c_char;

        use indy_api_types::{ErrorCode, INVALID_POOL_HANDLE};

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

        #[async_std::test]
        async fn pool_service_close_works() {
            test::cleanup_storage("pool_service_close_works");

            let ps = PoolService::new();
            let pool_id = next_pool_handle();
            let (send_cmd_sock, recv_cmd_sock) =
                pool_create_pair_of_sockets("pool_service_close_works");

            ps.open_pools.lock().await.insert(
                pool_id,
                Arc::new(ZMQPool::new(
                    Pool::new("", pool_id, PoolOpenConfig::default()),
                    send_cmd_sock,
                )),
            );

            let pool_mock = thread::spawn(move || {
                let recv = recv_cmd_sock.recv_multipart(0).unwrap();
                assert_eq!(recv.len(), 3);
                assert_eq!(COMMAND_EXIT, String::from_utf8(recv[0].clone()).unwrap());
                assert_eq!(pool_id, LittleEndian::read_i32(recv[1].as_slice()));
                block_on(PoolService::close_ack(pool_id, Ok(())));
            });

            block_on(ps.close(pool_id)).unwrap();
            pool_mock.join().unwrap();
        }

        #[async_std::test]
        async fn pool_service_refresh_works() {
            test::cleanup_storage("pool_service_refresh_works");

            let ps = PoolService::new();
            let pool_id = next_pool_handle();
            let (send_cmd_sock, recv_cmd_sock) =
                pool_create_pair_of_sockets("pool_service_refresh_works");

            ps.open_pools.lock().await.insert(
                pool_id,
                Arc::new(ZMQPool::new(
                    Pool::new("", pool_id, PoolOpenConfig::default()),
                    send_cmd_sock,
                )),
            );

            let pool_mock = thread::spawn(move || {
                assert_eq!(
                    1,
                    zmq::poll(&mut [recv_cmd_sock.as_poll_item(zmq::POLLIN)], 10_000).unwrap()
                );
                let recv = recv_cmd_sock.recv_multipart(zmq::DONTWAIT).unwrap();
                assert_eq!(recv.len(), 3);
                assert_eq!(COMMAND_REFRESH, String::from_utf8(recv[0].clone()).unwrap());
                let cmd_id = LittleEndian::read_i32(recv[1].as_slice());
                block_on(PoolService::refresh_ack(cmd_id, Ok(())));
            });

            ps.refresh(pool_id).await.unwrap();
            pool_mock.join().unwrap();
        }

        #[async_std::test]
        async fn pool_service_delete_works() {
            test::cleanup_storage("pool_service_delete_works");

            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works";
            let path: path::PathBuf = environment::pool_path(pool_name);
            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            ps.delete(pool_name).await.unwrap();
            assert!(!path.exists());

            test::cleanup_storage("pool_service_delete_works");
        }

        #[async_std::test]
        async fn pool_service_delete_works_for_opened() {
            test::cleanup_storage("pool_service_delete_works_for_opened");

            let (send_cmd_sock, _recv_cmd_sock) =
                pool_create_pair_of_sockets("pool_service_delete_works_for_opened");
            let ps = PoolService::new();
            let pool_name = "pool_service_delete_works_for_opened";
            let path: path::PathBuf = environment::pool_path(pool_name);
            let pool_id = next_pool_handle();

            let pool = Pool::new(pool_name, pool_id, PoolOpenConfig::default());
            ps.open_pools
                .lock()
                .await
                .insert(pool_id, Arc::new(ZMQPool::new(pool, send_cmd_sock)));

            fs::create_dir_all(path.as_path()).unwrap();
            assert!(path.exists());
            let res = ps.delete(pool_name).await;
            assert_eq!(IndyErrorKind::InvalidState, res.unwrap_err().kind());
            assert!(path.exists());

            test::cleanup_storage("pool_service_delete_works_for_opened");
        }

        #[async_std::test]
        async fn pool_send_tx_works() {
            test::cleanup_storage("pool_send_tx_works");

            let name = "test";
            let (send_cmd_sock, recv_cmd_sock) = pool_create_pair_of_sockets("pool_send_tx_works");
            let pool_id = next_pool_handle();
            let pool = Pool::new(name, pool_id, PoolOpenConfig::default());
            let ps = PoolService::new();

            ps.open_pools
                .lock()
                .await
                .insert(pool_id, Arc::new(ZMQPool::new(pool, send_cmd_sock)));

            let test_data = "str_instead_of_tx_json";

            let pool_mock = thread::spawn(move || {
                assert_eq!(
                    1,
                    zmq::poll(&mut [recv_cmd_sock.as_poll_item(zmq::POLLIN)], 10_000).unwrap()
                );

                assert_eq!(
                    recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(),
                    test_data
                );

                let cmd_id = LittleEndian::read_i32(
                    recv_cmd_sock.recv_bytes(zmq::DONTWAIT).unwrap().as_slice(),
                );
                block_on(PoolService::submit_ack(cmd_id, Ok("".to_owned())));
            });

            ps.send_tx(pool_id, test_data).await.unwrap();
            pool_mock.join().unwrap();
        }

        #[async_std::test]
        async fn pool_send_tx_works_for_closed_socket() {
            test::cleanup_storage("pool_send_tx_works_for_closed_socket");

            let name = "test";
            let zmq_ctx = zmq::Context::new();
            let send_cmd_sock = zmq_ctx.socket(zmq::SocketType::PAIR).unwrap();

            let pool_id = next_pool_handle();
            let pool = Pool::new(name, pool_id, PoolOpenConfig::default());
            let ps = PoolService::new();

            ps.open_pools
                .lock()
                .await
                .insert(pool_id, Arc::new(ZMQPool::new(pool, send_cmd_sock)));

            let res = ps.send_tx(pool_id, "test_data").await;
            assert_eq!(IndyErrorKind::IOError, res.unwrap_err().kind());
        }

        #[async_std::test]
        async fn pool_send_tx_works_for_invalid_handle() {
            test::cleanup_storage("pool_send_tx_works_for_invalid_handle");
            let ps = PoolService::new();
            let res = ps.send_tx(INVALID_POOL_HANDLE, "txn").await;
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        #[async_std::test]
        async fn pool_send_action_works() {
            test::cleanup_storage("pool_send_action_works");

            let (send_cmd_sock, recv_cmd_sock) =
                pool_create_pair_of_sockets("pool_send_action_works");

            let pool_id = next_pool_handle();
            let pool = Pool::new("pool_send_action_works", pool_id, PoolOpenConfig::default());
            let ps = PoolService::new();

            ps.open_pools
                .lock()
                .await
                .insert(pool_id, Arc::new(ZMQPool::new(pool, send_cmd_sock)));

            let test_data = "str_instead_of_tx_json";

            let pool_mock = thread::spawn(move || {
                assert_eq!(
                    1,
                    zmq::poll(&mut [recv_cmd_sock.as_poll_item(zmq::POLLIN)], 10_000).unwrap()
                );
                assert_eq!(
                    recv_cmd_sock.recv_string(zmq::DONTWAIT).unwrap().unwrap(),
                    test_data
                );
                let cmd_id = LittleEndian::read_i32(
                    recv_cmd_sock.recv_bytes(zmq::DONTWAIT).unwrap().as_slice(),
                );
                block_on(PoolService::submit_ack(cmd_id, Ok("".to_owned())));
            });

            ps.send_action(pool_id, test_data, None, None)
                .await
                .unwrap();
//            pool_mock.join().unwrap();
        }

        #[test]
        fn pool_close_works_for_invalid_handle() {
            test::cleanup_storage("pool_close_works_for_invalid_handle");
            let ps = PoolService::new();
            let res = block_on(ps.close(INVALID_POOL_HANDLE));
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        #[test]
        fn pool_refresh_works_for_invalid_handle() {
            test::cleanup_storage("pool_refresh_works_for_invalid_handle");
            let ps = PoolService::new();
            let res = block_on(ps.refresh(INVALID_POOL_HANDLE));
            assert_eq!(IndyErrorKind::InvalidPoolHandle, res.unwrap_err().kind());
        }

        // FIXME:
        // #[test]
        // fn pool_register_sp_parser_works() {
        //     test::cleanup_storage("pool_register_sp_parser_works");
        //     REGISTERED_SP_PARSERS.lock().unwrap().clear();
        //     extern "C" fn test_sp(
        //         _reply_from_node: *const c_char,
        //         _parsed_sp: *mut *const c_char,
        //     ) -> ErrorCode {
        //         ErrorCode::Success
        //     }
        //     extern "C" fn test_free(_data: *const c_char) -> ErrorCode {
        //         ErrorCode::Success
        //     }
        //     PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
        // }

        // FIMXE:
        // #[test]
        // fn pool_get_sp_parser_works() {
        //     test::cleanup_storage("pool_get_sp_parser_works");
        //     REGISTERED_SP_PARSERS.lock().unwrap().clear();
        //     extern "C" fn test_sp(
        //         _reply_from_node: *const c_char,
        //         _parsed_sp: *mut *const c_char,
        //     ) -> ErrorCode {
        //         ErrorCode::Success
        //     }
        //     extern "C" fn test_free(_data: *const c_char) -> ErrorCode {
        //         ErrorCode::Success
        //     }
        //     PoolService::register_sp_parser("test", test_sp, test_free).unwrap();
        //     PoolService::get_sp_parser("test").unwrap();
        // }

        // FIXME:
        // #[test]
        // fn pool_get_sp_parser_works_for_invalid_name() {
        //     test::cleanup_storage("pool_get_sp_parser_works_for_invalid_name");
        //     REGISTERED_SP_PARSERS.lock().unwrap().clear();
        //     assert_eq!(None, PoolService::get_sp_parser("test"));
        // }
    }

    #[async_std::test]
    async fn pool_drop_works_for_after_close() {
        use crate::utils::test;
        use std::time;

        test::cleanup_storage("pool_drop_works_for_after_close");

        async fn drop_test() {
            _set_protocol_version(TEST_PROTOCOL_VERSION);
            let ps = PoolService::new();

            let pool_name = "pool_drop_works_for_after_close";
            let gen_txn = test::gen_txns()[0].clone();

            let (send_cmd_sock, recv_cmd_sock) = pool_create_pair_of_sockets("drop_test");

            // create minimal fs config stub before Pool::new()
            let mut pool_path = environment::pool_path(pool_name);
            fs::create_dir_all(&pool_path).unwrap();
            pool_path.push(pool_name);
            pool_path.set_extension("txn");
            let mut file = fs::File::create(pool_path).unwrap();
            file.write(&gen_txn.as_bytes()).unwrap();

            let pool_id = next_pool_handle();
            let mut pool = Pool::new(pool_name, pool_id, PoolOpenConfig::default());
            pool.work(recv_cmd_sock);

            ps.open_pools
                .lock()
                .await
                .insert(pool_id, Arc::new(ZMQPool::new(pool, send_cmd_sock)));

            thread::sleep(time::Duration::from_secs(1));
            ps.close(pool_id).await.unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }

        drop_test().await;
        test::cleanup_storage("pool_drop_works_for_after_close");
    }

    pub mod nodes_emulator {
        use indy_utils::crypto::ed25519_sign;
        use rust_base58::{FromBase58, ToBase58};

        use super::*;

        use crate::services::pool::request_handler::DEFAULT_GENERATOR;
        use ursa::bls::{Generator, SignKey, VerKey};

        pub static POLL_TIMEOUT: i64 = 1_000; /* in ms */

        pub fn node() -> NodeTransactionV1 {
            let blskey = VerKey::new(
                &Generator::from_bytes(&DEFAULT_GENERATOR.from_base58().unwrap()).unwrap(),
                &SignKey::new(None).unwrap(),
            )
            .unwrap()
            .as_bytes()
            .to_base58();

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
                    metadata: TxnMetadata {
                        req_id: None,
                        from: String::new(),
                    },
                },
                txn_metadata: Metadata {
                    creation_time: None,
                    seq_no: None,
                    txn_id: None,
                },
                req_signature: ReqSignature {
                    type_: None,
                    values: None,
                },
                ver: String::new(),
            }
        }

        pub fn node_2() -> NodeTransactionV1 {
            let blskey = VerKey::new(
                &Generator::from_bytes(&DEFAULT_GENERATOR.from_base58().unwrap()).unwrap(),
                &SignKey::new(None).unwrap(),
            )
            .unwrap()
            .as_bytes()
            .to_base58();

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
                    metadata: TxnMetadata {
                        req_id: None,
                        from: String::new(),
                    },
                },
                txn_metadata: Metadata {
                    creation_time: None,
                    seq_no: None,
                    txn_id: None,
                },
                req_signature: ReqSignature {
                    type_: None,
                    values: None,
                },
                ver: String::new(),
            }
        }

        pub fn start(gt: &mut NodeTransactionV1) -> zmq::Socket {
            let (vk, sk) = ed25519_sign::create_key_pair_for_signature(None).unwrap();
            let pkc = ed25519_sign::vk_to_curve25519(&vk).expect("Invalid pkc");
            let skc = ed25519_sign::sk_to_curve25519(&sk).expect("Invalid skc");
            let ctx = zmq::Context::new();
            let s: zmq::Socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();

            gt.txn.data.dest = (&vk[..]).to_base58();

            s.set_curve_publickey(&zmq::z85_encode(&pkc[..]).unwrap().as_bytes())
                .expect("set public key");
            s.set_curve_secretkey(&zmq::z85_encode(&skc[..]).unwrap().as_bytes())
                .expect("set secret key");
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
                s.send_multipart(&[v[0].as_slice(), "po".as_bytes()], zmq::DONTWAIT)
                    .expect("send mulp");
                Some(String::from_utf8(v[1].clone()).unwrap())
            } else {
                warn!("Node emulator poll return {}", poll_res);
                None
            }
        }
    }
}
