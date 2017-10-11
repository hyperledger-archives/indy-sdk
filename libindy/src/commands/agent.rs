#![warn(unused_variables)]

extern crate rust_base58;
extern crate serde_json;
extern crate zmq_pw as zmq;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;

use self::rust_base58::{FromBase58, ToBase58};

use commands::{Command, CommandExecutor};
use commands::ledger::LedgerCommand;
use commands::utils::check_wallet_and_pool_handles_consistency;
use errors::indy::IndyError;
use errors::common::CommonError;
use services::agent::AgentService;
use services::ledger::LedgerService;
use services::ledger::types::{Reply, GetNymResultData, GetNymReplyResult};
use services::pool::PoolService;
use services::signus::types::{MyDid, TheirDid};
use services::wallet::WalletService;
use utils::crypto::box_::CryptoBox;
use utils::json::JsonDecodable;
use utils::sequence::SequenceUtils;
use utils::crypto::verkey_builder::build_full_verkey;

pub type AgentConnectCB = Box<Fn(Result<i32, IndyError>) + Send>;
pub type AgentMessageCB = Box<Fn(Result<(i32, String), IndyError>) + Send>;

pub enum AgentCommand {
    Connect(
        i32, // pool handle
        i32, // wallet handle
        String, // sender did
        String, // receiver did
        AgentConnectCB, // connect cb
        AgentMessageCB, // message cb
    ),
    ResumeConnectProcess(
        i32, // cmd handle
        Result<(MyConnectInfo, String /* get DDO result JSON */), IndyError>
    ),
    ConnectAck(
        i32, // cmd handle (eq conn handle)
        Result<i32, CommonError> // conn handle or error
    ),
    CloseConnection(
        i32, // connection handle
        Box<Fn(Result<(), IndyError>) + Send>, // close conn cb
    ),
    CloseConnectionAck(
        i32, // close cmd handle
        Result<(), CommonError>,
    ),
    Listen(
        String, // endpoint
        Box<Fn(Result<i32, IndyError>) + Send>, // listen cb
        Box<Fn(Result<(i32, i32, String, String), IndyError>) + Send>, // connect cb
        AgentMessageCB, // message cb
    ),
    ListenAck(
        i32, // cmd handle (eq listener handle)
        Result<i32, CommonError> // listener handle or error
    ),
    ListenerCheckConnect(
        String, // did
        String, // pk
        i32, // listener handle
        i32, // pool handle
        i32, // wallet handle
    ),
    ListenerResumeCheckConnect(
        i32, // listener handle
        String, // did
        String, // pk
        Result<String, IndyError> // get nym result
    ),
    ListenerOnConnect(
        i32, // listener handle
        Result<(i32, i32, String, String), CommonError>, // (listener handle, new connection handle, sender and receiver did) or error
    ),
    MessageReceived(
        i32, // connection handle
        Result<(i32, String), CommonError> // result for message
    ),
    CloseListener(
        i32, // listener handle
        Box<Fn(Result<(), IndyError>) + Send>, // close listener cb
    ),
    CloseListenerAck(
        i32, // close cmd handle
        Result<(), CommonError>,
    ),
    ListenerAddIdentity(
        i32, // listener handle
        i32, // pool handle
        i32, // wallet handle
        String, // did
        Box<Fn(Result<(), IndyError>) + Send>, // add identity cb
    ),
    ListenerAddIdentityAck(
        i32, // cmd handle
        Result<(), CommonError>,
    ),
    ListenerRmIdentity(
        i32, // listener handle
        i32, // wallet handle
        String, // did
        Box<Fn(Result<(), IndyError>) + Send>, // rm identity cb
    ),
    ListenerRmIdentityAck(
        i32, // cmd handle
        Result<(), CommonError>,
    ),
    Send(
        i32, // connection handle
        Option<String>, // message
        Box<Fn(Result<(), IndyError>) + Send>, // send cb
    ),
    SendAck(
        i32, // send cmd handle
        Result<(), CommonError>,
    )
}

