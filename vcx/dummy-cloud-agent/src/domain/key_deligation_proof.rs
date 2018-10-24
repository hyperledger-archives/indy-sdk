#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyDlgProof {
    #[serde(rename = "agentDID")]
    pub agent_did: String,
    #[serde(rename = "agentDelegatedKey")]
    pub agent_delegated_key: String,
    pub signature: String,
}

impl KeyDlgProof {
    pub fn challenge(&self) -> String {
        format!("{}{}", self.agent_did, self.agent_delegated_key)
    }
}