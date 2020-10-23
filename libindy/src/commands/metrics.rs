use std::collections::HashMap;
use indy_api_types::errors::prelude::*;

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send>)
}

pub struct MetricsCommandExecutor {}

impl MetricsCommandExecutor {
    pub fn new() -> MetricsCommandExecutor {
        MetricsCommandExecutor {}
    }

    pub fn execute(&self, command: MetricsCommand) {
        match command {
            MetricsCommand::CollectMetrics(cb) => {
                debug!(target: "metrics_command_executor", "CollectMetrics command received");
                cb(self._collect());
            }
        };
    }
    fn _collect(&self) -> IndyResult<String> {
        trace!("_collect >>>");

        let metrics_map: HashMap<String, f32> = HashMap::new();
        let res = serde_json::to_string(&metrics_map)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize a metrics map")?;

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from command thread");
        Ok(res)
    }
}
