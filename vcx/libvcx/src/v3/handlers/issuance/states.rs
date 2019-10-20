// Possible Transitions:
// Initial -> OfferSent
// Initial -> Finished
// OfferSent -> CredentialSent
// OfferSent -> Finished
// CredentialSent -> Finished
#[derive(Debug)]
enum IssuerState {
    Initial(InitialState),
    OfferSent(OfferSentState),
    CredentialSent(CredentialSentState),
    Finished(FinishedState)
}

struct InitialState {}

struct OfferSentState {}

struct CredentialSentState {}

struct FinishedState {}

impl From<InitialState> for OfferSentState {
    fn from(_state: InitialState) -> Self {
        OfferSentState {}
    }
}

impl From<InitialState> for FinishedState {
    fn from(_state: InitialState) -> Self {
        FinishedState {}
    }
}

impl From<OfferSentState> for CredentialSentState {
    fn from(_state: OfferSentState) -> Self {
        CredentialSentState {}
    }

}

impl From<OfferSentState> for FinishedState {
    fn from(_state: InitialState) -> Self {
        FinishedState {}
    }
}

impl From<CredentialSentState> for FinishedState {
    fn from(_state: InitialState) -> Self {
        FinishedState {}
    }
}