use super::constants::NODE;

#[derive(Serialize, PartialEq, Debug)]
pub struct NodeOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    pub data: NodeOperationData
}

impl NodeOperation {
    pub fn new(dest: String, data: NodeOperationData) -> NodeOperation {
        NodeOperation {
            _type: NODE.to_string(),
            dest,
            data
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub enum Services {
    VALIDATOR,
    OBSERVER
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct NodeOperationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_port: Option<i32>,
    pub alias: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Services>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blskey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blskey_pop: Option<String>
}
