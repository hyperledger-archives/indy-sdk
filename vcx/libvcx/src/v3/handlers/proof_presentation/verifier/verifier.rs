use error::prelude::*;
use std::convert::TryInto;

use messages::get_message::Message;

use connection;
use v3::messages::proof_presentation::presentation_request::*;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::handlers::proof_presentation::verifier::states::VerifierSM;
use v3::handlers::proof_presentation::verifier::messages::VerifierMessages;

use messages::proofs::proof_request::ProofRequestMessage;
use messages::proofs::proof_message::ProofMessage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Verifier {
    verifier_sm: VerifierSM
}

impl Verifier {
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
            verifier_sm: VerifierSM::new(presentation_request, source_id),
        })
    }

    pub fn get_source_id(&self) -> String { self.verifier_sm.source_id() }

    pub fn state(&self) -> u32 {
        trace!("Verifier::state >>>");
        self.verifier_sm.state()
    }

    pub fn presentation_status(&self) -> u32 {
        trace!("Verifier::presentation_state >>>");
        self.verifier_sm.presentation_status()
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        trace!("Verifier::update_state >>> message: {:?}", message);

        if !self.verifier_sm.has_transitions() { return Ok(()); }

        if let Some(message_) = message {
            return self.update_state_with_message(message_);
        }

        let connection_handle = self.verifier_sm.connection_handle()?;
        let messages = connection::get_messages(connection_handle)?;

        if let Some((uid, message)) = self.verifier_sm.find_message_to_handle(messages) {
            self.handle_message(message.into())?;
            connection::update_message_status(connection_handle, uid)?;
        };

        Ok(())
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Verifier::update_state_with_message >>> message: {:?}", message);

        let message: Message = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot update state with message: Message deserialization failed: {:?}", err)))?;

        let connection_handle = self.verifier_sm.connection_handle()?;

        let uid = message.uid.clone();
        let a2a_message = connection::decode_message(connection_handle, message)?;

        self.handle_message(a2a_message.into())?;
        connection::update_message_status(connection_handle, uid)?;

        Ok(())
    }

    pub fn handle_message(&mut self, message: VerifierMessages) -> VcxResult<()> {
        trace!("Verifier::handle_message >>> message: {:?}", message);
        self.step(message)
    }

    pub fn verify_presentation(&mut self, presentation: Presentation) -> VcxResult<()> {
        trace!("Verifier::verify_presentation >>> presentation: {:?}", presentation);
        self.step(VerifierMessages::VerifyPresentation(presentation))
    }

    pub fn send_presentation_request(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Verifier::send_presentation_request >>> connection_handle: {:?}", connection_handle);
        self.step(VerifierMessages::SendPresentationRequest(connection_handle))
    }

    pub fn generate_presentation_request_msg(&self) -> VcxResult<String> {
        trace!("Verifier::generate_presentation_request_msg >>>");

        let proof_request: ProofRequestMessage = self.verifier_sm.presentation_request()?.try_into()?;

        ::serde_json::to_string(&proof_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn get_presentation(&self) -> VcxResult<String> {
        trace!("Verifier::get_presentation >>>");

        let proof: ProofMessage = self.verifier_sm.presentation()?.try_into()?;

        ::serde_json::to_string(&proof)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofMessage: {:?}", err)))
    }

    pub fn step(&mut self, message: VerifierMessages) -> VcxResult<()> {
        self.verifier_sm = self.verifier_sm.clone().step(message)?;
        Ok(())
    }
}