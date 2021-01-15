use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BUCKET_COUNT: usize = 16;
const LIST_LE: [f64; BUCKET_COUNT-1] = [0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 5000.0, 10000.0, 20000.0];

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
    pub fn new() -> Self {
        CommandCounters {count: 0, duration_ms_sum: 0, duration_ms_bucket: [0; BUCKET_COUNT]}
    }

    pub fn add(&mut self, duration: u128) {
        self.count += 1;
        self.duration_ms_sum += duration;
        self.add_buckets(duration);
    }

    fn add_buckets(&mut self, duration: u128) {
        for (le_index, le_value) in LIST_LE.iter().enumerate() {
            if duration <= *le_value as u128 {
                self.duration_ms_bucket[le_index] += 1;
            }
        }
        self.duration_ms_bucket[self.duration_ms_bucket.len()-1] += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counters_are_initialized_as_zeros() {
        let command_counters = CommandCounters::new();
        assert_eq!(command_counters.count, 0);
        assert_eq!(command_counters.duration_ms_sum, 0);
        assert_eq!(command_counters.duration_ms_bucket, [0; BUCKET_COUNT]);
    }
}
