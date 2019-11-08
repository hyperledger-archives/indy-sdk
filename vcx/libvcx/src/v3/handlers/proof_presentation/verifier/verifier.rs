use error::prelude::*;
use std::convert::TryInto;
use std::collections::HashMap;

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
            state: VerifierSM::new(presentation_request, source_id),
        })
    }

    pub fn get_source_id(&self) -> String { self.state.source_id() }

    pub fn state(&self) -> u32 {
        trace!("Verifier::state >>>");
        self.state.state()
    }

    pub fn presentation_status(&self) -> u32 {
        trace!("Verifier::presentation_state >>>");
        self.state.presentation_status()
    }

    pub fn update_state(mut self, message: Option<&str>) -> VcxResult<Verifier> {
        trace!("Verifier::update_state >>> message: {:?}", message);

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

    pub fn update_state_with_message(mut self, message: &str) -> VcxResult<Verifier> {
        trace!("Verifier::update_state_with_message >>> message: {:?}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;

        let connection_handle = self.state.connection_handle()?;

        let messages: HashMap<String, A2AMessage> = map!{ message.uid.clone() => connection::decode_message(connection_handle, message)? };

        if let Some((uid, message)) = self.find_message_to_handle(messages) {
            self = self.handle_message(message)?;
            connection::update_message_status(connection_handle, uid)?;
        }

        Ok(self)
    }

    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, VerifierMessages)> {
        trace!("Verifier::find_message_to_handle >>> messages: {:?}", messages);

        for (uid, message) in messages {
            match self.state.state {
                VerifierState::Initiated(ref state) => {
                    // do not process message
                }
                VerifierState::PresentationRequestSent(ref state) => {
                    match message {
                        A2AMessage::Presentation(presentation) => {
                            if presentation.thread.is_reply(&self.state.thread_id()) {
                                return Some((uid, VerifierMessages::VerifyPresentation(presentation)));
                            }
                        }
                        A2AMessage::PresentationProposal(proposal) => {
                            if proposal.thread.is_reply(&self.state.thread_id()) {
                                return Some((uid, VerifierMessages::PresentationProposalReceived(proposal)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) => {
                            if problem_report.thread.is_reply(&self.state.thread_id()) {
                                return Some((uid, VerifierMessages::PresentationRejectReceived(problem_report)));
                            }
                        }
                        _ => {}
                    }
                }
                VerifierState::Finished(ref state) => {
                    // do not process message
                }
            };
        }

        None
    }

    pub fn handle_message(self, message: VerifierMessages) -> VcxResult<Verifier> {
        trace!("Verifier::handle_message >>> message: {:?}", message);
        self.step(message)
    }

    pub fn verify_presentation(self, presentation: Presentation) -> VcxResult<Verifier> {
        trace!("Verifier::verify_presentation >>> presentation: {:?}", presentation);
        self.step(VerifierMessages::VerifyPresentation(presentation))
    }

    pub fn send_presentation_request(self, connection_handle: u32) -> VcxResult<Verifier> {
        trace!("Verifier::send_presentation_request >>> connection_handle: {:?}", connection_handle);
        self.step(VerifierMessages::SendPresentationRequest(connection_handle))
    }

    pub fn generate_presentation_request_msg(&self) -> VcxResult<String> {
        trace!("Verifier::generate_presentation_request_msg >>>");

        let proof_request: ProofRequestMessage = self.state.presentation_request()?.try_into()?;

        ::serde_json::to_string(&proof_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn get_presentation(&self) -> VcxResult<String> {
        trace!("Verifier::get_presentation >>>");

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

    pub fn step(mut self, message: VerifierMessages) -> VcxResult<Verifier> {
        self.state = self.state.step(message)?;
        Ok(self)
    }
}