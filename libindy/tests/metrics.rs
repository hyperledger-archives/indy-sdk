#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as api;
extern crate indyrs as indy;
use crate::utils::constants::*;
use crate::utils::metrics;
use crate::utils::wallet;
use crate::utils::Setup;

mod collect {
    use super::*;
    use std::collections::HashMap;
    use serde_json::Value;

    #[test]
    fn collect_metrics_contains_thread_pool_and_wallet_service_statistics() {
        let result_metrics = metrics::collect_metrics().unwrap();
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
            json!({"tags":{"label":"threadpool_max_count"},"value":4}),
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

    #[test]
    fn collect_metrics_includes_statistics_for_wallet_command() {
        let setup = Setup::empty();
        let config = config(&setup.name);
        wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();

        let result_metrics = metrics::collect_metrics().unwrap();
        let metrics_map = serde_json::from_str::<HashMap<String, Value>>(&result_metrics).unwrap();

        assert!(metrics_map.contains_key("commands_count"));
        assert!(metrics_map.contains_key("commands_duration_ms"));

        let commands_count = metrics_map
            .get("commands_count")
            .unwrap()
            .as_array()
            .unwrap();
        let commands_duration_ms = metrics_map
            .get("commands_duration_ms")
            .unwrap()
            .as_array()
            .unwrap();

        let expected_commands_count = [
            json!({"tags":{"command":"payments","stage":"executed","subcommand":"build_set_txn_fees_req_ack"},"value":0}),
            json!({"tags":{"command":"pairwise","stage":"queued","subcommand":"pairwise_exists"},"value":0}),
            json!({"tags":{"command":"cache","stage":"executed","subcommand":"purge_cred_def_cache"},"value":0}),
            json!({"tags":{"command":"non_secrets","stage":"queued","subcommand":"fetch_search_next_records"},"value":0}),
        ];

        let expected_commands_duration_ms = [
            json!({"tags":{"command":"payments","stage":"executed","subcommand":"build_set_txn_fees_req_ack"},"value":0}),
            json!({"tags":{"command":"pairwise","stage":"queued","subcommand":"pairwise_exists"},"value":0}),
            json!({"tags":{"command":"cache","stage":"executed","subcommand":"purge_cred_def_cache"},"value":0}),
            json!({"tags":{"command":"non_secrets","stage":"queued","subcommand":"fetch_search_next_records"},"value":0}),
        ];

        for command in &expected_commands_count {
            assert!(commands_count.contains(&command));
        }

        for command in &expected_commands_duration_ms {
            assert!(commands_duration_ms.contains(&command));
        }
    }
    fn config(name: &str) -> String {
        json!({"id": name}).to_string()
    }
}
