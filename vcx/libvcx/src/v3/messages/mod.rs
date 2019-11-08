use std::u8;
use messages::message_type::parse_message_type;

use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::Value;

pub mod ack;
pub mod connection;
pub mod error;
pub mod forward;
pub mod attachment;
pub mod mime_type;
pub mod status;

#[allow(unused)] //FIXME:
pub mod issuance;

pub mod proof_presentation;

use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::SignedResponse;
use v3::messages::connection::problem_report::ProblemReport as ConnectionProblemReport;
use v3::messages::connection::ping::Ping;
use v3::messages::forward::Forward;
use v3::messages::error::ProblemReport as CommonProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use self::ack::Ack;

use utils::uuid;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;

use v3::messages::proof_presentation::presentation_proposal::PresentationProposal;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use v3::messages::proof_presentation::presentation::Presentation;

#[derive(Debug, PartialEq)]
pub enum A2AMessage {
    /// routing
    Forward(Forward),

    /// DID Exchange
    ConnectionInvitation(Invitation),
    ConnectionRequest(Request),
    ConnectionResponse(SignedResponse),
    ConnectionProblemReport(ConnectionProblemReport),
    Ping(Ping),

    /// notification
    Ack(Ack),
    CommonProblemReport(CommonProblemReport),

    /// credential issuance
    CredentialProposal(CredentialProposal),
    CredentialOffer(CredentialOffer),
    CredentialRequest(CredentialRequest),
    Credential(Credential),

    /// proof presentation
    PresentationProposal(PresentationProposal),
    PresentationRequest(PresentationRequest),
    Presentation(Presentation),

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
            "invitation" => {
                Invitation::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionInvitation(msg))
                    .map_err(de::Error::custom)
            }
            "request" => {
                Request::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            "response" => {
                SignedResponse::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            "ping" => {
                Ping::deserialize(value)
                    .map(|msg| A2AMessage::Ping(msg))
                    .map_err(de::Error::custom)
            }
            "problem_report" => {
                ConnectionProblemReport::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionProblemReport(msg))
                    .map_err(de::Error::custom)
            }
            "ack" => {
                Ack::deserialize(value)
                    .map(|msg| A2AMessage::Ack(msg))
                    .map_err(de::Error::custom)
            }
            "problem-report" => {
                CommonProblemReport::deserialize(value)
                    .map(|msg| A2AMessage::CommonProblemReport(msg))
                    .map_err(de::Error::custom)
            }
            "issue-credential" => {
                Credential::deserialize(value)
                    .map(|msg| A2AMessage::Credential(msg))
                    .map_err(de::Error::custom)
            }
            "propose-credential" => {
                CredentialProposal::deserialize(value)
                    .map(|msg| A2AMessage::CredentialProposal(msg))
                    .map_err(de::Error::custom)
            }
            "offer-credential" => {
                CredentialOffer::deserialize(value)
                    .map(|msg| A2AMessage::CredentialOffer(msg))
                    .map_err(de::Error::custom)
            }
            "request-credential" => {
                CredentialRequest::deserialize(value)
                    .map(|msg| A2AMessage::CredentialRequest(msg))
                    .map_err(de::Error::custom)
            }
            "propose-presentation" => {
                PresentationProposal::deserialize(value)
                    .map(|msg| A2AMessage::PresentationProposal(msg))
                    .map_err(de::Error::custom)
            }
            "request-presentation" => {
                PresentationRequest::deserialize(value)
                    .map(|msg| A2AMessage::PresentationRequest(msg))
                    .map_err(de::Error::custom)
            }
            "presentation" => {
                Presentation::deserialize(value)
                    .map(|msg| A2AMessage::Presentation(msg))
                    .map_err(de::Error::custom)
            }
            other_type => {
                warn!("Unexpected @type field structure: {}", other_type);
                Ok(A2AMessage::Generic(value.to_string()))
            }
        }
    }
}

