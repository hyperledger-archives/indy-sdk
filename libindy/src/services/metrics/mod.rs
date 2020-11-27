use crate::services::metrics::command_index::CommandIndex;
use convert_case::{Case, Casing};
use serde_json::{Map, Value};
use futures::lock::Mutex;

pub mod command_index;

const QUEUED_COMMANDS_COUNT: &str = "queued_commands_count";
const QUEUED_COMMANDS_DURATION_MS: &str = "queued_commands_duration_ms";
const EXECUTED_COMMANDS_COUNT: &str = "executed_commands_count";
const EXECUTED_COMMANDS_DURATION_MS: &str = "executed_commands_duration_ms";

const COMMANDS_COUNT: usize = MetricsService::commands_count();

pub struct MetricsService {
    queued_commands_count: Mutex<[u128; COMMANDS_COUNT]>,
    queued_commands_duration_ms: Mutex<[u128; COMMANDS_COUNT]>,

    executed_commands_count: Mutex<[u128; COMMANDS_COUNT]>,
    executed_commands_duration_ms: Mutex<[u128; COMMANDS_COUNT]>,
}

impl MetricsService {
    pub fn new() -> Self {
        MetricsService {
            queued_commands_count: Mutex::new([u128::MIN; COMMANDS_COUNT]),
            queued_commands_duration_ms: Mutex::new([u128::MIN; COMMANDS_COUNT]),

            executed_commands_count: Mutex::new([u128::MIN; COMMANDS_COUNT]),
            executed_commands_duration_ms: Mutex::new([u128::MIN; COMMANDS_COUNT]),
        }
    }

    pub async fn cmd_left_queue(&self, command_index: CommandIndex, duration: u128) {
        self.queued_commands_count.lock().await[command_index as usize] += 1;
        self.queued_commands_duration_ms.lock().await[command_index as usize] += duration;
    }

    pub async fn cmd_executed(&self, command_index: CommandIndex, duration: u128) {
        self.executed_commands_count.lock().await[command_index as usize] += 1;
        self.executed_commands_duration_ms.lock().await[command_index as usize] += duration;
    }
    pub fn cmd_name(index: usize) -> String {
        CommandIndex::from(index).to_string().to_case(Case::Snake)
    }
    const fn commands_count() -> usize {
        CommandIndex::VARIANT_COUNT
    }

    pub async fn append_command_metrics(&self, metrics_map: &mut Map<String, Value>) {
        for index in (0..MetricsService::commands_count()).rev() {
            let cmd_name = MetricsService::cmd_name(index);
            metrics_map.insert(format!("{}_{}", cmd_name.as_str(), EXECUTED_COMMANDS_COUNT).as_str().to_string(),
                               Value::from(self.executed_commands_count.lock().await[index] as usize));
            metrics_map.insert(format!("{}_{}", cmd_name.as_str(), EXECUTED_COMMANDS_DURATION_MS).as_str().to_string(),
                               Value::from(self.executed_commands_duration_ms.lock().await[index] as usize));
            metrics_map.insert(format!("{}_{}", cmd_name.as_str(), QUEUED_COMMANDS_COUNT).as_str().to_string(),
                               Value::from(self.queued_commands_count.lock().await[index] as usize));
            metrics_map.insert(format!("{}_{}", cmd_name.as_str(), QUEUED_COMMANDS_DURATION_MS).as_str().to_string(),
                               Value::from(self.queued_commands_duration_ms.lock().await[index] as usize));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counters_are_initialized_as_zeros() {
        let metrics_service = MetricsService::new();
        for index in (0..MetricsService::commands_count()).rev() {
            assert_eq!(metrics_service.queued_commands_count.lock().await[index as usize], 0);
            assert_eq!(metrics_service.queued_commands_duration_ms.lock().await[index as usize], 0);
            assert_eq!(metrics_service.executed_commands_count.lock().await[index as usize], 0);
            assert_eq!(metrics_service.executed_commands_duration_ms.lock().await[index as usize], 0);
        }
    }

    #[test]
    fn test_cmd_left_queue_increments_relevant_queued_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandIndex::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_left_queue(index, duration1);

        assert_eq!(metrics_service.queued_commands_count.lock().await[index as usize], 1);
        assert_eq!(metrics_service.queued_commands_duration_ms.lock().await[index as usize], duration1);

        metrics_service.cmd_left_queue(index, duration2);

        assert_eq!(metrics_service.queued_commands_count.lock().await[index as usize], 1 + 1);
        assert_eq!(metrics_service.queued_commands_duration_ms.lock().await[index as usize],
                   duration1 + duration2);
        assert_eq!(metrics_service.executed_commands_count.lock().await[index as usize], 0);
        assert_eq!(metrics_service.executed_commands_duration_ms.lock().await[index as usize], 0);
    }

    #[test]
    fn test_cmd_executed_increments_relevant_executed_counters() {
        let metrics_service = MetricsService::new();
        let index = CommandIndex::IssuerCommandCreateSchema;
        let duration1 = 5u128;
        let duration2 = 2u128;

        metrics_service.cmd_executed(index, duration1);

        assert_eq!(metrics_service.executed_commands_count.lock().await[index as usize], 1);
        assert_eq!(metrics_service.executed_commands_duration_ms.lock().await[index as usize], duration1);

        metrics_service.cmd_executed(index, duration2);

        assert_eq!(metrics_service.queued_commands_count.lock().await[index as usize], 0);
        assert_eq!(metrics_service.queued_commands_duration_ms.lock().await[index as usize], 0);
        assert_eq!(metrics_service.executed_commands_count.lock().await[index as usize], 1+1);
        assert_eq!(metrics_service.executed_commands_duration_ms.lock().await[index as usize], duration1 + duration2);
    }

    #[test]
    fn test_append_command_metrics() {
        let metrics_service = MetricsService::new();
        let mut metrics_map = serde_json::Map::new();

        metrics_service.append_command_metrics(&mut metrics_map);

        assert_eq!(metrics_map.len(), COMMANDS_COUNT * 4);
        assert!(metrics_map.contains_key("issuer_command_create_schema_queued_commands_count"));
        assert!(metrics_map.contains_key("issuer_command_create_schema_queued_commands_duration_ms"));
        assert!(metrics_map.contains_key("issuer_command_create_schema_executed_commands_count"));
        assert!(metrics_map.contains_key("issuer_command_create_schema_executed_commands_duration_ms"));
    }
}