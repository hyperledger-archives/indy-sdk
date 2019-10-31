pub mod states;

use settings;
use messages;
use messages::{MessageStatusCode, ObjectWithVersion};
use messages::update_message::{UIDsByConn, update_messages as update_messages_status};
use messages::get_message::Message;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_my_did;
use messages::update_connection::send_delete_connection_message;
use connection::create_agent_keys;

use v3::handlers::connection::states::*;
use v3::messages::A2AMessage;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::{Response, SignedResponse};
use v3::messages::connection::problem_report::{ProblemReport, ProblemCode};
use v3::messages::ack::Ack;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::connection::agent_info::AgentInfo;
use v3::utils::encryption_envelope::EncryptionEnvelope;

use std::collections::HashMap;

lazy_static! {
    pub static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    source_id: String,
    state: DidExchangeStateSM
}

impl Connection {
    const SERIALIZE_VERSION: &'static str = "2.0";

    fn create(source_id: &str, actor: Actor) -> VcxResult<Connection> {
        trace!("Connection::create >>> source_id: {}, actor: {:?}", source_id, actor);

        Ok(Connection {
            source_id: source_id.to_string(),
            state: DidExchangeStateSM::new(actor),
        })
    }

    fn state(&self) -> u32 { self.state.state() }

    fn actor(&self) -> Actor { self.state.actor() }

    fn agent_info(&self) -> &AgentInfo { self.state.agent_info() }

