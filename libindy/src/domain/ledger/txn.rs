use super::constants::GET_TXN;

#[derive(Serialize, PartialEq, Debug)]
pub struct GetTxnOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: i32,
    #[serde(rename = "ledgerId")]
    pub ledger_id: i32
}

impl GetTxnOperation {
    pub fn new(data: i32, ledger_id: i32) -> GetTxnOperation {
        GetTxnOperation {
            _type: GET_TXN.to_string(),
            data,
            ledger_id
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum LedgerType {
    POOL = 0,
    DOMAIN = 1,
    CONFIG = 2
}

impl LedgerType {
    pub fn to_id(&self) -> i32 {
        match *self {
            LedgerType::POOL => LedgerType::POOL as i32,
            LedgerType::DOMAIN => LedgerType::DOMAIN as i32,
            LedgerType::CONFIG => LedgerType::CONFIG as i32,
        }
    }
}
