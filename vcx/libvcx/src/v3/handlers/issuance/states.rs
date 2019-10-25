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
    pub connection_handle: u32,
}

impl InitialState {
    pub fn new(connection_handle: u32) -> Self {
        InitialState {
            connection_handle
        }
    }
}

#[derive(Debug)]
pub struct OfferSentState {
    pub connection_handle: u32,
}

#[derive(Debug)]
pub struct CredentialSentState {
    pub connection_handle: u32,
}

#[derive(Debug)]
pub struct FinishedState {
    pub cred_id: Option<String>
}

impl From<InitialState> for OfferSentState {
    fn from(state: InitialState) -> Self {
        trace!("SM is now in OfferSent state");
        OfferSentState {
            connection_handle: state.connection_handle
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
        CredentialSentState {
            connection_handle: state.connection_handle
        }
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
    pub connection_handle: u32,
    pub req_meta: String
}

impl From<(InitialState, String)> for RequestSentState {
    fn from((state, req_meta): (InitialState, String)) -> Self {
        trace!("SM is now in RequestSent state");
        RequestSentState {
            connection_handle: state.connection_handle,
            req_meta
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