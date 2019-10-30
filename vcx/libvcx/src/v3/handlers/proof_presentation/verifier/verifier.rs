use serde_json;

use settings;
use utils::error;
use error::prelude::*;

use messages::ObjectWithVersion;
use messages::get_message::Message;

use v3::handlers::connection;
use v3::messages::A2AMessage;
use v3::messages::proof_presentation::presentation_request::*;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::error::ProblemReport;
use v3::messages::ack::Ack;
use v3::handlers::proof_presentation::verifier::states::{VerifierSM, VerifierState, VerifierMessages};

use proof::Proof;

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
        trace!("Verifier::create >>> source_id: {}", source_id);

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

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        trace!("Verifier: update_state");

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
            VerifierState::Initiated(ref state) => {
                // do not process message
            }
            VerifierState::PresentationRequestSent(ref state) => {
                // TODO: FIXME better way of reply check
                let thread = match message {
                    A2AMessage::Presentation(ref presentation) => &presentation.thread,
                    A2AMessage::PresentationProposal(ref presentation) => &presentation.thread,
                    A2AMessage::CommonProblemReport(ref presentation) => &presentation.thread,
                    _ => { return Ok(None); }
                };

                if !thread.is_reply(&state.presentation_request.id.0) {
                    return Ok(None);
                }

                match message {
                    A2AMessage::Presentation(presentation) => {
                        if let Err(err) = self.verify_presentation(presentation) {
                            self.send_problem_report()?
                        }
                        return Ok(Some(uid));
                    }
                    A2AMessage::PresentationProposal(proposal) => {
                        self.step(VerifierMessages::PresentationProposalReceived(proposal))?;
                        return Ok(Some(uid));
                    }
                    A2AMessage::CommonProblemReport(problem_report) => {
                        self.step(VerifierMessages::PresentationRejectReceived(problem_report))?;
                        return Ok(Some(uid));
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

    pub fn verify_presentation(&mut self, presentation: Presentation) -> VcxResult<u32> {
        let presentation_json = presentation.presentations_attach.content()?;

        let presentation_request_json = self.state.presentation_request()?.request_presentations_attach.content()?;

        let valid = Proof::validate_indy_proof(&&presentation_json, &presentation_request_json)?;

        if !valid {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidProof, error::INVALID_PROOF.message));
        }

        let ack = Ack::create();

        self.step(VerifierMessages::SendPresentationAck((presentation, ack)))?;

        Ok(error::SUCCESS.code_num)
    }

    pub fn send_problem_report(&mut self) -> VcxResult<()> {
        let problem_report = ProblemReport::create();
        self.step(VerifierMessages::SendPresentationReject(problem_report))?;
        Ok(())
    }

    pub fn send_presentation_request(&mut self, connection_handle: u32) -> VcxResult<u32> {
        let remote_did = connection::get_their_pw_verkey(connection_handle)?;

        let presentation_request = self.build_proof_request(Some(&remote_did))?;

        self.step(VerifierMessages::SendPresentationRequest((presentation_request, connection_handle)))?;

        Ok(error::SUCCESS.code_num)
    }

    pub fn build_proof_request(&self, remote_did: Option<&str>) -> VcxResult<PresentationRequest> {
        let presentation_request: PresentationRequestData =
            self.state.presentation_request_data()?.clone()
                .set_format_version_for_did(&remote_did.unwrap_or_default());

        let title = format!("{} wants you to share {}",
                            settings::get_config_value(settings::CONFIG_INSTITUTION_NAME)?, presentation_request.name);

        PresentationRequest::create()
            .set_comment(title)
            .set_request_presentations_attach(presentation_request.to_string()?)
    }

    pub fn generate_proof_request_msg(&mut self) -> VcxResult<String> {
        let presentation_request: PresentationRequest =
            match self.state.presentation_request() {
                Ok(presentation_request) => presentation_request.clone(),
                Err(_) => self.build_proof_request(None)?
            };

        let proof_request_json = serde_json::to_string(&presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize PresentationRequest: {:?}", err)))?;

        Ok(proof_request_json)
    }

    pub fn get_proof(&self) -> VcxResult<String> {
        let presentation: &Presentation = self.state.presentation()?;

        let proof_request_json = serde_json::to_string(&presentation)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize Presentation: {:?}", err)))?;

        Ok(proof_request_json)
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