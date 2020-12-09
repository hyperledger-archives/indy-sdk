use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MetricsValue {
    value: u128,
    tags: HashMap<String, String>,
}

impl MetricsValue {
    pub fn new(value: u128, tags: HashMap<String, String>) -> Self {
        MetricsValue { value, tags }
    }
}
