use crate::services::metrics::command_index::CommandIndex;
use convert_case::{Case, Casing};
use serde_json::{Map, Value};
use std::cell::RefCell;
use models::MetricsValue;

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
            None => (CommandIndex::from(index).to_string().to_owned(), String::from("")),
            Some(cmd) => (CommandIndex::from(index).to_string()[0..Some(cmd).unwrap()].to_owned(), CommandIndex::from(index).to_string()[Some(cmd).unwrap()+7..].to_owned()) ,
        }
    }
    const fn commands_count() -> usize {
        CommandIndex::VARIANT_COUNT
    }

    pub fn append_command_metrics(&self, metrics_map: &mut Map<String, Value>) {
        struct MetricsTags {
            command: String,
            subcommand: String,
            stage: String,
        }
        let mut commands_count = Vec::new();
        let mut commands_duration = Vec::new();
        for index in (0..MetricsService::commands_count()).rev() {
            let (command, subcommand) = MetricsService::cmd_name(index);
            let mut tags = Value::from(MetricsTags{command, subcommand, stage: "executed".to_string() });

            commands_count.push(
            Value::from(MetricsValue::new(self.executed_commands_count.borrow()[index] as usize, &tags))
            );

            commands_count.push(
                Value::from(MetricsValue::new(self.queued_commands_count.borrow()[index] as usize, &tags))
            );

            commands_duration.push(
                Value::from(MetricsValue::new(self.executed_commands_duration_ms.borrow()[index] as usize, &tags))
            );

            commands_duration.push(
                Value::from(MetricsValue::new(self.queued_commands_duration_ms.borrow()[index] as usize, &tags))
            );
        }
        metrics_map.insert("commands_count".to_owned(), serde_json::to_value(commands_count).unwrap());
        metrics_map.insert("commands_duration".to_owned(), serde_json::to_value(commands_duration).unwrap());
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
        assert!(metrics_map.contains_key("commands_duration"));
        assert_eq!(metrics_map.get("commands_count").unwrap().as_array().unwrap().len() + metrics_map.get("commands_duration").unwrap().as_array().unwrap().len(), COMMANDS_COUNT * 4);
    }
}