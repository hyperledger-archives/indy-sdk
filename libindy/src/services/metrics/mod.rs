use crate::services::metrics::command_metrics::{CommandMetrics, CommandMetric, CommandAware, Counters};
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use models::MetricsValue;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::commands::Command;
use crate::commands::anoncreds::verifier::VerifierCommand;
use crate::commands::anoncreds::prover::ProverCommand;
use crate::commands::anoncreds::issuer::IssuerCommand;

pub mod command_metrics;
pub mod models;

pub struct MetricsService {
     metrics: RefCell<CommandMetrics>,
}

impl MetricsService {
    pub fn new() -> Self {
        MetricsService {
            metrics: RefCell::new(CommandMetrics::new()),
        }
    }

    pub fn get_command_tags(
        command: String,
        subcommand: String,
        stage: String,
    ) -> HashMap<String, String> {
        let mut tags = HashMap::<String, String>::new();
        tags.insert("command".to_owned(), command.clone());
        tags.insert("subcommand".to_owned(), subcommand.clone());
        tags.insert("stage".to_owned(), stage.to_owned());
        tags
    }

    fn get_metric_structure(command: &str, subcommand: &str, value: Value) -> (MetricsValue, MetricsValue, MetricsValue, MetricsValue) {
        let counters: Counters = serde_json::from_value(value.clone()).unwrap();

        let executed_tag = MetricsService::get_command_tags(String::from(command), String::from(subcommand), String::from("executed"));
        let queued_tag = MetricsService::get_command_tags(String::from(command), String::from(subcommand), String::from("queued"));
        let executed_count = MetricsValue::new(counters.count, executed_tag.clone());
        let queued_count = MetricsValue::new(counters.count, queued_tag.clone());
        let executed_duration = MetricsValue::new(counters.sum,executed_tag);
        let queued_duration = MetricsValue::new(counters.sum,queued_tag);

        return (executed_count, queued_count, executed_duration, queued_duration)
    }

    pub fn append_command_metrics(&self, metrics_map: &mut Map<String, Value>) -> IndyResult<()> {
        let mut commands_count = Vec::new();
        let mut commands_duration_ms = Vec::new();
        let root = serde_json::to_value(self.metrics.borrow().clone()).unwrap();

        for command in root.as_object().unwrap().iter() {
            for subcommand in command.1.as_object().unwrap().iter() {
                let (executed_count, queued_count, executed_duration, queued_duration) = MetricsService::get_metric_structure(command.0, subcommand.0, subcommand.1.clone());
                commands_count.push(executed_count);
                commands_count.push(queued_count);
                commands_duration_ms.push(executed_duration);
                commands_duration_ms.push(queued_duration);
            }
        }

        metrics_map.insert(
            String::from("commands_count"),
            serde_json::to_value(commands_count)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );
        metrics_map.insert(
            String::from("commands_duration_ms"),
            serde_json::to_value(commands_duration_ms)
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
        );

        Ok(())
    }
}

pub trait CommandAware2<T> {
    fn cmd_left_queue(&self, command: T, duration: u128);
    fn cmd_executed(&self, command: T, duration: u128);
    fn borrow_metric_mut(&self, command: T) -> &mut CommandMetric;
}

impl CommandAware2<&Command> for MetricsService {
    fn cmd_left_queue(&self, command: &Command, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_left_queue(duration);
    }

    fn cmd_executed(&self, command: &Command, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_executed(duration);
    }

    fn borrow_metric_mut(&self, command: &Command) -> &mut CommandMetric {
        let mut some = self.metrics.borrow_mut();
        some.borrow_metrics_mut(command)
    }
}

impl CommandAware2<&VerifierCommand> for MetricsService {
    fn cmd_left_queue(&self, command: &VerifierCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_left_queue(duration);
    }

    fn cmd_executed(&self, command: &VerifierCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_executed(duration);
    }

    fn borrow_metric_mut(&self, command: &VerifierCommand) -> &mut CommandMetric {
        self.metrics.borrow_mut().borrow_metrics_mut(command)
    }
}

impl CommandAware2<&ProverCommand> for MetricsService {
    fn cmd_left_queue(&self, command: &ProverCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_left_queue(duration);
    }

