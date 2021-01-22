use indy_api_types::errors::prelude::*;
use indy_wallet::WalletService;
use std::sync::Arc;
use crate::services::metrics::MetricsService;
use serde_json::{Map, Value};

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send + Sync>)
}

pub struct MetricsController {
    wallet_service:Arc<WalletService>,
    metrics_service:Arc<MetricsService>
}

impl MetricsController {
    pub fn new(wallet_service:Arc<WalletService>,
               metrics_service:Arc<MetricsService>) -> MetricsController {
        MetricsController {
            wallet_service,
            metrics_service
        }
    }

    pub async fn execute(&self, command: MetricsCommand) {
        match command {
            MetricsCommand::CollectMetrics(cb) => {
                debug!(target: "metrics_command_executor", "CollectMetrics command received");
                cb(self.collect().await);
            }
        };
    }

    async fn collect(&self) -> IndyResult<String> {
        trace!("_collect >>>");
        let mut metrics_map= serde_json::Map::new();
        self.append_threapool_metrics(&mut metrics_map);
        self.append_wallet_metrics(&mut metrics_map).await;
        self.metrics_service.append_command_metrics(&mut metrics_map);
        let res = serde_json::to_string(&metrics_map)
            .to_indy(IndyErrorKind::InvalidState,"Can't serialize a metrics map")?;

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from command thread");
        Ok(res)
    }

    fn append_threapool_metrics(&self, metrics_map: &mut Map<String, Value>) {
        let tp_instance = crate::commands::THREADPOOL.lock().unwrap();
        metrics_map.insert(String::from(THREADPOOL_ACTIVE_COUNT),
                           Value::from(tp_instance.active_count()));
        metrics_map.insert(String::from(THREADPOOL_QUEUED_COUNT),
                           Value::from(tp_instance.queued_count()));
        metrics_map.insert(String::from(THREADPOOL_MAX_COUNT),
                           Value::from(tp_instance.max_count()));
        metrics_map.insert(String::from(THREADPOOL_PANIC_COUNT),
                           Value::from(tp_instance.panic_count()));
    }

    async fn append_wallet_metrics(&self, metrics_map: &mut Map<String, Value>) {
        metrics_map.insert(String::from(OPENED_WALLETS_COUNT),
                           Value::from(self.wallet_service.get_wallets_count().await));
        metrics_map.insert(String::from(OPENED_WALLET_IDS_COUNT),
                           Value::from(self.wallet_service.get_wallet_ids_count().await));
        metrics_map.insert(String::from(PENDING_FOR_IMPORT_WALLETS_COUNT),
                           Value::from(self.wallet_service.get_pending_for_import_count().await));
        metrics_map.insert(String::from(PENDING_FOR_OPEN_WALLETS_COUNT),
                           Value::from(self.wallet_service.get_pending_for_open_count().await));
    }

}
