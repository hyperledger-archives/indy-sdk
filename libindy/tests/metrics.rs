//#[macro_use]
//mod utils;
//
//inject_indy_dependencies!();
//
//extern crate indyrs as indy;
//extern crate indyrs as api;
//use crate::utils::wallet;
////use crate::utils::metrics;
//use crate::utils::constants::*;
//use crate::utils::Setup;
//
//
//
//mod collect {
//    use super::*;
//    use std::collections::HashMap;
//
//    #[test]
//    fn collect_metrics_contains_thread_pool_and_wallet_service_statistics() {
//        let result_metrics = metrics::collect_metrics().unwrap();
//        let metrics_map = serde_json::from_str::<HashMap<String, usize>>(&result_metrics).unwrap();
//        assert!(metrics_map.contains_key("threadpool_active_count"));
//        assert!(metrics_map.contains_key("threadpool_queued_count"));
//        assert!(metrics_map.contains_key("threadpool_max_count"));
//        assert!(metrics_map.contains_key("threadpool_panic_count"));
//        assert!(metrics_map.contains_key("opened_wallets_count"));
//        assert!(metrics_map.contains_key("opened_wallet_ids_count"));
//        assert!(metrics_map.contains_key("pending_for_import_wallets_count"));
//        assert!(metrics_map.contains_key("pending_for_open_wallets_count"));
//    }
//
//    #[test]
//    fn collect_metrics_includes_statistics_for_wallet_command() {
//        let setup = Setup::empty();
//        let config = config(&setup.name);
//        wallet::create_wallet(&config, WALLET_CREDENTIALS).unwrap();
//
//        let result_metrics = metrics::collect_metrics().unwrap();
//        let metrics_map = serde_json::from_str::<HashMap<String, u128>>(&result_metrics).unwrap();
//
//        assert_eq!(metrics_map.get("wallet_command_create_queued_commands_count").unwrap(), &1u128);
//        assert!(metrics_map.contains_key("wallet_command_create_queued_commands_duration_ms"));
//        assert_eq!(metrics_map.get("wallet_command_create_executed_commands_count").unwrap(), &1u128);
//        assert!(metrics_map.contains_key("wallet_command_create_executed_commands_duration_ms"));
//    }
//    fn config(name: &str) -> String {
//        json!({"id": name}).to_string()
//    }
//}
