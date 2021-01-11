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

#[cfg(test)]
mod collect {
    use super::*;
    use std::collections::HashMap;
    use serde_json::Value;

    #[test]
    fn collect_metrics() {
        let result_metrics = metrics::collect_metrics().wait().unwrap();
        let metrics_map = serde_json::from_str::<HashMap<String, Value>>(&result_metrics).unwrap();

        assert!(metrics_map.contains_key("threadpool_threads_count"));
        assert!(metrics_map.contains_key("wallet_count"));

        let threadpool_threads_count = metrics_map
            .get("threadpool_threads_count")
            .unwrap()
            .as_array()
            .unwrap();
        let wallet_count = metrics_map
            .get("wallet_count")
            .unwrap()
            .as_array()
            .unwrap();

        let expected_threadpool_threads_count = [
            json!({"tags":{"label":"threadpool_active_count"},"value":0}),
            json!({"tags":{"label":"threadpool_queued_count"},"value":0}),
            json!({"tags":{"label":"threadpool_panic_count"},"value":0}),
        ];

        let expected_wallet_count = [
            json!({"tags":{"label":"opened_wallets_count"},"value":0}),
            json!({"tags":{"label":"opened_wallet_ids_count"},"value":0}),
            json!({"tags":{"label":"pending_for_import_wallets_count"},"value":0}),
            json!({"tags":{"label":"pending_for_open_wallets_count"},"value":0}),
        ];

        for command in &expected_threadpool_threads_count {
            assert!(threadpool_threads_count.contains(&command));
        }

        for command in &expected_wallet_count {
            assert!(wallet_count.contains(&command));
        }
    }
}
