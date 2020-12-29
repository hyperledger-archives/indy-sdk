use crate::services::metrics::models::MetricsValue;
use crate::services::metrics::MetricsService;
use indy_api_types::errors::prelude::*;
use indy_wallet::WalletService;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::rc::Rc;

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send>),
}

pub struct MetricsCommandExecutor {
    wallet_service: Rc<WalletService>,
    metrics_service: Rc<MetricsService>,
}

impl MetricsCommandExecutor {
    pub fn new(
        wallet_service: Rc<WalletService>,
        metrics_service: Rc<MetricsService>,
    ) -> MetricsCommandExecutor {
        MetricsCommandExecutor {
            wallet_service,
            metrics_service,
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
        let mut metrics_map = serde_json::Map::new();
        self.append_threapool_metrics(&mut metrics_map)?;
        self.append_wallet_metrics(&mut metrics_map)?;
        self.metrics_service
            .append_command_metrics(&mut metrics_map)?;
        let res = serde_json::to_string(&metrics_map)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize a metrics map")?;

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from command thread");
        Ok(res)
    }

    fn append_threapool_metrics(&self, metrics_map: &mut Map<String, Value>) -> IndyResult<()> {
        #[derive(Serialize, Deserialize)]
        struct MetricsTags {
            label: String,
        }

        let tp_instance = crate::commands::THREADPOOL.lock().unwrap();
        let mut threadpool_threads_count = Vec::new();
        let mut tag = HashMap::<String, String>::new();

        tag.insert(String::from("label"), String::from(THREADPOOL_ACTIVE_COUNT));
        threadpool_threads_count.push(
            serde_json::to_value(MetricsValue::new(tp_instance.active_count(), tag.clone()))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(THREADPOOL_QUEUED_COUNT));
        threadpool_threads_count.push(
            serde_json::to_value(MetricsValue::new(tp_instance.queued_count(), tag.clone()))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(THREADPOOL_MAX_COUNT));
        threadpool_threads_count.push(
            serde_json::to_value(MetricsValue::new(tp_instance.max_count(), tag.clone()))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(THREADPOOL_PANIC_COUNT));
        threadpool_threads_count.push(
            serde_json::to_value(MetricsValue::new(tp_instance.panic_count(), tag.clone()))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        metrics_map.insert(
            String::from("threadpool_threads_count"),
            serde_json::to_value(threadpool_threads_count)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        Ok(())
    }

    fn append_wallet_metrics(&self, metrics_map: &mut Map<String, Value>) -> IndyResult<()> {
        #[derive(Serialize, Deserialize)]
        struct MetricsTags {
            label: String,
        }
        let mut wallet_count = Vec::new();
        let mut tag = HashMap::<String, String>::new();

        tag.insert(String::from("label"), String::from(OPENED_WALLETS_COUNT));
        wallet_count.push(
            serde_json::to_value(MetricsValue::new(
                self.wallet_service.get_wallets_count(),
                tag.clone(),
            ))
            .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(OPENED_WALLET_IDS_COUNT));
        wallet_count.push(
            serde_json::to_value(MetricsValue::new(
                self.wallet_service.get_wallet_ids_count(),
                tag.clone(),
            ))
            .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(PENDING_FOR_IMPORT_WALLETS_COUNT));
        wallet_count.push(
            serde_json::to_value(MetricsValue::new(
                self.wallet_service.get_pending_for_import_count(),
                tag.clone(),
            ))
            .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        tag.insert(String::from("label"), String::from(PENDING_FOR_OPEN_WALLETS_COUNT));
        wallet_count.push(
            serde_json::to_value(MetricsValue::new(
                self.wallet_service.get_pending_for_open_count(),
                tag.clone(),
            ))
            .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        metrics_map.insert(
            String::from("wallet_count"),
            serde_json::to_value(wallet_count)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        Ok(())
    }
}
