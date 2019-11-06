use serde_json;

use messages::ObjectWithVersion;
use messages::get_message::Message;
use error::prelude::*;
use utils::libindy::anoncreds;
use std::convert::TryInto;
use std::collections::HashMap;

use v3::handlers::proof_presentation::prover::states::{ProverSM, ProverState, ProverMessages};

use v3::handlers::connection;
use v3::messages::A2AMessage;

use messages::proofs::proof_request::ProofRequestMessage;
use messages::proofs::proof_message::ProofMessage;

use v3::messages::MessageId;
use std::sync::Mutex;

lazy_static! {
    pub static ref PENDING_PRESENTATION_REQUESTS: Mutex<HashMap<MessageId, String>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prover {
    state: ProverSM
}

impl Prover {
    const SERIALIZE_VERSION: &'static str = "2.0";

    pub fn create(source_id: &str, presentation_request: &str) -> VcxResult<Prover> {
        trace!("Prover::create >>> source_id: {}, presentation_request: {:?}", source_id, presentation_request);

        let proof_request_message: ProofRequestMessage = serde_json::from_str(presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize PresentationRequest: {}", err)))?;

        Ok(Prover {
            state: ProverSM::new(proof_request_message.try_into()?, source_id.to_string()),
        })
    }

    pub fn state(&self) -> u32 { self.state.state() }

    pub fn presentation_state(&self) -> u32 {
        trace!("Prover::presentation_state >>>");
        self.state.presentation_status()
    }

    pub fn retrieve_credentials(&self) -> VcxResult<String> {
        trace!("Prover::retrieve_credentials >>>");
        let presentation_request = self.state.presentation_request().request_presentations_attach.content()?;
        anoncreds::libindy_prover_get_credentials_for_proof_req(&presentation_request)
    }

    pub fn generate_presentation(self, credentials: String, self_attested_attrs: String) -> VcxResult<Prover> {
        trace!("Prover::generate_presentation >>> credentials: {}, self_attested_attrs: {:?}", credentials, self_attested_attrs);
        self.step(ProverMessages::PreparePresentation((credentials, self_attested_attrs)))
    }

    pub fn generate_presentation_msg(&self) -> VcxResult<String> {
        trace!("Prover::generate_presentation_msg >>>");

        let proof: ProofMessage = self.state.presentation()?.clone().try_into()?;

        ::serde_json::to_string(&proof)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn send_presentation(self, connection_handle: u32) -> VcxResult<Prover> {
        trace!("Prover::send_presentation >>>");
        self.step(ProverMessages::SendPresentation(connection_handle))
    }

    pub fn update_state(mut self, message: Option<&str>) -> VcxResult<Prover> {
        trace!("Prover::update_state >>> message: {:?}", message);

        if !self.state.has_transitions() { return Ok(self); }

        match message {
            Some(message_) => {
                self = self.update_state_with_message(message_)?
            }
            None => {
                let connection_handle = self.state.connection_handle()?;
                let messages = connection::get_messages(connection_handle)?;

                if let Some((uid, message)) = self.find_message_to_handle(messages) {
                    self = self.handle_message(message)?;
                    connection::update_message_status(connection_handle, uid)?;
                };
            }
        };

        Ok(self)
    }

    pub fn update_state_with_message(mut self, message: &str) -> VcxResult<Prover> {
        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let connection_handle = self.state.connection_handle()?;

        let mut messages: HashMap<String, A2AMessage> = HashMap::new();
        messages.insert(message.uid.clone(), connection::decode_message(connection_handle, message)?);

        if let Some((uid, message)) = self.find_message_to_handle(messages) {
            self = self.handle_message(message)?;
            connection::update_message_status(connection_handle, uid)?;
        }

        Ok(self)
    }

    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, ProverMessages)> {
        trace!("Prover::get_message_to_handle >>> messages: {:?}", messages);

        let thid = self.state.presentation_request().id.0.clone();

        for (uid, message) in messages {
            match self.state.state {
                ProverState::Initiated(ref state) => {
                    match message {
                        A2AMessage::PresentationRequest(presentation_request) => {
                            // ignore it here??
                        }
                        _ => {}
                    }
                }
                ProverState::PresentationPrepared(_) => {
                    // do not process messages
                }
                ProverState::PresentationPreparationFailed(_) => {
                    // do not process messages
                }
                ProverState::PresentationSent(ref state) => {
                    match message {
                        A2AMessage::Ack(ack) => {
                            if ack.thread.is_reply(&thid) {
                                return Some((uid, ProverMessages::PresentationAckReceived(ack)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) => {
                            if problem_report.thread.is_reply(&thid) {
                                return Some((uid, ProverMessages::PresentationRejectReceived(problem_report)));
                            }
                        }
                        _ => {}
                    }
                }
                ProverState::Finished(ref state) => {
                    // do not process messages
                }
            };
        }

        None
    }

    pub fn handle_message(self, message: ProverMessages) -> VcxResult<Prover> {
        trace!("Prover::handle_message >>> message: {:?}", message);
        self.step(message)
    }

    pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
        trace!("Prover::get_presentation_request >>> connection_handle: {:?}, msg_id: {:?}", connection_handle, msg_id);

        let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

        let (id, presentation_request): (MessageId, ProofRequestMessage) = match message {
            A2AMessage::PresentationRequest(presentation_request) => (presentation_request.id.clone(), presentation_request.try_into()?),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Message has different type"))
        };

        let mut pending_messages: HashMap<MessageId, String> = HashMap::new();
        pending_messages.insert(id, msg_id.to_string());

        connection::add_pending_messages(connection_handle, pending_messages)?;

        serde_json::to_string_pretty(&presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize message: {}", err)))
    }

    pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<String> {
        trace!("Prover::get_presentation_request_messages >>> connection_handle: {:?}, match_name: {:?}", connection_handle, match_name);

        let (uids, presentation_requests): (HashMap<MessageId, String>, Vec<ProofRequestMessage>) =
            connection::get_messages(connection_handle)?
                .into_iter()
                .filter_map(|(uid, message)| {
                    match message {
                        A2AMessage::PresentationRequest(presentation_request) => {
                            let id = presentation_request.id.clone();
                            match presentation_request.try_into() {
                                Ok(proof_request) => Some((uid, id, proof_request)),
                                Err(_) => None
                            }
                        },
                        _ => None,
                    }
                }).fold((HashMap::new(), Vec::new()), |(mut uids, mut messages), (uid, id, presentation_request)| {
                    uids.insert(id, uid);
                    messages.push(presentation_request);
                    (uids, messages)
                });

        connection::add_pending_messages(connection_handle, uids)?;

        serde_json::to_string_pretty(&presentation_requests)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize proof request: {}", err)))
    }

    pub fn get_source_id(&self) -> String { self.state.source_id() }

    pub fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize DisclosedProof"))
    }

    pub fn from_str(data: &str) -> VcxResult<Prover> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Prover>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Prover"))
    }

    pub fn step(mut self, message: ProverMessages) -> VcxResult<Prover> {
        self.state = self.state.step(message)?;
        Ok(self)
    }
}