pub mod create_key;
pub mod invite;
pub mod validation;
pub mod get_message;
pub mod send_message;
pub mod update_profile;
pub mod proofs;
pub mod agent_utils;
pub mod update_connection;
pub mod update_message;

use std::u8;
use settings;
use utils::libindy::crypto;
use utils::error;
use self::create_key::{CreateKeyBuilder, CreateKey, CreateKeyResponse};
use self::update_connection::{DeleteConnectionBuilder, UpdateConnection, UpdateConnectionResponse};
use self::update_profile::{UpdateProfileDataBuilder, UpdateConfigs, UpdateConfigsResponse};
use self::invite::{
    SendInviteBuilder, SendInvitePayloadV1, SendInviteMessageDetails, SendInviteMessageDetailsResponse, ConnectionRequestResponse,
    AcceptInviteBuilder, AcceptInvitePayloadV1, AcceptInviteMessageDetails, ConnectionRequestAnswerResponse
};
use self::get_message::{GetMessagesBuilder, GetMessages, GetMessagesResponse, MessagesByConnections};
use self::send_message::SendMessageBuilder;
use self::update_message::{UpdateMessageStatusByConnections, UpdateMessageStatusByConnectionsResponse};
use self::proofs::proof_request::ProofRequestMessage;
use self::agent_utils::{Connect, ConnectResponse, SignUp, SignUpResponse, CreateAgent, CreateAgentResponse, UpdateConnectionMethod};

use serde::{de, Deserializer, Deserialize, Serializer, Serialize};
use serde_json::Value;
use regex::Regex;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum A2AMessage {
    /// routing
    Forward(Forward),

    /// onboarding
    Connect(Connect),
    ConnectResponse(ConnectResponse),
    SignUp(SignUp),
    SignUpResponse(SignUpResponse),
    CreateAgent(CreateAgent),
    CreateAgentResponse(CreateAgentResponse),

    /// PW Connection
    CreateKey(CreateKey),
    CreateKeyResponse(CreateKeyResponse),
    CreateMessage(CreateMessage),
    MessageDetail(MessageDetail),
    MessageCreated(MessageCreated),
    MessageSent(MessageSent),
    UpdateConnection(UpdateConnection),
    UpdateConnectionResponse(UpdateConnectionResponse),
    GetMessages(GetMessages),
    GetMessagesResponse(GetMessagesResponse),
    GetMessagesByConnections(GetMessages),
    MessagesByConnections(MessagesByConnections),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    UpdateMessageStatusByConnectionsResponse(UpdateMessageStatusByConnectionsResponse),
    UpdateConnectionMethod(UpdateConnectionMethod),

    /// PW Configs
    UpdateConfigs(UpdateConfigs),
    UpdateConfigsResponse(UpdateConfigsResponse),

    /// Version 1

    /// Pairwise connection
    ConnectionRequest(SendInvitePayloadV1),
    ConnectionRequestResponse(ConnectionRequestResponse),
    ConnectionRequestAnswer(AcceptInvitePayloadV1),
    ConnectionRequestAnswerResponse(ConnectionRequestAnswerResponse),
    /// Credential Exchange
    CredentialExchangeMessage(CredentialExchangeMessage),
    CredentialExchangeMessageResponse(CredentialExchangeMessageResponse),
}

// This macro allows to convert A2AMessage into specific variant
macro_rules! a2a_to_variant (($a2a_variant:ident, $type:path) => (
    impl $type {
        fn from_a2a_message(message: A2AMessage) -> Result<$type, u32> {
            match message {
                A2AMessage::$a2a_variant(msg) => Ok(msg),
                _ => Err(error::INVALID_HTTP_RESPONSE.code_num)
            }
        }
    }
));

