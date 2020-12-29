use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BUCKET_COUNT: usize = 16;
const LIST_LE: [&str; BUCKET_COUNT] = ["0.5", "1", "2", "5", "10", "20", "50", "100", "200", "500", "1000", "2000", "5000", "10000", "20000", "+Inf"];

#[derive(Serialize, Deserialize)]
pub struct MetricsValue {
    value: usize,
    tags: HashMap<String, String>,
}

impl MetricsValue {
    pub fn new(value: usize, tags: HashMap<String, String>) -> Self {
        MetricsValue { value, tags }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CommandCounters {
    pub count: u128,
    pub duration_ms_sum: u128,
    pub duration_ms_bucket: [u128; BUCKET_COUNT],
}

impl CommandCounters {
    pub fn new(count: u128, duration_ms_sum: u128, duration_ms_bucket: [u128; BUCKET_COUNT]) -> Self {
        CommandCounters {count, duration_ms_sum, duration_ms_bucket}
    }

    pub fn add(&mut self, duration: u128) {
        self.count += 1;
        self.duration_ms_sum += duration;
        self.add_buckets(duration);
    }

    fn add_buckets(&mut self, duration: u128) {
        for (mut le_index, le_value) in LIST_LE.iter().enumerate() {
            let index: f64;

            match le_value.parse::<f64>() {
                Ok(le_value) => {
                    index = le_value;
                },
                Err(_err) => {
                    index = duration as f64;
                    le_index = LIST_LE.len()-1;
                }
            }
            if duration <= index as u128 {
                self.duration_ms_bucket[le_index] += 1;
            }
        }

    }

}