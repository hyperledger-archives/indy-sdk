use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::domain::ledger::request::ProtocolVersion;
use crate::domain::pool::{PoolConfig, PoolOpenConfig};
use indy_api_types::errors::prelude::*;
use indy_api_types::CommandHandle;

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<()>) + Send>)
}

pub struct MetricsCommandExecutor {
    close_callbacks: RefCell<HashMap<CommandHandle, Box<dyn Fn(IndyResult<()>)>>>,
    refresh_callbacks: RefCell<HashMap<CommandHandle, Box<dyn Fn(IndyResult<()>)>>>,
    // TODO: change  PoolHandle to MetricsHandler
    open_callbacks: RefCell<HashMap<CommandHandle, Box<dyn Fn(IndyResult<PoolHandle>)>>>,
}

impl MetricsCommandExecutor {
    pub fn new() -> MetricsCommandExecutor {
        MetricsCommandExecutor {
            close_callbacks: RefCell::new(HashMap::new()),
            refresh_callbacks: RefCell::new(HashMap::new()),
            open_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: MetricsCommand) {
        match command {
            MetricsCommand::CollectMetrics(cb) => {
                debug!(target: "metrics_command_executor", "CollectMetrics command received");
                cb(self.collect());
            }

        };
    }
    fn collect(cb: Box<dyn Fn(IndyResult<String>) + Send>) {
        debug!("collecting metrics from the current thread");

    }
}