    fn cmd_executed(&self, command: &ProverCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_executed(duration);
    }

    fn borrow_metric_mut(&self, command: &ProverCommand) -> &mut CommandMetric {
        self.metrics.borrow_mut().borrow_metrics_mut(command)
    }
}

impl CommandAware2<&IssuerCommand> for MetricsService {
    fn cmd_left_queue(&self, command: &IssuerCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_left_queue(duration);
    }

    fn cmd_executed(&self, command: &IssuerCommand, duration: u128) {
        self.metrics.borrow_mut().borrow_metrics_mut(command).cmd_executed(duration);
    }

    fn borrow_metric_mut(&self, command: &IssuerCommand) -> &mut CommandMetric {
        self.metrics.borrow_mut().borrow_metrics_mut(command)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counters_are_initialized_as_zeros() {
        let metrics_service = MetricsService::new();
        for index in (0..MetricsService::commands_count()).rev() {
            assert_eq!(metrics_service.queued_commands_count.borrow()[index as usize], 0);
            assert_eq!(metrics_service.queued_commands_duration_ms.borrow()[index as usize], 0);
            assert_eq!(metrics_service.executed_commands_count.borrow()[index as usize], 0);
            assert_eq!(metrics_service.executed_commands_duration_ms.borrow()[index as usize], 0);
        }
    }

    #[test]
    fn test_cmd_left_queue_increments_relevant_queued_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandIndex::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_left_queue(index, duration1);

        assert_eq!(metrics_service.queued_commands_count.borrow()[index as usize], 1);
        assert_eq!(metrics_service.queued_commands_duration_ms.borrow()[index as usize], duration1);

        metrics_service.cmd_left_queue(index, duration2);

        assert_eq!(metrics_service.queued_commands_count.borrow()[index as usize], 1 + 1);
        assert_eq!(metrics_service.queued_commands_duration_ms.borrow()[index as usize],
                   duration1 + duration2);
        assert_eq!(metrics_service.executed_commands_count.borrow()[index as usize], 0);
        assert_eq!(metrics_service.executed_commands_duration_ms.borrow()[index as usize], 0);
    }

    #[test]
    fn test_cmd_executed_increments_relevant_executed_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandIndex::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_executed(index, duration1);

        assert_eq!(metrics_service.executed_commands_count.borrow()[index as usize], 1);
        assert_eq!(metrics_service.executed_commands_duration_ms.borrow()[index as usize], duration1);

        metrics_service.cmd_executed(index, duration2);

        assert_eq!(metrics_service.queued_commands_count.borrow()[index as usize], 0);
        assert_eq!(metrics_service.queued_commands_duration_ms.borrow()[index as usize], 0);
        assert_eq!(metrics_service.executed_commands_count.borrow()[index as usize], 1+1);
        assert_eq!(metrics_service.executed_commands_duration_ms.borrow()[index as usize], duration1 + duration2);
    }

    #[test]
    fn test_append_command_metrics() {
        let metrics_service = MetricsService::new();
        let mut metrics_map = serde_json::Map::new();

        metrics_service.append_command_metrics(&mut metrics_map);

        assert!(metrics_map.contains_key("commands_count"));
        assert!(metrics_map.contains_key("commands_duration_ms"));
        assert_eq!(
            metrics_map
                .get("commands_count")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            COMMANDS_COUNT * 2
        );
        assert_eq!(
            metrics_map
                .get("commands_duration_ms")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            COMMANDS_COUNT * 2
        );

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

        let mut expected_commands_count = [
            json!({"tags":{"command":"payments","stage":"executed","subcommand":"build_set_txn_fees_req_ack"},"value":0}),
            json!({"tags":{"command":"pairwise","stage":"queued","subcommand":"pairwise_exists"},"value":0}),
            json!({"tags":{"command":"cache","stage":"executed","subcommand":"purge_cred_def_cache"},"value":0}),
            json!({"tags":{"command":"non_secrets","stage":"queued","subcommand":"fetch_search_next_records"},"value":0}),
        ];

        let mut expected_commands_duration_ms = [
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
}