pub struct AgentCommandExecutor {
    agent_service: Rc<AgentService>,
    ledger_service: Rc<LedgerService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

    out_connections: RefCell<HashMap<i32, AgentMessageCB>>,
    listeners: RefCell<HashMap<i32, Listener>>,

    listen_callbacks: RefCell<HashMap<i32, (
        Box<Fn(Result<i32, IndyError>) + Send>, // listen cb
        Listener
    )>>,
    add_rm_identity_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    connect_callbacks: RefCell<HashMap<i32, (AgentConnectCB, AgentMessageCB)>>,
    send_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    close_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
}

struct Listener {
    on_connect: Box<Fn(Result<(i32, i32, String, String), IndyError>) + Send>,
    on_msg: AgentMessageCB,
    conn_handles: HashSet<i32>,
}

impl AgentCommandExecutor {
    pub fn new(agent_service: Rc<AgentService>, ledger_service: Rc<LedgerService>, pool_service: Rc<PoolService>, wallet_service: Rc<WalletService>) -> AgentCommandExecutor {
        AgentCommandExecutor {
            agent_service: agent_service,
            ledger_service: ledger_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
            out_connections: RefCell::new(HashMap::new()),
            listeners: RefCell::new(HashMap::new()),
            listen_callbacks: RefCell::new(HashMap::new()),
            add_rm_identity_callbacks: RefCell::new(HashMap::new()),
            connect_callbacks: RefCell::new(HashMap::new()),
            send_callbacks: RefCell::new(HashMap::new()),
            close_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, agent_cmd: AgentCommand) {
        match agent_cmd {
            AgentCommand::Connect(pool_handle, wallet_handle, sender_did, receiver_did, connect_cb, message_cb) => {
                info!(target: "agent_command_executor", "Connect command received");
                self.connect(pool_handle, wallet_handle, sender_did, receiver_did, connect_cb, message_cb)
            }
            AgentCommand::ResumeConnectProcess(cmd_id, res) => {
                info!(target: "agent_command_executor", "ResumeConnectProcess command received");
                self.resume_connect_process(cmd_id, res);
            }
            AgentCommand::ConnectAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ConnectAck command received");
                self.on_connect_ack(cmd_id, res);
            }
            AgentCommand::Listen(endpoint, listen_cb, connect_cb, message_cb) => {
                info!(target: "agent_command_executor", "Listen command received");
                self.listen(endpoint, listen_cb, connect_cb, message_cb);
            }
            AgentCommand::ListenAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ListenAck command received");
                self.on_listen_ack(cmd_id, res);
            }
            AgentCommand::ListenerCheckConnect(did, pk, listener_handle, pool_handle, wallet_handle) => {
                info!(target: "agent_command_executor", "ListenerCheckConnect command received");
                self.check_connect(did, pk, listener_handle, pool_handle, wallet_handle);
            }
            AgentCommand::ListenerResumeCheckConnect(listener_handle, did, pk, res) => {
                info!(target: "agent_command_executor", "ListenerResumeCheckConnect command received");
                self.resume_check_connect(listener_handle, did, pk, res);
            }
            AgentCommand::ListenerOnConnect(listener_id, res) => {
                info!(target: "agent_command_executor", "ListenerOnConnect command received");
                self.on_client_connected(listener_id, res);
            }
            AgentCommand::MessageReceived(connection_id, res) => {
                info!(target: "agent_command_executor", "ListenerOnConnect command received");
                self.on_message_received(connection_id, res);
            }
            AgentCommand::ListenerAddIdentity(listener_handle, pool_handle, wallet_handle, did, cb) => {
                info!(target: "agent_command_executor", "ListenerAddIdentity command received");
                self.add_identity(listener_handle, pool_handle, wallet_handle, did, cb);
            }
            AgentCommand::ListenerAddIdentityAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ListenerAddIdentityAck command received");
                self.on_add_rm_identity_ack(cmd_id, res);
            }
            AgentCommand::ListenerRmIdentity(listener_handle, wallet_handle, did, cb) => {
                info!(target: "agent_command_executor", "ListenerRmIdentity command received");
                self.rm_identity(listener_handle, wallet_handle, did, cb);
            }
            AgentCommand::ListenerRmIdentityAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ListenerRmIdentityAck command received");
                self.on_add_rm_identity_ack(cmd_id, res);
            }
            AgentCommand::Send(connection_id, msg, cb) => {
                info!(target: "agent_command_executor", "Send command received");
                self.send(connection_id, msg, cb)
            }
            AgentCommand::SendAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "SendAck command received");
                self.on_send_ack(cmd_id, res);
            }
            AgentCommand::CloseConnection(connection_id, cb) => {
                info!(target: "agent_command_executor", "CloseConnection command received");
                self.close_connection_or_listener(connection_id, cb, false)
            }
            AgentCommand::CloseConnectionAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "CloseConnectionAck command received");
                self.on_close_connection_ack(cmd_id, res);
            }
            AgentCommand::CloseListener(listener_id, cb) => {
                info!(target: "agent_command_executor", "CloseListener command received");
                self.close_connection_or_listener(listener_id, cb, true)
            }
            AgentCommand::CloseListenerAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "CloseListenerAck command received");
                self.on_close_listener_ack(cmd_id, res);
            }
        }
    }

    fn connect(&self, pool_handle: i32, wallet_handle: i32,
               sender_did: String, receiver_did: String,
               connect_cb: AgentConnectCB, message_cb: AgentMessageCB) {
        match self.get_connection_info_local(wallet_handle, &sender_did, &receiver_did) {
            Ok(info) => match info {
                (my_info, Some(info)) => self.do_connect(my_info, info, connect_cb, message_cb),
                (my_info, None) => self.request_connection_info_from_ledger(pool_handle,
                                                                            wallet_handle,
                                                                            my_info,
                                                                            connect_cb, message_cb),
            },
            Err(err) => connect_cb(Err(err))
        }
    }

    fn do_connect(&self, my_info: MyConnectInfo, info: ConnectInfo,
                  connect_cb: AgentConnectCB, message_cb: AgentMessageCB) {
        debug!("AgentCommandExecutor::connect try to service.connect with {:?}", info);
        let result = self.agent_service
            .connect(my_info.sender_did.as_str(), my_info.receiver_did.as_str(),
                     my_info.secret_key.as_str(), my_info.public_key.as_str(),
                     info.endpoint.as_str(), info.server_key.as_str())
            .map_err(From::from)
            .and_then(|conn_handle| {
                match self.connect_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, conn_handle)),
                    Err(err) => Err(IndyError::CommonError(CommonError::InvalidState(err.description().to_string()))),
                }
            });
        match result {
            Err(err) => { connect_cb(Err(err).map_err(map_err_err!())); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, (connect_cb, message_cb)); /* TODO check if map contains same key */ }
        };
    }

    fn resume_connect_process(&self, cmd_id: i32, res: Result<(MyConnectInfo, String), IndyError>) {
        let cbs = self.connect_callbacks.borrow_mut().remove(&cmd_id);
        if let Some((connect_cb, on_msg)) = cbs {
            let res = res.and_then(|(my_info, attrib_resp_json)| -> Result<(MyConnectInfo, ConnectInfo), IndyError> {
                let attrib_resp: serde_json::Value = serde_json::from_str(attrib_resp_json.as_str()).map_err(|err|
                    CommonError::InvalidStructure(
                        format!("Can't parse get ATTRIB response json {}", err.description())))?; // TODO change error type?
                let attrib_data_json = attrib_resp["result"]["data"].as_str().ok_or(
                    CommonError::InvalidStructure(
                        format!("Can't parse get ATTRIB response - sub-field result.data not found: {}", attrib_resp_json)))?; // TODO
                let attrib_data: AttribData = AttribData::from_json(attrib_data_json).map_err(|err|
                    CommonError::InvalidStructure(
                        format!("Can't parse get ATTRIB response data {}", err.description())))?; // TODO
                let conn_info = ConnectInfo {
                    endpoint: attrib_data.endpoint.ha,
                    server_key: attrib_data.endpoint.verkey,
                };
                Ok((my_info, conn_info))
            });
            match res {
                Ok((my_info, conn_info)) => self.do_connect(my_info, conn_info, connect_cb, on_msg),
                Err(err) => connect_cb(Err(err).map_err(map_err_trace!()))
            }
        } else {
            error!("Can't handle ResumeConnectProcess cmd - callback not found for {}", cmd_id);
        }
    }

    fn get_connection_info_local(&self, wallet_handle: i32, sender_did: &String, receiver_did: &String)
                                 -> Result<(MyConnectInfo, Option<ConnectInfo>), IndyError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", sender_did))?;
        let my_did: MyDid = MyDid::from_json(&my_did_json)
            .map_err(|_| CommonError::InvalidState((format!("Invalid my did json"))))?;
        let my_connect_info = MyConnectInfo {
            sender_did: sender_did.clone(),
            receiver_did: receiver_did.clone(),
            secret_key: my_did.sk,
            public_key: my_did.pk,
        };

        let their_did_json = self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", receiver_did));
        let their_did_json = if let Ok(their_did_json) = their_did_json {
            their_did_json
        } else {
            /* TODO match Ok/NotFound/OtherErr ? */
            return Ok((my_connect_info, None));
        };

        let their_did: TheirDid = TheirDid::from_json(&their_did_json)
            .map_err(|_| CommonError::InvalidState((format!("Invalid their did json"))))?;
        if let (Some(endpoint), Some(pk)) = (their_did.endpoint, their_did.pk) {
            Ok((my_connect_info,
                Some(ConnectInfo {
                    endpoint: endpoint,
                    server_key: pk,
                })))
        } else {
            Ok((my_connect_info, None))
        }
    }

    fn request_connection_info_from_ledger(&self, pool_handle: i32, wallet_handle: i32,
                                           my_conn_info: MyConnectInfo,
                                           connect_cb: AgentConnectCB, message_cb: AgentMessageCB) {
        check_wallet_and_pool_handles_consistency!(self.wallet_service, self.pool_service, wallet_handle, pool_handle, connect_cb);
        let attrib_request = match self.ledger_service
            .build_get_attrib_request(my_conn_info.sender_did.as_str(), /* TODO use DDO request */
                                      my_conn_info.receiver_did.as_str(),
                                      "endpoint") {
            Ok(attrib_request) => attrib_request,
            Err(err) => {
                return connect_cb(Err(IndyError::from(err)));
            }
        };
        let cmd_id = SequenceUtils::get_next_id();
        self.connect_callbacks.borrow_mut().insert(cmd_id, (connect_cb, message_cb));
        CommandExecutor::instance().send(Command::Ledger(LedgerCommand::SignAndSubmitRequest(
            pool_handle, wallet_handle, my_conn_info.sender_did.clone(), attrib_request.to_string(),
            Box::new(move |res: Result<String, IndyError>| {
                let res = res.map(|attrib_resp| { (my_conn_info.clone(), attrib_resp) });
                CommandExecutor::instance().send(Command::Agent(
                    AgentCommand::ResumeConnectProcess(cmd_id, res))).unwrap();
            })))).unwrap();
    }

    fn on_connect_ack(&self, cmd_id: i32, res: Result<i32, CommonError>) {
        if let Some(cbs) = self.connect_callbacks.borrow_mut().remove(&cmd_id) {
            if let &Ok(conn_handle) = &res {
                self.out_connections.borrow_mut().insert(conn_handle, cbs.1); /* TODO check insert result */
            }
            cbs.0(res.map_err(map_err_err!()).map_err(From::from));
        } else {
            error!("Can't handle ConnectAck cmd - callback not found for {}", cmd_id);
            return;
        }
    }

    fn listen(&self, endpoint: String,
              listen_cb: Box<Fn(Result<i32, IndyError>) + Send>,
              connect_cb: Box<Fn(Result<(i32, i32, String, String), IndyError>) + Send>,
              message_cb: AgentMessageCB) {
        let result = self.agent_service
            .listen(endpoint.as_str())
            .and_then(|cmd_id| {
                match self.listen_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, cmd_id)),
                    Err(err) => Err(CommonError::InvalidState(err.description().to_string())),
                }
            });
        match result {
            Err(err) => listen_cb(Err(From::from(err)).map_err(map_err_err!())),
            Ok((mut cbs, handle)) => {
                cbs.insert(handle, (listen_cb,
                                    Listener {
                                        on_connect: connect_cb,
                                        on_msg: message_cb,
                                        conn_handles: HashSet::new()
                                    })); /* TODO check if map contains same key */
            }
        };
    }

    fn on_listen_ack(&self, cmd_id: i32, res: Result<i32, CommonError>) {
        if let Some(cbs) = self.listen_callbacks.borrow_mut().remove(&cmd_id) {
            if let Ok(listener_handle) = res {
                self.listeners.borrow_mut().insert(listener_handle, cbs.1);
            }
            cbs.0(res.map_err(map_err_err!()).map_err(From::from))
        } else {
            error!("Can't handle ListenAck cmd - callback not found for {}", cmd_id);
        }
    }

    fn on_client_connected(&self, listener_id: i32, res: Result<(i32, i32, String, String), CommonError>) {
        if let Some(mut cbs) = self.listeners.borrow_mut().get_mut(&listener_id) {
            if let Ok((_, connection_handle, _, _)) = res {
                cbs.conn_handles.insert(connection_handle);
            }
            (cbs.on_connect)(res.map_err(map_err_err!()).map_err(From::from));
        } else {
            error!("Can't handle ListenerOnConnect cmd - callback not found for {}", listener_id);
        }
    }

    fn on_message_received(&self, connection_id: i32, res: Result<(i32, String), CommonError>) {
        let listeners = self.listeners.borrow();
        let out_connections = self.out_connections.borrow();
        let cb = match listeners.iter().find(|&(_, listener)| listener.conn_handles.contains(&connection_id)) {
            Some((_, listener)) => Some(&listener.on_msg),
            None => out_connections.get(&connection_id),
        };
        if let Some(cb) = cb {
            cb(res.map_err(From::from));
        } else {
            error!("Can't handle MessageReceived cmd - callback not found for {}", connection_id);
        }
    }

    fn check_connect(&self, did: String, pk: String, listener_handle: i32, pool_handle: i32, wallet_handle: i32) {
        trace!("check_connect >> for did {}, pk {}, listener_handle {}, pool_handle {}, wallet_handle {}", did, pk, listener_handle, pool_handle, wallet_handle);
        if let Ok(Some(actual_pk)) = self.get_info_for_check_connect(did.clone(), wallet_handle) {
            self.do_check_connect(listener_handle, did.as_str(), pk.as_str(), Some(actual_pk.as_str()));
        } else {
            self.request_check_connect_info_from_ledger(pool_handle, wallet_handle, listener_handle, did.clone(), pk.clone())
                .unwrap_or_else(|_| self.do_check_connect(listener_handle, did.as_str(), pk.as_str(), None));
        }
    }

    fn resume_check_connect(&self, listener_handle: i32, did: String, pk: String, res: Result<String, IndyError>) {
        trace!("resume_check_connect >> listener {}, did {}, pk {}, res {:?}", listener_handle, did, pk, res);
        let res = res.and_then(|get_nym_response| -> Result<String, IndyError> {
            let get_nym_response: Reply<GetNymReplyResult> = Reply::from_json(&get_nym_response)
                .map_err(map_err_trace!())
                .map_err(|_| CommonError::InvalidState(format!("Invalid their did json")))?;

            let gen_nym_result_data: GetNymResultData = GetNymResultData::from_json(&get_nym_response.result.data)
                .map_err(map_err_trace!())
                .map_err(|_| CommonError::InvalidState(format!("Invalid their did json")))?;

            trace!("parsed get_nym_result_data {:?}", gen_nym_result_data);

            let pk = CryptoBox::vk_to_curve25519(
                build_full_verkey(&gen_nym_result_data.dest, &gen_nym_result_data.verkey)
                    .unwrap()
                    .as_slice())
                .unwrap().to_base58();

            Ok(pk)
        });
        self.do_check_connect(listener_handle, did.as_str(), pk.as_str(), res.ok().as_ref().map(String::as_str));
    }

    fn do_check_connect(&self, listener_handle: i32, did: &str, received_pk: &str, actual_pk: Option<&str>) {
        let check_result = actual_pk.ok_or(())
            .and_then(|actual_pk| _base58_to_z85(actual_pk)
                .map_err(map_err_trace!()).map_err(|_| ()))
            .map(|pk| pk.eq(&received_pk))
            .unwrap_or(false);

        self.agent_service.on_connect_checked(listener_handle, did, check_result).unwrap();
    }

    fn get_info_for_check_connect(&self, did: String, wallet_handle: i32) -> Result<Option<String>, IndyError> {
        let td_json = self.wallet_service.get(wallet_handle, format!("their_did::{}", did).as_str())?;
        let td: TheirDid = TheirDid::from_json(td_json.as_str()).unwrap();
        Ok(Some(td.pk.unwrap()))
    }

    fn request_check_connect_info_from_ledger(&self, pool_handle: i32, wallet_handle: i32, listener_handle: i32, did: String, pk: String) -> Result<(), IndyError> {
        check_wallet_and_pool_handles_consistency(self.wallet_service.clone(), self.pool_service.clone(), wallet_handle, pool_handle)?;
        let get_nym_request = match self.ledger_service
            .build_get_nym_request(did.as_str() /* TODO receiver did */, did.as_str()) {
            Ok(get_nym_request) => get_nym_request,
            Err(err) => return Err(IndyError::from(err))
        };

        CommandExecutor::instance().send(Command::Ledger(LedgerCommand::SubmitRequest(
            pool_handle, get_nym_request.to_string(),
            Box::new(move |res: Result<String, IndyError>| {
                CommandExecutor::instance().send(Command::Agent(
                    AgentCommand::ListenerResumeCheckConnect(listener_handle, did.clone(), pk.clone(), res))).unwrap();
            })))).unwrap();

        Ok(())
    }

    fn add_identity(&self, listener_handle: i32, pool_handle: i32, wallet_handle: i32, did: String,
                    cb: Box<Fn(Result<(), IndyError>)>) {
        let result = self.get_mydid_from_wallet(wallet_handle, did.as_str())
            .and_then(|my_did: MyDid|
                self.agent_service.add_identity(listener_handle, did.as_str(), pool_handle, wallet_handle, my_did.sk.as_str(), my_did.pk.as_str()).map_err(IndyError::from))

            .and_then(|cmd_id| {
                match self.add_rm_identity_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, cmd_id)),
                    Err(err) => Err(IndyError::CommonError(CommonError::InvalidState(err.description().to_string()))),
                }
            });

        match result {
            Ok((mut cbs, cmd_id)) => { cbs.insert(cmd_id, cb); /* TODO check if map contains same key */ }
            Err(err) => cb(Err(err).map_err(map_err_err!())),
        }
    }

    fn rm_identity(&self, listener_handle: i32, wallet_handle: i32, did: String,
                   cb: Box<Fn(Result<(), IndyError>)>) {
        let result = self.get_mydid_from_wallet(wallet_handle, did.as_str())
            .and_then(|my_did: MyDid|
                self.agent_service.rm_identity(listener_handle, did.as_str(), my_did.pk.as_str()).map_err(IndyError::from))

            .and_then(|cmd_id| {
                match self.add_rm_identity_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, cmd_id)),
                    Err(err) => Err(IndyError::CommonError(CommonError::InvalidState(err.description().to_string()))),
                }
            });

        match result {
            Ok((mut cbs, cmd_id)) => { cbs.insert(cmd_id, cb); /* TODO check if map contains same key */ }
            Err(err) => cb(Err(err).map_err(map_err_err!())),
        }
    }

    fn get_mydid_from_wallet(&self, wallet_handle: i32, did: &str) -> Result<MyDid, IndyError> {
        self.wallet_service
            .get(wallet_handle, format!("my_did::{}", did).as_str())
            .map_err(IndyError::from)
            .and_then(|my_did_json|
                MyDid::from_json(my_did_json.as_str())
                    .map_err(|_| IndyError::CommonError(CommonError::InvalidState((format!("Invalid my did json"))))))
    }

    fn on_add_rm_identity_ack(&self, cmd_id: i32, res: Result<(), CommonError>) {
        match self.add_rm_identity_callbacks.borrow_mut().remove(&cmd_id) {
            Some(cb) => cb(res.map_err(From::from)),
            None => error!("Can't handle add/rm identity ack cmd - callback not found for {}", cmd_id),
        };
    }

    fn send(&self, conn_id: i32, msg: Option<String>, cb: Box<Fn(Result<(), IndyError>)>) {
        let result = self.agent_service
            .send(conn_id, msg.as_ref().map(String::as_str))
            .and_then(|cmd_id| {
                match self.send_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, cmd_id)),
                    Err(err) => Err(CommonError::InvalidState(err.description().to_string())),
                }
            });
        match result {
            Ok((mut cbs, cmd_id)) => { cbs.insert(cmd_id, cb); /* TODO check if map contains same key */ }
            Err(err) => cb(Err(From::from(err)).map_err(map_err_err!())),
        }
    }

    fn on_send_ack(&self, cmd_id: i32, res: Result<(), CommonError>) {
        match self.send_callbacks.borrow_mut().remove(&cmd_id) {
            Some(cb) => cb(res.map_err(From::from)),
            None => error!("Can't handle SendAck cmd - callback not found for {}", cmd_id),
        };
    }

    fn close_connection_or_listener(&self, handle: i32, cb: Box<Fn(Result<(), IndyError>)>, close_listener: bool) {
        let result = self.agent_service
            .close_connection_or_listener(handle, close_listener)
            .and_then(|cmd_id| {
                match self.close_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, cmd_id)),
                    Err(err) => Err(CommonError::InvalidState(err.description().to_string())),
                }
            });
        match result {
            Ok((mut cbs, cmd_id)) => { cbs.insert(cmd_id, cb); /* TODO check if map contains same key */ }
            Err(err) => cb(Err(From::from(err))),
        }
    }

    fn on_close_connection_ack(&self, cmd_id: i32, res: Result<(), CommonError>, ) {
        match self.close_callbacks.borrow_mut().remove(&cmd_id) {
            Some(cb) => cb(res.map_err(From::from)),
            None => error!("Can't handle CloseConnectionAck cmd - not found callback for {}", cmd_id)
        };
    }

    fn on_close_listener_ack(&self, cmd_id: i32, res: Result<(), CommonError>, ) {
        match self.close_callbacks.borrow_mut().remove(&cmd_id) {
            Some(cb) => cb(res.map_err(From::from)),
            None => error!("Can't handle CloseListenerAck cmd - not found callback for {}", cmd_id)
        };
    }
}

fn _base58_to_z85(str: &str) -> Result<String, CommonError> {
    str.from_base58()
        .map_err(|err| CommonError::InvalidStructure(format!("Can't decode base58: {}", err)))
        .and_then(|bytes: Vec<u8>| {
            zmq::z85_encode(bytes.as_slice())
                .map_err(|err| CommonError::InvalidStructure(format!("Can't encode to z85: {}", err)))
        })
}

#[derive(Debug, Clone)]
pub struct MyConnectInfo {
    sender_did: String,
    receiver_did: String,
    secret_key: String,
    public_key: String,
}

#[derive(Debug)]
pub struct ConnectInfo {
    //TODO push to public service structure and use in service calls?
    server_key: String,
    endpoint: String,
}

#[derive(Deserialize)]
struct Endpoint {
    verkey: String,
    ha: String,
}

#[derive(Deserialize)]
struct AttribData {
    endpoint: Endpoint,
}

impl<'a> JsonDecodable<'a> for AttribData {}
