use std::collections::HashMap;
use indy_api_types::errors::prelude::*;
use indy_wallet::WalletService;
use std::rc::Rc;

const THREADPOOL_ACTIVE_COUNT : &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT : &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT : &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT : &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT : &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT : &str = "opened_wallet_ids_count";
const  PENDING_FOR_IMPORT_WALLETS_COUNT : &str = "pending_for_import_wallets_count";
const  PENDING_FOR_OPEN_WALLETS_COUNT : &str = "pending_for_open_wallets_count";

pub enum MetricsCommand {
    CollectMetrics(Box<dyn Fn(IndyResult<String>) + Send>)
}

pub struct MetricsCommandExecutor {
    wallet_service: Rc<WalletService>
}

impl MetricsCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> MetricsCommandExecutor {
        MetricsCommandExecutor {
            wallet_service
        }
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

        let metrics_map : HashMap<&str, usize> = [
            (THREADPOOL_ACTIVE_COUNT, crate::commands::THREADPOOL.lock().unwrap().active_count()),
            (THREADPOOL_QUEUED_COUNT, crate::commands::THREADPOOL.lock().unwrap().queued_count()),
            (THREADPOOL_MAX_COUNT, crate::commands::THREADPOOL.lock().unwrap().max_count()),
            (THREADPOOL_PANIC_COUNT, crate::commands::THREADPOOL.lock().unwrap().panic_count()),
            (OPENED_WALLETS_COUNT, self.wallet_service.get_wallets_count()),
            (OPENED_WALLET_IDS_COUNT, self.wallet_service.get_wallet_ids_count()),
            (PENDING_FOR_IMPORT_WALLETS_COUNT, self.wallet_service.get_pending_for_import_count()),
            (PENDING_FOR_OPEN_WALLETS_COUNT, self.wallet_service.get_pending_for_open_count())
        ].iter().cloned().collect();
        let res = serde_json::to_string(&metrics_map).unwrap();

        trace!("_collect <<< res: {:?}", res);
        debug!("collecting metrics from the current thread");
        Ok(res)

    }
}
