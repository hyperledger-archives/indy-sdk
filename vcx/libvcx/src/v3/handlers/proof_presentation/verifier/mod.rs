mod states;

use serde_json;

use settings;
use messages::{ObjectWithVersion, MessageStatusCode};
use messages::get_message::Message;
use utils::error;
use utils::libindy::anoncreds;
use object_cache::ObjectCache;
use error::prelude::*;

use v3::handlers::connection;
use v3::handlers::proof_presentation::verifier::states::*;
//use v3::handlers::proof_presentation::messages::*;
use v3::messages::attachment::Attachment;
use v3::messages::A2AMessage;
use v3::messages::proof_presentation::presentation_request::*;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::error::ProblemReport;
use v3::messages::ack::Ack;
use self::states::VerifierMessages;

use messages::update_message::{UIDsByConn, update_messages};

use messages::proofs::proof_message::get_credential_info;
use messages::proofs::proof_request::ProofRequestVersion;
use proof::Proof;

lazy_static! {
    static ref VERIFIER_MAP: ObjectCache<Verifier> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Verifier {
    source_id: String,
    state: VerifierSM
}

impl Verifier {
    const SERIALIZE_VERSION: &'static str = "2.0";

    fn create(source_id: String,
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

    fn get_source_id(&self) -> String { self.source_id.clone() }

    fn state(&self) -> u32 { self.state.state() }

    fn presentation_state(&self) -> u32 {
        self.state.presentation_state()
    }

    fn update_state(&mut self, message: Option<&str>) -> VcxResult<u32> {
        trace!("Verifier: update_state");

        match self.state.state {
            VerifierState::Initiated(_) => return Ok(self.state()),
            VerifierState::PresentationRequestSent(_) => {}
            VerifierState::Finished(_) => return Ok(self.state()),
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
        trace!("Verifier: handle_message: {:?}", message);

        let uid = message.uid.clone();
        let connection_handle = self.state.connection_handle()?;

        let a2a_message = connection::decode_message(connection_handle, message)?;

        match self.state.state {
            VerifierState::Initiated(ref state) => {
                // do not process a2a_message
            }
            VerifierState::PresentationRequestSent(ref state) => {
                match a2a_message {
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
                // do not process a2a_message
            }
        };

        Ok(None)
    }

    fn verify_presentation(&mut self, presentation: Presentation) -> VcxResult<u32> {
        let presentation_json = match presentation.presentations_attach {
            Attachment::JSON(ref attach) => attach.get_data()?,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "Unsupported Attachment type"))
        };

        let presentation_request = serde_json::to_string(&self.state.presentation_request()?).unwrap();

        let credential_data = get_credential_info(&presentation_json)?;

        let valid = Proof::validate_indy_proof(&credential_data, &presentation_json, &presentation_request)?;

        if !valid {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidProof, error::INVALID_PROOF.message));
        }

        self.send_ack(presentation)?;

        Ok(error::SUCCESS.code_num)
    }

    fn send_ack(&mut self, presentation: Presentation) -> VcxResult<()> {
        let ack = Ack::create();

        self.step(VerifierMessages::SendPresentationAck((presentation, ack)))?;

        Ok(())
    }

    fn send_problem_report(&mut self) -> VcxResult<()> {
        let problem_report = ProblemReport::create();

        self.step(VerifierMessages::SendPresentationReject(problem_report))?;

        Ok(())
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> VcxResult<u32> {
        match self.state.state {
            VerifierState::PresentationRequestSent(_) => return Ok(self.state()),
            VerifierState::Finished(_) => return Ok(self.state()),
            VerifierState::Initiated(_) => {}
        };

        let mut proof_request: PresentationRequestData = self.state.presentation_request()?;

        let title = format!("{} wants you to share: {}",
                            settings::get_config_value(settings::CONFIG_INSTITUTION_NAME)?,
                            self.state.name());

        let remote_did = connection::get_their_pw_verkey(connection_handle)?;

        proof_request = proof_request.set_format_version_for_did(&remote_did);

        let mut proof_request_json = serde_json::to_string(&proof_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofRequest: {:?}", err)))?;

        match proof_request.ver {
            Some(ProofRequestVersion::V1) | None => {
                proof_request_json = anoncreds::libindy_to_unqualified(&proof_request_json).unwrap();
            }
            _ => {}
        };

        let presentation_request = PresentationRequest::create()
            .set_comment(title)
            .set_request_presentations_attach(proof_request_json)?;

        self.step(VerifierMessages::SendPresentationRequest((presentation_request, connection_handle)))?;

        Ok(error::SUCCESS.code_num)
    }

    fn generate_proof_request_msg(&mut self) -> VcxResult<String> {
        let presentation_request = match self.state.presentation_request() {
            Ok(presentation_request) => presentation_request,
            Err(_) => return Ok(String::new())
        };

        let proof_request_json = serde_json::to_string(&presentation_request)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize ProofRequest: {:?}", err)))?;

        Ok(proof_request_json)
    }

    fn get_proof(&self) -> VcxResult<String> {
        let presentation: &Presentation = self.state.presentation()?;

        match presentation.presentations_attach {
            Attachment::JSON(ref attach) => attach.get_data(),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "Unsupported Attachment type"))
        }
    }

    fn from_str(data: &str) -> VcxResult<Self> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Self>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Connection"))
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(Self::SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Connection"))
    }

    fn step(&mut self, message: VerifierMessages) -> VcxResult<()> {
        self.state = self.state.clone().step(message)?;
        Ok(())
    }
}

