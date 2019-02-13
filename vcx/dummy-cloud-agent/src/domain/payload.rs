use domain::message_type::{MessageTypeV2, MessageFamilies, MESSAGE_VERSION_V1, MESSAGE_VERSION_V2, DID};
use domain::a2a::RemoteMessageType;
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
    pub fn build(kind: PayloadKinds) -> PayloadTypes {
        match ProtocolType::get() {
            ProtocolTypes::V1 => {
                PayloadTypes::PayloadTypeV1(PayloadTypeV1 {
                    name: kind.name().to_string(),
                    ver: MESSAGE_VERSION_V1.to_string(),
                    fmt: "json".to_string(),
                })
            }
            ProtocolTypes::V2 => {
                PayloadTypes::PayloadTypeV2(PayloadTypeV2 {
                    did: DID.to_string(),
                    family: kind.family(),
                    version: MESSAGE_VERSION_V2.to_string(),
                    type_: kind.name().to_string(),
                })
            }
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
