use v3::handlers::connection;
use v3::messages::proof_presentation::presentation_request::{PresentationRequestData, PresentationRequest};
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;
use v3::messages::status::Status;
use messages::thread::Thread;


use disclosed_proof::DisclosedProof;

use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProverSM {
    source_id: String,
    pub state: ProverState
}

impl ProverSM {
    pub fn new(presentation_request: PresentationRequest, source_id: String) -> ProverSM {
        ProverSM { source_id, state: ProverState::Initiated(InitialState { presentation_request }) }
    }
}

// Possible Transitions:
//
// Initial -> PresentationPrepared, PresentationPreparationFailedState
// PresentationPrepared -> PresentationSent
// PresentationPreparationFailedState -> Finished
// PresentationSent -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProverState {
    Initiated(InitialState),
    PresentationPrepared(PresentationPreparedState),
    PresentationPreparationFailed(PresentationPreparationFailedState),
    PresentationSent(PresentationSentState),
    Finished(FinishedState)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ProverMessages {
    PresentationRequestReceived(PresentationRequestData),
    PreparePresentation((String, String)),
    SendPresentation(u32),
    PresentationAckReceived(Ack),
    PresentationRejectReceived(ProblemReport),
    SendPresentationReject(ProblemReport),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    presentation_request: PresentationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparedState {
    presentation_request: PresentationRequest,
    presentation: Presentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparationFailedState {
    presentation_request: PresentationRequest,
    problem_report: ProblemReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationSentState {
    connection_handle: u32,
    presentation_request: PresentationRequest,
    presentation: Presentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    connection_handle: u32,
    presentation_request: PresentationRequest,
    presentation: Presentation,
    status: Status
}

impl From<(InitialState, Presentation)> for PresentationPreparedState {
    fn from((state, presentation): (InitialState, Presentation)) -> Self {
        trace!("transit state from InitialState to PresentationPreparedState");
        PresentationPreparedState {
            presentation_request: state.presentation_request,
            presentation,
        }
    }
}

impl From<(InitialState, ProblemReport)> for PresentationPreparationFailedState {
    fn from((state, problem_report): (InitialState, ProblemReport)) -> Self {
        trace!("transit state from InitialState to PresentationPreparationFailedState");
        PresentationPreparationFailedState {
            presentation_request: state.presentation_request,
            problem_report,
        }
    }
}

impl From<(PresentationPreparedState, u32)> for PresentationSentState {
    fn from((state, connection_handle): (PresentationPreparedState, u32)) -> Self {
        trace!("transit state from PresentationPreparedState to PresentationSentState");
        PresentationSentState {
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            connection_handle
        }
    }
}

impl From<(PresentationPreparationFailedState, u32)> for FinishedState {
    fn from((state, connection_handle): (PresentationPreparationFailedState, u32)) -> Self {
        trace!("transit state from PresentationPreparationFailedState to FinishedState");
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Presentation::create(),
            connection_handle,
            status: Status::Failed(state.problem_report),
        }
    }
}

impl From<(PresentationSentState, Ack)> for FinishedState {
    fn from((state, ack): (PresentationSentState, Ack)) -> Self {
        trace!("transit state from PresentationSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            status: Status::Success,
        }
    }
}

impl From<(PresentationSentState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationSentState, ProblemReport)) -> Self {
        trace!("transit state from PresentationSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            status: Status::Failed(problem_report),
        }
    }
}

impl InitialState {
    fn build_presentation(&self, credentials: &str, self_attested_attrs: &str) -> VcxResult<Presentation> {
        let presentation = DisclosedProof::generate_indy_proof(credentials,
                                                               self_attested_attrs,
                                                               &self.presentation_request.request_presentations_attach.content()?)?;

        Presentation::create()
            .set_thread(Thread::new().set_thid(self.presentation_request.id.0.clone()))
            .set_presentations_attach(presentation)
    }
}

