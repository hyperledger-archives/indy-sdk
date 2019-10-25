pub mod states;

use settings;
use messages;
use messages::{MessageStatusCode, ObjectWithVersion};
use messages::update_message::{UIDsByConn, update_messages};
use messages::get_message::Message;
use messages::thread::Thread;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_my_did;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use utils::httpclient;

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

lazy_static! {
    pub static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    source_id: String,
    state: DidExchangeStateSM
}

impl Connection {
    fn create(source_id: &str, actor: Actor) -> VcxResult<Connection> {
        trace!("Connection::create >>> source_id: {}, actor: {:?}", source_id, actor);

        Ok(Connection {
            source_id: source_id.to_string(),
            state: DidExchangeStateSM::new(actor),
        })
    }

    fn state(&self) -> u32 { self.state.state() }

    fn agent_info(&self) -> &AgentInfo { self.state.agent_info() }

    fn agency_endpoint(&self) -> VcxResult<String> {
        settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)
    }

    fn routing_keys(&self) -> VcxResult<Vec<String>> {
        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        Ok(vec![agency_vk, self.agent_info().agent_vk.to_string()])
    }

    fn recipient_keys(&self) -> Vec<String> {
        vec![self.agent_info().pw_vk.to_string()]
    }

    fn remote_connection_info(&self) -> Option<RemoteConnectionInfo> {
        self.state.remote_connection_info()
    }

    fn remote_vk(&self) -> VcxResult<String> {
        self.remote_connection_info()
            .and_then(|remote_info| remote_info.recipient_keys.get(0).cloned())
            .ok_or(VcxError::from(VcxErrorKind::NotReady))
    }

    fn get_source_id(&self) -> VcxResult<String> {
        Ok(self.source_id.clone())
    }

    fn process_invite(&mut self, invitation: Invitation) -> VcxResult<()> {
        self.step(Messages::InvitationReceived(invitation))
    }

    fn get_invite_details(&self) -> VcxResult<String> {
        let remote_connection_info: Option<RemoteConnectionInfo> = self.state.remote_connection_info();
        Ok(json!(remote_connection_info).to_string())
    }

    fn connect(&mut self) -> VcxResult<u32> {
        trace!("Connection::connect >>> source_id: {}", self.source_id);

        self.provision_did()?;

        let message = match self.state.state {
            ActorDidExchangeState::Inviter(_) => {
                let invite: Invitation = Invitation::create()
                    .set_label(self.source_id.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                Messages::InvitationSent(invite)
            }
            ActorDidExchangeState::Invitee(_) => {
                let request = Request::create()
                    .set_label(self.source_id.to_string())
                    .set_did(self.agent_info().pw_did.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_keys(self.recipient_keys(), self.routing_keys()?);

                self.send_message(&request.to_a2a_message())?;

                Messages::ExchangeRequestSent(request)
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
                    .map(|message| self.handle_message(message))
                    .collect::<VcxResult<Vec<u32>>>()?;

                update_messages(MessageStatusCode::Reviewed, messages_to_update)?
            }
        }

        Ok(error::SUCCESS.code_num)
    }

    fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Connection: update_state_with_message: {}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        self.handle_message(message)?;

        Ok(())
    }

    fn get_messages(&self) -> VcxResult<(Vec<Message>, Vec<UIDsByConn>)> {
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

        Ok((messages, messages_to_update))
    }

    fn handle_message(&mut self, message: Message) -> VcxResult<u32> {
        trace!("Connection: handle_message: {:?}", message);

        let message = EncryptionEnvelope::open(&self.agent_info().pw_vk, message.payload()?)?;

        match self.state.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) => {
                match message {
                    A2AMessage::ConnectionRequest(request) => {
                        debug!("Inviter received ConnectionRequest message");
                        self.handle_connection_request(request)?;
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
                        self.handle_connection_response(response)?;
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

    fn handle_connection_request(&mut self, request: Request) -> VcxResult<()> {
        let thread = Thread::new().set_thid(request.id.0.clone());

        if let Err(err) = self._handle_connection_request(request) {
            self.send_problem_report(ProblemCode::RequestProcessingError, err, thread)?
        }
        Ok(())
    }

    fn _handle_connection_request(&mut self, request: Request) -> VcxResult<()> {
        trace!("Connection: handle_connection_request: {:?}", request);

        request.connection.did_doc.validate()?;

        let thread = Thread::new().set_thid(request.id.0.clone());

        self.step(Messages::ExchangeRequestReceived(request))?;

        // original Verkey need to sign rotated keys
        let prev_agent_info = self.agent_info().clone();

        // provision a new keys
        self.provision_did()?;

        let response = Response::create()
            .set_did(self.agent_info().pw_did.to_string())
            .set_service_endpoint(self.agency_endpoint()?)
            .set_keys(self.recipient_keys(), self.routing_keys()?)
            .set_thread(thread);

        self.send_message(&response.encode(&prev_agent_info.pw_vk)?.to_a2a_message())?;

        self.step(Messages::ExchangeResponseSent(response, prev_agent_info))?;

        Ok(())
    }

    fn handle_connection_response(&mut self, response: SignedResponse) -> VcxResult<()> {
        let thread = response.thread.clone();

        if let Err(err) = self.handle_connection_response(response) {
            self.send_problem_report(ProblemCode::ResponseProcessingError, err, thread)?
        }
        Ok(())
    }

    fn _handle_connection_response(&mut self, response: SignedResponse) -> VcxResult<()> {
        trace!("Connection: handle_connection_response: {:?}", response);

        let response: Response = response.decode(&self.remote_vk()?)?;

        let thread = response.thread.clone();

        self.step(Messages::ExchangeResponseReceived(response))?;

        let ack = Ack::create().set_thread(thread);

        self.send_message(&ack.to_a2a_message())?;

        self.step(Messages::AckSent(ack))?;

        Ok(())
    }

    fn handle_problem_report(&mut self, problem_report: ProblemReport) -> VcxResult<()> {
        trace!("Connection: handle_problem_report: {:?}", problem_report);
        self.step(Messages::ProblemReport(problem_report))
    }

    fn send_message(&self, message: &A2AMessage) -> VcxResult<()> {
        let remote_connection_info = self.remote_connection_info()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot get Remote Connection information"))?;

        let envelope = EncryptionEnvelope::create(&message, &self.agent_info().pw_vk, &remote_connection_info)?;

        httpclient::post_u8(&envelope.0)?;

        Ok(())
    }

    fn send_generic_message(&self, message: &str, _message_options: &str) -> VcxResult<String> {
        self.send_message(&A2AMessage::Generic(message.to_string()))?;
        Ok(String::new())
    }

    fn send_problem_report(&mut self, problem_code: ProblemCode, err: VcxError, thread: Thread) -> VcxResult<()> {
        let problem_report = ProblemReport::create()
            .set_problem_code(problem_code)
            .set_explain(err.to_string())
            .set_thread(thread);

        self.send_message(&problem_report.to_a2a_message())?;

        self.step(Messages::ProblemReport(problem_report))?;

        Ok(())
    }

    fn provision_did(&mut self) -> VcxResult<()> {
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
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
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
    CONNECTION_MAP.get(handle, |cxn| {
        Ok(cxn.state())
    }).unwrap_or(0)
}

pub fn send_message(handle: u32, message: A2AMessage) -> VcxResult<()> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.send_message(&message)
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

pub fn get_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_vk.to_string())
    })
}

pub fn get_pw_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_did.to_string())
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
        t.handle_message(message.clone())
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