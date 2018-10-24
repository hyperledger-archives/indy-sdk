#[derive(Deserialize, Debug)]
pub struct Pairwise {
    pub my_did: String,
    pub their_did: String,
    pub metadata: String,
}

#[derive(Deserialize, Debug)]
pub struct PairwiseInfo {
    pub my_did: String,
    pub metadata: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ForwardAgentConnectionState {
    pub is_signed_up: bool,
    pub registrations: Vec<(String, String, String)>,
}