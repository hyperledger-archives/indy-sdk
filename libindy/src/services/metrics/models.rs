use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct MetricsValue {
    value: usize,
    tags: serde::Map<String, String>,
}


impl MetricsValue {
    pub fn new(value: usize, tags: &Value) -> Self {
        MetricsValue { value, tags }
    }
    /*
    pub fn add(value: string, tag: &Value) -> Self {
        Self.tags.insert(value, tag)
    }
    */
}

/*

#[derive(Serialize, Deserialize)]
pub struct MetricsValue {
    value: usize,
    tags: MetricsTags,
}

impl MetricsValue {
    pub fn new(value: usize, command: &str, subcommand: &str, stage: MetricsStage) -> Self {
        let tags = MetricsTags::new(command.to_owned(), subcommand.to_owned(), stage);
        MetricsValue { value, tags }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetricsTags {
    command: String,
    subcommand: String,
    stage: MetricsStage,
}

impl MetricsTags {
    pub fn new(command: String, subcommand: String, stage: MetricsStage) -> Self {
        MetricsTags {
            command: command.to_owned(),
            subcommand: subcommand.to_owned(),
            stage,
        }
    }

    pub fn add(key: String, value: String){

    }
}


#[derive(Serialize, Deserialize)]
pub enum MetricsStage {
    Queued,
    Executed,
}

pub fn checkMetricsStructure(map: &mut Map<String, Value>) -> &mut Map<String, Value> {
    if map.get("commands_count") == None {
        map.insert("commands_count".to_owned(), serde_json::to_value(Vec::new()).unwrap())
    }

    if map.get("commands_duration") == None {
        map.insert("commands_duration".to_owned(), serde_json::to_value(Vec::new()).unwrap())
    }


    map
}

 */
