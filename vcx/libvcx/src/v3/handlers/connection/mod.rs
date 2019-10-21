pub mod states;

use settings;
use messages;
use messages::{MessageStatusCode, ObjectWithVersion};
use messages::invite::InviteDetail;
use messages::update_message::{UIDsByConn, update_messages};
use messages::get_message::Message;
use messages::thread::Thread;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::signus::create_my_did;
use utils::constants::DEFAULT_SERIALIZE_VERSION;

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
use v3::utils::encryption_envelope::EncryptionEnvelope;

lazy_static! {
    pub static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    source_id: String,
    actor: Actor,
    pw_did: String,
    pw_vk: String,
    agent_did: String,
    agent_vk: String,
    state: DidExchangeState
}

impl Connection {
    fn create(source_id: &str, actor: Actor) -> VcxResult<Connection> {
        let mut connection = Connection {
            source_id: source_id.to_string(),
            actor,
            pw_did: String::new(),
            pw_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            state: DidExchangeState::Null(NullState {}),
        };

        connection.provision_did()?;

        Ok(connection)
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
            DidExchangeState::Invited(ref state) => state.remote_info.clone(),
            DidExchangeState::Requested(ref state) => Some(state.remote_info.clone()),
            DidExchangeState::Responded(ref state) => Some(state.remote_info.clone()),
            DidExchangeState::Complete(ref state) => Some(state.remote_info.clone()),
        }
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
        self.step(Messages::InvitationReceived(invitation));
        Ok(())
    }

    fn get_invite(&self) -> VcxResult<String> {
        let (status_code, remote_info): (MessageStatusCode, &RemoteConnectionInfo) = match self.state {
            DidExchangeState::Null(_) => {
                return Ok(json!({}).to_string());
            }
            DidExchangeState::Invited(ref state) => {
                match state.remote_info {
                    Some(ref remote_info) => (MessageStatusCode::Received, &remote_info),
                    None => return Ok(json!({}).to_string())
                }
            }
            DidExchangeState::Requested(ref state) => (MessageStatusCode::Reviewed, &state.remote_info),
            DidExchangeState::Responded(ref state) => (MessageStatusCode::Reviewed, &state.remote_info),
            DidExchangeState::Complete(ref state) => (MessageStatusCode::Accepted, &state.remote_info),
        };

        let mut invite = InviteDetail::new();

        invite.status_msg = status_code.message().to_string();
        invite.status_code = status_code.to_string();
        invite.target_name = remote_info.label.clone();
        invite.sender_agency_detail.verkey = remote_info.routing_keys.get(0).cloned().unwrap_or_default();
        invite.sender_agency_detail.endpoint = remote_info.service_endpoint.clone();
        invite.sender_detail.verkey = remote_info.recipient_keys.get(0).cloned().unwrap_or_default();

        let invite_json = json!(invite).to_string();

        Ok(invite_json)
    }

    fn connect(&mut self) -> VcxResult<()> {
        let message = match self.actor {
            Actor::Inviter => {
                let invite: Invitation = Invitation::create()
                    .set_label(self.source_id.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                Messages::InvitationSent(invite)
            }
            Actor::Invitee => {
                let request = Request::create()
                    .set_label(self.source_id.to_string())
                    .set_did(self.pw_did.to_string())
                    .set_service_endpoint(self.agency_endpoint()?)
                    .set_recipient_keys(self.recipient_keys())
                    .set_routing_keys(self.routing_keys()?);

                self.send_message(&request.to_a2a_message())?;

                Messages::ExchangeRequestSent(request)
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

        let messages = messages::get_message::get_connection_messages(&self.pw_did,
                                                                      &self.pw_vk,
                                                                      &self.agent_did,
                                                                      &self.agent_vk,
                                                                      None,
                                                                      Some(vec![MessageStatusCode::Received]))?;

        let mut uids: Vec<String> = Vec::new();

        for message in messages {
            let uid = message.uid.clone();
            self.handle_message(message)?;
            uids.push(uid);
        }

        update_messages(MessageStatusCode::Reviewed, vec![UIDsByConn { pairwise_did: self.pw_did.clone(), uids }])?;

        Ok(error::SUCCESS.code_num)
    }

    fn handle_message(&mut self, message: Message) -> VcxResult<u32> {
        let message = EncryptionEnvelope::open(&self.pw_vk, message.payload()?)?;
        match (&self.state, &self.actor) {
            (DidExchangeState::Invited(ref state), Actor::Inviter) => {
                match message {
                    A2AMessage::ConnectionRequest(request) => {
                        let thread = Thread::new().set_thid(request.id.0.clone());

                        if let Err(err) = self.handle_connection_request(request) {
                            self.send_problem_report(ProblemCode::RequestProcessingError, err.to_string(), thread)?
                        }
                    }
                    _ => {}
                }
            }
            (DidExchangeState::Requested(ref state), Actor::Invitee) => {
                match message {
                    A2AMessage::ConnectionResponse(response) => {
                        let thread = response.thread.clone();

                        if let Err(err) = self.handle_connection_response(response) {
                            self.send_problem_report(ProblemCode::ResponseProcessingError, err.to_string(), thread)?
                        }
                    }
                    _ => {}
                }
            }
            (DidExchangeState::Responded(ref state), _) => {
                match message {
                    A2AMessage::Ack(ack) => {
                        self.handle_ack(ack)?;
                    }
                    A2AMessage::ProblemReport(problem_report) => {
                        self.handle_problem_report(problem_report)?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(error::SUCCESS.code_num)
    }

    fn handle_connection_request(&mut self, request: Request) -> VcxResult<()> {
        request.connection.did_doc.validate()?;

        let thread = Thread::new().set_thid(request.id.0.clone());

        self.step(Messages::ExchangeRequestReceived(request));

        // original Verkey need to sign rotated keys
        let current_vk = self.pw_vk.clone();

        // provision a new keys
        self.provision_did()?;

        let response = Response::create()
            .set_did(self.pw_did.to_string())
            .set_service_endpoint(self.agency_endpoint()?)
            .set_recipient_keys(self.recipient_keys())
            .set_routing_keys(self.routing_keys()?)
            .set_thread(thread);

        self.send_message(&response.encode(&current_vk)?.to_a2a_message())?;

        self.step(Messages::ExchangeResponseSent(response));

        Ok(())
    }

    fn handle_connection_response(&mut self, response: SignedResponse) -> VcxResult<()> {
        let response: Response = response.decode(&self.remote_vk()?)?;

        let thread = response.thread.clone();

        self.step(Messages::ExchangeResponseReceived(response));

        let ack = Ack::create().set_thread(thread);

        self.send_message(&ack.to_a2a_message())?;

        self.step(Messages::AckSent(ack));

        Ok(())
    }

    fn handle_ack(&mut self, ack: Ack) -> VcxResult<()> {
        self.step(Messages::AckReceived(ack));
        Ok(())
    }

    fn handle_problem_report(&mut self, problem_report: ProblemReport) -> VcxResult<()> {
        self.step(Messages::Error(problem_report));
        Ok(())
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

    fn send_problem_report(&mut self, problem_code: ProblemCode, explain: String, thread: Thread) -> VcxResult<()> {
        let problem_report = ProblemReport::create()
            .set_problem_code(problem_code)
            .set_explain(explain)
            .set_thread(thread);

        self.send_message(&problem_report.to_a2a_message())?;

        self.step(Messages::Error(problem_report));

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

        self.pw_did = pw_did;
        self.pw_vk = pw_vk;
        self.agent_did = agent_did;
        self.agent_vk = agent_vk;

        Ok(())
    }

    pub fn delete(&mut self) -> VcxResult<u32> {
        send_delete_connection_message(&self.pw_did, &self.pw_vk, &self.agent_did, &self.agent_vk)?;
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

pub fn connect(handle: u32, _options: Option<String>) -> VcxResult<()> {
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
        connection.get_invite()
    })
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