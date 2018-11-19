use actix::prelude::*;
use failure::*;

pub mod router;
pub mod forward_agent;
pub mod forward_agent_connection;
pub mod agent;
pub mod agent_connection;
pub mod requester;

use domain::a2connection::A2ConnMessage;

// Common messages

pub struct AddA2ARoute(pub String, pub Recipient<HandleA2AMsg>);

impl Message for AddA2ARoute {
    type Result = ();
}

pub struct AddA2ConnRoute(pub String, pub Recipient<HandleA2ConnMsg>);

impl Message for AddA2ConnRoute {
    type Result = ();
}

#[derive(Debug)]
pub struct GetEndpoint();

#[derive(Debug, Serialize)]
pub struct Endpoint {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
}

impl Message for GetEndpoint {
    type Result = Result<Endpoint, Error>;
}

#[derive(Debug)]
pub struct ForwardA2AMsg(pub Vec<u8>);

impl Message for ForwardA2AMsg {
    type Result = Result<Vec<u8>, Error>;
}

#[derive(Debug)]
pub struct HandleA2AMsg(pub Vec<u8>);

impl Message for HandleA2AMsg {
    type Result = Result<Vec<u8>, Error>;
}

#[derive(Debug)]
pub struct RouteA2AMsg(pub String, pub Vec<u8>);

impl Message for RouteA2AMsg {
    type Result = Result<Vec<u8>, Error>;
}

#[derive(Debug)]
pub struct HandleA2ConnMsg(pub A2ConnMessage);

impl Message for HandleA2ConnMsg {
    type Result = Result<A2ConnMessage, Error>;
}

#[derive(Debug)]
pub struct RouteA2ConnMsg(pub String, pub A2ConnMessage);

impl Message for RouteA2ConnMsg {
    type Result = Result<A2ConnMessage, Error>;
}

#[derive(Debug, Serialize)]
pub struct RemoteMsg {
    pub endpoint: String,
    pub body: Vec<u8>
}

impl Message for RemoteMsg {
    type Result = Result<(), Error>;
}