fn set_a2a_message_type<T>(msg: T, kind: A2AMessageKinds) -> Result<serde_json::Value, serde_json::Error> where T: Serialize {
    let mut value = ::serde_json::to_value(msg)?;
    let type_ = ::serde_json::to_value(MessageType::build(kind))?;
    value.as_object_mut().unwrap().insert("@type".into(), type_);
    Ok(value)
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessage::Forward(msg) => set_a2a_message_type(msg, A2AMessageKinds::Forward),
            A2AMessage::ConnectionInvitation(msg) => set_a2a_message_type(msg, A2AMessageKinds::ExchangeInvitation),
            A2AMessage::ConnectionRequest(msg) => set_a2a_message_type(msg, A2AMessageKinds::ExchangeRequest),
            A2AMessage::ConnectionResponse(msg) => set_a2a_message_type(msg, A2AMessageKinds::ExchangeResponse),
            A2AMessage::ConnectionProblemReport(msg) => set_a2a_message_type(msg, A2AMessageKinds::ExchangeProblemReport),
            A2AMessage::Ping(msg) => set_a2a_message_type(msg, A2AMessageKinds::Ping),
            A2AMessage::Ack(msg) => set_a2a_message_type(msg, A2AMessageKinds::Ack),
            A2AMessage::CommonProblemReport(msg) => set_a2a_message_type(msg, A2AMessageKinds::ProblemReport),
            A2AMessage::CredentialProposal(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialProposal),
            A2AMessage::CredentialOffer(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialOffer),
            A2AMessage::CredentialRequest(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialRequest),
            A2AMessage::Credential(msg) => set_a2a_message_type(msg, A2AMessageKinds::Credential),
            A2AMessage::PresentationProposal(msg) => set_a2a_message_type(msg, A2AMessageKinds::PresentationProposal),
            A2AMessage::PresentationRequest(msg) => set_a2a_message_type(msg, A2AMessageKinds::PresentationRequest),
            A2AMessage::Presentation(msg) => set_a2a_message_type(msg, A2AMessageKinds::Presentation),
            A2AMessage::Generic(msg) => {
                ::serde_json::to_value(msg)
            }
        }.map_err(ser::Error::custom)?;

        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum A2AMessageKinds {
    Forward,
    ExchangeInvitation,
    ExchangeRequest,
    ExchangeResponse,
    ExchangeProblemReport,
    Ed25519Signature,
    Ping,
    Ack,
    CredentialOffer,
    CredentialProposal,
    CredentialRequest,
    Credential,
    CredentialPreview,
    ProblemReport,
    PresentationProposal,
    PresentationPreview,
    PresentationRequest,
    Presentation,
}

impl A2AMessageKinds {
    pub fn family(&self) -> MessageFamilies {
        match self {
            A2AMessageKinds::Forward => MessageFamilies::Routing,
            A2AMessageKinds::ExchangeInvitation => MessageFamilies::DidExchange,
            A2AMessageKinds::ExchangeRequest => MessageFamilies::DidExchange,
            A2AMessageKinds::ExchangeResponse => MessageFamilies::DidExchange,
            A2AMessageKinds::ExchangeProblemReport => MessageFamilies::DidExchange,
            A2AMessageKinds::Ping => MessageFamilies::Notification,
            A2AMessageKinds::Ack => MessageFamilies::Notification,
            A2AMessageKinds::ProblemReport => MessageFamilies::ReportProblem,
            A2AMessageKinds::Ed25519Signature => MessageFamilies::Signature,
            A2AMessageKinds::CredentialOffer => MessageFamilies::CredentialIssuance,
            A2AMessageKinds::Credential => MessageFamilies::CredentialIssuance,
            A2AMessageKinds::CredentialProposal => MessageFamilies::CredentialIssuance,
            A2AMessageKinds::CredentialRequest => MessageFamilies::CredentialIssuance,
            A2AMessageKinds::CredentialPreview => MessageFamilies::CredentialIssuance,
            A2AMessageKinds::PresentationProposal => MessageFamilies::PresentProof,
            A2AMessageKinds::PresentationPreview => MessageFamilies::PresentProof,
            A2AMessageKinds::PresentationRequest => MessageFamilies::PresentProof,
            A2AMessageKinds::Presentation => MessageFamilies::PresentProof,
        }
    }

