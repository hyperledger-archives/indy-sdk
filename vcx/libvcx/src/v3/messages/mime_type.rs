#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MimeType {
    #[serde(rename = "text/plain")]
    Plain
}