impl ProverSM {
    pub fn step(self, message: ProverMessages) -> VcxResult<ProverSM> {
        trace!("ProverSM::step >>> message: {:?}", message);

        let ProverSM { source_id, state } = self;

        let state = match state {
            ProverState::Initiated(state) => {
                match message {
                    ProverMessages::PreparePresentation((credentials, self_attested_attrs)) => {
                        match state.build_presentation(&credentials, &self_attested_attrs) {
                            Ok(presentation) => {
                                ProverState::PresentationPrepared((state, presentation).into())
                            }
                            Err(err) => {
                                let problem_report =
                                    ProblemReport::create()
                                        .set_comment(err.to_string())
                                        .set_thread(Thread::new().set_thid(state.presentation_request.id.0.clone()));

                                ProverState::PresentationPreparationFailed((state, problem_report).into())
                            }
                        }
                    }
                    _ => {
                        ProverState::Initiated(state)
                    }
                }
            }
            ProverState::PresentationPrepared(state) => {
                match message {
                    ProverMessages::SendPresentation(connection_handle) => {
                        connection::send_message(connection_handle, state.presentation.to_a2a_message())?;
                        connection::remove_pending_message(connection_handle, &state.presentation_request.id)?;
                        ProverState::PresentationSent((state, connection_handle).into())
                    }
                    _ => {
                        ProverState::PresentationPrepared(state)
                    }
                }
            }
            ProverState::PresentationPreparationFailed(state) => {
                match message {
                    ProverMessages::SendPresentation(connection_handle) => {
                        connection::send_message(connection_handle, state.problem_report.to_a2a_message())?;
                        ProverState::Finished((state, connection_handle).into())
                    }
                    _ => {
                        ProverState::PresentationPreparationFailed(state)
                    }
                }
            }
            ProverState::PresentationSent(state) => {
                match message {
                    ProverMessages::PresentationAckReceived(ack) => {
                        ProverState::Finished((state, ack).into())
                    }
                    ProverMessages::PresentationRejectReceived(problem_report) => {
                        ProverState::Finished((state, problem_report).into())
                    }
                    _ => {
                        ProverState::PresentationSent(state)
                    }
                }
            }
            ProverState::Finished(state) => ProverState::Finished(state)
        };

        Ok(ProverSM { source_id, state })
    }

    pub fn source_id(&self) -> String { self.source_id.clone() }

    pub fn thread_id(&self) -> String { self.presentation_request().id.0.clone() }

    pub fn state(&self) -> u32 {
        match self.state {
            ProverState::Initiated(_) => 1,
            ProverState::PresentationPrepared(_) => 1,
            ProverState::PresentationPreparationFailed(_) => 1,
            ProverState::PresentationSent(_) => 2,
            ProverState::Finished(_) => 4,
        }
    }

    pub fn has_transitions(&self) -> bool {
        match self.state {
            ProverState::Initiated(_) => false,
            ProverState::PresentationPrepared(_) => true,
            ProverState::PresentationPreparationFailed(_) => true,
            ProverState::PresentationSent(_) => true,
            ProverState::Finished(_) => false,
        }
    }

    pub fn presentation_status(&self) -> u32 {
        match self.state {
            ProverState::Finished(ref state) => state.status.code(),
            _ => Status::Undefined.code()
        }
    }

    pub fn connection_handle(&self) -> VcxResult<u32> {
        match self.state {
            ProverState::Initiated(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            ProverState::PresentationPrepared(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            ProverState::PresentationPreparationFailed(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            ProverState::PresentationSent(ref state) => Ok(state.connection_handle),
            ProverState::Finished(ref state) => Ok(state.connection_handle),
        }
    }

    pub fn presentation_request(&self) -> &PresentationRequest {
        match self.state {
            ProverState::Initiated(ref state) => &state.presentation_request,
            ProverState::PresentationPrepared(ref state) => &state.presentation_request,
            ProverState::PresentationPreparationFailed(ref state) => &state.presentation_request,
            ProverState::PresentationSent(ref state) => &state.presentation_request,
            ProverState::Finished(ref state) => &state.presentation_request,
        }
    }

    pub fn presentation(&self) -> VcxResult<&Presentation> {
        match self.state {
            ProverState::Initiated(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            ProverState::PresentationPrepared(ref state) => Ok(&state.presentation),
            ProverState::PresentationPreparationFailed(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            ProverState::PresentationSent(ref state) => Ok(&state.presentation),
            ProverState::Finished(ref state) => Ok(&state.presentation),
        }
    }
}