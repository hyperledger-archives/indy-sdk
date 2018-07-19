#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Output {
    pub recipient: String,
    pub amount: i32,
    pub extra: Option<String>
}

impl Clone for Output {
    fn clone(&self) -> Self {
        Output {
            recipient: self.recipient.clone(),
            amount: self.amount.clone(),
            extra: self.extra.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReceiptInfo {
    pub receipt: String,
    pub recipient: String,
    pub amount: i32,
    pub extra: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct SourceInfo {
    pub source: String,
    #[serde(rename = "paymentAddress")]
    pub payment_address: String,
    pub amount: i32,
    pub extra: Option<String>
}