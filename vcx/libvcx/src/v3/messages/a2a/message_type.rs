use v3::messages::a2a::message_family::MessageFamilies;
use messages::message_type::parse_message_type;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MessageType {
    pub did: String,
    pub family: MessageFamilies,
    pub version: String,
    pub type_: String,
}

impl MessageType {
    pub fn build(family: MessageFamilies, name: &str) -> MessageType {
        MessageType {
            did: MessageFamilies::DID.to_string(),
            version: family.version().to_string(),
            family,
            type_: name.to_string(),
        }
    }
}


impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match value.as_str() {
            Some(type_) => {
                let (did, family, version, type_) = parse_message_type(type_).map_err(de::Error::custom)?;
                Ok(MessageType {
                    did,
                    family: MessageFamilies::from(family),
                    version,
                    type_,
                })
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = Value::String(self.to_string());
        value.serialize(serializer)
    }
}

impl ::std::string::ToString for MessageType {
    fn to_string(&self) -> String {
        format!("{};spec/{}/{}/{}", self.did, self.family.to_string(), self.version, self.type_)
    }
}