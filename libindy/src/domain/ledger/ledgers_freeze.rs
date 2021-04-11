use super::constants::{LEDGERS_FREEZE, GET_FROZEN_LEDGERS};

#[derive(Serialize, PartialEq, Debug)]
pub struct LedgersFreezeOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub ledgers_ids: Vec<u64>,
}

impl LedgersFreezeOperation {
    pub fn new(ledgers_ids: Vec<u64>) -> LedgersFreezeOperation {
        LedgersFreezeOperation {
            _type: LEDGERS_FREEZE.to_string(),
            ledgers_ids
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
