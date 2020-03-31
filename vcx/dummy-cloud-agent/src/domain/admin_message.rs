use indyrs::WalletHandle;

// --------
// Requests
// --------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdminQuery {
    GetActorOverview,
    GetDetailForwardAgents,
    GetDetailAgent(GetDetailAgentParams),
    GetDetailAgentConnection(GetDetailAgentConnParams),
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
    Agent(ResQueryAgent),
    AgentConn(ResQueryAgentConn),
}

// --------
// Responses
// --------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResQueryAdmin {
    pub forward_agent_connections: Vec<String>,
    pub agents: Vec<String>,
    pub agent_connections: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResQueryForwardAgent {
    pub wallet_handle: WalletHandle,
    #[serde(rename = "forwardAgentEndpoint")]
    pub endpoint: String,
    pub pairwise_list:Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResQueryAgent {
    pub owner_did: String,
    pub owner_verkey: String,
    pub did: String,
    pub verkey: String,
    pub configs: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResQueryAgentConn {
    pub owner_did: String,
    pub owner_verkey: String,
    pub user_pairwise_did: String,
    pub user_pairwise_verkey: String,
    pub agent_pairwise_did: String,
    pub agent_pairwise_verkey: String,

    pub name: String,
    pub logo: String,
    pub agent_configs: Vec<(String, String)>,

    pub remote_agent_detail_verkey: String,
    pub remote_agent_detail_did: String,
    pub remote_forward_agent_detail_verkey: String,
    pub remote_forward_agent_detail_did: String,
    pub remote_forward_agent_detail_endpoint: String,

}