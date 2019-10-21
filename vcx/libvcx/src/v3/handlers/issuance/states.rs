// Possible Transitions:
// Initial -> OfferSent
// Initial -> Finished
// OfferSent -> CredentialSent
// OfferSent -> Finished
// CredentialSent -> Finished
#[derive(Debug)]
pub enum IssuerState {
    Initial(InitialState),
    OfferSent(OfferSentState),
    CredentialSent(CredentialSentState),
    Finished(FinishedState)
}

#[derive(Debug)]
pub struct InitialState {}

#[derive(Debug)]
pub struct OfferSentState {}

#[derive(Debug)]
pub struct CredentialSentState {}

#[derive(Debug)]
pub struct FinishedState {}

impl From<InitialState> for OfferSentState {
    fn from(_state: InitialState) -> Self {
        trace!("SM is now in OfferSent state");
        OfferSentState {}
    }
}

impl From<InitialState> for FinishedState {
    fn from(_state: InitialState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {}
    }
}

impl From<OfferSentState> for CredentialSentState {
    fn from(_state: OfferSentState) -> Self {
        trace!("SM is now in CredentialSent state");
        CredentialSentState {}
    }

}

impl From<OfferSentState> for FinishedState {
    fn from(_state: OfferSentState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {}
    }
}

impl From<CredentialSentState> for FinishedState {
    fn from(_state: CredentialSentState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {}
    }
}

#[derive(Debug)]
pub enum HolderState {
    Initial(InitialState),
    RequestSent(RequestSentState),
    Finished(FinishedState)
}

#[derive(Debug)]
pub struct RequestSentState {}

impl From<InitialState> for RequestSentState {
    fn from(_: InitialState) -> Self {
        trace!("SM is now in RequestSent state");
        RequestSentState{}
    }
}

impl From<RequestSentState> for FinishedState {
    fn from(_: RequestSentState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {}
    }
}