    pub fn name(&self) -> String {
        match self {
            A2AMessageKinds::Forward => "forward".to_string(),
            A2AMessageKinds::ExchangeInvitation => "invitation".to_string(),
            A2AMessageKinds::ExchangeRequest => "request".to_string(),
            A2AMessageKinds::ExchangeResponse => "response".to_string(),
            A2AMessageKinds::ExchangeProblemReport => "problem_report".to_string(),
            A2AMessageKinds::Ping => "ping".to_string(),
            A2AMessageKinds::Ack => "ack".to_string(),
            A2AMessageKinds::ProblemReport => "problem-report".to_string(),
            A2AMessageKinds::Ed25519Signature => "ed25519Sha512_single".to_string(),
            A2AMessageKinds::Credential => "issue-credential".to_string(),
            A2AMessageKinds::CredentialProposal => "propose-credential".to_string(),
            A2AMessageKinds::CredentialPreview => "credential-preview".to_string(),
            A2AMessageKinds::CredentialOffer => "offer-credential".to_string(),
            A2AMessageKinds::CredentialRequest => "request-credential".to_string(),
            A2AMessageKinds::PresentationProposal => "propose-presentation".to_string(),
            A2AMessageKinds::PresentationPreview => "presentation-preview".to_string(),
            A2AMessageKinds::PresentationRequest => "request-presentation".to_string(),
            A2AMessageKinds::Presentation => "presentation".to_string(),

        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageFamilies {
    Routing,
    DidExchange,
    Notification,
    Signature,
    CredentialIssuance,
    ReportProblem,
    PresentProof,
    TrustPing,
    Unknown(String)
}

impl MessageFamilies {
    pub fn version(&self) -> &'static str {
        match self {
            MessageFamilies::Routing => "1.0",
            MessageFamilies::DidExchange => "1.0",
            MessageFamilies::Notification => "1.0",
            MessageFamilies::Signature => "1.0",
            MessageFamilies::CredentialIssuance => "1.0",
            MessageFamilies::ReportProblem => "1.0",
            MessageFamilies::PresentProof => "1.0",
            MessageFamilies::TrustPing => "1.0",
            MessageFamilies::Unknown(_) => "1.0"
        }
    }
}

impl From<String> for MessageFamilies {
    fn from(family: String) -> Self {
        match family.as_str() {
            "routing" => MessageFamilies::Routing,
            "connections" => MessageFamilies::DidExchange, // TODO: should be didexchange
            "signature" => MessageFamilies::Signature,
            "notification" => MessageFamilies::Notification,
            "issue-credential" => MessageFamilies::CredentialIssuance,
            "report-problem" => MessageFamilies::ReportProblem,
            "present-proof" => MessageFamilies::PresentProof,
            "trust_ping" => MessageFamilies::TrustPing,
            family @ _ => MessageFamilies::Unknown(family.to_string())
        }
    }
}

impl ::std::string::ToString for MessageFamilies {
    fn to_string(&self) -> String {
        match self {
            MessageFamilies::Routing => "routing".to_string(),
            MessageFamilies::DidExchange => "connections".to_string(), // TODO: should be didexchange
            MessageFamilies::Notification => "notification".to_string(),
            MessageFamilies::Signature => "signature".to_string(),
            MessageFamilies::CredentialIssuance => "issue-credential".to_string(),
            MessageFamilies::ReportProblem => "report-problem".to_string(),
            MessageFamilies::PresentProof => "present-proof".to_string(),
            MessageFamilies::TrustPing => "trust_ping".to_string(),
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
    const DID: &'static str = "did:sov:BzCbsNYhMrjHiqZDTUASHg";

    pub fn build(kind: A2AMessageKinds) -> MessageType {
        MessageType {
            did: Self::DID.to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageId(pub String);

impl MessageId {
    pub fn new() -> MessageId {
        MessageId(uuid::uuid())
    }
}