#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MimeType {
    #[serde(rename = "text/plain")]
    Plain
}

impl Default for MimeType {
    fn default() -> MimeType {
        MimeType::Plain
    }
}