a2a_to_variant!(ConnectResponse, ConnectResponse);
a2a_to_variant!(SignUpResponse, SignUpResponse);
a2a_to_variant!(CreateAgentResponse, CreateAgentResponse);
a2a_to_variant!(CreateKeyResponse, CreateKeyResponse);
a2a_to_variant!(GetMessagesResponse, GetMessagesResponse);
a2a_to_variant!(MessagesByConnections, MessagesByConnections);
a2a_to_variant!(ConnectionRequestResponse, ConnectionRequestResponse);
a2a_to_variant!(MessageCreated, MessageCreated);
a2a_to_variant!(ConnectionRequestAnswerResponse, ConnectionRequestAnswerResponse);
a2a_to_variant!(MessageSent, MessageSent);
a2a_to_variant!(CredentialExchangeMessageResponse, CredentialExchangeMessageResponse);
a2a_to_variant!(UpdateConnectionResponse, UpdateConnectionResponse);
a2a_to_variant!(UpdateMessageStatusByConnectionsResponse, UpdateMessageStatusByConnectionsResponse);
a2a_to_variant!(UpdateConfigsResponse, UpdateConfigsResponse);

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        let message_type: MessageTypes = serde_json::from_value(value["@type"].clone()).unwrap();
        match (message_type.name(), message_type.version()) {
            ("FWD", "1.0") => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessage::Forward(msg))
                    .map_err(de::Error::custom)
            }
            ("CONNECT", "1.0") => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessage::Connect(msg))
                    .map_err(de::Error::custom)
            }
            ("CONNECTED", "1.0") => {
                ConnectResponse::deserialize(value)
                    .map(|msg| A2AMessage::ConnectResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("SIGNUP", "1.0") => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessage::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            ("SIGNED_UP", "1.0") => {
                SignUpResponse::deserialize(value)
                    .map(|msg| A2AMessage::SignUpResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("CREATE_AGENT", "1.0") => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessage::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            ("AGENT_CREATED", "1.0") => {
                CreateAgentResponse::deserialize(value)
                    .map(|msg| A2AMessage::CreateAgentResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("CREATE_KEY", "1.0") => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessage::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            ("KEY_CREATED", "1.0") => {
                CreateKeyResponse::deserialize(value)
                    .map(|msg| A2AMessage::CreateKeyResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("CREATE_MSG", "1.0") => {
                CreateMessage::deserialize(value)
                    .map(|msg| A2AMessage::CreateMessage(msg))
                    .map_err(de::Error::custom)
            }
            ("MSG_DETAIL", "1.0") => {
                MessageDetail::deserialize(value)
                    .map(|msg| A2AMessage::MessageDetail(msg))
                    .map_err(de::Error::custom)
            }
            ("MSG_CREATED", "1.0") => {
                MessageCreated::deserialize(value)
                    .map(|msg| A2AMessage::MessageCreated(msg))
                    .map_err(de::Error::custom)
            }
            ("MSG_SENT", "1.0") | ("MSGS_SENT", "1.0") => {
                // TODO: STRANGE
                MessageSent::deserialize(value)
                    .map(|msg| A2AMessage::MessageSent(msg))
                    .map_err(de::Error::custom)
            }
            ("UPDATE_CONN_STATUS", "1.0") => {
                UpdateConnection::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConnection(msg))
                    .map_err(de::Error::custom)
            }
            ("CONN_STATUS_UPDATED", "1.0") => {
                UpdateConnectionResponse::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("GET_MSGS", "1.0") => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessage::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            ("MSGS", "1.0") => {
                GetMessagesResponse::deserialize(value)
                    .map(|msg| A2AMessage::GetMessagesResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("GET_MSGS_BY_CONNS", "1.0") => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessage::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            ("MSGS_BY_CONNS", "1.0") => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessage::MessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            ("UPDATE_MSG_STATUS_BY_CONNS", "1.0") => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessage::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            ("MSG_STATUS_UPDATED_BY_CONNS", "1.0") => {
                UpdateMessageStatusByConnectionsResponse::deserialize(value)
                    .map(|msg| A2AMessage::UpdateMessageStatusByConnectionsResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("UPDATE_COM_METHOD", "1.0") => {
                UpdateConnectionMethod::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConnectionMethod(msg))
                    .map_err(de::Error::custom)
            }
            ("UPDATE_CONFIGS", "1.0") => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            ("CONFIGS_UPDATED", "1.0") => {
                UpdateConfigsResponse::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConfigsResponse(msg))
                    .map_err(de::Error::custom)
            }
            ("CONNECTION_REQUEST", "1.0") => {
                SendInvitePayloadV1::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            ("CONNECTION_REQUEST_ANSWER", "1.0") => {
                AcceptInvitePayloadV1::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionRequestAnswer(msg))
                    .map_err(de::Error::custom)
            }
            ("CREDENTIAL_EXCHANGE", "1.0") => {
                CredentialExchangeMessage::deserialize(value)
                    .map(|msg| A2AMessage::CredentialExchangeMessage(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Forward {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "@fwd")]
    fwd: String,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

impl Forward {
    fn new(fwd: String, msg: Vec<u8>) -> Forward {
        Forward {
            msg_type: MessageTypes::build(A2AMessageKinds::Forward),
            fwd,
            msg,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CreateMessage {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    mtype: CredentialExchangeMessageType,
    #[serde(rename = "sendMsg")]
    send_msg: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(rename = "replyToMsgId")]
    reply_to_msg_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralMessageDetail {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
    title: Option<String>,
    detail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageCreated {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV0,
    pub uid: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageSent {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV0,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default)]
    pub uids: Vec<String>,
}

#[serde(untagged)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageDetail {
    ConnectionRequestAnswer(AcceptInviteMessageDetails),
    ConnectionRequest(SendInviteMessageDetails),
    ConnectionRequestResp(SendInviteMessageDetailsResponse),
    General(GeneralMessageDetail),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CredentialExchangeMessage {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    pub mtype: CredentialExchangeMessageType,
    #[serde(rename = "replyToMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_msg_id: Option<String>,
    pub send_msg: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CredentialExchangeMessageResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV0,
    pub uid: String,
    pub uids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CredentialExchangeMessageType {
    Other(String),
    ConnReq,
    ConnReqAnswer,
    CredOffer,
    CredReq,
    Cred,
    ProofReq,
    Proof,
}

impl Serialize for CredentialExchangeMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            CredentialExchangeMessageType::ConnReq => "connReq",
            CredentialExchangeMessageType::ConnReqAnswer => "connReqAnswer",
            CredentialExchangeMessageType::CredOffer => "credOffer",
            CredentialExchangeMessageType::CredReq => "credReq",
            CredentialExchangeMessageType::Cred => "cred",
            CredentialExchangeMessageType::ProofReq => "proofReq",
            CredentialExchangeMessageType::Proof => "proof",
            CredentialExchangeMessageType::Other(_type) => "_type",
        };
        Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CredentialExchangeMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        match value.as_str() {
            Some("connReq") => Ok(CredentialExchangeMessageType::ConnReq),
            Some("connReqAnswer") => Ok(CredentialExchangeMessageType::ConnReqAnswer),
            Some("credOffer") => Ok(CredentialExchangeMessageType::CredOffer),
            Some("credReq") => Ok(CredentialExchangeMessageType::CredReq),
            Some("cred") => Ok(CredentialExchangeMessageType::Cred),
            Some("proofReq") => Ok(CredentialExchangeMessageType::ProofReq),
            Some("proof") => Ok(CredentialExchangeMessageType::Proof),
            Some(_type) => Ok(CredentialExchangeMessageType::Other(_type.to_string())),
            _ => Err(de::Error::custom("Unexpected message type."))
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PayloadTypes {
    PayloadTypeV0(PayloadTypeV0),
    PayloadTypeV1(PayloadTypeV1),
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadTypeV0 {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadTypeV1 {
    did: String,
    family: MessageFamilies,
    version: String,
    type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PayloadKinds {
    CredOffer,
    CredReq,
    Cred,
    Proof,
    ProofRequest,
    Other(String)
}

impl PayloadKinds {
    pub fn family(&self) -> MessageFamilies {
        match self {
            PayloadKinds::CredOffer => MessageFamilies::CredentialExchange,
            PayloadKinds::CredReq => MessageFamilies::CredentialExchange,
            PayloadKinds::Cred => MessageFamilies::CredentialExchange,
            PayloadKinds::Proof => MessageFamilies::CredentialExchange,
            PayloadKinds::ProofRequest => MessageFamilies::CredentialExchange,
            PayloadKinds::Other(family) => MessageFamilies::Unknown(family.to_string()),
        }
    }

    pub fn name(&self) -> String {
        match self {
            PayloadKinds::CredOffer => "CRED_OFFER".to_string(),
            PayloadKinds::CredReq => "CRED_REQ".to_string(),
            PayloadKinds::Cred => "CRED".to_string(),
            PayloadKinds::Proof => "PROOF".to_string(),
            PayloadKinds::ProofRequest => "PROOF_REQUEST".to_string(),
            PayloadKinds::Other(kind) => kind.to_string(),
        }
    }
}

impl PayloadTypes {
    pub fn build(kind: PayloadKinds, fmt: &str) -> PayloadTypes {
        match settings::get_protocol_type() {
            settings::ProtocolTypes::V0 => {
                PayloadTypes::PayloadTypeV0(PayloadTypeV0 {
                    name: kind.name(),
                    ver: MESSAGE_VERSION.to_string(),
                    fmt: fmt.to_string(),
                })
            }
            settings::ProtocolTypes::V1 => {
                PayloadTypes::PayloadTypeV1(PayloadTypeV1 {
                    did: DID_GROUP_OWNER.to_string(),
                    family: kind.family(),
                    version: MESSAGE_VERSION.to_string(),
                    type_: kind.name(),
                })
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Payload {
    #[serde(rename = "@type")]
    pub type_: PayloadTypes,
    #[serde(rename = "@msg")]
    pub msg: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageStatusCode {
    Created,
    Sent,
    Pending,
    Accepted,
    Rejected,
    Reviewed,
}

impl Serialize for MessageStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            MessageStatusCode::Created => "MS-101",
            MessageStatusCode::Sent => "MS-102",
            MessageStatusCode::Pending => "MS-103",
            MessageStatusCode::Accepted => "MS-104",
            MessageStatusCode::Rejected => "MS-105",
            MessageStatusCode::Reviewed => "MS-106",
        };
        Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        match value.as_str() {
            Some("MS-101") => Ok(MessageStatusCode::Created),
            Some("MS-102") => Ok(MessageStatusCode::Sent),
            Some("MS-103") => Ok(MessageStatusCode::Pending),
            Some("MS-104") => Ok(MessageStatusCode::Accepted),
            Some("MS-105") => Ok(MessageStatusCode::Rejected),
            Some("MS-106") => Ok(MessageStatusCode::Reviewed),
            _ => Err(de::Error::custom("Unexpected message type."))
        }
    }
}

const MESSAGE_VERSION: &str = "1.0";
const DID_GROUP_OWNER: &str = "did:sov:123456789abcdefghi1234";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageTypes {
    MessageTypeV0(MessageTypeV0),
    MessageTypeV1(MessageTypeV1),
}

impl MessageTypes {
    fn build(kind: A2AMessageKinds) -> MessageTypes {
        match settings::get_protocol_type() {
            settings::ProtocolTypes::V0 => {
                MessageTypes::MessageTypeV0(MessageTypeV0 {
                    name: kind.name(),
                    ver: MESSAGE_VERSION.to_string(),
                })
            }
            settings::ProtocolTypes::V1 => {
                MessageTypes::MessageTypeV1(MessageTypeV1 {
                    did: DID_GROUP_OWNER.to_string(),
                    family: kind.family(),
                    version: MESSAGE_VERSION.to_string(),
                    type_: kind.name(),
                })
            }
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        match self {
            MessageTypes::MessageTypeV0(type_) => type_.name.as_str(),
            MessageTypes::MessageTypeV1(type_) => type_.type_.as_str(),
        }
    }

    pub fn version<'a>(&'a self) -> &'a str {
        match self {
            MessageTypes::MessageTypeV0(type_) => type_.ver.as_str(),
            MessageTypes::MessageTypeV1(type_) => type_.version.as_str(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct MessageTypeV0 {
    name: String,
    ver: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageTypeV1 {
    did: String,
    family: MessageFamilies,
    version: String,
    type_: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageFamilies {
    Routing,
    Onboarding,
    Pairwise,
    Configs,
    CredentialExchange,
    Unknown(String),
}

impl From<String> for MessageFamilies {
    fn from(family: String) -> Self {
        match family.as_str() {
            "routing" => MessageFamilies::Routing,
            "onboarding" => MessageFamilies::Onboarding,
            "pairwise" => MessageFamilies::Pairwise,
            "configs" => MessageFamilies::Configs,
            "credential_exchange" => MessageFamilies::CredentialExchange,
            family @ _ => {
                error!("Unknown Message family");
                MessageFamilies::Unknown(family.to_string())
            }
        }
    }
}

impl ::std::string::ToString for MessageFamilies {
    fn to_string(&self) -> String {
        match self {
            MessageFamilies::Routing => "routing".to_string(),
            MessageFamilies::Onboarding => "onboarding".to_string(),
            MessageFamilies::Pairwise => "pairwise".to_string(),
            MessageFamilies::CredentialExchange => "credential_exchange".to_string(),
            MessageFamilies::Configs => "configs".to_string(),
            MessageFamilies::Unknown(family) => family.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum A2AMessageKinds {
    Forward,
    Connect,
    Connected,
    SignUp,
    SignedUp,
    CreateAgent,
    AgentCreated,
    CreateKey,
    KeyCreated,
    CreateMessage,
    MessageDetail,
    MessageCreated,
    MessageSent,
    GetMessages,
    GetMessagesByConnections,
    Messages,
    UpdateMessageStatusByConnections,
    MessageStatusUpdatedByConnections,
    UpdateConnectionStatus,
    UpdateConfigs,
    ConfigsUpdated,
    UpdateConMethod,
    ConnectionRequest,
    ConnectionRequestAnswer,
    CredentialExchange,
}

impl A2AMessageKinds {
    pub fn family(&self) -> MessageFamilies {
        match self {
            A2AMessageKinds::Forward => MessageFamilies::Routing,
            A2AMessageKinds::Connect => MessageFamilies::Onboarding,
            A2AMessageKinds::Connected => MessageFamilies::Onboarding,
            A2AMessageKinds::CreateAgent => MessageFamilies::Onboarding,
            A2AMessageKinds::AgentCreated => MessageFamilies::Onboarding,
            A2AMessageKinds::SignUp => MessageFamilies::Onboarding,
            A2AMessageKinds::SignedUp => MessageFamilies::Onboarding,
            A2AMessageKinds::CreateKey => MessageFamilies::Pairwise,
            A2AMessageKinds::KeyCreated => MessageFamilies::Pairwise,
            A2AMessageKinds::CreateMessage => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageDetail => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageCreated => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageSent => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessages => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessagesByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::Messages => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConnectionStatus => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequest => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequestAnswer => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateMessageStatusByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageStatusUpdatedByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConfigs => MessageFamilies::Configs,
            A2AMessageKinds::ConfigsUpdated => MessageFamilies::Configs,
            A2AMessageKinds::UpdateConMethod => MessageFamilies::Configs,
            A2AMessageKinds::CredentialExchange => MessageFamilies::CredentialExchange,
        }
    }

    pub fn name(&self) -> String {
        match self {
            A2AMessageKinds::Forward => "FWD".to_string(),
            A2AMessageKinds::Connect => "CONNECT".to_string(),
            A2AMessageKinds::Connected => "CONNECTED".to_string(),
            A2AMessageKinds::CreateAgent => "CREATE_AGENT".to_string(),
            A2AMessageKinds::AgentCreated => "AGENT_CREATED".to_string(),
            A2AMessageKinds::SignUp => "SIGNUP".to_string(),
            A2AMessageKinds::SignedUp => "SIGNED_UP".to_string(),
            A2AMessageKinds::CreateKey => "CREATE_KEY".to_string(),
            A2AMessageKinds::KeyCreated => "KEY_CREATED".to_string(),
            A2AMessageKinds::CreateMessage => "CREATE_MSG".to_string(),
            A2AMessageKinds::MessageDetail => "MSG_DETAIL".to_string(),
            A2AMessageKinds::MessageCreated => "MSG_CREATED".to_string(),
            A2AMessageKinds::MessageSent => "MSGS_SENT".to_string(),
            A2AMessageKinds::GetMessages => "GET_MSGS".to_string(),
            A2AMessageKinds::GetMessagesByConnections => "GET_MSGS_BY_CONNS".to_string(),
            A2AMessageKinds::UpdateMessageStatusByConnections => "UPDATE_MSG_STATUS_BY_CONNS".to_string(),
            A2AMessageKinds::MessageStatusUpdatedByConnections => "MSG_STATUS_UPDATED_BY_CONNS".to_string(),
            A2AMessageKinds::Messages => "MSGS".to_string(),
            A2AMessageKinds::UpdateConnectionStatus => "UPDATE_CONN_STATUS".to_string(),
            A2AMessageKinds::ConnectionRequest => "CONNECTION_REQUEST".to_string(),
            A2AMessageKinds::ConnectionRequestAnswer => "CONNECTION_REQUEST_ANSWER".to_string(),
            A2AMessageKinds::UpdateConfigs => "UPDATE_CONFIGS".to_string(),
            A2AMessageKinds::ConfigsUpdated => "CONFIGS_UPDATED".to_string(),
            A2AMessageKinds::UpdateConMethod => "UPDATE_CONNECTION_METHOD".to_string(),
            A2AMessageKinds::CredentialExchange => "CREDENTIAL_EXCHANGE".to_string(),
        }
    }
}

fn parse_message_type(message_type: &str) -> Result<(String, String, String, String), u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<did>[\d\w:]*);
            (?P<spec>.*)/
            (?P<family>.*)/
            (?P<version>.*)/
            (?P<type>.*)").unwrap();
    }

    RE.captures(message_type)
        .and_then(|cap| {
            let did = cap.name("did").map(|s| s.as_str());
            let family = cap.name("family").map(|s| s.as_str());
            let version = cap.name("version").map(|s| s.as_str());
            let type_ = cap.name("type").map(|s| s.as_str());

            match (did, family, version, type_) {
                (Some(did), Some(family), Some(version), Some(type_)) =>
                    Some((did.to_string(), family.to_string(), version.to_string(), type_.to_string())),
                _ => None
            }
        }).ok_or(error::INVALID_OPTION.code_num) // TODO: Check Error
}

impl<'de> Deserialize<'de> for MessageTypeV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match value.as_str() {
            Some(type_) => {
                let (did, family, version, type_) = parse_message_type(type_)
                    .map_err(|err| de::Error::custom(format!("Can not parse message type_: {:?}", err)))?;

                match MessageFamilies::from(family) {
                    family @ _ =>
                        Ok(MessageTypeV1 {
                            did,
                            family,
                            version,
                            type_,
                        })
                }
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl Serialize for MessageTypeV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = Value::String(format!("{};spec/{}/{}/{}", self.did, self.family.to_string(), self.version, self.type_));
        value.serialize(serializer)
    }
}

pub fn prepare_message_for_agency(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V0 => bundle_for_agency_v0(message, &agency_did),
        settings::ProtocolTypes::V1 => pack_for_agency_v1(message, agency_did)
    }
}

fn bundle_for_agency_v0(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let message = rmp_serde::to_vec_named(&message).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let message = Bundled::create(message).encode()?;

    let message = crypto::prep_msg(&my_vk, &agent_vk, &message[..])?;

    prepare_forward_message(message, agency_did)
}

fn pack_for_agency_v1(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let message = serde_json::to_string(&message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = ::serde_json::to_string(&vec![&agent_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::pack_message(Some(&my_vk), &receiver_keys, message.as_bytes())?;

    prepare_forward_message(message, agency_did)
}

fn parse_response_from_agency(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V0 => parse_response_from_agency_v0(response),
        settings::ProtocolTypes::V1 => parse_response_from_agency_v1(response)
    }
}

fn parse_response_from_agency_v0(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    let verkey = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;
    let (_, data) = crypto::parse_msg(&verkey, &response)?;
    let bundle: Bundled<Vec<u8>> = bundle_from_u8(data)?;
    bundle.bundled
        .iter()
        .map(|msg| rmp_serde::from_slice(msg)
            .map_err(|err| error::INVALID_JSON.code_num))
        .collect::<Result<Vec<A2AMessage>, u32>>()
}

fn parse_response_from_agency_v1(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    let (_, message) = crypto::unpack_message(&response[..])?;
    let message: A2AMessage = serde_json::from_str(&message)
        .map_err(|ec| { error::INVALID_JSON.code_num })?;
    Ok(vec![message])
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Bundled<T> {
    bundled: Vec<T>,
}

impl<T> Bundled<T> {
    pub fn create(bundled: T) -> Bundled<T> {
        let mut vec = Vec::new();
        vec.push(bundled);
        Bundled {
            bundled: vec,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, u32> where T: serde::Serialize {
        rmp_serde::to_vec_named(self)
            .map_err(|err| {
                error!("Could not convert bundle to messagepack: {}", err);
                error::INVALID_MSGPACK.code_num
            })
    }
}

pub fn try_i8_bundle(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let bundle: Bundled<Vec<i8>> =
        rmp_serde::from_slice(&data[..])
            .map_err(|err| {
                warn!("could not deserialize bundle with i8, will try u8");
                error::INVALID_MSGPACK.code_num
            })?;

    let mut new_bundle: Bundled<Vec<u8>> = Bundled { bundled: Vec::new() };
    for i in bundle.bundled {
        let mut buf: Vec<u8> = Vec::new();
        for j in i { buf.push(j as u8); }
        new_bundle.bundled.push(buf);
    }
    Ok(new_bundle)
}

pub fn to_u8(bytes: &Vec<i8>) -> Vec<u8> {
    bytes.iter().map(|i| *i as u8).collect()
}

pub fn to_i8(bytes: &Vec<u8>) -> Vec<i8> {
    bytes.iter().map(|i| *i as i8).collect()
}

pub fn bundle_from_u8(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    try_i8_bundle(data.clone())
        .or_else(|_| rmp_serde::from_slice::<Bundled<Vec<u8>>>(&data[..]))
        .map_err(|err| {
            error!("could not deserialize bundle with i8 or u8: {}", err);
            error::INVALID_MSGPACK.code_num
        })
}

pub fn extract_json_payload(data: &Vec<u8>) -> Result<String, u32> {
    let my_payload: Payload = rmp_serde::from_slice(&data[..])
        .map_err(|x| {
            error!("could not deserialize bundle with i8 or u8: {}", x);
            error::INVALID_MSGPACK.code_num
        })?;

    Ok(my_payload.msg)
}

fn prepare_forward_message(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;

    let message = Forward::new(did.to_owned(), message);

    match settings::get_protocol_type() {
        settings::ProtocolTypes::V0 => {
            prepare_forward_message_for_agency_v0(&message, &agency_vk)
        }
        settings::ProtocolTypes::V1 => {
            prepare_forward_message_for_agency_v1(&message, &agency_vk)
        }
    }
}

fn prepare_forward_message_for_agency_v0(message: &Forward, agency_vk: &str) -> Result<Vec<u8>, u32> {
    let message = rmp_serde::to_vec_named(message).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let message = Bundled::create(message).encode()?;
    crypto::prep_anonymous_msg(agency_vk, &message[..])
}

fn prepare_forward_message_for_agency_v1(message: &Forward, agency_vk: &str) -> Result<Vec<u8>, u32> {
    let message = serde_json::to_string(message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = serde_json::to_string(&vec![agency_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    crypto::pack_message(None, &receiver_keys, message.as_bytes())
}

pub fn prepare_message_for_agent(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V0 => {
            prepare_message_for_agent_v0(messages, pw_vk, agent_did, agent_vk)
        }
        settings::ProtocolTypes::V1 => {
            prepare_message_for_agent_v1(messages, pw_vk, agent_did, agent_vk)
        }
    }
}

fn prepare_message_for_agent_v0(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    let message = messages
        .iter()
        .map(|msg| rmp_serde::to_vec_named(msg))
        .collect::<Result<Vec<_>, _>>()
        .map(|msgs| Bundled { bundled: msgs })
        .and_then(|bundle| rmp_serde::to_vec_named(&bundle))
        .or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::prep_msg(&pw_vk, agent_vk, &message[..])?;

    /* forward to did */
    let message = Forward::new(agent_did.to_owned(), message);

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    bundle_for_agency_v0(&A2AMessage::Forward(message), &to_did)
}

fn prepare_message_for_agent_v1(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    let message = messages.get(0).ok_or(error::SERIALIZATION_ERROR.code_num)?;
    let message = serde_json::to_string(message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = serde_json::to_string(&vec![&agent_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::pack_message(Some(pw_vk), &receiver_keys, message.as_bytes())?;

    /* forward to did */
    let message = Forward::new(agent_did.to_owned(), message);

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    pack_for_agency_v1(&A2AMessage::Forward(message), &to_did)
}

pub trait GeneralMessage {
    type Msg;

    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(to_did)?;
        self.set_to_did(to_did.to_string());
        Ok(self)
    }

    fn to_vk(&mut self, to_vk: &str) -> Result<&mut Self, u32> {
        validation::validate_verkey(to_vk)?;
        self.set_to_vk(to_vk.to_string());
        Ok(self)
    }

    fn agent_did(&mut self, did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(did)?;
        self.set_agent_did(did.to_string());
        Ok(self)
    }

    fn agent_vk(&mut self, to_vk: &str) -> Result<&mut Self, u32> {
        validation::validate_verkey(to_vk)?;
        self.set_agent_vk(to_vk.to_string());
        Ok(self)
    }

    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_agent_did(&mut self, did: String);
    fn set_agent_vk(&mut self, vk: String);

    fn prepare(&mut self) -> Result<Vec<u8>, u32>;
}

pub fn create_keys() -> CreateKeyBuilder { CreateKeyBuilder::create() }

pub fn send_invite() -> SendInviteBuilder { SendInviteBuilder::create() }

pub fn delete_connection() -> DeleteConnectionBuilder { DeleteConnectionBuilder::create() }

pub fn accept_invite() -> AcceptInviteBuilder { AcceptInviteBuilder::create() }

pub fn update_data() -> UpdateProfileDataBuilder { UpdateProfileDataBuilder::create() }

pub fn get_messages() -> GetMessagesBuilder { GetMessagesBuilder::create() }

pub fn send_message() -> SendMessageBuilder { SendMessageBuilder::create() }

pub fn proof_request() -> ProofRequestMessage { ProofRequestMessage::create() }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -111, -36, 5, -74];

        let buf = to_u8(&vec);
        println!("new bundle: {:?}", buf);
    }

    #[test]
    fn test_to_i8() {
        let vec: Vec<u8> = vec![129, 167, 98, 117, 110, 100, 108, 101, 100, 145, 220, 19, 13];
        let buf = to_i8(&vec);
        println!("new bundle: {:?}", buf);
    }
}
