use v3::handlers::connection;
use v3::handlers::proof_presentation::messages::PresentationState;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::proof_presentation::presentation_request::PresentationRequestData;
use v3::messages::ack::Ack;
use v3::messages::error::ProblemReport;

use error::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProverSM {
    pub state: ProverState
}

impl ProverSM {
    pub fn new(presentation_request: PresentationRequestData) -> ProverSM {
        ProverSM { state: ProverState::Initiated(InitialState { presentation_request }) }
    }
}

// Possible Transitions:
//
// Initial -> PresentationPrepared, Finished
// PresentationPrepared -> PresentationSent, Finished
// PresentationSent -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProverState {
    Initiated(InitialState),
    PresentationPrepared(PresentationPreparedState),
    PresentationSent(PresentationSentState),
    Finished(FinishedState)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ProverMessages {
    PresentationRequestReceived(PresentationRequestData),
    PresentationPrepared(Presentation),
    SendPresentation(u32),
    PresentationAckReceived(Ack),
    PresentationRejectReceived(ProblemReport),
    SendPresentationReject(ProblemReport),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    pub presentation_request: PresentationRequestData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparedState {
    pub presentation: Presentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationSentState {
    pub connection_handle: u32,
    pub presentation: Presentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    connection_handle: u32,
    presentation: Presentation,
    presentation_state: PresentationState
}

impl From<(InitialState, Presentation)> for PresentationPreparedState {
    fn from((state, presentation): (InitialState, Presentation)) -> Self {
        trace!("transit state from InitialState to PresentationPreparedState");
        PresentationPreparedState {
            presentation,
        }
    }
}

impl From<InitialState> for FinishedState {
    fn from(state: InitialState) -> Self {
        trace!("transit state from InitialState to FinishedState");
        FinishedState {
            connection_handle: 0,
            presentation: Presentation::create(),
            presentation_state: PresentationState::Undefined
        }
    }
}

impl From<(PresentationPreparedState, u32)> for PresentationSentState {
    fn from((state, connection_handle): (PresentationPreparedState, u32)) -> Self {
        trace!("transit state from PresentationPreparedState to PresentationSentState");
        PresentationSentState {
            presentation: state.presentation,
            connection_handle
        }
    }
}

impl From<(PresentationPreparedState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationPreparedState, ProblemReport)) -> Self {
        trace!("transit state from PresentationPreparedState to FinishedState");
        FinishedState {
            presentation: state.presentation,
            connection_handle: 0,
            presentation_state: PresentationState::Undefined,
        }
    }
}

impl From<(PresentationSentState, Ack)> for FinishedState {
    fn from((state, ack): (PresentationSentState, Ack)) -> Self {
        trace!("transit state from PresentationSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation: state.presentation,
            presentation_state: PresentationState::Verified,
        }
    }
}

impl From<(PresentationSentState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationSentState, ProblemReport)) -> Self {
        trace!("transit state from PresentationSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation: state.presentation,
            presentation_state: PresentationState::Invalid(problem_report),
        }
    }
}

impl ProverSM {
    pub fn step(self, message: ProverMessages) -> VcxResult<ProverSM> {
        trace!("ProverSM::step >>> message: {:?}", message);

        let ProverSM { state } = self;

        let state = match state {
            ProverState::Initiated(state) => {
                match message {
                    ProverMessages::PresentationPrepared(presentation) => {
                        ProverState::PresentationPrepared((state, presentation).into())
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
                        ProverState::PresentationSent((state, connection_handle).into())
                    }
                    _ => {
                        ProverState::PresentationPrepared(state)
                    }
                }
            }
            ProverState::PresentationSent(state) => {
                match message {
                    ProverMessages::PresentationAckReceived(ack) => {
                        ProverState::Finished((state, ack).into())
                    }
                    ProverMessages::SendPresentationReject(problem_report) => {
                        ProverState::Finished((state, problem_report).into())
                    }
                    _ => {
                        ProverState::PresentationSent(state)
                    }
                }
            }
            ProverState::Finished(state) => ProverState::Finished(state)
        };

        Ok(ProverSM { state })
    }

    pub fn state(&self) -> u32 {
        match self.state {
            ProverState::Initiated(_) => 1,
            ProverState::PresentationPrepared(_) => 1,
            ProverState::PresentationSent(_) => 2,
            ProverState::Finished(_) => 4,
        }
    }

    pub fn presentation_state(&self) -> u32 {
        match self.state {
            ProverState::Finished(ref state) =>
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
            ProverState::Initiated(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            ProverState::PresentationPrepared(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection handle isn't set")),
            ProverState::PresentationSent(ref state) => Ok(state.connection_handle),
            ProverState::Finished(ref state) => Ok(state.connection_handle),
        }
    }

    pub fn presentation_request(&self) -> VcxResult<&PresentationRequestData> {
        match self.state {
            ProverState::Initiated(ref state) => Ok(&state.presentation_request),
            ProverState::PresentationPrepared(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            ProverState::PresentationSent(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            ProverState::Finished(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
        }
    }

    pub fn presentation(&self) -> VcxResult<&Presentation> {
        match self.state {
            ProverState::Initiated(ref state) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            ProverState::PresentationPrepared(ref state) => Ok(&state.presentation),
            ProverState::PresentationSent(ref state) => Ok(&state.presentation),
            ProverState::Finished(ref state) => Ok(&state.presentation),
        }
    }
}