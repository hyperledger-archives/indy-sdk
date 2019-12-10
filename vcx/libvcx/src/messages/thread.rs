use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Thread {
    pub thid: Option<String>,
    pub pthid: Option<String>,
    #[serde(default)]
    pub sender_order: u32,
    #[serde(default)]
    pub received_orders: HashMap<String, u32>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread::default()
    }

    pub fn set_thid(mut self, thid: String) -> Thread {
        self.thid = Some(thid);
        self
    }

    pub fn increment_receiver(&mut self, did: &str) {
        self.received_orders.entry(did.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(0);
    }

    pub fn is_reply(&self, id: &str) -> bool {
        self.thid.clone().unwrap_or_default() == id.to_string()
    }
}

impl Default for Thread {
    fn default() -> Thread {
        Thread {
            thid: None,
            pthid: None,
            sender_order: 0,
            received_orders: HashMap::new(),
        }
    }
}