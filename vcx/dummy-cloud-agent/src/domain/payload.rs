use domain::message_type::{MessageTypeV2, MessageFamilies, MESSAGE_VERSION, DID};
use domain::a2a::ExchangeMessageType;
use domain::protocol_type::{ProtocolType, ProtocolTypes};

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    #[serde(rename = "@type")]
    pub type_: PayloadTypes,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
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
    ConnReq,
    ConnReqAnswer,
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
            PayloadKinds::ConnReq => MessageFamilies::Pairwise,
            PayloadKinds::ConnReqAnswer => MessageFamilies::Pairwise,
            PayloadKinds::CredOffer => MessageFamilies::CredentialExchange,
            PayloadKinds::CredReq => MessageFamilies::CredentialExchange,
            PayloadKinds::Cred => MessageFamilies::CredentialExchange,
            PayloadKinds::Proof => MessageFamilies::CredentialExchange,
            PayloadKinds::ProofRequest => MessageFamilies::CredentialExchange,
            PayloadKinds::Other(family) => MessageFamilies::Other(family.to_string()),
        }
    }

    pub fn name(&self) -> String {
        match self {
            PayloadKinds::ConnReq => "CONN_REQ".to_string(),
            PayloadKinds::ConnReqAnswer => "CONN_REQ_ANSWER".to_string(),
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
    pub fn build(kind: PayloadKinds) -> PayloadTypes {
        match ProtocolType::get() {
            ProtocolTypes::V1 => {
                PayloadTypes::PayloadTypeV1(PayloadTypeV1 {
                    name: kind.name(),
                    ver: MESSAGE_VERSION.to_string(),
                    fmt: "json".to_string(),
                })
            }
            ProtocolTypes::V2 => {
                PayloadTypes::PayloadTypeV2(PayloadTypeV2 {
                    did: DID.to_string(),
                    family: kind.family(),
                    version: MESSAGE_VERSION.to_string(),
                    type_: kind.name(),
                })
            }
        }
    }
}

impl From<ExchangeMessageType> for PayloadKinds {
    fn from(type_: ExchangeMessageType) -> PayloadKinds {
        match type_ {
            ExchangeMessageType::ConnReq => PayloadKinds::ConnReq,
            ExchangeMessageType::ConnReqAnswer => PayloadKinds::ConnReqAnswer,
            ExchangeMessageType::CredOffer => PayloadKinds::CredOffer,
            ExchangeMessageType::CredReq => PayloadKinds::CredReq,
            ExchangeMessageType::Cred => PayloadKinds::Cred,
            ExchangeMessageType::ProofReq => PayloadKinds::ProofRequest,
            ExchangeMessageType::Proof => PayloadKinds::Proof,
            ExchangeMessageType::Other(other) => PayloadKinds::Other(other.to_string()),
        }
    }
}
