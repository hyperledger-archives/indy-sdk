#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub pw_did: String,
    pub pw_vk: String,
    pub agent_did: String,
    pub agent_vk: String
}

impl Default for AgentInfo {
    fn default() -> AgentInfo {
        AgentInfo{
            pw_did: String::new(),
            pw_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }
}