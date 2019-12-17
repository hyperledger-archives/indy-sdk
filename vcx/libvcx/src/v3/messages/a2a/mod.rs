pub mod message_family;
pub mod message_type;

use self::message_type::MessageType;
use self::message_family::MessageFamilies;

use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::Value;

use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::SignedResponse;
use v3::messages::connection::problem_report::ProblemReport as ConnectionProblemReport;
use v3::messages::trust_ping::ping::Ping;
use v3::messages::trust_ping::ping_response::PingResponse;
use v3::messages::forward::Forward;
use v3::messages::error::ProblemReport as CommonProblemReport;
use v3::messages::issuance::credential_proposal::CredentialProposal;
use v3::messages::ack::Ack;

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

    /// trust ping
    Ping(Ping),
    PingResponse(PingResponse),

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

        let message_type: MessageType = match serde_json::from_value(value["@type"].clone()) {
            Ok(message_type) => message_type,
            Err(_) => return Ok(A2AMessage::Generic(value.to_string()))
        };

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
            "ping_response" => {
                PingResponse::deserialize(value)
                    .map(|msg| A2AMessage::PingResponse(msg))
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
            A2AMessage::PingResponse(msg) => set_a2a_message_type(msg, A2AMessageKinds::PingResponse),
            A2AMessage::Ack(msg) => set_a2a_message_type(msg, A2AMessageKinds::Ack),
            A2AMessage::CommonProblemReport(msg) => set_a2a_message_type(msg, A2AMessageKinds::ProblemReport),
            A2AMessage::CredentialProposal(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialProposal),
            A2AMessage::CredentialOffer(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialOffer),
            A2AMessage::CredentialRequest(msg) => set_a2a_message_type(msg, A2AMessageKinds::CredentialRequest),
            A2AMessage::Credential(msg) => set_a2a_message_type(msg, A2AMessageKinds::Credential),
            A2AMessage::PresentationProposal(msg) => set_a2a_message_type(msg, A2AMessageKinds::PresentationProposal),
            A2AMessage::PresentationRequest(msg) => set_a2a_message_type(msg, A2AMessageKinds::PresentationRequest),
            A2AMessage::Presentation(msg) => set_a2a_message_type(msg, A2AMessageKinds::Presentation),
            A2AMessage::Generic(msg) => ::serde_json::to_value(msg),
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
    PingResponse,
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
            A2AMessageKinds::Ping => MessageFamilies::TrustPing,
            A2AMessageKinds::PingResponse => MessageFamilies::TrustPing,
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
            A2AMessageKinds::PingResponse => "ping_response".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageId(pub String);

impl MessageId {
    #[cfg(test)]
    pub fn id() -> MessageId {
        MessageId(String::from("testid"))
    }

    #[cfg(test)]
    pub fn new() -> MessageId {
        MessageId::id()
    }

    #[cfg(not(test))]
    pub fn new() -> MessageId {
        use utils::uuid;
        MessageId(uuid::uuid())
    }
}