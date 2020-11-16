use std::collections::HashMap;

use api::VcxStateType;
use connection::{get_pw_did, get_their_pw_verkey};
use connection;
use error::prelude::*;
use proof::Proof;
use v3::handlers::proof_presentation::verifier::messages::VerifierMessages;
use v3::messages::a2a::A2AMessage;
use v3::messages::error::ProblemReport;
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::proof_presentation::presentation_ack::PresentationAck;
use v3::messages::proof_presentation::presentation_request::{PresentationRequest, PresentationRequestData};
use v3::messages::status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierSM {
    source_id: String,
    state: VerifierState,
}

impl VerifierSM {
    pub fn new(presentation_request: PresentationRequestData, source_id: String) -> VerifierSM {
        VerifierSM { source_id, state: VerifierState::Initiated(InitialState { presentation_request_data: presentation_request }) }
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
pub enum RevocationStatus {
    Revoked,
    NonRevoked
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    connection_handle: u32,
    presentation_request: PresentationRequest,
    presentation: Option<Presentation>,
    status: Status,
    revocation_status: Option<RevocationStatus>
}

impl From<(InitialState, PresentationRequest, u32)> for PresentationRequestSentState {
    fn from((_state, presentation_request, connection_handle): (InitialState, PresentationRequest, u32)) -> Self {
        trace!("transit state from InitialState to PresentationRequestSentState");
        PresentationRequestSentState { connection_handle, presentation_request }
    }
}

impl From<(PresentationRequestSentState, Presentation, RevocationStatus)> for FinishedState {
    fn from((state, presentation, was_revoked): (PresentationRequestSentState, Presentation, RevocationStatus)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            connection_handle: state.connection_handle,
            presentation_request: state.presentation_request,
            presentation: Some(presentation),
            status: Status::Success,
            revocation_status: Some(was_revoked)
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
            status: Status::Failed(problem_report),
            revocation_status: None
        }
    }
}


impl PresentationRequestSentState {
    fn verify_presentation(&self, presentation: &Presentation) -> VcxResult<()> {
        let valid = Proof::validate_indy_proof(&presentation.presentations_attach.content()?,
                                               &self.presentation_request.request_presentations_attach.content()?)?;

        if !valid {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidProof, "Presentation verification failed"));
        }

        if presentation.please_ack.is_some() {
            let ack = PresentationAck::create().set_thread_id(&self.presentation_request.id.0);
            connection::send_message(self.connection_handle, A2AMessage::PresentationAck(ack))?;
        }

        Ok(())
    }
}

impl VerifierSM {
    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("VerifierSM::find_message_to_handle >>> messages: {:?}", messages);

