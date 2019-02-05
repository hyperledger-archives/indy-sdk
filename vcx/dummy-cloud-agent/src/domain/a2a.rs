use failure::*;
use futures::*;
use indy::crypto;
use rmp_serde;
use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::{self, Value};
use utils::futures::*;

use domain::a2connection::*;
use domain::invite::{InviteDetail, SenderDetail, ForwardAgentDetail};
use domain::key_deligation_proof::KeyDlgProof;
use domain::status::{MessageStatusCode, ConnectionStatus};

// TODO: For simplification we avoid complex versioning logic
// TODO: There should be additional enum level for versions
#[derive(Debug)]
pub enum A2AMessage {
    /// base
    Forward(Forward),
    /// onboarding
    Connect(Connect),
    Connected(Connected),
    SignUp(SignUp),
    SignedUp(SignedUp),
    CreateAgent(CreateAgent),
    AgentCreated(AgentCreated),
    /// PW Connection
    CreateKey(CreateKey),
    KeyCreated(KeyCreated),
    CreateMessage(CreateMessage),
    MessageDetail(MessageDetail),
    MessageCreated(MessageCreated),
    SendMessages(SendMessages),
    MessageSent(MessageSent),
    UpdateConnectionStatus(UpdateConnectionStatus),
    ConnectionStatusUpdated(ConnectionStatusUpdated),
    UpdateMessageStatus(UpdateMessageStatus),
    MessageStatusUpdated(MessageStatusUpdated),
    GetMessages(GetMessages),
    Messages(Messages),
    GetMessagesByConnections(GetMessagesByConnections),
    MessagesByConnections(MessagesByConnections),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    MessageStatusUpdatedByConnections(MessageStatusUpdatedByConnections),
    UpdateConfigs(UpdateConfigs),
    ConfigsUpdated(ConfigsUpdated),
    GetConfigs(GetConfigs),
    Configs(Configs),
    RemoveConfigs(RemoveConfigs),
    ConfigsRemoved(ConfigsRemoved),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentCreated {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forward {
    #[serde(rename = "@fwd")]
    pub fwd: String,
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connect {
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connected {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAgent {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateKey {
    #[serde(rename = "forDID")]
    pub for_did: String,
    #[serde(rename = "forDIDVerKey")]
    pub for_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyCreated {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUp {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedUp {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMessage {
    pub mtype: MessageType,
    #[serde(rename = "sendMsg")]
    pub send_msg: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "replyToMsgId")]
    pub reply_to_msg_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageCreated {
    pub uid: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendMessages {
    pub uids: Vec<String>,
}

#[serde(untagged)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageDetail {
    ConnectionRequestAnswer(ConnectionRequestAnswerMessageDetail),
    ConnectionRequest(ConnectionRequestMessageDetail),
    ConnectionRequestResp(ConnectionRequestMessageDetailResp),
    General(GeneralMessageDetail),
    Send(SendMessageDetail),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageSent {
    pub uids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateConnectionStatus {
    #[serde(rename = "statusCode")]
    pub status_code: ConnectionStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionStatusUpdated {
    #[serde(rename = "statusCode")]
    pub status_code: ConnectionStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMessageStatus {
    pub uids: Vec<String>,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageStatusUpdated {
    pub uids: Vec<String>,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetMessages {
    #[serde(rename = "excludePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_payload: Option<String>,
    #[serde(default)]
    pub uids: Vec<String>,
    #[serde(rename = "statusCodes")]
    #[serde(default)]
    pub status_codes: Vec<MessageStatusCode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMessagesByConnections {
    #[serde(rename = "excludePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_payload: Option<String>,
    #[serde(default)]
    pub uids: Vec<String>,
    #[serde(rename = "statusCodes")]
    #[serde(default)]
    pub status_codes: Vec<MessageStatusCode>,
    #[serde(rename = "pairwiseDIDs")]
    #[serde(default)]
    pub pairwise_dids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Messages {
    pub msgs: Vec<GetMessagesDetailResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagesByConnections {
    #[serde(rename = "msgsByConns")]
    #[serde(default)]
    pub msgs: Vec<MessagesByConnection>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct GetMessagesDetailResponse {
    pub uid: String,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    #[serde(rename = "senderDID")]
    pub sender_did: String,
    #[serde(rename = "type")]
    pub type_: MessageType,
    pub payload: Option<Vec<i8>>,
    #[serde(rename = "refMsgId")]
    pub ref_msg_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMessageStatusByConnections {
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    #[serde(rename = "uidsByConns")]
    pub uids_by_conn: Vec<UidByConnection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageStatusUpdatedByConnections {
    #[serde(rename = "updatedUidsByConns")]
    pub updated_uids_by_conn: Vec<UidByConnection>,
    pub failed: Vec<FailedMessageUpdateInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailedMessageUpdateInfo {
    #[serde(rename = "pairwiseDID")]
    pub pairwise_did: String,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    #[serde(rename = "statusMsg")]
    pub status_msg: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    ConnReq,
    ConnReqAnswer,
    CredOffer,
    CredReq,
    Cred,
    ProofReq,
    Proof,
    Other(String),
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::ConnReq => "connReq",
            MessageType::ConnReqAnswer => "connReqAnswer",
            MessageType::CredOffer => "credOffer",
            MessageType::CredReq => "credReq",
            MessageType::Cred => "cred",
            MessageType::ProofReq => "proofReq",
            MessageType::Proof => "proof",
            MessageType::Other(other) => other.as_str(),
        }.to_string()
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            MessageType::ConnReq => "connReq",
            MessageType::ConnReqAnswer => "connReqAnswer",
            MessageType::CredOffer => "credOffer",
            MessageType::CredReq => "credReq",
            MessageType::Cred => "cred",
            MessageType::ProofReq => "proofReq",
            MessageType::Proof => "proof",
            MessageType::Other(other) => other.as_str(),
        };
        serde_json::Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match value.as_str() {
            Some("connReq") => Ok(MessageType::ConnReq),
            Some("connReqAnswer") => Ok(MessageType::ConnReqAnswer),
            Some("credOffer") => Ok(MessageType::CredOffer),
            Some("credReq") => Ok(MessageType::CredReq),
            Some("cred") => Ok(MessageType::Cred),
            Some("proofReq") => Ok(MessageType::ProofReq),
            Some("proof") => Ok(MessageType::Proof),
            Some(mtype) => Ok(MessageType::Other(mtype.to_string())),
            _ => Err(de::Error::custom("Unexpected message type."))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestMessageDetail {
    #[serde(rename = "keyDlgProof")]
    pub key_dlg_proof: KeyDlgProof,
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,
    #[serde(rename = "phoneNo")]
    pub phone_no: Option<String>,
    #[serde(rename = "usePublicDID")]
    pub use_public_did: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestMessageDetailResp {
    #[serde(rename = "inviteDetail")]
    pub invite_detail: InviteDetail,
    #[serde(rename = "urlToInviteDetail")]
    pub url_to_invite_detail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestAnswerMessageDetail {
    #[serde(rename = "keyDlgProof")]
    pub key_dlg_proof: Option<KeyDlgProof>,
    #[serde(rename = "senderDetail")]
    pub sender_detail: SenderDetail,
    #[serde(rename = "senderAgencyDetail")]
    pub sender_agency_detail: ForwardAgentDetail,
    #[serde(rename = "answerStatusCode")]
    pub answer_status_code: MessageStatusCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralMessageDetail {
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
    pub title: Option<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendMessageDetail {
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
    pub title: Option<String>,
    pub detail: Option<String>,
    #[serde(rename = "answerStatusCode")]
    pub answer_status_code: MessageStatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PayloadMessageType {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

impl PayloadMessageType {
    pub fn new(type_: &MessageType) -> PayloadMessageType {
        PayloadMessageType {
            name: type_.to_string(),
            ver: "1.0".to_string(),
            fmt: "indy.msgpack".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadMessage {
    #[serde(rename = "@type")]
    pub type_: PayloadMessageType,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConfigOption {
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateConfigs {
    pub configs: Vec<ConfigOption>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigsUpdated {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetConfigs {
    pub configs: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs {
    pub configs: Vec<ConfigOption>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveConfigs {
    pub configs: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigsRemoved {}

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match (value["@type"]["name"].as_str(), value["@type"]["ver"].as_str()) {
            (Some("AGENT_CREATED"), Some("1.0")) => {
                AgentCreated::deserialize(value)
                    .map(|msg| A2AMessage::AgentCreated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONNECT"), Some("1.0")) => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessage::Connect(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONNECTED"), Some("1.0")) => {
                Connected::deserialize(value)
                    .map(|msg| A2AMessage::Connected(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CREATE_AGENT"), Some("1.0")) => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessage::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CREATE_KEY"), Some("1.0")) => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessage::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            (Some("FWD"), Some("1.0")) => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessage::Forward(msg))
                    .map_err(de::Error::custom)
            }
            (Some("KEY_CREATED"), Some("1.0")) => {
                KeyCreated::deserialize(value)
                    .map(|msg| A2AMessage::KeyCreated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("SIGNUP"), Some("1.0")) => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessage::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            (Some("SIGNED_UP"), Some("1.0")) => {
                SignedUp::deserialize(value)
                    .map(|msg| A2AMessage::SignedUp(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CREATE_MSG"), Some("1.0")) => {
                CreateMessage::deserialize(value)
                    .map(|msg| A2AMessage::CreateMessage(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSG_DETAIL"), Some("1.0")) => {
                MessageDetail::deserialize(value)
                    .map(|msg| A2AMessage::MessageDetail(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSG_CREATED"), Some("1.0")) => {
                MessageCreated::deserialize(value)
                    .map(|msg| A2AMessage::MessageCreated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSGS_SENT"), Some("1.0")) => {
                MessageSent::deserialize(value)
                    .map(|msg| A2AMessage::MessageSent(msg))
                    .map_err(de::Error::custom)
            }
            (Some("SEND_MSGS"), Some("1.0")) => {
                SendMessages::deserialize(value)
                    .map(|msg| A2AMessage::SendMessages(msg))
                    .map_err(de::Error::custom)
            }
            (Some("UPDATE_CONN_STATUS"), Some("1.0")) => {
                UpdateConnectionStatus::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConnectionStatus(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONN_STATUS_UPDATED"), Some("1.0")) => {
                ConnectionStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("UPDATE_MSG_STATUS"), Some("1.0")) => {
                UpdateMessageStatus::deserialize(value)
                    .map(|msg| A2AMessage::UpdateMessageStatus(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSG_STATUS_UPDATED"), Some("1.0")) => {
                MessageStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessage::MessageStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("GET_MSGS"), Some("1.0")) => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessage::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSGS"), Some("1.0")) => {
                Messages::deserialize(value)
                    .map(|msg| A2AMessage::Messages(msg))
                    .map_err(de::Error::custom)
            }
            (Some("GET_MSGS_BY_CONNS"), Some("1.0")) => {
                GetMessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessage::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSGS_BY_CONNS"), Some("1.0")) => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessage::MessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            (Some("UPDATE_MSG_STATUS_BY_CONNS"), Some("1.0")) => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessage::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            (Some("MSG_STATUS_UPDATED_BY_CONNS"), Some("1.0")) => {
                MessageStatusUpdatedByConnections::deserialize(value)
                    .map(|msg| A2AMessage::MessageStatusUpdatedByConnections(msg))
                    .map_err(de::Error::custom)
            }
            (Some("UPDATE_CONFIGS"), Some("1.0")) => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessage::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONFIGS_UPDATED"), Some("1.0")) => {
                ConfigsUpdated::deserialize(value)
                    .map(|msg| A2AMessage::ConfigsUpdated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("GET_CONFIGS"), Some("1.0")) => {
                GetConfigs::deserialize(value)
                    .map(|msg| A2AMessage::GetConfigs(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONFIGS"), Some("1.0")) => {
                Configs::deserialize(value)
                    .map(|msg| A2AMessage::Configs(msg))
                    .map_err(de::Error::custom)
            }
            (Some("REMOVE_CONFIGS"), Some("1.0")) => {
                RemoveConfigs::deserialize(value)
                    .map(|msg| A2AMessage::RemoveConfigs(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONFIGS_REMOVED"), Some("1.0")) => {
                ConfigsRemoved::deserialize(value)
                    .map(|msg| A2AMessage::ConfigsRemoved(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessage::AgentCreated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "AGENT_CREATED", "ver": "1.0"}));
                value
            }
            A2AMessage::Connect(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECT", "ver": "1.0"}));
                value
            }
            A2AMessage::Connected(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECTED", "ver": "1.0"}));
                value
            }
            A2AMessage::Forward(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "FWD", "ver": "1.0"}));
                value
            }
            A2AMessage::CreateAgent(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CREATE_AGENT", "ver": "1.0"}));
                value
            }
            A2AMessage::CreateKey(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CREATE_KEY", "ver": "1.0"}));
                value
            }
            A2AMessage::KeyCreated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "KEY_CREATED", "ver": "1.0"}));
                value
            }
            A2AMessage::SignUp(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "SIGNUP", "ver": "1.0"}));
                value
            }
            A2AMessage::SignedUp(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "SIGNED_UP", "ver": "1.0"}));
                value
            }
            A2AMessage::CreateMessage(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CREATE_MSG", "ver": "1.0"}));
                value
            }
            A2AMessage::MessageDetail(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSG_DETAIL", "ver": "1.0"}));
                value
            }
            A2AMessage::MessageCreated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSG_CREATED", "ver": "1.0"}));
                value
            }
            A2AMessage::MessageSent(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSGS_SENT", "ver": "1.0"}));
                value
            }
            A2AMessage::SendMessages(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "SEND_MSGS", "ver": "1.0"}));
                value
            }
            A2AMessage::UpdateConnectionStatus(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "UPDATE_CONN_STATUS", "ver": "1.0"}));
                value
            }
            A2AMessage::ConnectionStatusUpdated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONN_STATUS_UPDATED", "ver": "1.0"}));
                value
            }
            A2AMessage::UpdateMessageStatus(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "UPDATE_MSG_STATUS", "ver": "1.0"}));
                value
            }
            A2AMessage::MessageStatusUpdated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSG_STATUS_UPDATED", "ver": "1.0"}));
                value
            }
            A2AMessage::GetMessages(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "GET_MSGS", "ver": "1.0"}));
                value
            }
            A2AMessage::Messages(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSGS", "ver": "1.0"}));
                value
            }
            A2AMessage::GetMessagesByConnections(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "GET_MSGS_BY_CONNS", "ver": "1.0"}));
                value
            }
            A2AMessage::MessagesByConnections(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSGS_BY_CONNS", "ver": "1.0"}));
                value
            }
            A2AMessage::UpdateMessageStatusByConnections(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "UPDATE_MSG_STATUS_BY_CONNS", "ver": "1.0"}));
                value
            }
            A2AMessage::MessageStatusUpdatedByConnections(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "MSG_STATUS_UPDATED_BY_CONNS", "ver": "1.0"}));
                value
            }
            A2AMessage::UpdateConfigs(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "UPDATE_CONFIGS", "ver": "1.0"}));
                value
            }
            A2AMessage::ConfigsUpdated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONFIGS_UPDATED", "ver": "1.0"}));
                value
            }
            A2AMessage::GetConfigs(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "GET_CONFIGS", "ver": "1.0"}));
                value
            }
            A2AMessage::Configs(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONFIGS", "ver": "1.0"}));
                value
            }
            A2AMessage::RemoveConfigs(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "REMOVE_CONFIGS", "ver": "1.0"}));
                value
            }
            A2AMessage::ConfigsRemoved(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONFIGS_REMOVED", "ver": "1.0"}));
                value
            }
        };

        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct A2AMsgsBundle {
    pub bundled: Vec<Vec<u8>>,
}

impl A2AMessage {
    pub fn bundle_plain(msgs: &[A2AMessage]) -> Result<Vec<u8>, Error> {
        msgs
            .iter()
            .map(|msg| rmp_serde::to_vec_named(msg))
            .collect::<Result<Vec<_>, _>>()
            .map(|msgs| A2AMsgsBundle { bundled: msgs })
            .and_then(|bundle| rmp_serde::to_vec_named(&bundle))
            .map_err(|err| err.context("Can't bundle messages").into())
    }

    pub fn bundle_authcrypted(wallet_handle: i32,
                              sender_vk: &str,
                              recipient_vk: &str,
                              msgs: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        let bundle = ftry!(Self::bundle_plain(msgs));

        crypto::auth_crypt(wallet_handle, sender_vk, recipient_vk, &bundle)
            .from_err()
            .into_box()
    }

    pub fn bundle_anoncrypted(recipient_vk: &str,
                              msgs: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        let bundle = ftry!(Self::bundle_plain(msgs));

        crypto::anon_crypt(recipient_vk, &bundle)
            .from_err()
            .into_box()
    }

    pub fn unbundle(bundle: &[u8]) -> Result<Vec<A2AMessage>, Error> {
        rmp_serde::from_slice::<A2AMsgsBundle>(bundle)
            .and_then(|bundle| {
                bundle.bundled
                    .iter()
                    .map(|msg| rmp_serde::from_slice::<A2AMessage>(msg))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|err| err.context("Can't unbundle messages").into())
    }

    pub fn unbundle_anoncrypted(wallet_handle: i32,
                                recipient_vk: &str,
                                bundle: &[u8]) -> BoxedFuture<Vec<A2AMessage>, Error> {
        crypto::anon_decrypt(wallet_handle, recipient_vk, bundle)
            .from_err()
            .and_then(|bundle| {
                Self::unbundle(&bundle)
            })
            .into_box()
    }

    pub fn unbundle_authcrypted(wallet_handle: i32,
                                recipient_vk: &str,
                                bundle: &[u8]) -> BoxedFuture<(String, Vec<A2AMessage>), Error> {
        crypto::auth_decrypt(wallet_handle, recipient_vk, bundle)
            .from_err()
            .and_then(|(sender_vk, bundle)| {
                Self::unbundle(&bundle).map(|msgs| (sender_vk, msgs))
            })
            .into_box()
    }
}

