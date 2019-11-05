use v3::messages::proof_presentation::presentation_proposal::PresentationProposal;
use v3::messages::proof_presentation::presentation_request::{PresentationRequest, PresentationRequestData};
use v3::messages::proof_presentation::presentation::{Presentation, PresentationStatus};
use v3::messages::error::ProblemReport;
use v3::messages::ack::Ack;
use messages::thread::Thread;
use proof::Proof;

use v3::handlers::connection;
use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierSM {
    pub state: VerifierState
}

impl VerifierSM {
    pub fn new(presentation_request: PresentationRequestData) -> VerifierSM {
        VerifierSM { state: VerifierState::Initiated(InitialState { presentation_request_data: presentation_request }) }
    }
}

// Possible Transitions:
//
// Initial -> PresentationRequestSent
// SendPresentationRequest -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerifierState {
    Initiated(InitialState),
    PresentationRequestSent(PresentationRequestSentState),
    Finished(FinishedState)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum VerifierMessages {
    SendPresentationRequest(u32),
    VerifyPresentation(Presentation),
    PresentationProposalReceived(PresentationProposal),
    PresentationRejectReceived(ProblemReport),
    SendPresentationReject(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    presentation_request_data: PresentationRequestData
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationRequestSentState {
    connection_handle: u32,
    presentation_request: PresentationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    connection_handle: u32,
    presentation_request: PresentationRequest,
    presentation: Option<Presentation>,
    presentation_state: PresentationStatus
}

impl From<(InitialState, PresentationRequest, u32)> for PresentationRequestSentState {
    fn from((state, presentation_request, connection_handle): (InitialState, PresentationRequest, u32)) -> Self {
        trace!("transit state from InitialState to PresentationRequestSentState");
        PresentationRequestSentState { connection_handle, presentation_request }
    }
}

impl From<(PresentationRequestSentState, Presentation, Ack)> for FinishedState {
    fn from((state, presentation, ack): (PresentationRequestSentState, Presentation, Ack)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation_request: state.presentation_request,
            presentation: Some(presentation),
            presentation_state: PresentationStatus::Verified,
        }
    }
}

impl From<(PresentationRequestSentState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationRequestSentState, ProblemReport)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation_request: state.presentation_request,
            presentation: None,
            presentation_state: PresentationStatus::Invalid(problem_report),
        }
    }
}

impl VerifierSM {
    pub fn step(self, message: VerifierMessages) -> VcxResult<VerifierSM> {
        trace!("VerifierSM::step >>> message: {:?}", message);

        let VerifierSM { state } = self;

        let state = match state {
            VerifierState::Initiated(state) => {
                match message {
                    VerifierMessages::SendPresentationRequest(connection_handle) => {
                        let my_did = connection::get_pw_did(connection_handle)?;
                        let remote_did = connection::get_their_pw_verkey(connection_handle)?;

                        let presentation_request: PresentationRequestData =
                            state.presentation_request_data.clone()
                                .set_format_version_for_did(&my_did, &remote_did)?;

                        let title = format!("{} wants you to share {}",
                                            ::settings::get_config_value(::settings::CONFIG_INSTITUTION_NAME)?, presentation_request.name);

                        let presentation_request =
                            PresentationRequest::create()
                                .set_comment(title)
                                .set_request_presentations_attach(&presentation_request)?;

                        connection::send_message(connection_handle, presentation_request.to_a2a_message())?;
                        VerifierState::PresentationRequestSent((state, presentation_request, connection_handle).into())
                    }
                    _ => {
                        VerifierState::Initiated(state)
                    }
                }
            }
            VerifierState::PresentationRequestSent(state) => {
                match message {
                    VerifierMessages::VerifyPresentation(presentation) => {
                        let valid = Proof::validate_indy_proof(&presentation.presentations_attach.content()?,
                                                               &state.presentation_request.request_presentations_attach.content()?)?;

                        if !valid {
                            return Err(VcxError::from_msg(VcxErrorKind::InvalidProof, "Presentation verification failed"));
                        }

                        let ack = Ack::create().set_thread(presentation.thread.clone());

                        connection::send_message(state.connection_handle, ack.to_a2a_message())?;
                        VerifierState::Finished((state, presentation, ack).into())
                    }
                    VerifierMessages::SendPresentationReject(err) => {
                        let problem_report =
                            ProblemReport::create()
                                .set_comment(err)
                                .set_thread(Thread::new().set_thid(state.presentation_request.id.0.clone()));
                        connection::send_message(state.connection_handle, problem_report.to_a2a_message())?;
                        VerifierState::Finished((state, problem_report).into())
                    }
                    VerifierMessages::PresentationRejectReceived(problem_report) => {
                        VerifierState::Finished((state, problem_report).into())
                    }
                    VerifierMessages::PresentationProposalReceived(presentation_proposal) => {
                        let problem_report =
                            ProblemReport::create()
                                .set_comment(String::from("PresentationProposal is not supported"))
                                .set_thread(Thread::new().set_thid(state.presentation_request.id.0.clone()));

                        connection::send_message(state.connection_handle, problem_report.to_a2a_message())?;
                        VerifierState::Finished((state, problem_report).into())
                    }
                    _ => {
                        VerifierState::PresentationRequestSent(state)
                    }
                }
            }
            VerifierState::Finished(state) => VerifierState::Finished(state)
        };

        Ok(VerifierSM { state })
    }

    pub fn state(&self) -> u32 {
        match self.state {
            VerifierState::Initiated(_) => 1,
            VerifierState::PresentationRequestSent(_) => 2,
            VerifierState::Finished(_) => 4,
        }
    }

    pub fn has_transitions(&self) -> bool {
        match self.state {
            VerifierState::Initiated(_) => false,
            VerifierState::PresentationRequestSent(_) => true,
            VerifierState::Finished(_) => false,
        }
    }

    pub fn presentation_status(&self) -> u32 {
        match self.state {
            VerifierState::Finished(ref state) =>
                match state.presentation_state {
                    PresentationStatus::Undefined => 0,
                    PresentationStatus::Verified => 1,
                    PresentationStatus::Invalid(_) => 2,
                },
            _ => 0
        }
    }

    pub fn connection_handle(&self) -> VcxResult<u32> {
        match self.state {
            VerifierState::Initiated(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            VerifierState::PresentationRequestSent(ref state) => Ok(state.connection_handle),
            VerifierState::Finished(ref state) => Ok(state.connection_handle),
        }
    }

    pub fn presentation_request_data(&self) -> VcxResult<&PresentationRequestData> {
        match self.state {
            VerifierState::Initiated(ref state) => Ok(&state.presentation_request_data),
            VerifierState::PresentationRequestSent(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::Finished(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
        }
    }

    pub fn presentation_request(&self) -> VcxResult<PresentationRequest> {
        match self.state {
            VerifierState::Initiated(ref state) => {
                PresentationRequest::create().set_request_presentations_attach(&state.presentation_request_data)
            }
            VerifierState::PresentationRequestSent(ref state) => Ok(state.presentation_request.clone()),
            VerifierState::Finished(ref state) => Ok(state.presentation_request.clone()),
        }
    }

    pub fn presentation(&self) -> VcxResult<Presentation> {
        match self.state {
            VerifierState::Initiated(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::PresentationRequestSent(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::Finished(ref state) => {
                state.presentation.clone()
                    .ok_or(VcxError::from(VcxErrorKind::InvalidProofHandle))
            }
        }
    }
}