use std::rc::Rc;

use errors::sovrin::SovrinError;
use services::agent::AgentService;
use services::pool::PoolService;
use services::wallet::WalletService;

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
        unimplemented!();
    }
}