        for (uid, message) in messages {
            match self.state {
                VerifierState::Initiated(_) => {
                    // do not process message
                }
                VerifierState::PresentationRequestSent(_) => {
                    match message {
                        A2AMessage::Presentation(presentation) => {
                            if presentation.from_thread(&self.thread_id()) {
                                return Some((uid, A2AMessage::Presentation(presentation)));
                            }
                        }
                        A2AMessage::PresentationProposal(proposal) => {
                            if proposal.from_thread(&self.thread_id()) {
                                return Some((uid, A2AMessage::PresentationProposal(proposal)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) => {
                            if problem_report.from_thread(&self.thread_id()) {
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        _ => {}
                    }
                }
                VerifierState::Finished(_) => {
                    // do not process message
                }
            };
        }

        None
    }

    pub fn step(self, message: VerifierMessages) -> VcxResult<VerifierSM> {
        trace!("VerifierSM::step >>> message: {:?}", message);

        let VerifierSM { source_id, state } = self;

        let state = match state {
            VerifierState::Initiated(state) => {
                match message {
                    VerifierMessages::SendPresentationRequest(connection_handle) => {
                        let my_did = get_pw_did(connection_handle)?;
                        let remote_did = get_their_pw_verkey(connection_handle)?;

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
                        match state.verify_presentation(&presentation) {
                            Ok(()) => {
                                VerifierState::Finished((state, presentation, RevocationStatus::NonRevoked).into())
                            }
                            Err(err) => {
                                let problem_report =
                                    ProblemReport::create()
                                        .set_comment(err.to_string())
                                        .set_thread_id(&state.presentation_request.id.0);

                                connection::send_message(state.connection_handle, problem_report.to_a2a_message())?;
                                match err.kind() {
                                    VcxErrorKind::InvalidProof => {
                                        VerifierState::Finished((state, presentation, RevocationStatus::Revoked).into())
                                    }
                                    _ => VerifierState::Finished((state, problem_report).into())
                                }
                            }
                        }
                    }
                    VerifierMessages::PresentationRejectReceived(problem_report) => {
                        VerifierState::Finished((state, problem_report).into())
                    }
                    VerifierMessages::PresentationProposalReceived(_) => { // TODO: handle Presentation Proposal
                        let problem_report =
                            ProblemReport::create()
                                .set_comment(String::from("PresentationProposal is not supported"))
                                .set_thread_id(&state.presentation_request.id.0);

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

        Ok(VerifierSM { source_id, state })
    }

    pub fn source_id(&self) -> String { self.source_id.clone() }

    pub fn thread_id(&self) -> String { self.presentation_request().map(|request| request.id.0.clone()).unwrap_or_default() }

    pub fn state(&self) -> u32 {
        match self.state {
            VerifierState::Initiated(_) => VcxStateType::VcxStateInitialized as u32,
            VerifierState::PresentationRequestSent(_) => VcxStateType::VcxStateOfferSent as u32,
            VerifierState::Finished(ref status) => {
                match status.status {
                    Status::Success => VcxStateType::VcxStateAccepted as u32,
                    _ => VcxStateType::VcxStateNone as u32,
                }
            }
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
            VerifierState::Finished(ref state) => {
                match &state.status {
                    Status::Success => {
                        match state.revocation_status {
                            Some(RevocationStatus::NonRevoked) => Status::Success.code(),
                            None => Status::Success.code(), // for backward compatibility
                            Some(RevocationStatus::Revoked) => {
                                let problem_report = ProblemReport::create().set_comment(String::from("Revoked credential was used."));
                                Status::Failed(problem_report).code()
                            },
                        }
                    }
                    _ => state.status.code(),
                }


            },
            _ => Status::Undefined.code()
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
            VerifierState::PresentationRequestSent(_) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
            VerifierState::Finished(_) => Err(VcxError::from(VcxErrorKind::InvalidProofHandle)),
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
            VerifierState::Initiated(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Presentation is not received yet")),
            VerifierState::PresentationRequestSent(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Presentation is not received yet")),
            VerifierState::Finished(ref state) => {
                state.presentation.clone()
                    .ok_or(VcxError::from(VcxErrorKind::InvalidProofHandle))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use utils::devsetup::SetupAriesMocks;
    use v3::handlers::connection::tests::mock_connection;
    use v3::messages::proof_presentation::presentation::tests::_presentation;
    use v3::messages::proof_presentation::presentation_proposal::tests::_presentation_proposal;
    use v3::messages::proof_presentation::presentation_request::tests::_presentation_request;
    use v3::messages::proof_presentation::presentation_request::tests::_presentation_request_data;
    use v3::messages::proof_presentation::test::{_ack, _problem_report};
    use v3::test::source_id;

    use super::*;

    pub fn _verifier_sm() -> VerifierSM {
        VerifierSM::new(_presentation_request_data(), source_id())
    }

    impl VerifierSM {
        fn to_presentation_request_sent_state(mut self) -> VerifierSM {
            self = self.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            self
        }

        fn to_finished_state(mut self) -> VerifierSM {
            self = self.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            self = self.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();
            self
        }
    }

    mod new {
        use super::*;

        #[test]
        fn test_verifier_new() {
            let _setup = SetupAriesMocks::init();

            let verifier_sm = _verifier_sm();

            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
            assert_eq!(source_id(), verifier_sm.source_id());
        }
    }

    mod step {
        use super::*;
        use settings::set_config_value;
        use settings;

        #[test]
        fn test_verifier_init() {
            let _setup = SetupAriesMocks::init();

            let verifier_sm = _verifier_sm();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_send_presentation_request_message_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();

            assert_match!(VerifierState::PresentationRequestSent(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_other_messages_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);

            verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_verify_presentation_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(Status::Success.code(), verifier_sm.presentation_status());
        }

        #[test]
        fn test_prover_handle_invalid_presentation_message() {
            let _setup = SetupAriesMocks::init();
            set_config_value(settings::MOCK_INDY_PROOF_VALIDATION, "false");

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(VcxStateType::VcxStateAccepted as u32, verifier_sm.state());
            assert_eq!(Status::Failed(ProblemReport::create()).code(), verifier_sm.presentation_status());
        }

        //    #[test]
        //    fn test_prover_handle_verify_presentation_message_from_presentation_request_sent_state_for_invalid_presentation() {
        //        let _setup = Setup::init();
        //
        //        let mut verifier_sm = _verifier_sm();
        //        verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
        //        verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();
        //
        //        assert_match!(VerifierState::Finished(_), verifier_sm.state);
        //        assert_eq!(Status::Failed(_problem_report()).code(), verifier_sm.presentation_status());
        //    }

        #[test]
        fn test_prover_handle_presentation_proposal_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationProposalReceived(_presentation_proposal())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(Status::Failed(_problem_report()).code(), verifier_sm.presentation_status());
        }

        #[test]
        fn test_prover_handle_presentation_reject_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(Status::Failed(_problem_report()).code(), verifier_sm.presentation_status());
        }

        #[test]
        fn test_prover_handle_other_messages_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();

            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            assert_match!(VerifierState::PresentationRequestSent(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_messages_from_presentation_finished_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(VerifierState::Finished(_), verifier_sm.state);

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationProposalReceived(_presentation_proposal())).unwrap();
            assert_match!(VerifierState::Finished(_), verifier_sm.state);
        }
    }

    mod find_message_to_handle {
        use super::*;

        #[test]
        fn test_verifier_find_message_to_handle_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_verifier_find_message_to_handle_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm().to_presentation_request_sent_state();

            // Presentation
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_2", uid);
                assert_match!(A2AMessage::Presentation(_), message);
            }

            // Presentation Proposal
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_2", uid);
                assert_match!(A2AMessage::PresentationProposal(_), message);
            }

            // Problem Report
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_3".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // No messages for different Thread ID
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal().set_thread_id("")),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation().set_thread_id("")),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack().set_thread_id("")),
                    "key_4".to_string() => A2AMessage::CommonProblemReport(_problem_report().set_thread_id(""))
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_verifier_find_message_to_handle_from_finished_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm().to_finished_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }
    }

    mod get_state {
        use super::*;

        #[test]
        fn test_get_state() {
            let _setup = SetupAriesMocks::init();

            assert_eq!(VcxStateType::VcxStateInitialized as u32, _verifier_sm().state());
            assert_eq!(VcxStateType::VcxStateOfferSent as u32, _verifier_sm().to_presentation_request_sent_state().state());
            assert_eq!(VcxStateType::VcxStateAccepted as u32, _verifier_sm().to_finished_state().state());
        }
    }
}