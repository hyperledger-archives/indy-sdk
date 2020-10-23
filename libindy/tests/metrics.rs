#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as indy;
extern crate indyrs as api;

use crate::utils::metrics;

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

mod collect {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn collect_metrics_works() {
        let result_metrics = metrics::collect_metrics().unwrap();
        let metrics_map = serde_json::from_str::<HashMap<String, usize>>(&result_metrics).unwrap();
        let expected_keys: HashSet<&str> = [
            THREADPOOL_ACTIVE_COUNT,
            THREADPOOL_QUEUED_COUNT,
            THREADPOOL_MAX_COUNT,
            THREADPOOL_PANIC_COUNT,
            OPENED_WALLETS_COUNT,
            OPENED_WALLET_IDS_COUNT,
            PENDING_FOR_IMPORT_WALLETS_COUNT,
            PENDING_FOR_OPEN_WALLETS_COUNT
        ].iter().cloned().collect();
        for exp_key in expected_keys {
            assert!(metrics_map.contains_key(exp_key));
        }
    }
}
