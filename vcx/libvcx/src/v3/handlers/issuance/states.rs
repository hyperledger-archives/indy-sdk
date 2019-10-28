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
pub struct InitialState {
}

impl InitialState {
    pub fn new() -> Self {
        InitialState {}
    }
}

#[derive(Debug)]
pub struct OfferSentState {
    pub offer: String,
    pub cred_data: String
}

#[derive(Debug)]
pub struct CredentialSentState {
}

#[derive(Debug)]
pub struct FinishedState {
    pub cred_id: Option<String>
}

impl From<(InitialState, String, String)> for OfferSentState {
    fn from((state, offer, cred_data): (InitialState, String, String)) -> Self {
        trace!("SM is now in OfferSent state");
        OfferSentState {
            offer,
            cred_data
        }
    }
}

impl From<InitialState> for FinishedState {
    fn from(state: InitialState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {
            cred_id: None
        }
    }
}

impl From<OfferSentState> for CredentialSentState {
    fn from(state: OfferSentState) -> Self {
        trace!("SM is now in CredentialSent state");
        CredentialSentState {}
    }

}

impl From<OfferSentState> for FinishedState {
    fn from(_state: OfferSentState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {
            cred_id: None
        }
    }
}

impl From<CredentialSentState> for FinishedState {
    fn from(_state: CredentialSentState) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {
            cred_id: None
        }
    }
}

#[derive(Debug)]
pub enum HolderState {
    Initial(InitialState),
    RequestSent(RequestSentState),
    Finished(FinishedState)
}

#[derive(Debug)]
pub struct RequestSentState {
    pub req_meta: String,
    pub cred_def_json: String,

}

impl From<(InitialState, String, String)> for RequestSentState {
    fn from((state, req_meta, cred_def_json): (InitialState, String, String)) -> Self {
        trace!("SM is now in RequestSent state");
        RequestSentState {
            req_meta,
            cred_def_json
        }
    }
}

impl From<(RequestSentState, Option<String>)> for FinishedState {
    fn from((_, cred_id): (RequestSentState, Option<String>)) -> Self {
        trace!("SM is now in Finished state");
        FinishedState {
            cred_id
        }
    }
}