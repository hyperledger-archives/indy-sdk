#[serde(tag = "mime-type")]
#[derive(Debug, Serialize, Deserialize)]
pub enum Attachment {
    #[serde(rename = "application/json")]
    JSON(Json)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json {
    id: AttachmentId,
    data: AttachmentData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentId(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentData(Vec<u8>);

// TODO: implement deserialization