use serde_json;
use serde_json::Value;
use rmp_serde;

use connection::{ConnectionOptions, create_agent_keys};
use api::VcxStateType;
use settings;
use messages as messages_v2;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_my_did;
use utils::libindy::crypto;
use utils::json::mapped_key_rewrite;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use utils::json::KeyMatch;
use std::collections::HashMap;

use v3::messages::A2AMessage;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::remote_connection::RemoteConnectionInfo;
use v3::utils::remote_message::RemoteMessage;

lazy_static! {
    static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Debug)]
struct Connection {
    source_id: String,
    actor: Actor,
    pw_did: String,
    pw_verkey: String,
    state: ConnectionState
}

/// Transitions of Connection state
/// Null -> Invited
/// Invited -> Requested, Null
/// Requested -> Responded, Null
/// Responded -> Complete, Invited
/// Complete
#[derive(Debug)]
enum ConnectionState {
    Null(NullState),
    Invited(InvitedState),
    Requested(RequestedState),
    Responded(RespondedState),
    Complete(CompleteState),
}

#[derive(Debug)]
struct NullState {}

#[derive(Debug)]
struct InvitedState {
    invitation: Invitation
}

#[derive(Debug)]
struct RequestedState {
    remote_connection_info: RemoteConnectionInfo
}

#[derive(Debug)]
struct RespondedState {}

#[derive(Debug)]
struct CompleteState {}

#[derive(Debug)]
enum Messages {
    Error(Error),
    Invitation(Invitation),
    ExchangeRequest(Request),
    ExchangeResponse(ExchangeResponse),
    Ack(Ack)
}

#[derive(Debug)]
struct Error {}

#[derive(Debug)]
struct ExchangeResponse {}

#[derive(Debug)]
struct Ack {}

#[derive(Debug)]
enum Actor {
    Inviter,
    Invitee
}

impl Connection {
    fn step(&mut self, message: Messages) -> VcxResult<()> {
        match &self.state {
            ConnectionState::Null(_) => {
                match message {
                    Messages::Invitation(invitation) => {
                        self.state = ConnectionState::Invited(InvitedState { invitation })
                    }
                    _ => {}
                }
            }
            ConnectionState::Invited(_) => {
                match message {
                    Messages::Error(error) => {
                        self.state = ConnectionState::Null(NullState {})
                    }
                    Messages::ExchangeRequest(request) => {
                        self.state = ConnectionState::Requested(RequestedState { remote_connection_info: RemoteConnectionInfo::from(request) })
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn state(&self) -> u32 {
        match self.state {
            ConnectionState::Null(_) => 1,
            ConnectionState::Invited(_) => 2,
            ConnectionState::Requested(_) => 3,
            ConnectionState::Responded(_) => 5, // for backward compatibility
            ConnectionState::Complete(_) => 4,
        }
    }

    fn remote_connection_info<'a>(&'a self) -> Option<&'a RemoteConnectionInfo> {
        match self.state {
            ConnectionState::Null(_) => None,
            ConnectionState::Invited(ref state) => None,
            ConnectionState::Requested(ref state) => Some(&state.remote_connection_info),
            ConnectionState::Responded(_) => None,
            ConnectionState::Complete(_) => None
        }
    }
}

impl Connection {
    fn create(source_id: &str, actor: Actor) -> VcxResult<Connection> {
        let method_name = settings::get_config_value(settings::CONFIG_DID_METHOD).ok();
        let (pw_did, pw_verkey) = create_my_did(None, method_name.as_ref().map(String::as_str))?;

        debug!("did: {} verkey: {}, source id: {}", pw_did, pw_verkey, source_id);

        Ok(Connection {
            source_id: source_id.to_string(),
            actor,
            pw_did,
            pw_verkey,
            state: ConnectionState::Null(NullState {}),
        })
    }

    fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        let remote_connection_info = self.remote_connection_info()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot get Remote Connection information"))?;

        RemoteMessage::build(&message, &self.pw_verkey, remote_connection_info)?
            .send()?;

        Ok(())
    }
}

pub fn create_connection(source_id: &str) -> VcxResult<u32> {
    let connection = Connection::create(source_id, Actor::Inviter)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn create_connection_with_invite(source_id: &str, invite: &str) -> VcxResult<u32> {
    let mut connection: Connection = Connection::create(source_id, Actor::Invitee)?;

    let invite: Invitation = serde_json::from_str(&invite).unwrap();

    connection.step(Messages::Invitation(invite))?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn connect(handle: u32, options: Option<String>) -> VcxResult<()> {
    let mut connection: Connection = Connection::create("", Actor::Inviter)?;
    //    let options_obj: ConnectionOptions = ConnectionOptions::from_opt_str(options)?;
    //
    //    CONNECTION_MAP.get_mut(handle, |connection| {
    //        connection.update_agent_profile(&options_obj)?;
    //        connection.create_agent_pairwise()?;
    //        connection.connect(&options_obj)
    //    })

    let agency_endpoint = settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)?;
    let agency_verkey = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;

    /*
        Create User Pairwise Agent in old way.
        Send Messages corresponding to V2 Protocol version to avoid code changes on Agency side.
    */
    let (_, agent_verkey) = create_agent_keys(&connection.source_id, &connection.pw_did, &connection.pw_verkey)?;

    match connection.actor {
        Actor::Inviter => {
            let invite: Invitation = Invitation::create()
                .set_label(connection.source_id.to_string())
                .set_service_endpoint(agency_endpoint)
                .set_recipient_keys(vec![connection.pw_verkey.clone()])
                .set_routing_keys(vec![agency_verkey, agent_verkey]);

            connection.step(Messages::Invitation(invite))?;
        }
        Actor::Invitee => {
            let request = Request::create()
                .set_label(connection.source_id.to_string())
                .set_did(connection.pw_did.to_string())
                .set_service_endpoint(agency_endpoint)
                .set_recipient_keys(vec![connection.pw_verkey.clone()])
                .set_routing_keys(vec![agency_verkey, agent_verkey]);

            connection.send_message(&request.to_a2a_message())?;

            connection.step(Messages::ExchangeRequest(request))?;
        }
    }

    Ok(())
}