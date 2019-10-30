use serde_json;

use messages::ObjectWithVersion;
use messages::get_message::Message;
use error::prelude::*;
use utils::error;
use utils::libindy::anoncreds;
use disclosed_proof::DisclosedProof;

use v3::handlers::proof_presentation::prover::states::{ProverSM, ProverState, ProverMessages};

use v3::handlers::connection;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use v3::messages::proof_presentation::presentation::Presentation;
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

    pub fn generate_presentation(&mut self, credentials: &str, self_attested_attrs: &str) -> VcxResult<u32> {
        let presentation_request: &PresentationRequest = self.state.presentation_request();

        let presentation_request = {
            let presentation_req_data_json = presentation_request.request_presentations_attach.content()?;

            serde_json::from_str(&presentation_req_data_json)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize PresentationRequest: {:?}", err)))?
        };

        let presentation = DisclosedProof::generate_indy_proof(credentials, self_attested_attrs, &presentation_request)?;

        let presentation = Presentation::create()
            .set_presentations_attach(presentation)?;

        self.step(ProverMessages::PresentationPrepared(presentation))?;

        Ok(error::SUCCESS.code_num)
    }

    pub fn generate_presentation_msg(&self) -> VcxResult<String> {
        let presentation: &Presentation = self.state.presentation()?;

        let presentation = serde_json::to_string(&presentation)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize Presentation: {}", err)))?;

        Ok(presentation)
    }

    pub fn send_proof(&mut self, connection_handle: u32) -> VcxResult<u32> {
        self.step(ProverMessages::SendPresentation(connection_handle))?;
        return Ok(error::SUCCESS.code_num);
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        if !self.state.has_transitions() {
            return Ok(self.state());
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
                    .map(|(uid, message)| self.handle_message(uid, message))
                    .collect::<VcxResult<Vec<Option<String>>>>()?
                    .into_iter()
                    .filter_map(|e| e)
                    .collect::<Vec<String>>();

                connection::update_messages(connection_handle, uids)?;
            }
        }

        Ok(error::SUCCESS.code_num)
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
            ProverState::PresentationSent(ref state) => {
                // TODO: FIXME better way of reply check
                let thread = match message {
                    A2AMessage::Ack(ref ack) => &ack.thread,
                    A2AMessage::CommonProblemReport(ref presentation) => &presentation.thread,
                    _ => { return Ok(None); }
                };

                if !thread.is_reply(&state.presentation.thread.thid.clone().unwrap_or_default()) {
                    return Ok(None);
                }

                match message {
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
                // do not process messages
            }
        };

        Ok(None)
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