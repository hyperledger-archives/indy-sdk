use std::cell::RefCell;
use std::collections::HashMap;
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
        Result<i32, SovrinError> // conn handle or error
    ),
    CloseConnection,
    Listen(
        i32, // wallet handle
        Box<Fn(Result<i32, SovrinError>) + Send>, // listen cb
        Box<Fn(Result<(i32, i32, String, String), SovrinError>) + Send>, // connect cb
        Box<Fn(Result<(i32, String), SovrinError>) + Send>, // message cb
    ),
    CloseListener,
    Send,
}

pub struct AgentCommandExecutor {
    agent_service: Rc<AgentService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,

    open_callbacks: RefCell<HashMap<i32, Box<Fn(Result<i32, SovrinError>)>>>,
}

impl AgentCommandExecutor {
    pub fn new(agent_service: Rc<AgentService>, pool_service: Rc<PoolService>, wallet_service: Rc<WalletService>) -> AgentCommandExecutor {
        AgentCommandExecutor {
            agent_service: agent_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
            open_callbacks: RefCell::new(HashMap::new())
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
                self.open_callbacks.borrow_mut().remove(&cmd_id).unwrap()(res) //TODO extract method
            }
            _ => unimplemented!(),
        }
    }

    fn connect(&self, wallet_handle: i32, sender_did: String, receiver_did: String,
               connect_cb: Box<Fn(Result<i32, SovrinError>) + Send>,
               message_cb: Box<Fn(Result<(i32, String), SovrinError>) + Send>,
    ) {
        let result = self._get_connection_info(wallet_handle, &sender_did, &receiver_did)
            .and_then(|info: ConnectInfo| {
                debug!("AgentCommandExecutor::connect try to service.connect with {:?}", info);
                self.agent_service.as_ref().connect(sender_did.as_str(),
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
            Err(err) => { connect_cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, connect_cb); /* TODO check if map contains same key */ }
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
}

#[derive(Debug)]
struct ConnectInfo {
    //TODO push to public service structure and use in service calls?
    secret_key: String,
    public_key: String,
    server_key: String,
    endpoint: String,
}
