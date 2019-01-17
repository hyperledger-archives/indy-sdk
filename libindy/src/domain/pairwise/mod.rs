use named_type::NamedType;

#[derive(Serialize, Deserialize, NamedType)]
pub struct Pairwise {
    pub my_did: String,
    pub their_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PairwiseInfo {
    pub my_did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl From<Pairwise> for PairwiseInfo {
    fn from(pairwise: Pairwise) -> Self {
        PairwiseInfo {
            my_did: pairwise.my_did,
            metadata: pairwise.metadata
        }
    }
}