use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

#[serde(tag = "mime-type")]
pub enum Attachment {
    #[serde(rename = "application/json")]
    JSON(Json)
}

struct Json {
    id: AttachmentId,
    data: AttachmentData
}

struct AttachmentId(String);

struct AttachmentData([u8]);

// TODO: implement deserialization