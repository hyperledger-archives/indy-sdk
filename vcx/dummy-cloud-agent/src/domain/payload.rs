use domain::message_type::{MessageTypeV2, MessageFamilies, MESSAGE_VERSION_V1, DID};
use domain::a2a::RemoteMessageType;
use domain::protocol_type::{ProtocolType, ProtocolTypes};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Payloads {
    PayloadV1(PayloadV1),
    PayloadV2(PayloadV2),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadV1 {
    #[serde(rename = "@type")]
    pub type_: PayloadTypeV1,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadV2 {
    #[serde(rename = "@type")]
    pub type_: PayloadTypeV2,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@msg")]
    pub msg: String,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PayloadTypes {
    PayloadTypeV1(PayloadTypeV1),
    PayloadTypeV2(PayloadTypeV2),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayloadTypeV1 {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

pub type PayloadTypeV2 = MessageTypeV2;

#[derive(Debug, Deserialize, Serialize)]
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
            PayloadKinds::Other(family) => MessageFamilies::Other(family.to_string()),
        }
    }

    fn name<'a>(&'a self) -> &'a str {
        match ProtocolType::get() {
            ProtocolTypes::V1 => {
                match self {
                    PayloadKinds::CredOffer => "CRED_OFFER",
                    PayloadKinds::CredReq => "CRED_REQ",
                    PayloadKinds::Cred => "CRED",
                    PayloadKinds::ProofRequest => "PROOF_REQUEST",
                    PayloadKinds::Proof => "PROOF",
                    PayloadKinds::Other(kind) => kind,
                }
            }
            ProtocolTypes::V2 => {
                match self {
                    PayloadKinds::CredOffer => "credential-offer",
                    PayloadKinds::CredReq => "credential-request",
                    PayloadKinds::Cred => "credential",
                    PayloadKinds::ProofRequest => "presentation-request",
                    PayloadKinds::Proof => "presentation",
                    PayloadKinds::Other(kind) => kind,
                }
            }
        }
    }
}

impl PayloadTypes {
    pub fn build_v1(kind: PayloadKinds, fmt: &str) -> PayloadTypeV1 {
        PayloadTypeV1 {
            name: kind.name().to_string(),
            ver: MESSAGE_VERSION_V1.to_string(),
            fmt: fmt.to_string(),
        }
    }

    pub fn build_v2(kind: PayloadKinds) -> PayloadTypeV2 {
        PayloadTypeV2 {
            did: DID.to_string(),
            family: kind.family(),
            version: kind.family().version().to_string(),
            type_: kind.name().to_string(),
        }
    }
}

impl From<RemoteMessageType> for PayloadKinds {
    fn from(type_: RemoteMessageType) -> PayloadKinds {
        match type_ {
            RemoteMessageType::ConnReq => PayloadKinds::Other("connReq".to_string()),
            RemoteMessageType::ConnReqAnswer => PayloadKinds::Other("ConnReqAnswer".to_string()),
            RemoteMessageType::CredOffer => PayloadKinds::CredOffer,
            RemoteMessageType::CredReq => PayloadKinds::CredReq,
            RemoteMessageType::Cred => PayloadKinds::Cred,
            RemoteMessageType::ProofReq => PayloadKinds::ProofRequest,
            RemoteMessageType::Proof => PayloadKinds::Proof,
            RemoteMessageType::Other(other) => PayloadKinds::Other(other.to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Thread {
    pub thid: Option<String>,
    pub pthid: Option<String>,
    pub sender_order: u32,
    pub received_orders: HashMap<String, u32>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            thid: None,
            pthid: None,
            sender_order: 0,
            received_orders: HashMap::new(),
        }
    }
}