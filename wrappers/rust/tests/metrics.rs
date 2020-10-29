#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate futures;
extern crate indyrs as indy;
#[macro_use]
use indy::metrics;

use indy::ErrorCode;

mod utils;
#[allow(unused_imports)]
use futures::Future;

const THREADPOOL_ACTIVE_COUNT: &str = "threadpool_active_count";
const THREADPOOL_QUEUED_COUNT: &str = "threadpool_queued_count";
const THREADPOOL_MAX_COUNT: &str = "threadpool_max_count";
const THREADPOOL_PANIC_COUNT: &str = "threadpool_panic_count";
const OPENED_WALLETS_COUNT: &str = "opened_wallets_count";
const OPENED_WALLET_IDS_COUNT: &str = "opened_wallet_ids_count";
const PENDING_FOR_IMPORT_WALLETS_COUNT: &str = "pending_for_import_wallets_count";
const PENDING_FOR_OPEN_WALLETS_COUNT: &str = "pending_for_open_wallets_count";

#[cfg(test)]
mod collect {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn collect_metrics() {
        let result_metrics = metrics::collect_metrics().wait().unwrap();
        let metrics_map = serde_json::from_str::<HashMap<String, usize>>(&result_metrics).unwrap();
        assert!(metrics_map.contains_key(THREADPOOL_ACTIVE_COUNT));
        assert!(metrics_map.contains_key(THREADPOOL_QUEUED_COUNT));
        assert!(metrics_map.contains_key(THREADPOOL_MAX_COUNT));
        assert!(metrics_map.contains_key(THREADPOOL_PANIC_COUNT));
        assert!(metrics_map.contains_key(OPENED_WALLETS_COUNT));
        assert!(metrics_map.contains_key(OPENED_WALLET_IDS_COUNT));
        assert!(metrics_map.contains_key(PENDING_FOR_IMPORT_WALLETS_COUNT));
        assert!(metrics_map.contains_key(PENDING_FOR_OPEN_WALLETS_COUNT));
    }
}
