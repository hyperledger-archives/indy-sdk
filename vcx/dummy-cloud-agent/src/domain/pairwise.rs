#[derive(Deserialize, Serialize, Debug)]
pub struct ForwardAgentConnectionState {
    pub is_signed_up: bool,
    pub registrations: Vec<(String, String, String)>,
}