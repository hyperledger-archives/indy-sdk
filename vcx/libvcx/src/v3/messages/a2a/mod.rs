pub mod message_family;
pub mod message_type;
pub mod protocol_registry;

use self::message_type::MessageType;
use self::message_family::MessageFamilies;

use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::Value;

use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::SignedResponse;
use v3::messages::connection::problem_report::ProblemReport as ConnectionProblemReport;
use v3::messages::connection::ping::Ping;
use v3::messages::connection::ping_response::PingResponse;
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

use v3::messages::discovery::query::Query;
use v3::messages::discovery::disclose::Disclose;

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

    /// discovery features
    Query(Query),
    Disclose(Disclose),

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

        match (message_type.family, message_type.type_.as_str()) {
            (MessageFamilies::Routing, "forward") => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessage::Forward(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Connections, "invitation") => {
                Invitation::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionInvitation(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Connections, "request") => {
                Request::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Connections, "response") => {
                SignedResponse::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Notification, "ping") => {
                Ping::deserialize(value)
                    .map(|msg| A2AMessage::Ping(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Notification, "ping_response") => {
                PingResponse::deserialize(value)
                    .map(|msg| A2AMessage::PingResponse(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Connections, "problem_report") => {
                ConnectionProblemReport::deserialize(value)
                    .map(|msg| A2AMessage::ConnectionProblemReport(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::Notification, "ack") => {
                Ack::deserialize(value)
                    .map(|msg| A2AMessage::Ack(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::ReportProblem, "problem-report") => {
                CommonProblemReport::deserialize(value)
                    .map(|msg| A2AMessage::CommonProblemReport(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::CredentialIssuance, "issue-credential") => {
                Credential::deserialize(value)
                    .map(|msg| A2AMessage::Credential(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::CredentialIssuance, "propose-credential") => {
                CredentialProposal::deserialize(value)
                    .map(|msg| A2AMessage::CredentialProposal(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::CredentialIssuance, "offer-credential") => {
                CredentialOffer::deserialize(value)
                    .map(|msg| A2AMessage::CredentialOffer(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::CredentialIssuance, "request-credential") => {
                CredentialRequest::deserialize(value)
                    .map(|msg| A2AMessage::CredentialRequest(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::PresentProof, "propose-presentation") => {
                PresentationProposal::deserialize(value)
                    .map(|msg| A2AMessage::PresentationProposal(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::PresentProof, "request-presentation") => {
                PresentationRequest::deserialize(value)
                    .map(|msg| A2AMessage::PresentationRequest(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::PresentProof, "presentation") => {
                Presentation::deserialize(value)
                    .map(|msg| A2AMessage::Presentation(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::DiscoveryFeatures, "query") => {
                Query::deserialize(value)
                    .map(|msg| A2AMessage::Query(msg))
                    .map_err(de::Error::custom)
            }
            (MessageFamilies::DiscoveryFeatures, "disclose") => {
                Disclose::deserialize(value)
                    .map(|msg| A2AMessage::Disclose(msg))
                    .map_err(de::Error::custom)
            }
            (_, other_type) => {
                warn!("Unexpected @type field structure: {}", other_type);
                Ok(A2AMessage::Generic(value.to_string()))
            }
        }
    }
}

fn set_a2a_message_type<T>(msg: T, family: MessageFamilies, name: &str) -> Result<serde_json::Value, serde_json::Error> where T: Serialize {
    let mut value = ::serde_json::to_value(msg)?;
    let type_ = ::serde_json::to_value(MessageType::build(family, name))?;
    value.as_object_mut().unwrap().insert("@type".into(), type_);
    Ok(value)
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessage::Forward(msg) => set_a2a_message_type(msg, MessageFamilies::Routing, "forward"),
            A2AMessage::ConnectionInvitation(msg) => set_a2a_message_type(msg, MessageFamilies::Connections, "invitation"),
            A2AMessage::ConnectionRequest(msg) => set_a2a_message_type(msg, MessageFamilies::Connections, "request"),
            A2AMessage::ConnectionResponse(msg) => set_a2a_message_type(msg, MessageFamilies::Connections, "response"),
            A2AMessage::ConnectionProblemReport(msg) => set_a2a_message_type(msg, MessageFamilies::Connections, "problem_report"),
            A2AMessage::Ping(msg) => set_a2a_message_type(msg, MessageFamilies::Notification, "ping"),
            A2AMessage::PingResponse(msg) => set_a2a_message_type(msg, MessageFamilies::Notification, "ping_response"), // TODO: trust_ping Message family
            A2AMessage::Ack(msg) => set_a2a_message_type(msg, MessageFamilies::Notification, "ack"),
            A2AMessage::CommonProblemReport(msg) => set_a2a_message_type(msg, MessageFamilies::ReportProblem, "problem-report"),
            A2AMessage::CredentialOffer(msg) => set_a2a_message_type(msg, MessageFamilies::CredentialIssuance, "offer-credential"),
            A2AMessage::Credential(msg) => set_a2a_message_type(msg, MessageFamilies::CredentialIssuance, "issue-credential"),
            A2AMessage::CredentialProposal(msg) => set_a2a_message_type(msg, MessageFamilies::CredentialIssuance, "propose-credential"),
            A2AMessage::CredentialRequest(msg) => set_a2a_message_type(msg, MessageFamilies::CredentialIssuance, "request-credential"),
            A2AMessage::PresentationProposal(msg) => set_a2a_message_type(msg, MessageFamilies::PresentProof, "propose-presentation"),
            A2AMessage::PresentationRequest(msg) => set_a2a_message_type(msg, MessageFamilies::PresentProof, "request-presentation"),
            A2AMessage::Presentation(msg) => set_a2a_message_type(msg, MessageFamilies::PresentProof, "presentation"),
            A2AMessage::Query(msg) => set_a2a_message_type(msg, MessageFamilies::DiscoveryFeatures, "query"),
            A2AMessage::Disclose(msg) => set_a2a_message_type(msg, MessageFamilies::DiscoveryFeatures, "disclose"),
            A2AMessage::Generic(msg) => ::serde_json::to_value(msg),
        }.map_err(ser::Error::custom)?;

        value.serialize(serializer)
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