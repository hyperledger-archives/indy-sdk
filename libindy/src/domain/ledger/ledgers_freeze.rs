use super::constants::{FREEZE_LEDGERS, GET_FROZEN_LEDGERS};

#[derive(Serialize, PartialEq, Debug)]
pub struct FreezeLedgersOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub ledgers_ids: Vec<u64>,
}

impl FreezeLedgersOperation {
    pub fn new(ledgers_ids: Vec<u64>) -> FreezeLedgersOperation {
        FreezeLedgersOperation {
            ledgers_ids,
            _type: FREEZE_LEDGERS.to_string()
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetFrozenLedgersOperation {
    #[serde(rename = "type")]
    pub _type: String
}

impl GetFrozenLedgersOperation {
    pub fn new() -> GetFrozenLedgersOperation {
        GetFrozenLedgersOperation {
            _type: GET_FROZEN_LEDGERS.to_string()
        }
    }
}
