use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
