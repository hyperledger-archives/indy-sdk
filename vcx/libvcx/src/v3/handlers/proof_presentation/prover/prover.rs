use error::prelude::*;
use utils::libindy::anoncreds;
use std::convert::TryInto;

use v3::handlers::proof_presentation::prover::states::ProverSM;
use v3::handlers::proof_presentation::prover::messages::ProverMessages;
use v3::messages::a2a::A2AMessage;
use v3::messages::proof_presentation::presentation_proposal::PresentationPreview;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use ::{connection, settings};

use messages::proofs::proof_message::ProofMessage;

use v3::messages::proof_presentation::presentation::Presentation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prover {
    prover_sm: ProverSM
}

impl Prover {
    pub fn create(source_id: &str, presentation_request: PresentationRequest) -> VcxResult<Prover> {
        trace!("Prover::create >>> source_id: {}, presentation_request: {:?}", source_id, presentation_request);
        Ok(Prover {
            prover_sm: ProverSM::new(presentation_request, source_id.to_string()),
        })
    }

    pub fn state(&self) -> u32 { self.prover_sm.state() }

    pub fn presentation_status(&self) -> u32 {
        trace!("Prover::presentation_state >>>");
        self.prover_sm.presentation_status()
    }

    pub fn retrieve_credentials(&self) -> VcxResult<String> {
        trace!("Prover::retrieve_credentials >>>");
        let presentation_request = self.prover_sm.presentation_request().request_presentations_attach.content()?;
        anoncreds::libindy_prover_get_credentials_for_proof_req(&presentation_request)
    }

    pub fn generate_presentation(&mut self, credentials: String, self_attested_attrs: String) -> VcxResult<()> {
        trace!("Prover::generate_presentation >>> credentials: {}, self_attested_attrs: {:?}", credentials, self_attested_attrs);
        self.step(ProverMessages::PreparePresentation((credentials, self_attested_attrs)))
    }

    pub fn generate_presentation_msg(&self) -> VcxResult<String> {
        trace!("Prover::generate_presentation_msg >>>");

        let proof = self.prover_sm.presentation()?.to_owned();

        // strict aries protocol is set. return aries formatted Proof
        if settings::is_strict_aries_protocol_set() {
            return Ok(json!(proof).to_string())
        }

        // convert Proof into proprietary format
        let proof: ProofMessage = proof.try_into()?;

        return Ok(json!(proof).to_string())
    }

    pub fn set_presentation(&mut self, presentation: Presentation) -> VcxResult<()> {
        trace!("Prover::set_presentation >>>");
        self.step(ProverMessages::SetPresentation(presentation))
    }

    pub fn send_presentation(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Prover::send_presentation >>>");
        self.step(ProverMessages::SendPresentation(connection_handle))
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        trace!("Prover::update_state >>> message: {:?}", message);

        if !self.prover_sm.has_transitions() { return Ok(()); }

        if let Some(message_) = message {
            return self.update_state_with_message(message_);
        }

        let connection_handle = self.prover_sm.connection_handle()?;
        let messages = connection::get_messages(connection_handle)?;

        if let Some((uid, message)) = self.prover_sm.find_message_to_handle(messages) {
            self.handle_message(message.into())?;
            connection::update_message_status(connection_handle, uid)?;
        };

        Ok(())
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Prover::update_state_with_message >>> message: {:?}", message);

        let a2a_message: A2AMessage = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot updated state with message: Message deserialization failed: {:?}", err)))?;

        self.handle_message(a2a_message.into())?;

        Ok(())
    }

    pub fn handle_message(&mut self, message: ProverMessages) -> VcxResult<()> {
        trace!("Prover::handle_message >>> message: {:?}", message);
        self.step(message)
    }

    pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<PresentationRequest> {
        trace!("Prover::get_presentation_request >>> connection_handle: {:?}, msg_id: {:?}", connection_handle, msg_id);

        let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

        let presentation_request: PresentationRequest = match message {
            A2AMessage::PresentationRequest(presentation_request) => presentation_request,
            msg => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages,
                                              format!("Message of different type was received: {:?}", msg)));
            }
        };

        Ok(presentation_request)
    }

    pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<Vec<PresentationRequest>> {
        trace!("Prover::get_presentation_request_messages >>> connection_handle: {:?}, match_name: {:?}", connection_handle, match_name);

        let presentation_requests: Vec<PresentationRequest> =
            connection::get_messages(connection_handle)?
                .into_iter()
                .filter_map(|(_, message)| {
                    match message {
                        A2AMessage::PresentationRequest(presentation_request) => {
                            Some(presentation_request)
                        }
                        _ => None,
                    }
                })
                .collect();

        Ok(presentation_requests)
    }

    pub fn get_source_id(&self) -> String { self.prover_sm.source_id() }

    pub fn step(&mut self, message: ProverMessages) -> VcxResult<()> {
        self.prover_sm = self.prover_sm.clone().step(message)?;
        Ok(())
    }

    pub fn decline_presentation_request(&mut self, connection_handle: u32, reason: Option<String>, proposal: Option<String>) -> VcxResult<()> {
        trace!("Prover::decline_presentation_request >>> connection_handle: {}, reason: {:?}, proposal: {:?}", connection_handle, reason, proposal);
        match (reason, proposal) {
            (Some(reason), None) => {
                self.step(ProverMessages::RejectPresentationRequest((connection_handle, reason)))
            }
            (None, Some(proposal)) => {
                let presentation_preview: PresentationPreview = serde_json::from_str(&proposal)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize Presentation Preview: {:?}", err)))?;

                self.step(ProverMessages::ProposePresentation((connection_handle, presentation_preview)))
            }
            (None, None) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidOption, "Either `reason` or `proposal` parameter must be specified."));
            }
            (Some(_), Some(_)) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidOption, "Only one of `reason` or `proposal` parameters must be specified."));
            }
        }
    }
}