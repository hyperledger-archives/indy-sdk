pub mod states;

use serde_json;

use object_cache::ObjectCache;
use messages::{ObjectWithVersion, MessageStatusCode};
use messages::get_message::Message;
use error::prelude::*;
use utils::error;
use utils::libindy::anoncreds;
use messages::update_message::{UIDsByConn, update_messages};

use v3::handlers::connection;
use self::states::{ProverSM, ProverState, ProverMessages};
use v3::messages::proof_presentation::presentation_request::{PresentationRequest, PresentationRequestData};
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::A2AMessage;

use disclosed_proof::DisclosedProof;

lazy_static! {
    static ref PROVER_MAP: ObjectCache<Prover>  = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prover {
    source_id: String,
    state: ProverSM
}

impl Prover {
    const SERIALIZE_VERSION: &'static str = "2.0";

    pub fn create(source_id: &str, presentation_request: &str) -> VcxResult<Prover> {
        let presentation_request: PresentationRequestData = serde_json::from_str(presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize PresentationRequest: {}", err)))?;

        Ok(Prover {
            source_id: source_id.to_string(),
            state: ProverSM::new(presentation_request),
        })
    }

    fn state(&self) -> u32 { self.state.state() }

    fn presentation_state(&self) -> u32 { self.state.presentation_state() }

    fn retrieve_credentials(&self) -> VcxResult<String> {
        let presentation_request = self.state.presentation_request()?;

        let presentation_request_json = serde_json::to_string(presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize PresentationRequest: {}", err)))?;

        anoncreds::libindy_prover_get_credentials_for_proof_req(&presentation_request_json)
    }

    fn generate_proof(&mut self, credentials: &str, self_attested_attrs: &str) -> VcxResult<u32> {
        let presentation_request = self.state.presentation_request()?;

        let presentation = DisclosedProof::generate_indy_proof(credentials, self_attested_attrs, &presentation_request)?;

        let presentation = Presentation::create()
            .set_presentations_attach(presentation)?;

        self.step(ProverMessages::PresentationPrepared(presentation))?;

        Ok(error::SUCCESS.code_num)
    }

    fn generate_proof_msg(&self) -> VcxResult<String> {
        let presentation: &Presentation = self.state.presentation()?;

        let presentation = serde_json::to_string(&presentation)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize Presentation: {}", err)))?;

        Ok(presentation)
    }

    fn send_proof(&mut self, connection_handle: u32) -> VcxResult<u32> {
        let presentation = self.generate_proof_msg()?;

        self.step(ProverMessages::SendPresentation(connection_handle))?;

        return Ok(error::SUCCESS.code_num);
    }

    fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        trace!("Verifier: update_state");

        match self.state.state {
            ProverState::Initiated(_) => return Ok(self.state()),
            ProverState::PresentationPrepared(_) => {}
            ProverState::PresentationSent(_) => {}
            ProverState::Finished(_) => return Ok(self.state()),
        }

        match message {
            Some(message_) => {
                self.update_state_with_message(message_)?;
            }
            None => {
                let connection_handle = self.state.connection_handle()?;

                let (messages, _) = connection::get_messages(connection_handle)?;

                let uids = messages
                    .into_iter()
                    .map(|message| self.handle_message(message))
                    .collect::<VcxResult<Vec<Option<String>>>>()?
                    .into_iter()
                    .filter_map(|e| e)
                    .collect::<Vec<String>>();

                let messages_to_update = vec![UIDsByConn {
                    pairwise_did: connection::get_pw_did(connection_handle)?,
                    uids
                }];

                update_messages(MessageStatusCode::Reviewed, messages_to_update)?;
            }
        }

        Ok(error::SUCCESS.code_num)
    }

    fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        self.handle_message(message)?;

        Ok(())
    }

    fn handle_message(&mut self, message: Message) -> VcxResult<Option<String>> {
        let uid = message.uid.clone();

        let a2a_message = connection::decode_message(self.state.connection_handle()?, message)?;

        match self.state.state {
            ProverState::Initiated(ref state) => {
                match a2a_message {
                    A2AMessage::PresentationRequest(presentation_request) => {
                        // ignore it here??
                    }
                    _ => {}
                }
            }
            ProverState::PresentationPrepared(_) => {}
            ProverState::PresentationSent(ref state) => {
                match a2a_message {
                    A2AMessage::Ack(ack) => {
                        self.step(ProverMessages::PresentationAckReceived(ack))?;
                        return Ok(Some(uid));
                    }
                    A2AMessage::CommonProblemReport(problem_report) => {
                        self.step(ProverMessages::PresentationRejectReceived(problem_report))?;
                        return Ok(Some(uid));
                    }
                    _ => {}
                }
            }
            ProverState::Finished(ref state) => {
                // do not process a2a_message
            }
        };

        Ok(None)
    }

    fn get_source_id(&self) -> String { self.source_id.clone() }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize DisclosedProof"))
    }
    fn from_str(data: &str) -> VcxResult<Prover> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Prover>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Prover"))
    }

    fn step(&mut self, message: ProverMessages) -> VcxResult<()> {
        self.state = self.state.clone().step(message)?;
        Ok(())
    }
}

pub fn create_proof(source_id: &str, presentation_request: &str) -> VcxResult<u32> {
    let prover: Prover = Prover::create(source_id, presentation_request)?;
    PROVER_MAP.add(prover)
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get(handle, |obj| {
        Ok(obj.state())
    })
}

// update_state is just the same as get_state for disclosed_proof
pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.update_state(message.as_ref().map(String::as_str))
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        Prover::to_string(&prover)
    })
}

pub fn from_string(prover_data: &str) -> VcxResult<u32> {
    let prover: Prover = Prover::from_str(prover_data)?;
    PROVER_MAP.add(prover)
}

pub fn release(handle: u32) -> VcxResult<()> {
    PROVER_MAP.release(handle)
}

pub fn release_all() {
    PROVER_MAP.drain().ok();
}

pub fn generate_proof_msg(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.generate_proof_msg()
    })
}

pub fn send_proof(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.send_proof(connection_handle)
    })
}

pub fn generate_proof(handle: u32, credentials: String, self_attested_attrs: String) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.generate_proof(&credentials, &self_attested_attrs)
    })
}

pub fn retrieve_credentials(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.retrieve_credentials()
    })
}

pub fn is_valid_handle(handle: u32) -> bool {
    PROVER_MAP.has_handle(handle)
}

pub fn get_proof_request(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
    let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

    match message {
        A2AMessage::PresentationRequest(presentation_request) => {
            serde_json::to_string_pretty(&presentation_request)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize message: {}", err)))
        }
        _ => Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Message has different type"))
    }
}

pub fn get_proof_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<String> {
    let (messages, _) = connection::get_messages(connection_handle)?;

    let mut presentation_requests: Vec<PresentationRequest> = Vec::new();

    for message in messages {
        let message = connection::decode_message(connection_handle, message)?;

        if let A2AMessage::PresentationRequest(presentation_request) = message {
            presentation_requests.push(presentation_request)
        }
    }

    serde_json::to_string_pretty(&presentation_requests)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize proof request: {}", err)))
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        Ok(prover.get_source_id().clone())
    })
}
