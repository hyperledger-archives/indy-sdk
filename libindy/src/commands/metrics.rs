use indy_api_types::errors::prelude::*;
use indy_wallet::WalletService;
use std::rc::Rc;
use crate::services::metrics::MetricsService;
use serde_json::{Map, Value};
use crate::services::metrics::models::MetricsValue;

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send>)
}

pub struct MetricsCommandExecutor {
    wallet_service: Rc<WalletService>,
    metrics_service: Rc<MetricsService>
}

impl MetricsCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               metrics_service: Rc<MetricsService>) -> MetricsCommandExecutor {
        MetricsCommandExecutor {
            wallet_service,
            metrics_service
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

    fn collect(&self) -> IndyResult<String> {
        trace!("_collect >>>");
        let mut metrics_map= serde_json::Map::new();
        self.append_threapool_metrics(&mut metrics_map);
        self.append_wallet_metrics(&mut metrics_map);
        self.metrics_service.append_command_metrics(&mut metrics_map);
        let res = serde_json::to_string(&metrics_map)
            .to_indy(IndyErrorKind::InvalidState,"Can't serialize a metrics map")?;

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from command thread");
        Ok(res)
    }

    fn append_threapool_metrics(&self, metrics_map: &mut Map<String, Value>) {
        struct MetricsTags {
            stage: String,
        }
        let tp_instance = crate::commands::THREADPOOL.lock().unwrap();
        let metrics_map = checkMetricsStructure(metrics_map);
        let mut threadpool_threads_count = Vec::new();

        threadpool_threads_count.push(Value::from("value", tp_instance.active_count()));
        threadpool_threads_count.push(Value::from("tags", MetricsTags{stage: "active".to_string()}));

        threadpool_threads_count.push(Value::from("value", tp_instance.queued_count()));
        threadpool_threads_count.push(Value::from("tags", MetricsTags{stage: "queued".to_string()}));

        threadpool_threads_count.push(Value::from("value", tp_instance.max_count()));
        threadpool_threads_count.push(Value::from("tags", MetricsTags{stage: "max".to_string()}));

        threadpool_threads_count.push(Value::from("value", tp_instance.panic_count()));
        threadpool_threads_count.push(Value::from("tags", MetricsTags{stage: "panic".to_string()}));

        metrics_map.insert(String::from("threadpool_threads_count"), Value::from(threadpool_threads_count));
    }

    fn append_wallet_metrics(&self, metrics_map: &mut Map<String, Value>) {
        struct MetricsTags {
            stage: String,
        }
        let mut wallet_count = Vec::new();

        wallet_count.push(Value::from( "value", self.wallet_service.get_wallets_count()));
        wallet_count.push(Value::from("tags", MetricsTags{stage: "wallets".to_string()}));

        wallet_count.push(Value::from("value", self.wallet_service.get_wallet_ids_count()));
        wallet_count.push(Value::from("tags", MetricsTags{stage: "ids".to_string()}));

        wallet_count.push(Value::from("value", self.wallet_service.get_pending_for_import_count()));
        wallet_count.push(Value::from("tags", MetricsTags{stage: "pending_import".to_string()}));

        wallet_count.push(Value::from("value", self.wallet_service.get_pending_for_open_count()));
        wallet_count.push(Value::from("tags", MetricsTags{stage: "pending_open".to_string()}));

        metrics_map.insert(String::from("wallet_count"),Value::from(wallet_count));
    }

}
