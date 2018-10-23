use domain::status::MessageStatusCode;
use domain::key_deligation_proof::KeyDlgProof;

#[derive(Debug, Deserialize, Serialize)]
pub struct InviteDetail {
    #[serde(rename = "connReqId")]
    pub conn_req_id: String,
    #[serde(rename = "targetName")]
    pub target_name: Option<String>,
    #[serde(rename = "senderAgencyDetail")]
    pub sender_agency_detail: AgentDetail,
    #[serde(rename = "senderDetail")]
    pub sender_detail: SenderDetail,
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    #[serde(rename = "statusMsg")]
    pub status_msg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentDetail {
    pub did: String,
    pub verkey: String,
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderDetail {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
    #[serde(rename = "agentKeyDlgProof")]
    pub agent_key_dlg_proof: KeyDlgProof,
    pub name: Option<String>,
    #[serde(rename = "logoUrl")]
    pub logo_url: Option<String>,
}