use v3::messages::proof_presentation::presentation_proposal::PresentationProposal;
use v3::messages::proof_presentation::presentation_request::{PresentationRequest, PresentationRequestData};
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::error::ProblemReport;
use v3::messages::ack::Ack;
use v3::handlers::proof_presentation::messages::PresentationState;

use v3::handlers::connection;
use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierSM {
    pub state: VerifierState
}

impl VerifierSM {
    pub fn new(presentation_request: PresentationRequestData) -> VerifierSM {
        VerifierSM { state: VerifierState::Initiated(InitialState { presentation_request }) }
    }
}

// Possible Transitions:
//
// Initial -> PresentationRequestSent, Finished
// SendPresentationRequest -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerifierState {
    Initiated(InitialState),
    PresentationRequestSent(PresentationRequestSentState),
    Finished(FinishedState)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum VerifierMessages {
    SendPresentationRequest((PresentationRequest, u32)),
    SendPresentationAck((Presentation, Ack)),
    PresentationProposalReceived(PresentationProposal),
    PresentationRejectReceived(ProblemReport),
    SendPresentationReject(ProblemReport),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    presentation_request: PresentationRequestData
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationRequestSentState {
    presentation_request: PresentationRequestData,
    connection_handle: u32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    connection_handle: u32,
    presentation: Option<Presentation>,
    presentation_state: PresentationState
}

impl From<(InitialState, u32)> for PresentationRequestSentState {
    fn from((state, connection_handle): (InitialState, u32)) -> Self {
        trace!("transit state from InitialState to PresentationRequestSentState");
        PresentationRequestSentState { connection_handle, presentation_request: state.presentation_request }
    }
}

impl From<(InitialState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (InitialState, ProblemReport)) -> Self {
        trace!("transit state from InitialState to FinishedState");
        FinishedState {
            connection_handle: 0,
            presentation: None,
            presentation_state: PresentationState::Undefined
        }
    }
}

impl From<(PresentationRequestSentState, Presentation, Ack)> for FinishedState {
    fn from((state, presentation, ack): (PresentationRequestSentState, Presentation, Ack)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation: Some(presentation),
            presentation_state: PresentationState::Verified
        }
    }
}

impl From<(PresentationRequestSentState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationRequestSentState, ProblemReport)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation: None,
            presentation_state: PresentationState::Invalid(problem_report)
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
                    VerifierMessages::SendPresentationRequest((presentation_request, connection_handle)) => {
                        connection::send_message(connection_handle, presentation_request.to_a2a_message())?;
                        VerifierState::PresentationRequestSent((state, connection_handle).into())
                    }
                    VerifierMessages::PresentationRejectReceived(problem_report) => {
                        VerifierState::Finished((state, problem_report).into())
                    }
                    _ => {
                        VerifierState::Initiated(state)
                    }
                }
            }
            VerifierState::PresentationRequestSent(state) => {
                match message {
                    VerifierMessages::SendPresentationAck((presentation, ack)) => {
                        connection::send_message(state.connection_handle, ack.to_a2a_message())?;
                        VerifierState::Finished((state, presentation, ack).into())
                    }
                    VerifierMessages::SendPresentationReject(problem_report) => {
                        connection::send_message(state.connection_handle, problem_report.to_a2a_message())?;
                        VerifierState::Finished((state, problem_report).into())
                    }
                    VerifierMessages::PresentationRejectReceived(problem_report) => {
                        VerifierState::Finished((state, problem_report).into())
                    }
                    VerifierMessages::PresentationProposalReceived(presentation_proposal) => {
                        let problem_report = ProblemReport::create();
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

    pub fn presentation_state(&self) -> u32 {
        match self.state {
            VerifierState::Finished(ref state) =>
                match state.presentation_state {
                    PresentationState::Undefined => 0,
                    PresentationState::Verified => 1,
                    PresentationState::Invalid(_) => 2,
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

    pub fn name(&self) -> String {
        match self.state {
            VerifierState::Initiated(ref state) => state.presentation_request.name.clone(),
            VerifierState::PresentationRequestSent(ref state) => String::new(),
            VerifierState::Finished(ref state) => String::new(),
        }
    }

    pub fn presentation_request(&self) -> VcxResult<PresentationRequestData> {
        match self.state {
            VerifierState::Initiated(ref state) => Ok(state.presentation_request.clone()),
            VerifierState::PresentationRequestSent(ref state) => Ok(state.presentation_request.clone()),
            VerifierState::Finished(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
        }
    }

    pub fn presentation(&self) -> VcxResult<&Presentation> {
        match self.state {
            VerifierState::Initiated(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::PresentationRequestSent(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::Finished(ref state) => {
                state.presentation.as_ref()
                    .ok_or(VcxError::from(VcxErrorKind::InvalidProofHandle))
            }
        }
    }
}