pub fn create_proof(source_id: String,
                    requested_attrs: String,
                    requested_predicates: String,
                    revocation_details: String,
                    name: String) -> VcxResult<u32> {
    trace!("create_proof >>> source_id: {}, requested_attrs: {}, requested_predicates: {}, name: {}", source_id, requested_attrs, requested_predicates, name);

    let verifier = Verifier::create(source_id,
                                    requested_attrs,
                                    requested_predicates,
                                    revocation_details,
                                    name)?;

    VERIFIER_MAP.add(verifier)
        .or(Err(VcxError::from(VcxErrorKind::CreateProof)))
}

pub fn is_valid_handle(handle: u32) -> bool {
    VERIFIER_MAP.has_handle(handle)
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    VERIFIER_MAP.get_mut(handle, |verifier| {
        verifier.update_state(message.as_ref().map(String::as_str))
    })
}

pub fn get_state(handle: u32) -> u32 {
    VERIFIER_MAP.get(handle, |verifier| {
        Ok(verifier.state())
    }).unwrap_or(0)
}

pub fn get_proof_state(handle: u32) -> VcxResult<u32> {
    VERIFIER_MAP.get(handle, |p| {
        Ok(p.presentation_state())
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |verifier| {
        Verifier::to_string(&verifier)
    })
}

pub fn from_string(verifier_data: &str) -> VcxResult<u32> {
    let verifier: Verifier = Verifier::from_str(verifier_data)?;
    VERIFIER_MAP.add(verifier)
}

pub fn release(handle: u32) -> VcxResult<()> {
    VERIFIER_MAP.release(handle).or(Err(VcxError::from(VcxErrorKind::InvalidProofHandle)))
}

pub fn release_all() {
    VERIFIER_MAP.drain().ok();
}

pub fn send_proof_request(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    VERIFIER_MAP.get_mut(handle, |verifier| {
        verifier.send_proof_request(connection_handle)
    })
}

pub fn generate_proof_request_msg(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get_mut(handle, |verifier| {
        verifier.generate_proof_request_msg()
    })
}

pub fn get_proof(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |verifier| {
        verifier.get_proof()
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |obj| {
        Ok(obj.get_source_id())
    })
}
