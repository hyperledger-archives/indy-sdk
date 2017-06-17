#![warn(unused_variables)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;

use errors::sovrin::SovrinError;
use errors::common::CommonError;
use services::agent::AgentService;
use services::pool::PoolService;
use services::signus::types::{MyDid, TheirDid};
use services::wallet::WalletService;
use utils::json::JsonDecodable;

pub enum AgentCommand {
    Connect(
        i32, // wallet handle
        String, // sender did
        String, // receiver did
        Box<Fn(Result<i32, SovrinError>) + Send>, // connect cb
        Box<Fn(Result<(i32, String), SovrinError>) + Send>, // message cb
    ),
    ConnectAck(
        i32, // cmd handle (eq conn handle)
        Result<i32, CommonError> // conn handle or error
    ),
    CloseConnection(
        i32, // connection handle
        Box<Fn(Result<(), SovrinError>) + Send>, // close conn cb
    ),
    CloseConnectionAck(
        i32, // close cmd handle
        Result<(), CommonError>,
    ),
    Listen(
        i32, // wallet handle
        String, // endpoint
        Box<Fn(Result<i32, SovrinError>) + Send>, // listen cb
        Box<Fn(Result<(i32, i32, String, String), SovrinError>) + Send>, // connect cb
        Box<Fn(Result<(i32, String), SovrinError>) + Send>, // message cb
    ),
    ListenAck(
        i32, // cmd handle (eq listener handle)
        Result<i32, CommonError> // listener handle or error
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
        Box<Fn(Result<(), SovrinError>) + Send>, // close listener cb
    ),
    CloseListenerAck(
        i32, // close cmd handle
        Result<(), CommonError>,
    ),
    Send(
        i32, // connection handle
        Option<String>, // message
        Box<Fn(Result<(), SovrinError>) + Send>, // send cb
    ),
    SendAck(
        i32, // send cmd handle
        Result<(), CommonError>,
    )
}

pub struct AgentCommandExecutor {
    agent_service: Rc<AgentService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

    out_connections: RefCell<HashMap<i32, Box<Fn(Result<(i32, String), SovrinError>)>>>,
    listeners: RefCell<HashMap<i32, Listener>>,

    listen_callbacks: RefCell<HashMap<i32, (
        Box<Fn(Result<i32, SovrinError>) + Send>, // listen cb
        Listener
    )>>,
    open_callbacks: RefCell<HashMap<i32,
        (Box<Fn(Result<i32, SovrinError>)>,
         Box<Fn(Result<(i32, String), SovrinError>)>)>>,
    send_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), SovrinError>)>>>,
    close_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), SovrinError>)>>>,
}

struct Listener {
    on_connect: Box<Fn(Result<(i32, i32, String, String), SovrinError>) + Send>,
    on_msg: Box<Fn(Result<(i32, String), SovrinError>) + Send>,
    conn_handles: HashSet<i32>,
}

