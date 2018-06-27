#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UTXOOutput {
    #[serde(rename = "paymentAddress")]
    pub payment_address: String,
    pub amount: i32,
    pub extra: Option<String>
}

impl Clone for UTXOOutput {
    fn clone(&self) -> Self {
        UTXOOutput {
            payment_address: self.payment_address.clone(),
            amount: self.amount.clone(),
            extra: self.extra.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UTXOInfo {
    pub txo: String,
    #[serde(rename = "paymentAddress")]
    pub payment_address: String,
    pub amount: i32,
    pub extra: Option<String>
}