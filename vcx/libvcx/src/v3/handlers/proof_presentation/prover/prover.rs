use serde_json;

use messages::ObjectWithVersion;
use messages::get_message::Message;
use error::prelude::*;
use utils::libindy::anoncreds;

use v3::handlers::proof_presentation::prover::states::{ProverSM, ProverState, ProverMessages};

use v3::handlers::connection;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use v3::messages::A2AMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prover {
    source_id: String,
    state: ProverSM
}

impl Prover {
    const SERIALIZE_VERSION: &'static str = "2.0";

    pub fn create(source_id: &str, presentation_request: &str) -> VcxResult<Prover> {
        let presentation_request: PresentationRequest = serde_json::from_str(presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize PresentationRequest: {}", err)))?;

        Ok(Prover {
            source_id: source_id.to_string(),
            state: ProverSM::new(presentation_request),
        })
    }

    pub fn state(&self) -> u32 { self.state.state() }

    pub fn presentation_state(&self) -> u32 { self.state.presentation_status() }

    pub fn retrieve_credentials(&self) -> VcxResult<String> {
        let presentation_request = self.state.presentation_request().request_presentations_attach.content()?;
        anoncreds::libindy_prover_get_credentials_for_proof_req(&presentation_request)
    }

    pub fn generate_presentation(&mut self, credentials: String, self_attested_attrs: String) -> VcxResult<()> {
        match self.step(ProverMessages::PreparePresentation((credentials, self_attested_attrs))) {
            Err(err) => {
                self.step(ProverMessages::PreparePresentationFail(err.to_string()))
            }
            Ok(_) => Ok(())
        }
    }

    pub fn generate_presentation_msg(&self) -> VcxResult<String> {
        self.state.presentation()?.to_json()
    }

    pub fn send_presentation(&mut self, connection_handle: u32) -> VcxResult<()> {
        self.step(ProverMessages::SendPresentation(connection_handle))
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        if !self.state.has_transitions() { return Ok(()); }

        match message {
            Some(message_) => {
                self.update_state_with_message(message_)?;
            }
            None => {
                let connection_handle = self.state.connection_handle()?;

                let (messages, _) = connection::get_messages(connection_handle)?;

                let uids = messages
                    .into_iter()
                    .map(|(uid, message)| self.handle_message(uid, message))
                    .collect::<VcxResult<Vec<Option<String>>>>()?
                    .into_iter()
                    .filter_map(|e| e)
                    .collect::<Vec<String>>();

                connection::update_messages(connection_handle, uids)?;
            }
        }

        Ok(())
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let uid = message.uid.clone();
        let message = connection::decode_message(self.state.connection_handle()?, message)?;

        self.handle_message(uid, message)?;

        Ok(())
    }

    pub fn handle_message(&mut self, uid: String, message: A2AMessage) -> VcxResult<Option<String>> {
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
                let thid = state.presentation.thread.thid.clone().unwrap_or_default();
                match message {
                    A2AMessage::Ack(ack) => {
                        if ack.thread.is_reply(&thid) {
                            self.step(ProverMessages::PresentationAckReceived(ack))?;
                            return Ok(Some(uid));
                        }
                    }
                    A2AMessage::CommonProblemReport(problem_report) => {
                        if problem_report.thread.is_reply(&thid) {
                            self.step(ProverMessages::PresentationRejectReceived(problem_report))?;
                            return Ok(Some(uid));
                        }
                    }
                    _ => {}
                }
            }
            ProverState::Finished(ref state) => {
                // do not process messages
            }
        };

        Ok(None)
    }

    pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
        let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

        let presentation_request = match message {
            A2AMessage::PresentationRequest(ref presentation_request) => presentation_request,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Message has different type"))
        };

        serde_json::to_string_pretty(presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize message: {}", err)))
    }

    pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<String> {
        let (messages, _) = connection::get_messages(connection_handle)?;

        let presentation_requests: Vec<PresentationRequest> =
            messages
                .into_iter()
                .filter_map(|(_, message)| {
                    match message {
                        A2AMessage::PresentationRequest(presentation_request) => Some(presentation_request),
                        _ => None,
                    }
                })
                .collect();

        serde_json::to_string_pretty(&presentation_requests)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize proof request: {}", err)))
    }

    pub fn get_source_id(&self) -> String { self.source_id.clone() }

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

    pub fn step(&mut self, message: ProverMessages) -> VcxResult<()> {
        self.state = self.state.clone().step(message)?;
        Ok(())
    }
}