impl AgentCommandExecutor {
    pub fn new(agent_service: Rc<AgentService>, pool_service: Rc<PoolService>, wallet_service: Rc<WalletService>) -> AgentCommandExecutor {
        AgentCommandExecutor {
            agent_service: agent_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
            out_connections: RefCell::new(HashMap::new()),
            listeners: RefCell::new(HashMap::new()),
            listen_callbacks: RefCell::new(HashMap::new()),
            open_callbacks: RefCell::new(HashMap::new()),
            send_callbacks: RefCell::new(HashMap::new()),
            close_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, agent_cmd: AgentCommand) {
        match agent_cmd {
            AgentCommand::Connect(wallet_handle, sender_did, receiver_did, connect_cb, message_cb) => {
                info!(target: "agent_command_executor", "Connect command received");
                self.connect(wallet_handle, sender_did, receiver_did, connect_cb, message_cb)
            }
            AgentCommand::ConnectAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ConnectAck command received");
                let cbs = self.open_callbacks.borrow_mut().remove(&cmd_id).unwrap();
                if let &Ok(conn_handle) = &res {
                    self.out_connections.borrow_mut().insert(conn_handle, cbs.1);
                }
                cbs.0(res.map_err(map_err_err!()).map_err(From::from));
            }
            AgentCommand::Listen(wallet_handle, endpoint, listen_cb, connect_cb, message_cb) => {
                info!(target: "agent_command_executor", "Listen command received");
                self.listen(wallet_handle, endpoint, listen_cb, connect_cb, message_cb);
            }
            AgentCommand::ListenAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "ListenAck command received");
                //TODO extract method
                let cbs = self.listen_callbacks.borrow_mut().remove(&cmd_id).unwrap();
                if let Ok(listener_handle) = res {
                    self.listeners.borrow_mut().insert(listener_handle, cbs.1);
                }
                cbs.0(res.map_err(map_err_err!()).map_err(From::from))
            }
            AgentCommand::ListenerOnConnect(listener_id, res) => {
                info!(target: "agent_command_executor", "ListenerOnConnect command received");
                let mut listeners = self.listeners.borrow_mut();
                let mut cbs = listeners.get_mut(&listener_id).unwrap();
                if let Ok((_, connection_handle, _, _)) = res {
                    cbs.conn_handles.insert(connection_handle);
                }
                (cbs.on_connect)(res.map_err(map_err_err!()).map_err(From::from));
            }
            AgentCommand::MessageReceived(connection_id, res) => {
                info!(target: "agent_command_executor", "ListenerOnConnect command received");
                let res: Result<(i32, String), SovrinError> = res.map_err(From::from);
                match self.listeners.borrow_mut().iter().find(|&(_, listener)| listener.conn_handles.contains(&connection_id)) {
                    Some((_, listener)) => (listener.on_msg)(res),
                    None => self.out_connections.borrow().get(&connection_id).unwrap()(res),
                }
            }
            AgentCommand::Send(connection_id, msg, cb) => {
                info!(target: "agent_command_executor", "Send command received");
                self.send(connection_id, msg, cb)
            }
            AgentCommand::SendAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "SendAck command received");
                self.send_callbacks.borrow_mut().remove(&cmd_id).unwrap()(res.map_err(From::from));
            }
            AgentCommand::CloseConnection(connection_id, cb) => {
                info!(target: "agent_command_executor", "CloseConnection command received");
                self.close_connection_or_listener(connection_id, cb, false)
            }
            AgentCommand::CloseConnectionAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "CloseConnectionAck command received");
                self.close_callbacks.borrow_mut().remove(&cmd_id).unwrap()(res.map_err(From::from));
            }
            AgentCommand::CloseListener(listener_id, cb) => {
                info!(target: "agent_command_executor", "CloseListener command received");
                self.close_connection_or_listener(listener_id, cb, true)
            }
            AgentCommand::CloseListenerAck(cmd_id, res) => {
                info!(target: "agent_command_executor", "CloseListenerAck command received");
                self.close_callbacks.borrow_mut().remove(&cmd_id).unwrap()(res.map_err(From::from));
            }
        }
    }

    fn connect(&self, wallet_handle: i32, sender_did: String, receiver_did: String,
               connect_cb: Box<Fn(Result<i32, SovrinError>) + Send>,
               message_cb: Box<Fn(Result<(i32, String), SovrinError>) + Send>,
    ) {
        let result = self._get_connection_info(wallet_handle, &sender_did, &receiver_did)
            .and_then(|info: ConnectInfo| {
                debug!("AgentCommandExecutor::connect try to service.connect with {:?}", info);
                self.agent_service
                    .connect(sender_did.as_str(),
                             info.secret_key.as_str(), info.public_key.as_str(),
                             info.endpoint.as_str(), info.server_key.as_str())
                    .map_err(From::from)
            })
            .and_then(|conn_handle| {
                match self.open_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, conn_handle)),
                    Err(err) => Err(SovrinError::CommonError(CommonError::InvalidState(err.description().to_string()))),
                }
            });
        match result {
            Err(err) => { connect_cb(Err(err).map_err(map_err_err!())); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, (connect_cb, message_cb)); /* TODO check if map contains same key */ }
        };
    }

    fn _get_connection_info(&self, wallet_handle: i32, sender_did: &String, receiver_did: &String)
                            -> Result<ConnectInfo, SovrinError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", sender_did))?;
        let my_did: MyDid = MyDid::from_json(&my_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid my did json"))))?;

        let their_did_json = self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", receiver_did))?; //FIXME implement: get from ledger
        let their_did: TheirDid = TheirDid::from_json(&their_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid their did json"))))?;
        assert!(their_did.endpoint.is_some()); //FIXME implement: get from ledger
        Ok(ConnectInfo {
            secret_key: my_did.sk,
            public_key: my_did.pk,
            endpoint: their_did.endpoint.unwrap(),
            server_key: their_did.pk.unwrap()
        })
    }

    fn listen(&self, wallet_handle: i32, endpoint: String,
              listen_cb: Box<Fn(Result<i32, SovrinError>) + Send>,
              connect_cb: Box<Fn(Result<(i32, i32, String, String), SovrinError>) + Send>,
              message_cb: Box<Fn(Result<(i32, String), SovrinError>) + Send>) {
        let my_did_json: String = self.wallet_service.list(wallet_handle, "my_did::").as_ref().unwrap().get(0).as_ref().unwrap().1.clone();
        let my_did: MyDid = MyDid::from_json(my_did_json.as_str()).map_err(|_| CommonError::InvalidState((format!("Invalid my did json")))).unwrap();

        let result = self.agent_service
            .listen(endpoint.as_str(), my_did.pk.as_str(), my_did.sk.as_str())
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

    fn send(&self, conn_id: i32, msg: Option<String>, cb: Box<Fn(Result<(), SovrinError>)>) {
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

    fn close_connection_or_listener(&self, handle: i32, cb: Box<Fn(Result<(), SovrinError>)>, close_listener: bool) {
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
}

#[derive(Debug)]
struct ConnectInfo {
    //TODO push to public service structure and use in service calls?
    secret_key: String,
    public_key: String,
    server_key: String,
    endpoint: String,
}
