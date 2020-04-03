use utils::libindy::wallet;
use error::VcxResult;

pub struct PendingMessage;

impl PendingMessage {
    const TYPE: &'static str = "PENDING_MESSAGE";

    pub fn add(id: &str, message: &str) -> VcxResult<()> {
        wallet::add_record(Self::TYPE, id, &message, None)
    }

    pub fn get(id: &str) -> Option<String> {
        match wallet::get_record(Self::TYPE, id, "{}") {
            Ok(record) => {
                let record: serde_json::Value = serde_json::from_str(&record).ok()?;
                record["value"].as_str().map(String::from)
            }
            _ => None
        }
    }
}