    fn agency_endpoint(&self) -> VcxResult<String> {
        settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)
            .map(|str| format!("{}/agency/msg", str))
    }

    fn routing_keys(&self) -> VcxResult<Vec<String>> {
        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        Ok(vec![self.agent_info().agent_vk.to_string(), agency_vk])
    }

    fn recipient_keys(&self) -> Vec<String> {
        vec![self.agent_info().pw_vk.to_string()]
    }

    fn remote_connection_info(&self) -> Option<RemoteConnectionInfo> {
        self.state.remote_connection_info()
    }

    fn remote_vk(&self) -> VcxResult<String> {
        self.state.remote_vk()
    }

    fn get_source_id(&self) -> VcxResult<String> {
        Ok(self.source_id.clone())
    }

    fn process_invite(&mut self, invitation: Invitation) -> VcxResult<()> {
        self.step(Messages::InvitationReceived(invitation))
    }

    fn get_invite_details(&self) -> VcxResult<String> {
        if let Some(invitation) = self.state.get_invitation() {
            return Ok(json!(invitation).to_string());
        } else if let Some(remote_info) = self.state.remote_connection_info() {
            return Ok(json!(remote_info).to_string());
        } else {
            Ok(json!({}).to_string())
        }
    }

    fn connect(&mut self) -> VcxResult<u32> {
        trace!("Connection::connect >>> source_id: {}", self.source_id);

        self.create_agent()?;

        let message = match self.actor() {
            Actor::Inviter => {
                let invite: Invitation = Invitation::create()
                    .set_label(self.source_id.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                Messages::SendInvitation(invite)
            }
            Actor::Invitee => {
                let request = Request::create()
                    .set_label(self.source_id.to_string())
                    .set_did(self.agent_info().pw_did.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_keys(self.recipient_keys(), self.routing_keys()?);

                Messages::SendExchangeRequest(request)
            }
        };
        self.step(message)?;

        Ok(error::SUCCESS.code_num)
    }

    fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        trace!("Connection: update_state");

        match message {
            Some(message_) => self.update_state_with_message(message_)?,
            None => {
                let (messages, messages_to_update) = self.get_messages()?;

                messages
                    .into_iter()
                    .map(|(_, message)| self.handle_message(message))
                    .collect::<VcxResult<Vec<u32>>>()?;

                update_messages_status(MessageStatusCode::Reviewed, messages_to_update)?
            }
        }

        Ok(error::SUCCESS.code_num)
    }

    fn update_message_status(&self, uid: String) -> VcxResult<()> {
        let messages_to_update = vec![UIDsByConn {
            pairwise_did: self.agent_info().pw_did.clone(),
            uids: vec![uid]
        }];

        update_messages_status(MessageStatusCode::Reviewed, messages_to_update)
    }

    fn process_acceptance_message(&mut self, message: &Message) -> VcxResult<u32> {
        trace!("Connection: process_acceptance_message: {:?}", message);

        let message = self.decode_message(&message)?;

        self.handle_message(message)?;

        Ok(error::SUCCESS.code_num)
    }

    fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Connection: update_state_with_message: {}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let message = self.decode_message(&message)?;

        self.handle_message(message)?;

        Ok(())
    }

    fn get_messages(&self) -> VcxResult<(HashMap<String, A2AMessage>, Vec<UIDsByConn>)> {
        trace!("Connection: get_messages");

        let mut messages_to_update: Vec<UIDsByConn> = vec![];

        let agent_info = self.agent_info();
        let mut messages = messages::get_message::get_connection_messages(&agent_info.pw_did,
                                                                          &agent_info.pw_vk,
                                                                          &agent_info.agent_did,
                                                                          &agent_info.agent_vk,
                                                                          None,
                                                                          Some(vec![MessageStatusCode::Received]))?;

        messages_to_update.push(
            UIDsByConn {
                pairwise_did: agent_info.pw_did.clone(),
                uids: messages.iter().map(|message| message.uid.clone()).collect()
            });

        if let Some(prev_agent_info) = self.state.prev_agent_info() {
            let mut add_messages = messages::get_message::get_connection_messages(&prev_agent_info.pw_did,
                                                                                  &prev_agent_info.pw_vk,
                                                                                  &prev_agent_info.agent_did,
                                                                                  &prev_agent_info.agent_vk,
                                                                                  None,
                                                                                  Some(vec![MessageStatusCode::Received]))?;


            messages_to_update.push(
                UIDsByConn {
                    pairwise_did: prev_agent_info.pw_did.clone(),
                    uids: add_messages.iter().map(|message| message.uid.clone()).collect()
                });

            messages.append(&mut add_messages);
        }

        let mut a2a_messages: HashMap<String, A2AMessage> = HashMap::new();

        for message in messages {
            a2a_messages.insert(message.uid.clone(), self.decode_message(&message)?);
        }

        Ok((a2a_messages, messages_to_update))
    }

    fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        trace!("Connection: get_messages");

        let agent_info = self.agent_info();

        let mut messages = messages::get_message::get_connection_messages(&agent_info.pw_did,
                                                                          &agent_info.pw_vk,
                                                                          &agent_info.agent_did,
                                                                          &agent_info.agent_vk,
                                                                          Some(vec![msg_id.to_string()]),
                                                                          None)?;

        let message =
            messages
                .pop()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidMessages, format!("Message not found for id: {:?}", msg_id)))?;

        let message = self.decode_message(&message)?;

        Ok(message)
    }

    fn handle_message(&mut self, message: A2AMessage) -> VcxResult<u32> {
        trace!("Connection: handle_message: {:?}", message);

        match self.state.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) => {
                match message {
                    A2AMessage::ConnectionRequest(request) => {
                        debug!("Inviter received ConnectionRequest message");
                        if let Err(err) = self.handle_connection_request(request) {
                            self.send_problem_report(ProblemCode::RequestProcessingError, err)?
                        }
                    }
                    A2AMessage::ConnectionProblemReport(problem_report) => {
                        debug!("Inviter received ProblemReport message");
                        self.handle_problem_report(problem_report)?;
                    }
                    message @ _ => {
                        debug!("Inviter received unexpected message: {:?}", message);
                    }
                }
            }
            ActorDidExchangeState::Invitee(DidExchangeState::Requested(ref state)) => {
                match message {
                    A2AMessage::ConnectionResponse(response) => {
                        debug!("Invitee received ConnectionResponse message");
                        if let Err(err) = self.handle_connection_response(response) {
                            self.send_problem_report(ProblemCode::ResponseProcessingError, err)?
                        }
                    }
                    A2AMessage::ConnectionProblemReport(problem_report) => {
                        debug!("Invitee received ProblemReport message");
                        self.handle_problem_report(problem_report)?;
                    }
                    message @ _ => {
                        debug!("Invitee received unexpected message: {:?}", message);
                    }
                }
            }
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => {
                match message {
                    A2AMessage::Ack(ack) => {
                        debug!("Ack message received");
                        self.step(Messages::AckReceived(ack))?;
                    }
                    A2AMessage::ConnectionProblemReport(problem_report) => {
                        debug!("ProblemReport message received");
                        self.handle_problem_report(problem_report)?;
                    }
                    message @ _ => {
                        debug!("Unexpected message received in Responded state: {:?}", message);
                    }
                }
            }
            _ => {
                debug!("Unexpected message received: message: {:?}", message);
            }
        }

        Ok(error::SUCCESS.code_num)
    }

    fn decode_message(&self, message: &Message) -> VcxResult<A2AMessage> {
        EncryptionEnvelope::open(&self.agent_info().pw_vk, message.payload()?)
    }

    fn handle_connection_request(&mut self, request: Request) -> VcxResult<()> {
        trace!("Connection: handle_connection_request: {:?}", request);

        self.step(Messages::ReceivedExchangeRequest(request))?;

        // provision a new keys
        self.create_agent()?;

        let response = Response::create()
            .set_did(self.agent_info().pw_did.to_string())
            .set_service_endpoint(self.agency_endpoint()?)
            .set_keys(self.recipient_keys(), self.routing_keys()?);

        self.step(Messages::SendExchangeResponse(response))?;

        Ok(())
    }

    fn handle_connection_response(&mut self, response: SignedResponse) -> VcxResult<()> {
        trace!("Connection: handle_connection_response: {:?}", response);

        self.step(Messages::ReceivedExchangeResponse(response))?;

        let ack = Ack::create();

        self.step(Messages::SendAck(ack))?;

        Ok(())
    }

    fn handle_problem_report(&mut self, problem_report: ProblemReport) -> VcxResult<()> {
        trace!("Connection: handle_problem_report: {:?}", problem_report);
        self.step(Messages::ReceivedProblemReport(problem_report))
    }

    fn send_message(&self, message: &A2AMessage) -> VcxResult<String> {
        self.state.send_message(message)?;
        Ok(String::new())
    }

    fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        self.send_message(&A2AMessage::Generic(message.to_string()))?;
        Ok(String::new())
    }

    fn send_problem_report(&mut self, problem_code: ProblemCode, err: VcxError) -> VcxResult<()> {
        let problem_report = ProblemReport::create()
            .set_problem_code(problem_code)
            .set_explain(err.to_string());

        self.step(Messages::SendProblemReport(problem_report))?;

        Ok(())
    }

    fn create_agent(&mut self) -> VcxResult<()> {
        let method_name = settings::get_config_value(settings::CONFIG_DID_METHOD).ok();
        let (pw_did, pw_vk) = create_my_did(None, method_name.as_ref().map(String::as_str))?;

        /*
            Create User Pairwise Agent in old way.
            Send Messages corresponding to V2 Protocol to avoid code changes on Agency side.
        */
        let (agent_did, agent_vk) = create_agent_keys(&self.source_id, &pw_did, &pw_vk)?;

        let agent_info = AgentInfo { pw_did, pw_vk, agent_did, agent_vk };

        self.state.set_agent_info(agent_info);

        Ok(())
    }

    fn delete(&mut self) -> VcxResult<u32> {
        trace!("Connection: delete: {:?}", self.source_id);
        send_delete_connection_message(&self.agent_info().pw_did, &self.agent_info().pw_vk, &self.agent_info().agent_did, &self.agent_info().agent_vk)?;
        Ok(error::SUCCESS.code_num)
    }

    fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    fn step(&mut self, message: Messages) -> VcxResult<()> {
        self.state = self.state.clone().step(message)?;
        Ok(())
    }
}

