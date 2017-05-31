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
        self._connect(wallet_handle, sender_did, receiver_did)
            .and_then(|_| self.agent_service.connect().map_err(From::from))
            .unwrap_or_else(|err| connect_cb(Err(err)))
    }

    fn _connect(&self, wallet_handle: i32, sender_did: String, receiver_did: String)
                -> Result<(), SovrinError> {
        let my_did_json = self.wallet_service.get(wallet_handle, &format!("my_did::{}", sender_did))?;
        let my_did: MyDid = MyDid::from_json(&my_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid my did json"))))?;

        let their_did_json = self.wallet_service.get_not_expired(wallet_handle, &format!("their_did::{}", receiver_did))?; //FIXME implement: get from ledger
        let their_did: TheirDid = TheirDid::from_json(&their_did_json).map_err(|_| CommonError::InvalidState((format!("Invalid their did json"))))?;
        assert!(their_did.endpoint.is_some()); //FIXME implement: get from ledger
        Ok(())
    }
}