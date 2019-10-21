pub mod states;

use serde_json;

use settings;
use messages;
use messages::{MessageStatusCode, ObjectWithVersion};
use messages::invite::InviteDetail;
use messages::get_message::Message;
use messages::update_message::{UIDsByConn, update_messages};
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_my_did;
use utils::constants::DEFAULT_SERIALIZE_VERSION;

use connection::{ConnectionOptions, create_agent_keys};

use v3::handlers::connection::states::*;
use v3::messages::A2AMessage;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::ack::Ack;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::utils::encryption_envelope::EncryptionEnvelope;

lazy_static! {
    static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Connection {
    source_id: String,
    actor: Actor,
    pw_did: String,
    pw_vk: String,
    agent_did: String,
    agent_vk: String,
    state: DidExchangeState
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Actor {
    Inviter,
    Invitee
}

impl Connection {
    fn create(source_id: &str, actor: Actor) -> VcxResult<Connection> {
        let method_name = settings::get_config_value(settings::CONFIG_DID_METHOD).ok();
        let (pw_did, pw_vk) = create_my_did(None, method_name.as_ref().map(String::as_str))?;

        /*
            Create User Pairwise Agent in old way.
            Send Messages corresponding to V2 Protocol to avoid code changes on Agency side.
        */
        let (agent_did, agent_vk) = create_agent_keys(source_id, &pw_did, &pw_vk)?;

        Ok(Connection {
            source_id: source_id.to_string(),
            actor,
            pw_did,
            pw_vk,
            agent_did,
            agent_vk,
            state: DidExchangeState::Null(NullState {}),
        })
    }

    fn state(&self) -> u32 { self.state.state() }

    fn agency_endpoint(&self) -> VcxResult<String> {
        settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)
    }

    fn routing_keys(&self) -> VcxResult<Vec<String>> {
        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        Ok(vec![agency_vk, self.agent_vk.clone()])
    }

    fn recipient_keys(&self) -> Vec<String> {
        vec![self.pw_vk.clone()]
    }

    fn step(&mut self, message: Messages) {
        self.state = self.state.step(message)
    }

    fn remote_connection_info(&self) -> Option<RemoteConnectionInfo> {
        match self.state {
            DidExchangeState::Null(_) => None,
            DidExchangeState::Invited(ref state) => Some(RemoteConnectionInfo::from(state.invitation.clone())),
            DidExchangeState::Requested(ref state) => Some(RemoteConnectionInfo::from(state.request.clone())),
            DidExchangeState::Responded(_) => None,
            DidExchangeState::Complete(_) => None
        }
    }

    fn remote_connection_vk(&self) -> VcxResult<String> {
        match self.state {
            DidExchangeState::Complete(_) => Ok(String::new()),
            _ => Err(VcxError::from(VcxErrorKind::NotReady)),
        }
    }

    fn process_invite(&mut self, invite: &str) -> VcxResult<()> {
        let invitation: Invitation = serde_json::from_str(&invite)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Invitation: {:?}", err)))?;

        self.step(Messages::Invitation(invitation));

        Ok(())
    }

    fn get_invite(&self) -> VcxResult<String> {
        // TODO: ASK AND DO
        Ok(json!(InviteDetail::new()).to_string())
    }

    fn connect(&mut self, _options: &ConnectionOptions) -> VcxResult<()> {
        let message = match self.actor {
            Actor::Inviter => {
                let invite: Invitation = Invitation::create()
                    .set_label(self.source_id.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                Messages::Invitation(invite)
            }
            Actor::Invitee => {
                let request = Request::create()
                    .set_label(self.source_id.to_string())
                    .set_did(self.pw_did.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                self.send_message(&request.to_a2a_message())?;

                Messages::ExchangeRequest(request)
            }
        };

        self.step(message);

        Ok(())
    }

    fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        match self.state {
            DidExchangeState::Null(_) => return Ok(error::SUCCESS.code_num),
            DidExchangeState::Complete(_) => return Ok(error::SUCCESS.code_num),
            _ => {}
        }

        let messages: Vec<Message> = messages::get_message::get_connection_messages(&self.pw_did,
                                                                                    &self.pw_vk,
                                                                                    &self.agent_did,
                                                                                    &self.agent_vk,
                                                                                    None,
                                                                                    Some(vec![MessageStatusCode::Received]))?;

        let mut uids: Vec<String> = Vec::new();

        for messages in messages {
            uids.push(messages.uid.clone());
            let message = EncryptionEnvelope::open(&self.pw_vk, messages.payload()?)?;

            match self.state {
                DidExchangeState::Invited(ref state) => {
                    match message {
                        A2AMessage::ConnectionRequest(request) => {
                            self.handle_incoming_request(request)?;
                        }
                        _ => {}
                    }
                }
                DidExchangeState::Requested(ref state) => {
                    match message {
                        A2AMessage::ConnectionResponse(response) => {
                            self.handle_incoming_response(response)?;
                        }
                        _ => {}
                    }
                }
                DidExchangeState::Responded(ref state) => {
                    match message {
                        A2AMessage::Ack(ack) => {
                            self.handle_incoming_ack(ack)?;
                        }
                        A2AMessage::ProblemReport(problem_report) => {
                            self.handle_incoming_problem_report(problem_report)?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        self.update_messages_status(uids)?;

        Ok(error::SUCCESS.code_num)
    }

    fn handle_incoming_request(&mut self, request: Request) -> VcxResult<()> {
        match self.actor {
            Actor::Inviter => return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Unsupported message")),
            Actor::Invitee => {}
        };

        self.step(Messages::ExchangeRequest(request));

        let request = Response::create()
            .set_did(self.pw_did.to_string())
            .set_service_endpoint(self.agency_endpoint()?)
            .set_recipient_keys(self.recipient_keys())
            .set_routing_keys(self.routing_keys()?);

        self.send_message(&request.to_a2a_message())?;

        self.step(Messages::ExchangeResponse(request));

        Ok(())
    }

    fn handle_incoming_response(&mut self, response: Response) -> VcxResult<()> {
        match self.actor {
            Actor::Invitee => return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Unsupported message")),
            Actor::Inviter => {}
        };

        self.step(Messages::ExchangeResponse(response));

        let ack = Ack::create();

        self.send_message(&ack.to_a2a_message())?;

        self.step(Messages::Ack(ack));

        Ok(())
    }

    fn handle_incoming_ack(&mut self, ack: Ack) -> VcxResult<()> {
        self.step(Messages::Ack(ack));
        Ok(())
    }

    fn handle_incoming_problem_report(&mut self, problem_report: ProblemReport) -> VcxResult<()> {
        self.step(Messages::Error(problem_report));
        Ok(())
    }

    fn update_messages_status(&self, uids: Vec<String>) -> VcxResult<()> {
        let targets = vec![UIDsByConn { pairwise_did: self.pw_did.clone(), uids }];
        update_messages(MessageStatusCode::Reviewed, targets)
    }

    fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        let remote_connection_info = self.remote_connection_info()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot get Remote Connection information"))?;

        EncryptionEnvelope::create(&message, &self.pw_vk, &remote_connection_info)?
            .send()?;

        Ok(())
    }

    fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        match self.state {
            DidExchangeState::Complete(_) => {}
            _ => return Err(VcxError::from(VcxErrorKind::NotReady))
        };

        self.send_message(&A2AMessage::Generic(message.to_string()))?;

        Ok(String::new())
    }

    fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }
}

pub fn create_connection(source_id: &str) -> VcxResult<u32> {
    let connection = Connection::create(source_id, Actor::Inviter)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn create_connection_with_invite(source_id: &str, invitation: &str) -> VcxResult<u32> {
    let mut connection: Connection = Connection::create(source_id, Actor::Invitee)?;

    connection.process_invite(invitation)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn connect(handle: u32, options: Option<String>) -> VcxResult<()> {
    // Do we need it now????
    let options_obj: ConnectionOptions = ConnectionOptions::from_opt_str(options)?;

    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.connect(&options_obj)
    })
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.update_state(message.as_ref().map(String::as_str))
    })
}

pub fn get_state(handle: u32) -> u32 {
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.state().clone())
    }).unwrap_or(0)
}

pub fn get_invite_details(handle: u32, abbreviated: bool) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_invite()
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Connection::to_string(&connection)
    })
}

pub fn from_string(connection_data: &str) -> VcxResult<u32> {
    let connection: Connection = Connection::from_str(connection_data)?;
    CONNECTION_MAP.add(connection)
}

pub fn send_generic_message(handle: u32, msg: &str, msg_options: &str) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.send_generic_message(msg, msg_options)
    })
}

pub fn get_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.pw_vk.clone())
    })
}

pub fn get_their_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.remote_connection_vk()
    })
}

pub fn release(handle: u32) -> VcxResult<()> {
    CONNECTION_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}