pub fn create_connection(source_id: &str) -> VcxResult<u32> {
    let connection = Connection::create(source_id, Actor::Inviter)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn create_connection_with_invite(source_id: &str, invitation: Invitation) -> VcxResult<u32> {
    let mut connection: Connection = Connection::create(source_id, Actor::Invitee)?;

    connection.process_invite(invitation)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn connect(handle: u32, _options: Option<String>) -> VcxResult<u32> {
    // Do we need it now????
    // let options_obj: ConnectionOptions = ConnectionOptions::from_opt_str(options)?;
    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.connect()
    })
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.update_state(message.as_ref().map(String::as_str))
    })
}

pub fn get_state(handle: u32) -> u32 {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.state())
    }).unwrap_or(0)
}

pub fn get_messages(handle: u32) -> VcxResult<(HashMap<String, A2AMessage>, Vec<UIDsByConn>)> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_messages()
    })
}

pub fn update_message_status(handle: u32, uid: String) -> VcxResult<()> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.update_message_status(uid.clone())
    })
}

pub fn get_message_by_id(handle: u32, msg_id: String) -> VcxResult<A2AMessage> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_message_by_id(&msg_id)
    })
}

pub fn decode_message(handle: u32, message: Message) -> VcxResult<A2AMessage> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.decode_message(&message)
    })
}

pub fn send_message(handle: u32, message: A2AMessage) -> VcxResult<()> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.state.send_message(&message)
    })
}

pub fn get_invite_details(handle: u32, abbreviated: bool) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_invite_details()
    })
}

pub fn send_generic_message(handle: u32, msg: &str, msg_options: &str) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.send_generic_message(msg, msg_options)
    })
}

pub fn get_pw_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_vk.to_string())
    })
}

pub fn get_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_vk.to_string())
    })
}

pub fn get_their_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.remote_vk()
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_source_id()
    })
}

pub fn release(handle: u32) -> VcxResult<()> {
    CONNECTION_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn delete_connection(handle: u32) -> VcxResult<u32> {
    CONNECTION_MAP.get_mut(handle, |t| {
        t.delete()
    })
        .or(Err(VcxError::from(VcxErrorKind::DeleteConnection)))
        .and(release(handle))
        .and_then(|_| Ok(error::SUCCESS.code_num))
}

// Actually it handles any message
pub fn process_acceptance_message(handle: u32, message: Message) -> VcxResult<u32> {
    CONNECTION_MAP.get_mut(handle, |t| {
        t.process_acceptance_message(&message)
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