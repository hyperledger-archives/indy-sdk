// --------
// Requests
// --------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdminQuery {
    GetActorOverview,
    GetDetailForwardAgents,
    GetDetailAgent(GetDetailAgentParams),
    GetDetailForwardAgentConnection,
    GetDetailAgentConnection(GetDetailAgentConnParams),
    GetDetailRouter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDetailAgentParams {
    pub agent_did: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDetailAgentConnParams {
    pub agent_pairwise_did: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResAdminQuery {
    Admin(ResQueryAdmin),
    ForwardAgent(ResQueryForwardAgent),
    Router,
    ForwardAgentConn,
    Agent(ResQueryAgent),
    AgentConn(ResQueryAgentConn),
}

// --------
// Responses
// --------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResQueryAdmin {
    #[serde(rename = "forwardAgentConnections")]
    pub forward_agent_connections: Vec<String>,
    #[serde(rename = "agents")]
    pub agents: Vec<String>,
    #[serde(rename = "agentConnections")]
    pub agent_connections: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResQueryForwardAgent {
    #[serde(rename = "walletHandle")]
    pub wallet_handle: i32,
    #[serde(rename = "forwardAgentEndpoint")]
    pub endpoint: String,
    #[serde(rename = "pairwiseList")]
    pub pairwise_list:Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResQueryAgent {
    #[serde(rename = "ownerDid")]
    pub owner_did: String,
    #[serde(rename = "ownerVerkey")]
    pub owner_verkey: String,
    pub did: String,
    pub verkey: String,
    pub configs: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResQueryAgentConn {
    #[serde(rename = "agentDetailVerkey")]
    pub agent_detail_verkey: String,
    #[serde(rename = "agentDetailDid")]
    pub agent_detail_did: String,
    #[serde(rename = "forwardAgentDetailVerkey")]
    pub forward_agent_detail_verkey: String,
    #[serde(rename = "forwardAgentDetailDid")]
    pub forward_agent_detail_did: String,
    #[serde(rename = "forwardAgentDetailEndpoint")]
    pub forward_agent_detail_endpoint: String,
    #[serde(rename = "agentConfigs")]
    pub agent_configs: Vec<(String, String)>,
    pub name: String,
    pub logo: String
}