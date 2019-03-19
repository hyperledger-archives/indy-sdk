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
use domain::message_type::*;
use domain::protocol_type::{ProtocolType, ProtocolTypes};
use domain::payload::Thread;

#[derive(Debug)]
pub enum A2AMessageV1 {
    /// base
    Forward(ForwardV1),

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
    MessageSent(MessageSent),

    UpdateConnectionStatus(UpdateConnectionStatus),
    ConnectionStatusUpdated(ConnectionStatusUpdated),
    SendMessages(SendMessages),
    UpdateMessageStatus(UpdateMessageStatus),
    MessageStatusUpdated(MessageStatusUpdated),
    GetMessages(GetMessages),
    Messages(Messages),
    GetMessagesByConnections(GetMessagesByConnections),
    MessagesByConnections(MessagesByConnections),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    MessageStatusUpdatedByConnections(MessageStatusUpdatedByConnections),

    /// PW Configs
    UpdateConfigs(UpdateConfigs),
    ConfigsUpdated(ConfigsUpdated),
    GetConfigs(GetConfigs),
    Configs(Configs),
    RemoveConfigs(RemoveConfigs),
    ConfigsRemoved(ConfigsRemoved),
    UpdateComMethod(UpdateComMethod),
    ComMethodUpdated(ComMethodUpdated),
}

#[derive(Debug)]
pub enum A2AMessageV2 {
    /// base
    Forward(ForwardV2),

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
    UpdateConnectionStatus(UpdateConnectionStatus),
    ConnectionStatusUpdated(ConnectionStatusUpdated),
    SendMessages(SendMessages),
    MessageSent(MessageSent),
    UpdateMessageStatus(UpdateMessageStatus),
    MessageStatusUpdated(MessageStatusUpdated),
    GetMessages(GetMessages),
    Messages(Messages),
    GetMessagesByConnections(GetMessagesByConnections),
    MessagesByConnections(MessagesByConnections),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    MessageStatusUpdatedByConnections(MessageStatusUpdatedByConnections),

    /// PW Configs
    UpdateConfigs(UpdateConfigs),
    ConfigsUpdated(ConfigsUpdated),
    GetConfigs(GetConfigs),
    Configs(Configs),
    RemoveConfigs(RemoveConfigs),
    ConfigsRemoved(ConfigsRemoved),
    UpdateComMethod(UpdateComMethod),
    ComMethodUpdated(ComMethodUpdated),

    ConnectionRequest(ConnectionRequest),
    ConnectionRequestResponse(ConnectionRequestResponse),
    ConnectionRequestAnswer(ConnectionRequestAnswer),
    ConnectionRequestAnswerResponse(ConnectionRequestAnswerResponse),
    SendRemoteMessage(SendRemoteMessage),
    SendRemoteMessageResponse(SendRemoteMessageResponse),
}

