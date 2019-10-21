use std::u8;
use messages::message_type::{parse_message_type, DID};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub mod ack;
pub mod connection;
pub mod error;
pub mod forward;
pub mod attachment;

use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::forward::Forward;
use self::ack::Ack;

use utils::uuid;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum A2AMessage {
    /// routing
    Forward(Forward),

    /// DID Exchange
    ConnectionRequest(Request),
    ConnectionResponse(Response),
    ProblemReport(ProblemReport),

    /// notification
    Ack(Ack),

    /// Any Raw Message
    Generic(String)
}

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageType = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type.type_.as_str() {
            "forward" => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessage::Forward(msg))
                    .map_err(de::Error::custom)
            }
            "request" => {
                Request::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            "response" => {
                Response::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            "problem_report" => {
                ProblemReport::deserialize(value)
                    .map(|msg| A2AMessage::ProblemReport(msg))
                    .map_err(de::Error::custom)
            }
            "ack" => {
                Ack::deserialize(value)
                    .map(|msg| A2AMessage::Ack(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum A2AMessageKinds {
    Forward,
    Invitation,
    Request,
    Response,
    ProblemReport,
    Ack
}

impl A2AMessageKinds {
    pub fn family(&self) -> MessageFamilies {
        match self {
            A2AMessageKinds::Forward => MessageFamilies::Routing,
            A2AMessageKinds::Invitation => MessageFamilies::DidExchange,
            A2AMessageKinds::Request => MessageFamilies::DidExchange,
            A2AMessageKinds::Response => MessageFamilies::DidExchange,
            A2AMessageKinds::ProblemReport => MessageFamilies::DidExchange,
            A2AMessageKinds::Ack => MessageFamilies::Notification,
        }
    }

    pub fn name(&self) -> String {
        match self {
            A2AMessageKinds::Forward => "forward".to_string(),
            A2AMessageKinds::Invitation => "invitation".to_string(),
            A2AMessageKinds::Request => "request".to_string(),
            A2AMessageKinds::Response => "response".to_string(),
            A2AMessageKinds::ProblemReport => "problem_report".to_string(),
            A2AMessageKinds::Ack => "ack".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageFamilies {
    Routing,
    DidExchange,
    Notification,
    Unknown(String)
}

impl MessageFamilies {
    pub fn version(&self) -> &'static str {
        match self {
            MessageFamilies::Routing => "1.0",
            MessageFamilies::DidExchange => "1.0",
            MessageFamilies::Notification => "1.0",
            MessageFamilies::Unknown(_) => "1.0"
        }
    }
}

impl From<String> for MessageFamilies {
    fn from(family: String) -> Self {
        match family.as_str() {
            "routing" => MessageFamilies::Routing,
            "didexchange" => MessageFamilies::DidExchange,
            family @ _ => MessageFamilies::Unknown(family.to_string())
        }
    }
}

impl ::std::string::ToString for MessageFamilies {
    fn to_string(&self) -> String {
        match self {
            MessageFamilies::Routing => "routing".to_string(),
            MessageFamilies::DidExchange => "didexchange".to_string(),
            MessageFamilies::Notification => "notification".to_string(),
            MessageFamilies::Unknown(family) => family.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageType {
    pub did: String,
    pub family: MessageFamilies,
    pub version: String,
    pub type_: String,
}

impl MessageType {
    pub fn build(kind: A2AMessageKinds) -> MessageType {
        MessageType {
            did: DID.to_string(),
            family: kind.family(),
            version: kind.family().version().to_string(),
            type_: kind.name(),
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
        let value = Value::String(format!("{};spec/{}/{}/{}", self.did, self.family.to_string(), self.version, self.type_));
        value.serialize(serializer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(pub String);

impl MessageId {
    pub fn new() -> MessageId {
        MessageId(uuid::uuid())
    }
}