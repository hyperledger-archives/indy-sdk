use v3::messages::MessageId;

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
    pub cred_def_id: String,
    pub credential_json: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
}

impl IssuerState {
    pub fn get_connection_handle(&self) -> u32 {
        match self {
            IssuerState::Initial(state) => 0,
            IssuerState::OfferSent(state) => state.connection_handle,
            IssuerState::CredentialSent(state) => state.connection_handle,
            IssuerState::Finished(state) => 0
        }
    }

    pub fn get_last_id(&self) -> Option<String> {
        match self {
            IssuerState::Initial(state) => None,
            IssuerState::OfferSent(state) => Some(state.last_id.0),
            IssuerState::CredentialSent(state) => Some(state.last_id.0),
            IssuerState::Finished(state) => None
        }
    }
}

impl InitialState {
    pub fn new(cred_def_id: &str, credential_json: &str, rev_reg_id: Option<String>, tails_file: Option<String>) -> Self {
        InitialState {
            cred_def_id: cred_def_id.to_string(),
            credential_json: credential_json.to_string(),
            rev_reg_id,
            tails_file
        }
    }
}

#[derive(Debug)]
pub struct OfferSentState {
    pub offer: String,
    pub cred_data: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    pub connection_handle: u32,
    pub last_id: MessageId
}

#[derive(Debug)]
pub struct CredentialSentState {
    pub connection_handle: u32,
    pub last_id: MessageId
}

#[derive(Debug)]
pub struct FinishedState {
    pub cred_id: Option<String>
}

impl From<(InitialState, String, u32, MessageId)> for OfferSentState {
    fn from((state, offer, connection_handle, sent_id): (InitialState, String, u32, MessageId)) -> Self {
        trace!("SM is now in OfferSent state");
        OfferSentState {
            offer,
            cred_data: state.credential_json,
            rev_reg_id: state.rev_reg_id,
            tails_file: state.tails_file,
            connection_handle,
            last_id: sent_id
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

impl From<(OfferSentState, MessageId)> for CredentialSentState {
    fn from((state, sent_id): (OfferSentState, MessageId)) -> Self {
        trace!("SM is now in CredentialSent state");
        CredentialSentState {
            connection_handle: state.connection_handle,
            last_id: sent_id
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
    Initial(InitialHolderState),
    RequestSent(RequestSentState),
    Finished(FinishedHolderState)
}

#[derive(Debug)]
pub struct RequestSentState {
    pub req_meta: String,
    pub cred_def_json: String,

}

#[derive(Debug)]
pub struct InitialHolderState {}

impl InitialHolderState {
    pub fn new() -> Self {
        InitialHolderState {}
    }
}


#[derive(Debug)]
pub struct FinishedHolderState {
    pub cred_id: Option<String>
}

impl From<(InitialHolderState, String, String)> for RequestSentState {
    fn from((state, req_meta, cred_def_json): (InitialHolderState, String, String)) -> Self {
        trace!("SM is now in RequestSent state");
        RequestSentState {
            req_meta,
            cred_def_json
        }
    }
}

impl From<(RequestSentState, Option<String>)> for FinishedHolderState {
    fn from((_, cred_id): (RequestSentState, Option<String>)) -> Self {
        trace!("SM is now in Finished state");
        FinishedHolderState {
            cred_id
        }
    }
}

impl From<InitialHolderState> for FinishedHolderState {
    fn from (_state: InitialHolderState) -> Self {
        trace!("SM is now in Finished state");
        FinishedHolderState {
            cred_id: None
        }
    }
}