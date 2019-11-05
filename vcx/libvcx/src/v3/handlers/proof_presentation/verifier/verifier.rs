use error::prelude::*;
use std::convert::TryInto;

use messages::ObjectWithVersion;
use messages::get_message::Message;

use v3::handlers::connection;
use v3::messages::A2AMessage;
use v3::messages::proof_presentation::presentation_request::*;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::handlers::proof_presentation::verifier::states::{VerifierSM, VerifierState, VerifierMessages};

use messages::proofs::proof_request::ProofRequestMessage;
use messages::proofs::proof_message::ProofMessage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Verifier {
    source_id: String,
    state: VerifierSM
}

impl Verifier {
    const SERIALIZE_VERSION: &'static str = "2.0";

    pub fn create(source_id: String,
                  requested_attrs: String,
                  requested_predicates: String,
                  revocation_details: String,
                  name: String) -> VcxResult<Verifier> {
        trace!("Verifier::create >>> source_id: {:?}, requested_attrs: {:?}, requested_predicates: {:?}, revocation_details: {:?}, name: {:?}",
               source_id, requested_attrs, requested_predicates, revocation_details, name);

        let presentation_request =
            PresentationRequestData::create()
                .set_name(name)
                .set_requested_attributes(requested_attrs)?
                .set_requested_predicates(requested_predicates)?
                .set_not_revoked_interval(revocation_details)?
                .set_nonce()?;

        Ok(Verifier {
            source_id,
            state: VerifierSM::new(presentation_request),
        })
    }

    pub fn get_source_id(&self) -> String { self.source_id.clone() }

    pub fn state(&self) -> u32 { self.state.state() }

    pub fn presentation_state(&self) -> u32 {
        self.state.presentation_status()
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        trace!("Verifier::update_state >>> message: {:?}", message);

        if !self.state.has_transitions() { return Ok(()); }

        match message {
            Some(message_) => {
                self.update_state_with_message(message_)?;
            }
            None => {
                let connection_handle = self.state.connection_handle()?;

                let (messages, _) = connection::get_messages(connection_handle)?;

                for (uid, message) in messages {
                    match self.handle_message(message)? {
                        Some(_) => {
                            connection::update_message_status(connection_handle, uid)?;
                            break;
                        }
                        None => {}
                    }
                }
            }
        }

        Ok(())
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let message = connection::decode_message(self.state.connection_handle()?, message)?;

        self.handle_message(message)?;

        Ok(())
    }

    pub fn handle_message(&mut self, message: A2AMessage) -> VcxResult<Option<()>> {
        trace!("Verifier::handle_message >>> message: {:?}", message);
        let thid = &self.state.presentation_request()?.id.0;

        match self.state.state {
            VerifierState::Initiated(ref state) => {
                // do not process message
            }
            VerifierState::PresentationRequestSent(ref state) => {
                match message {
                    A2AMessage::Presentation(presentation) => {
                        if presentation.thread.is_reply(&thid) {
                            if let Err(err) = self.verify_presentation(presentation) {
                                self.send_problem_report(err)?
                            }
                            return Ok(Some(()));
                        }
                    }
                    A2AMessage::PresentationProposal(proposal) => {
                        if proposal.thread.is_reply(&thid) {
                            self.step(VerifierMessages::PresentationProposalReceived(proposal))?;
                            return Ok(Some(()));
                        }
                    }
                    A2AMessage::CommonProblemReport(problem_report) => {
                        if problem_report.thread.is_reply(&thid) {
                            self.step(VerifierMessages::PresentationRejectReceived(problem_report))?;
                            return Ok(Some(()));
                        }
                    }
                    _ => {}
                }
            }
            VerifierState::Finished(ref state) => {
                // do not process message
            }
        };

        Ok(None)
    }

    pub fn verify_presentation(&mut self, presentation: Presentation) -> VcxResult<()> {
        trace!("Verifier::verify_presentation >>> presentation: {:?}", presentation);
        self.step(VerifierMessages::VerifyPresentation(presentation))
    }

    pub fn send_problem_report(&mut self, err: VcxError) -> VcxResult<()> {
        trace!("Verifier::send_problem_report >>> err: {:?}", err);
        self.step(VerifierMessages::SendPresentationReject(err.to_string()))
    }

    pub fn send_presentation_request(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Verifier::send_presentation_request >>> connection_handle: {:?}", connection_handle);
        self.step(VerifierMessages::SendPresentationRequest(connection_handle))
    }

    pub fn generate_proof_request_msg(&mut self) -> VcxResult<String> {
        trace!("Verifier::generate_proof_request_msg >>>");

        let proof_request: ProofRequestMessage = self.state.presentation_request()?.try_into()?;

        ::serde_json::to_string(&proof_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn get_proof(&self) -> VcxResult<String> {
        trace!("Verifier::get_proof >>>");

        let proof: ProofMessage = self.state.presentation()?.try_into()?;

        ::serde_json::to_string(&proof)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    pub fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    pub fn step(&mut self, message: VerifierMessages) -> VcxResult<()> {
        self.state = self.state.clone().step(message)?;
        Ok(())
    }
}