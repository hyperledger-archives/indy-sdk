use v3::messages::error::ProblemReport;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresentationState {
    Undefined,
    Verified,
    Invalid(ProblemReport),
}

impl PresentationState {
    pub fn state(&self) -> u32 {
        match self {
            PresentationState::Undefined => 0,
            PresentationState::Verified => 1,
            PresentationState::Invalid(_) => 2,
        }
    }
}