#[derive(Debug)]
pub enum A2AMessage {
    Version1(A2AMessageV1),
    Version2(A2AMessageV2),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForwardV1 {
    #[serde(rename = "@fwd")]
    pub fwd: String,
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ForwardV2 {
    #[serde(rename = "@fwd")]
    pub fwd: String,
    #[serde(rename = "@msg")]
    pub msg: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_did_verkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connected {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUp {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedUp {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgent {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentCreated {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

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
pub struct CreateMessage {
    pub mtype: RemoteMessageType,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum MessageDetailPayload {
    V1(Vec<i8>),
    V2(serde_json::Value),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GetMessagesDetailResponse {
    pub uid: String,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    #[serde(rename = "senderDID")]
    pub sender_did: String,
    #[serde(rename = "type")]
    pub type_: RemoteMessageType,
    pub payload: Option<MessageDetailPayload>,
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
pub enum RemoteMessageType {
    ConnReq,
    ConnReqAnswer,
    CredOffer,
    CredReq,
    Cred,
    ProofReq,
    Proof,
    Other(String),
}

impl Serialize for RemoteMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            RemoteMessageType::ConnReq => "connReq",
            RemoteMessageType::ConnReqAnswer => "connReqAnswer",
            RemoteMessageType::CredOffer => "credOffer",
            RemoteMessageType::CredReq => "credReq",
            RemoteMessageType::Cred => "cred",
            RemoteMessageType::ProofReq => "proofReq",
            RemoteMessageType::Proof => "proof",
            RemoteMessageType::Other(other) => other.as_str(),
        };
        serde_json::Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RemoteMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        match value.as_str() {
            Some("connReq") => Ok(RemoteMessageType::ConnReq),
            Some("connReqAnswer") => Ok(RemoteMessageType::ConnReqAnswer),
            Some("credOffer") => Ok(RemoteMessageType::CredOffer),
            Some("credReq") => Ok(RemoteMessageType::CredReq),
            Some("cred") => Ok(RemoteMessageType::Cred),
            Some("proofReq") => Ok(RemoteMessageType::ProofReq),
            Some("proof") => Ok(RemoteMessageType::Proof),
            Some(mtype) => Ok(RemoteMessageType::Other(mtype.to_string())),
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
    pub thread_id: Option<String>,
}

impl From<ConnectionRequest> for ConnectionRequestMessageDetail {
    fn from(con_req: ConnectionRequest) -> Self {
        ConnectionRequestMessageDetail {
            key_dlg_proof: con_req.key_dlg_proof,
            target_name: con_req.target_name,
            phone_no: con_req.phone_no,
            use_public_did: Some(con_req.include_public_did),
            thread_id: Some(con_req.id),
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>
}

impl From<ConnectionRequestAnswer> for ConnectionRequestAnswerMessageDetail {
    fn from(con_req_answer: ConnectionRequestAnswer) -> Self {
        ConnectionRequestAnswerMessageDetail {
            key_dlg_proof: con_req_answer.key_dlg_proof,
            sender_detail: con_req_answer.sender_detail,
            sender_agency_detail: con_req_answer.sender_agency_detail,
            answer_status_code: con_req_answer.answer_status_code,
            thread: Some(con_req_answer.thread),
        }
    }
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ComMethod {
    id: String,
    #[serde(rename = "type")]
    e_type: i32,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateComMethod {
    #[serde(rename = "comMethod")]
    com_method: ComMethod,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComMethodUpdated {
    pub id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequest {
    #[serde(rename = "sendMsg")]
    pub send_msg: bool,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "replyToMsgId")]
    pub reply_to_msg_id: Option<String>,
    #[serde(rename = "keyDlgProof")]
    pub key_dlg_proof: KeyDlgProof,
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,
    #[serde(rename = "phoneNo")]
    pub phone_no: Option<String>,
    #[serde(rename = "usePublicDID")]
    pub include_public_did: bool,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestResponse {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "inviteDetail")]
    pub invite_detail: InviteDetail,
    #[serde(rename = "urlToInviteDetail")]
    pub url_to_invite_detail: String,
    pub sent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestAnswer {
    #[serde(rename = "sendMsg")]
    pub send_msg: bool,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "replyToMsgId")]
    pub reply_to_msg_id: Option<String>,
    #[serde(rename = "keyDlgProof")]
    pub key_dlg_proof: Option<KeyDlgProof>,
    #[serde(rename = "senderDetail")]
    pub sender_detail: SenderDetail,
    #[serde(rename = "senderAgencyDetail")]
    pub sender_agency_detail: ForwardAgentDetail,
    #[serde(rename = "answerStatusCode")]
    pub answer_status_code: MessageStatusCode,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionRequestAnswerResponse {
    #[serde(rename = "@id")]
    pub id: String,
    pub sent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRemoteMessage {
    pub mtype: RemoteMessageType,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "replyToMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_msg_id: Option<String>,
    #[serde(rename = "sendMsg")]
    pub send_msg: bool,
    #[serde(rename = "@msg")]
    pub msg: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRemoteMessageResponse {
    #[serde(rename = "@id")]
    pub id: String,
    pub sent: bool,
}

pub enum A2AMessageKinds {
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
    SendMessages,
    GetMessages,
    GetMessagesByConnections,
    MessagesByConnections,
    Messages,
    UpdateMessageStatus,
    MessageStatusUpdated,
    UpdateMessageStatusByConnections,
    MessageStatusUpdatedByConnections,
    UpdateConnectionStatus,
    ConnectionStatusUpdated,
    UpdateConfigs,
    ConfigsUpdated,
    GetConfigs,
    Configs,
    RemoveConfigs,
    ConfigsRemoved,
    ConnectionRequest,
    ConnectionRequestResponse,
    ConnectionRequestAnswer,
    ConnectionRequestAnswerResponse,
    SendRemoteMessage,
    SendRemoteMessageResponse,
    UpdateComMethod,
    ComMethodUpdated,
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
            A2AMessageKinds::SendMessages => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessages => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessagesByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::MessagesByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::Messages => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConnectionStatus => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionStatusUpdated => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequest => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequestResponse => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequestAnswer => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequestAnswerResponse => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateMessageStatus => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageStatusUpdated => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateMessageStatusByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageStatusUpdatedByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConfigs => MessageFamilies::Configs,
            A2AMessageKinds::ConfigsUpdated => MessageFamilies::Configs,
            A2AMessageKinds::GetConfigs => MessageFamilies::Configs,
            A2AMessageKinds::Configs => MessageFamilies::Configs,
            A2AMessageKinds::RemoveConfigs => MessageFamilies::Configs,
            A2AMessageKinds::ConfigsRemoved => MessageFamilies::Configs,
            A2AMessageKinds::UpdateComMethod => MessageFamilies::Configs,
            A2AMessageKinds::ComMethodUpdated => MessageFamilies::Configs,
            A2AMessageKinds::SendRemoteMessage => MessageFamilies::Routing,
            A2AMessageKinds::SendRemoteMessageResponse => MessageFamilies::Routing,
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
            A2AMessageKinds::SendMessages => "SEND_MSGS".to_string(),
            A2AMessageKinds::GetMessages => "GET_MSGS".to_string(),
            A2AMessageKinds::GetMessagesByConnections => "GET_MSGS_BY_CONNS".to_string(),
            A2AMessageKinds::MessagesByConnections => "MSGS_BY_CONNS".to_string(),
            A2AMessageKinds::UpdateMessageStatus => "UPDATE_MSG_STATUS".to_string(),
            A2AMessageKinds::MessageStatusUpdated => "MSG_STATUS_UPDATED".to_string(),
            A2AMessageKinds::UpdateMessageStatusByConnections => "UPDATE_MSG_STATUS_BY_CONNS".to_string(),
            A2AMessageKinds::MessageStatusUpdatedByConnections => "MSG_STATUS_UPDATED_BY_CONNS".to_string(),
            A2AMessageKinds::Messages => "MSGS".to_string(),
            A2AMessageKinds::UpdateConnectionStatus => "UPDATE_CONN_STATUS".to_string(),
            A2AMessageKinds::ConnectionStatusUpdated => "CONN_STATUS_UPDATED".to_string(),
            A2AMessageKinds::ConnectionRequest => "CONN_REQUEST".to_string(),
            A2AMessageKinds::ConnectionRequestResponse => "CONN_REQUEST_RESP".to_string(),
            A2AMessageKinds::ConnectionRequestAnswer => "CONN_REQUEST_ANSWER".to_string(),
            A2AMessageKinds::ConnectionRequestAnswerResponse => "CONN_REQUEST_ANSWER_RESP".to_string(),
            A2AMessageKinds::UpdateConfigs => "UPDATE_CONFIGS".to_string(),
            A2AMessageKinds::ConfigsUpdated => "CONFIGS_UPDATED".to_string(),
            A2AMessageKinds::GetConfigs => "GET_CONFIGS".to_string(),
            A2AMessageKinds::Configs => "CONFIGS".to_string(),
            A2AMessageKinds::RemoveConfigs => "REMOVE_CONFIGS".to_string(),
            A2AMessageKinds::ConfigsRemoved => "CONFIGS_REMOVED".to_string(),
            A2AMessageKinds::UpdateComMethod => "UPDATE_COM_METHOD".to_string(),
            A2AMessageKinds::ComMethodUpdated => "COM_METHOD_UPDATED".to_string(),
            A2AMessageKinds::SendRemoteMessage => "SEND_REMOTE_MSG".to_string(),
            A2AMessageKinds::SendRemoteMessageResponse => "REMOTE_MSG_SENT".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for A2AMessageV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypeV1 = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type.name.as_str() {
            "FWD" => {
                ForwardV1::deserialize(value)
                    .map(|msg| A2AMessageV1::Forward(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECT" => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessageV1::Connect(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECTED" => {
                Connected::deserialize(value)
                    .map(|msg| A2AMessageV1::Connected(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNUP" => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessageV1::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNED_UP" => {
                SignedUp::deserialize(value)
                    .map(|msg| A2AMessageV1::SignedUp(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_AGENT" => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            "AGENT_CREATED" => {
                AgentCreated::deserialize(value)
                    .map(|msg| A2AMessageV1::AgentCreated(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_KEY" => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            "KEY_CREATED" => {
                KeyCreated::deserialize(value)
                    .map(|msg| A2AMessageV1::KeyCreated(msg))
                    .map_err(de::Error::custom)
            }
            "SEND_MSGS" => {
                SendMessages::deserialize(value)
                    .map(|msg| A2AMessageV1::SendMessages(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONN_STATUS" => {
                UpdateConnectionStatus::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConnectionStatus(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_STATUS_UPDATED" => {
                ConnectionStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessageV1::ConnectionStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS" => {
                UpdateMessageStatus::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateMessageStatus(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED" => {
                MessageStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS" => {
                Messages::deserialize(value)
                    .map(|msg| A2AMessageV1::Messages(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS_BY_CONNS" => {
                GetMessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_BY_CONNS" => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::MessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS_BY_CONNS" => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED_BY_CONNS" => {
                MessageStatusUpdatedByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageStatusUpdatedByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONFIGS" => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_UPDATED" => {
                ConfigsUpdated::deserialize(value)
                    .map(|msg| A2AMessageV1::ConfigsUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "GET_CONFIGS" => {
                GetConfigs::deserialize(value)
                    .map(|msg| A2AMessageV1::GetConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS" => {
                Configs::deserialize(value)
                    .map(|msg| A2AMessageV1::Configs(msg))
                    .map_err(de::Error::custom)
            }
            "REMOVE_CONFIGS" => {
                RemoveConfigs::deserialize(value)
                    .map(|msg| A2AMessageV1::RemoveConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_REMOVED" => {
                ConfigsRemoved::deserialize(value)
                    .map(|msg| A2AMessageV1::ConfigsRemoved(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_COM_METHOD" => {
                UpdateComMethod::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateComMethod(msg))
                    .map_err(de::Error::custom)
            }
            "COM_METHOD_UPDATED" => {
                ComMethodUpdated::deserialize(value)
                    .map(|msg| A2AMessageV1::ComMethodUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_MSG" => {
                CreateMessage::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateMessage(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_DETAIL" => {
                MessageDetail::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageDetail(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_CREATED" => {
                MessageCreated::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageCreated(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_SENT" => {
                MessageSent::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageSent(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl<'de> Deserialize<'de> for A2AMessageV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypeV2 = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type.type_.as_str() {
            "FWD" => {
                ForwardV2::deserialize(value)
                    .map(|msg| A2AMessageV2::Forward(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECT" => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessageV2::Connect(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECTED" => {
                Connected::deserialize(value)
                    .map(|msg| A2AMessageV2::Connected(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNUP" => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessageV2::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNED_UP" => {
                SignedUp::deserialize(value)
                    .map(|msg| A2AMessageV2::SignedUp(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_AGENT" => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            "AGENT_CREATED" => {
                AgentCreated::deserialize(value)
                    .map(|msg| A2AMessageV2::AgentCreated(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_KEY" => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            "KEY_CREATED" => {
                KeyCreated::deserialize(value)
                    .map(|msg| A2AMessageV2::KeyCreated(msg))
                    .map_err(de::Error::custom)
            }
            "SEND_MSGS" => {
                SendMessages::deserialize(value)
                    .map(|msg| A2AMessageV2::SendMessages(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_SENT" => {
                MessageSent::deserialize(value)
                    .map(|msg| A2AMessageV2::MessageSent(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONN_STATUS" => {
                UpdateConnectionStatus::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConnectionStatus(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_STATUS_UPDATED" => {
                ConnectionStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS" => {
                UpdateMessageStatus::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateMessageStatus(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED" => {
                MessageStatusUpdated::deserialize(value)
                    .map(|msg| A2AMessageV2::MessageStatusUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_COM_METHOD" => {
                UpdateComMethod::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateComMethod(msg))
                    .map_err(de::Error::custom)
            }
            "COM_METHOD_UPDATED" => {
                ComMethodUpdated::deserialize(value)
                    .map(|msg| A2AMessageV2::ComMethodUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS" => {
                Messages::deserialize(value)
                    .map(|msg| A2AMessageV2::Messages(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS_BY_CONNS" => {
                GetMessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_BY_CONNS" => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::MessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS_BY_CONNS" => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED_BY_CONNS" => {
                MessageStatusUpdatedByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::MessageStatusUpdatedByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONFIGS" => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_UPDATED" => {
                ConfigsUpdated::deserialize(value)
                    .map(|msg| A2AMessageV2::ConfigsUpdated(msg))
                    .map_err(de::Error::custom)
            }
            "GET_CONFIGS" => {
                GetConfigs::deserialize(value)
                    .map(|msg| A2AMessageV2::GetConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS" => {
                Configs::deserialize(value)
                    .map(|msg| A2AMessageV2::Configs(msg))
                    .map_err(de::Error::custom)
            }
            "REMOVE_CONFIGS" => {
                RemoveConfigs::deserialize(value)
                    .map(|msg| A2AMessageV2::RemoveConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_REMOVED" => {
                ConfigsRemoved::deserialize(value)
                    .map(|msg| A2AMessageV2::ConfigsRemoved(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST" => {
                ConnectionRequest::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_RESP" => {
                ConnectionRequestResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_ANSWER" => {
                ConnectionRequestAnswer::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestAnswer(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_ANSWER_RESP" => {
                ConnectionRequestAnswerResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestAnswerResponse(msg))
                    .map_err(de::Error::custom)
            }
            "SEND_REMOTE_MSG" => {
                SendRemoteMessage::deserialize(value)
                    .map(|msg| A2AMessageV2::SendRemoteMessage(msg))
                    .map_err(de::Error::custom)
            }
            "REMOTE_MSG_SENT" => {
                SendRemoteMessageResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::SendRemoteMessageResponse(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypes = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type {
            MessageTypes::MessageTypeV1(_) =>
                A2AMessageV1::deserialize(value)
                    .map(|msg| A2AMessage::Version1(msg))
                    .map_err(de::Error::custom),
            MessageTypes::MessageTypeV2(_) =>
                A2AMessageV2::deserialize(value)
                    .map(|msg| A2AMessage::Version2(msg))
                    .map_err(de::Error::custom),
        }
    }
}

fn set_a2a_message_type_v1<T>(msg: T, kind: A2AMessageKinds) -> Result<serde_json::Value, serde_json::Error> where T: Serialize {
    set_a2a_message_type(msg, MessageTypes::build_v1(kind))
}

fn set_a2a_message_type_v2<T>(msg: T, kind: A2AMessageKinds) -> Result<serde_json::Value, serde_json::Error> where T: Serialize {
    set_a2a_message_type(msg, MessageTypes::build_v2(kind))
}

fn set_a2a_message_type<T>(msg: T, message_type: MessageTypes) -> Result<serde_json::Value, serde_json::Error> where T: Serialize {
    let mut value = serde_json::to_value(msg)?;
    let type_ = serde_json::to_value(message_type)?;
    value.as_object_mut().unwrap().insert("@type".into(), type_);
    Ok(value)
}

impl Serialize for A2AMessageV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessageV1::Forward(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::Forward),
            A2AMessageV1::Connect(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::Connect),
            A2AMessageV1::Connected(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::Connected),
            A2AMessageV1::SignUp(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::SignUp),
            A2AMessageV1::SignedUp(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::SignedUp),
            A2AMessageV1::CreateAgent(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::CreateAgent),
            A2AMessageV1::AgentCreated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::AgentCreated),
            A2AMessageV1::CreateKey(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::CreateKey),
            A2AMessageV1::KeyCreated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::KeyCreated),
            A2AMessageV1::SendMessages(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::SendMessages),
            A2AMessageV1::UpdateConnectionStatus(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::UpdateConnectionStatus),
            A2AMessageV1::ConnectionStatusUpdated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::ConnectionStatusUpdated),
            A2AMessageV1::UpdateMessageStatus(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::UpdateMessageStatus),
            A2AMessageV1::MessageStatusUpdated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessageStatusUpdated),
            A2AMessageV1::GetMessages(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::GetMessages),
            A2AMessageV1::Messages(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::Messages),
            A2AMessageV1::GetMessagesByConnections(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::GetMessagesByConnections),
            A2AMessageV1::MessagesByConnections(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessagesByConnections),
            A2AMessageV1::UpdateMessageStatusByConnections(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::UpdateMessageStatusByConnections),
            A2AMessageV1::MessageStatusUpdatedByConnections(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessageStatusUpdatedByConnections),
            A2AMessageV1::UpdateConfigs(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::UpdateConfigs),
            A2AMessageV1::ConfigsUpdated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::ConfigsUpdated),
            A2AMessageV1::UpdateComMethod(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::UpdateComMethod),
            A2AMessageV1::ComMethodUpdated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::ComMethodUpdated),
            A2AMessageV1::GetConfigs(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::GetConfigs),
            A2AMessageV1::Configs(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::Configs),
            A2AMessageV1::RemoveConfigs(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::RemoveConfigs),
            A2AMessageV1::ConfigsRemoved(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::ConfigsRemoved),
            A2AMessageV1::CreateMessage(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::CreateMessage),
            A2AMessageV1::MessageDetail(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessageDetail),
            A2AMessageV1::MessageCreated(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessageCreated),
            A2AMessageV1::MessageSent(msg) => set_a2a_message_type_v1(msg, A2AMessageKinds::MessageSent),
        }.map_err(ser::Error::custom)?;

        value.serialize(serializer)
    }
}

impl Serialize for A2AMessageV2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessageV2::Forward(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::Forward),
            A2AMessageV2::Connect(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::Connect),
            A2AMessageV2::Connected(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::Connected),
            A2AMessageV2::SignUp(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::SignUp),
            A2AMessageV2::SignedUp(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::SignedUp),
            A2AMessageV2::CreateAgent(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::CreateAgent),
            A2AMessageV2::AgentCreated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::AgentCreated),
            A2AMessageV2::CreateKey(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::CreateKey),
            A2AMessageV2::KeyCreated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::KeyCreated),
            A2AMessageV2::SendMessages(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::SendMessages),
            A2AMessageV2::MessageSent(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::MessageSent),
            A2AMessageV2::UpdateConnectionStatus(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::UpdateConnectionStatus),
            A2AMessageV2::ConnectionStatusUpdated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConnectionStatusUpdated),
            A2AMessageV2::UpdateMessageStatus(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::UpdateMessageStatus),
            A2AMessageV2::MessageStatusUpdated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::MessageStatusUpdated),
            A2AMessageV2::GetMessages(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::GetMessages),
            A2AMessageV2::Messages(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::Messages),
            A2AMessageV2::GetMessagesByConnections(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::GetMessagesByConnections),
            A2AMessageV2::MessagesByConnections(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::MessagesByConnections),
            A2AMessageV2::UpdateMessageStatusByConnections(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::UpdateMessageStatusByConnections),
            A2AMessageV2::MessageStatusUpdatedByConnections(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::MessageStatusUpdatedByConnections),
            A2AMessageV2::UpdateConfigs(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::UpdateConfigs),
            A2AMessageV2::ConfigsUpdated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConfigsUpdated),
            A2AMessageV2::UpdateComMethod(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::UpdateComMethod),
            A2AMessageV2::ComMethodUpdated(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ComMethodUpdated),
            A2AMessageV2::GetConfigs(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::GetConfigs),
            A2AMessageV2::Configs(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::Configs),
            A2AMessageV2::RemoveConfigs(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::RemoveConfigs),
            A2AMessageV2::ConfigsRemoved(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConfigsRemoved),
            A2AMessageV2::ConnectionRequest(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConnectionRequest),
            A2AMessageV2::ConnectionRequestResponse(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConnectionRequestResponse),
            A2AMessageV2::ConnectionRequestAnswer(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConnectionRequestAnswer),
            A2AMessageV2::ConnectionRequestAnswerResponse(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::ConnectionRequestAnswerResponse),
            A2AMessageV2::SendRemoteMessage(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::SendRemoteMessage),
            A2AMessageV2::SendRemoteMessageResponse(msg) => set_a2a_message_type_v2(msg, A2AMessageKinds::SendRemoteMessageResponse),
        }.map_err(ser::Error::custom)?;

        value.serialize(serializer)
    }
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            A2AMessage::Version1(msg) => msg.serialize(serializer).map_err(ser::Error::custom),
            A2AMessage::Version2(msg) => msg.serialize(serializer).map_err(ser::Error::custom)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct A2AMsgsBundle {
    pub bundled: Vec<Vec<u8>>,
}

impl A2AMessage {
    fn bundle_plain(msgs: &[A2AMessage]) -> Result<Vec<u8>, Error> {
        msgs
            .iter()
            .map(|msg| rmp_serde::to_vec_named(msg))
            .collect::<Result<Vec<_>, _>>()
            .map(|msgs| A2AMsgsBundle { bundled: msgs })
            .and_then(|bundle| rmp_serde::to_vec_named(&bundle))
            .map_err(|err| err.context("Can't bundle messages").into())
    }

    pub fn prepare_authcrypted(wallet_handle: i32,
                               sender_vk: &str,
                               recipient_vk: &str,
                               msgs: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::bundle_authcrypted(wallet_handle, sender_vk, recipient_vk, msgs),
            ProtocolTypes::V2 => A2AMessage::pack(wallet_handle, Some(sender_vk), recipient_vk, msgs)
        }
    }


    pub fn prepare_anoncrypted(wallet_handle: i32, recipient_vk: &str, message: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::bundle_anoncrypted(recipient_vk, message),
            ProtocolTypes::V2 => A2AMessage::pack(wallet_handle, None, recipient_vk, message)
        }
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

    pub fn pack(wallet_handle: i32, sender_vk: Option<&str>, recipient_vk: &str, msgs: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        if msgs.len() != 1 {
            return err!(err_msg("Invalid messages count."));
        }

        let message = ftry!(serde_json::to_string(&msgs[0]));
        let receiver_keys = ftry!(serde_json::to_string(&vec![&recipient_vk]));

        crypto::pack_message(wallet_handle, sender_vk, &receiver_keys, message.as_bytes())
            .from_err()
            .into_box()
    }

    pub fn pack_v2(wallet_handle: i32, sender_vk: Option<&str>, recipient_vk: &str, msg: &A2AMessageV2) -> BoxedFuture<Vec<u8>, Error> {
        let message = ftry!(serde_json::to_string(msg));
        let receiver_keys = ftry!(serde_json::to_string(&vec![&recipient_vk]));

        crypto::pack_message(wallet_handle, sender_vk, &receiver_keys, message.as_bytes())
            .from_err()
            .into_box()
    }

    fn unbundle(bundle: &[u8]) -> Result<Vec<A2AMessage>, Error> {
        rmp_serde::from_slice::<A2AMsgsBundle>(bundle)
            .and_then(|bundle| {
                bundle.bundled
                    .iter()
                    .map(|msg| rmp_serde::from_slice::<A2AMessage>(msg))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|err| err.context("Can't unbundle messages").into())
    }

    pub fn parse_anoncrypted(wallet_handle: i32,
                             recipient_vk: &str,
                             bundle: &[u8]) -> BoxedFuture<Vec<A2AMessage>, Error> {
        match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::unbundle_anoncrypted(wallet_handle, recipient_vk, bundle),
            ProtocolTypes::V2 =>
                A2AMessage::unpack(wallet_handle, bundle)
                    .map(|(_, message)| message)
                    .into_box()
        }
    }

    pub fn parse_authcrypted(wallet_handle: i32,
                             recipient_vk: &str,
                             message: &[u8]) -> BoxedFuture<(String, Vec<A2AMessage>), Error> {
        match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::unbundle_authcrypted(wallet_handle, recipient_vk, message),
            ProtocolTypes::V2 =>
                A2AMessage::unpack(wallet_handle, message)
                    .map(|(sender_vk, message)| (sender_vk.unwrap(), message))
                    .into_box()
        }
    }

    pub fn unbundle_anoncrypted(wallet_handle: i32,
                                recipient_vk: &str,
                                message: &[u8]) -> BoxedFuture<Vec<A2AMessage>, Error> {
        crypto::anon_decrypt(wallet_handle, recipient_vk, message)
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

    pub fn unpack(wallet_handle: i32, message: &[u8]) -> BoxedFuture<(Option<String>, Vec<A2AMessage>), Error> {
        crypto::unpack_message(wallet_handle, message)
            .from_err()
            .and_then(|message| {
                let message: UnpackMessage = ftry!(serde_json::from_slice(message.as_slice()));

                serde_json::from_str::<A2AMessage>(&message.message)
                    .map(|msg| (message.sender_verkey, vec![msg]))
                    .map_err(|err| err.context("Can't unpack message").into())
                    .into_future()
                    .into_box()
            })
            .into_box()
    }
}

#[derive(Deserialize)]
struct UnpackMessage {
    message: String,
    sender_verkey: Option<String>
}
