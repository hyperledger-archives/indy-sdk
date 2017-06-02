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
    CloseConnection,
    Listen,
    CloseListener,
    Send,
}

pub struct AgentCommandExecutor {
    agent_service: Rc<AgentService>,
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
}

impl AgentCommandExecutor {
    pub fn new(agent_service: Rc<AgentService>, pool_service: Rc<PoolService>, wallet_service: Rc<WalletService>) -> AgentCommandExecutor {
        AgentCommandExecutor {
            agent_service: agent_service,
            pool_service: pool_service,
            wallet_service: wallet_service,
        }
    }

    pub fn execute(&self, agent_cmd: AgentCommand) {
        match agent_cmd {
            AgentCommand::Connect(wallet_handle, sender_did, receiver_did, connect_cb, message_cb) => {
                info!(target: "agent_command_executor", "Connect command received");
                self.connect(wallet_handle, sender_did, receiver_did, connect_cb, message_cb)
            }
            _ => unimplemented!(),
        }
    }

    fn connect(&self, wallet_handle: i32, sender_did: String, receiver_did: String,
               connect_cb: Box<Fn(Result<i32, SovrinError>) + Send>,
               message_cb: Box<Fn(Result<(i32, String), SovrinError>) + Send>,
    ) {
        let agent_serv: &AgentService = self.agent_service.as_ref();
        self._connect(wallet_handle, &sender_did, &receiver_did)
            .and_then(|info: Option<ConnectInfo>| {
                let info = info.unwrap();
                debug!("{:?}", info);
                agent_serv.connect(sender_did.as_str(),
                                   info.secret_key.as_str(), info.public_key.as_str(),
                                   info.endpoint.as_str(), info.server_key.as_str())
                    .map_err(From::from)
            })
            .and_then(|conn_handle| { unimplemented!() })
            .unwrap_or_else(|err| connect_cb(Err(err)))
    }

    fn _connect(&self, wallet_handle: i32, sender_did: &String, receiver_did: &String)
                -> Result<Option<ConnectInfo>, SovrinError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", sender_did))?;
        let my_did: MyDid = MyDid::from_json(&my_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid my did json"))))?;

        let their_did_json = self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", receiver_did))?; //FIXME implement: get from ledger
        let their_did: TheirDid = TheirDid::from_json(&their_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid their did json"))))?;
        assert!(their_did.endpoint.is_some()); //FIXME implement: get from ledger
        Ok(Some(ConnectInfo {
            secret_key: my_did.sk,
            public_key: my_did.pk,
            endpoint: their_did.endpoint.unwrap(),
            server_key: their_did.pk.unwrap()
        }))
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
