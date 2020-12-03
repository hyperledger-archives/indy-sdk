use crate::services::metrics::command_index::CommandIndex;
use convert_case::{Case, Casing};
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use models::MetricsValue;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::HashMap;

pub mod command_index;
pub mod models;

const QUEUED_COMMANDS_COUNT: &str = "queued_commands_count";
const QUEUED_COMMANDS_DURATION_MS: &str = "queued_commands_duration_ms";
const EXECUTED_COMMANDS_COUNT: &str = "executed_commands_count";
const EXECUTED_COMMANDS_DURATION_MS: &str = "executed_commands_duration_ms";

const COMMANDS_COUNT: usize = MetricsService::commands_count();

pub struct MetricsService {
    queued_commands_count: RefCell<[u128; COMMANDS_COUNT]>,
    queued_commands_duration_ms: RefCell<[u128; COMMANDS_COUNT]>,

    executed_commands_count: RefCell<[u128; COMMANDS_COUNT]>,
    executed_commands_duration_ms: RefCell<[u128; COMMANDS_COUNT]>,
}

impl MetricsService {
    pub fn new() -> Self {
        MetricsService {
            queued_commands_count: RefCell::new([u128::MIN; COMMANDS_COUNT]),
            queued_commands_duration_ms: RefCell::new([u128::MIN; COMMANDS_COUNT]),

            executed_commands_count: RefCell::new([u128::MIN; COMMANDS_COUNT]),
            executed_commands_duration_ms: RefCell::new([u128::MIN; COMMANDS_COUNT]),
        }
    }

    pub fn cmd_left_queue(&self, command_index: CommandIndex, duration: u128) {
        self.queued_commands_count.borrow_mut()[command_index as usize] += 1;
        self.queued_commands_duration_ms.borrow_mut()[command_index as usize] += duration;
    }

    pub fn cmd_executed(&self, command_index: CommandIndex, duration: u128) {
        self.executed_commands_count.borrow_mut()[command_index as usize] += 1;
        self.executed_commands_duration_ms.borrow_mut()[command_index as usize] += duration;
    }

    pub fn cmd_name(index: usize) -> (String, String) {
        let cmd = CommandIndex::from(index).to_string().find("Command");
        match cmd {
            None => (
                CommandIndex::from(index)
                    .to_string()
                    .to_case(Case::Snake)
                    .to_owned(),
                String::from(""),
            ),
            Some(cmd) => (
                CommandIndex::from(index).to_string()[0..Some(cmd).unwrap()]
                    .to_case(Case::Snake)
                    .to_owned(),
                CommandIndex::from(index).to_string()[Some(cmd).unwrap() + 7..]
                    .to_case(Case::Snake)
                    .to_owned(),
            ),
        }
    }
    const fn commands_count() -> usize {
        CommandIndex::VARIANT_COUNT
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

    pub fn append_command_metrics(&self, metrics_map: &mut Map<String, Value>) -> IndyResult<()> {
        let mut commands_count = Vec::new();
        let mut commands_duration_ms = Vec::new();

        for index in (0..MetricsService::commands_count()).rev() {
            let (command, subcommand) = MetricsService::cmd_name(index);
            let mut tags_executed = MetricsService::get_command_tags(
                command.clone(),
                subcommand.clone(),
                String::from("executed"),
            );
            let mut tags_queued = MetricsService::get_command_tags(
                command.clone(),
                subcommand.clone(),
                String::from("queued"),
            );

            commands_count.push(
                serde_json::to_value(MetricsValue::new(
                    self.executed_commands_count.borrow()[index] as usize,
                    tags_executed.clone(),
                ))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
            );
            commands_count.push(
                serde_json::to_value(MetricsValue::new(
                    self.queued_commands_count.borrow()[index] as usize,
                    tags_queued.clone(),
                ))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
            );

            commands_duration_ms.push(
                serde_json::to_value(MetricsValue::new(
                    self.executed_commands_duration_ms.borrow()[index] as usize,
                    tags_executed,
                ))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
            );
            commands_duration_ms.push(
                serde_json::to_value(MetricsValue::new(
                    self.queued_commands_duration_ms.borrow()[index] as usize,
                    tags_queued,
                ))
                .to_indy(IndyErrorKind::IOError, "Unable to convert json")?,
            );
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
            json!({"tags":{"command":"metrics","stage":"queued","subcommand":"collect_metrics"},"value":0}),
            json!({"tags":{"command":"cache","stage":"executed","subcommand":"purge_cred_def_cache"},"value":0}),
            json!({"tags":{"command":"non_secrets","stage":"queued","subcommand":"fetch_search_next_records"},"value":0}),
        ];

        let mut expected_commands_duration_ms = [
            json!({"tags":{"command":"payments","stage":"executed","subcommand":"build_set_txn_fees_req_ack"},"value":0}),
            json!({"tags":{"command":"metrics","stage":"queued","subcommand":"collect_metrics"},"value":0}),
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